#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::vertex::VertexBufferAny;
use glium::Display;
use crate::support;


#[derive(Copy, Clone)]
pub struct VoxelVertex {
    pub position: [f32; 4],
    pub colour: u32
}
implement_vertex!(VoxelVertex, position, colour);

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
                    float ambient = 0.3;
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
    let mut camera = support::camera::CameraState::new();

    // the main loop
    support::start_loop(event_loop, move |events| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            projectionMatrix: camera.get_perspective(),
            viewMatrix: camera.get_view(),
            modelMatrix: support::identity_matrix(),
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
                    ev => camera.process_input(&ev),
                },
                _ => (),
            }
        };

        action
    });
}