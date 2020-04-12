#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::vertex::VertexBufferAny;
use glium::Display;
use crate::support;
use crate::support::camera::Mode;
use crate::support::{rotation_matrix, multiply_matrix, scale_matrix};

use glutin::dpi::PhysicalPosition;
use glutin::event::*;

#[derive(Copy, Clone)]
pub struct VoxelVertex {
    pub position: [f32; 4],
    pub colour: u32
}
implement_vertex!(VoxelVertex, position, colour);

struct State {
    r: f32,
    th: f32,
    phi: f32,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_in: bool,
    moving_out: bool,

    mouse_down: bool,
    last_mouse: Option<PhysicalPosition<f64>>
}

pub fn run<F>(build_display_list: F)
where F: Fn(&Display) -> VertexBufferAny {

    // building the display, ie. the main object
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // building the vertex and index buffers
    let vertex_buffer = build_display_list(&display);

    // the program
    let program = program!(&display,
        330 => {
            vertex: "
                #version 330 core

                layout(location = 0) in vec4 position;
                layout(location = 1) in uint colour;

                // The usual matrices are provided
                uniform mat4 projectionMatrix;
                uniform mat4 viewMatrix;
                uniform mat4 modelMatrix;

                vec3 decodeColor(uint quantizedColor)
                {
                    float blue = (quantizedColor & 0xffu);
                    quantizedColor >>= 8;
                    float green = (quantizedColor & 0xffu);
                    quantizedColor >>= 8;
                    float red = (quantizedColor & 0xffu);

                    vec3 result = vec3(red, green, blue);
                    result *= (1.0 / 255.0);
                    return result;
                }

                out vec4 worldPosition;
                out vec3 voxelColor;

                void main()
                {
                    // Extract and decode the color.
                    voxelColor = decodeColor(colour);
                    // Standard sequence of OpenGL transformations.
                    worldPosition = modelMatrix * position;
                    vec4 cameraPosition = viewMatrix * worldPosition;
                    gl_Position = projectionMatrix * cameraPosition;
                }
            ",

            fragment: "
                #version 330 core

                // Interpolated values from the vertex shaders
                in vec3 voxelColor;
                in vec4 worldPosition;

                // Output data
                out vec3 outputColor;

                void main()
                {
                    vec3 worldSpaceNormal = normalize(cross(dFdy(worldPosition.xyz), dFdx(worldPosition.xyz)));
                    worldSpaceNormal *= -1.0; // Not sure why we have to invert this... to be checked.

                    // Basic lighting calculation for overhead light.
                    float ambient = 0.5;
                    float diffuse = 0.7;
                    vec3 lightDir = normalize(vec3(0.2, 0.8, 0.4));
                    float nDotL = clamp(dot(normalize(worldSpaceNormal), lightDir), 0.0, 1.0);
                    float lightIntensity = ambient + diffuse * nDotL;

                    outputColor = voxelColor * lightIntensity;
                }
            ",
        },
    ).unwrap();

    //
    let mut camera = support::camera::CameraState::new(Mode::Cartesian);
    let (cx, cy, cz) = camera.get_position();

    // point the camera back at the origin
    camera.set_look_direction((-cx, -cy, -cz));
    camera.update();

    let mut state = State{ r: 1.0, th: 0.0, phi: 0.0, moving_up: false, moving_left: false, moving_down: false, moving_right: false, moving_in: false, moving_out: false, mouse_down: false, last_mouse: None };

    // the main loop
    support::start_loop(event_loop, move |events| {
        // building the uniforms
        let uniforms = uniform! {
            projectionMatrix: camera.get_perspective(),
            viewMatrix: camera.get_view(),
            modelMatrix: state.matrix(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        for event in events {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = support::Action::Stop,
                    ev => state.process_input(&ev),
                },
                _ => (),
            }
        };

        action
    });
}

impl State {
    fn matrix(&self) -> [[f32; 4]; 4] {
        let m1 = rotation_matrix([1.0, 0.0, 0.0], self.th);
        let m2 = rotation_matrix([0.0, 0.0, 1.0], self.phi);
        multiply_matrix(multiply_matrix(m1, m2), scale_matrix(self.r))
    }

    fn process_input(&mut self, event: &glutin::event::WindowEvent) {
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            glutin::event::WindowEvent::MouseWheel { delta, ..}=> {

                match delta {
                    glutin::event::MouseScrollDelta::LineDelta(_x, y) => {
                        self.r += y * 0.1;
                    },
                    _ => ()
                }
                return;
            },

            glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    self.mouse_down = state == ElementState::Pressed;
                }
                return;
            },
            glutin::event::WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_down {
                    if let Some(pos) = self.last_mouse {
                        self.th += ((position.y - pos.y) * 0.01) as f32;
                        self.phi += ((position.x - pos.x) * 0.01) as f32;
                    }

                    self.last_mouse = Some(position);
                } else {
                    self.last_mouse = None
                }
                return;
            },
            _ => return,
        };
        let pressed = input.state == glutin::event::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            glutin::event::VirtualKeyCode::Up => self.moving_up = pressed,
            glutin::event::VirtualKeyCode::Down => self.moving_down = pressed,
            glutin::event::VirtualKeyCode::Left => self.moving_left = pressed,
            glutin::event::VirtualKeyCode::Right => self.moving_right = pressed,
            glutin::event::VirtualKeyCode::PageUp => self.moving_in = pressed,
            glutin::event::VirtualKeyCode::PageDown => self.moving_out = pressed,
            _ => (),
        };

        if self.moving_right {
            self.phi += 0.01;
        }
        if self.moving_left {
            self.phi -= 0.01;
        }
        if self.moving_up {
            self.th += 0.01;
        }
        if self.moving_down {
            self.th -= 0.01;
        }
        if self.moving_out {
            self.r += 0.25;
        }
        if self.moving_in {
            self.r -= 0.25;
        }
    }
}