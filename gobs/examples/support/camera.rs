extern crate glutin;

pub enum Mode {
    Cartesian,
    Spherical,
}

pub struct CameraState {
    mode: Mode,
    aspect_ratio: f32,
    position: (f32, f32, f32),
    spherical_position: (f32, f32, f32),
    look_direction: (f32, f32, f32),
    forward: (f32, f32, f32),

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new(mode: Mode) -> CameraState {
        CameraState {
            mode,
            aspect_ratio: 1024.0 / 768.0,
            position: (0.1, 1.0, 1.0),
            spherical_position: (0.0, 0.0, 1.0),
            look_direction: (0.0, 0.0, -1.0),
            forward: (0.0, 0.0, -1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn get_position(&self) -> (f32, f32, f32) {
        self.position
    }

    pub fn set_look_direction(&mut self, dir: (f32, f32, f32)) {
        self.look_direction = dir;
    }

    pub fn get_look_direction(&self) -> (f32, f32, f32) {
        self.look_direction
    }

    pub fn set_forward(&mut self, dir: (f32, f32, f32)) {
        self.forward = dir;
    }

    pub fn get_forward(&self) -> (f32, f32, f32) {
        self.forward
    }
    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.look_direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (
            f.1 * up.2 - f.2 * up.1,
            f.2 * up.0 - f.0 * up.2,
            f.0 * up.1 - f.1 * up.0,
        );

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s_norm.1 * f.2 - s_norm.2 * f.1,
            s_norm.2 * f.0 - s_norm.0 * f.2,
            s_norm.0 * f.1 - s_norm.1 * f.0,
        );

        let p = (
            -self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
            -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
            -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2,
        );

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1, p.2, 1.0],
        ]
    }

    fn update_cartesian(&mut self) {
        let f = {
            let f = self.forward;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (
            f.1 * up.2 - f.2 * up.1,
            f.2 * up.0 - f.0 * up.2,
            f.0 * up.1 - f.1 * up.0,
        );

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s.1 * f.2 - s.2 * f.1,
            s.2 * f.0 - s.0 * f.2,
            s.0 * f.1 - s.1 * f.0,
        );

        if self.moving_up {
            self.position.0 += u.0 * 0.01;
            self.position.1 += u.1 * 0.01;
            self.position.2 += u.2 * 0.01;
        }

        if self.moving_left {
            self.position.0 -= s.0 * 0.01;
            self.position.1 -= s.1 * 0.01;
            self.position.2 -= s.2 * 0.01;
        }

        if self.moving_down {
            self.position.0 -= u.0 * 0.01;
            self.position.1 -= u.1 * 0.01;
            self.position.2 -= u.2 * 0.01;
        }

        if self.moving_right {
            self.position.0 += s.0 * 0.01;
            self.position.1 += s.1 * 0.01;
            self.position.2 += s.2 * 0.01;
        }

        if self.moving_forward {
            self.position.0 += f.0 * 0.01;
            self.position.1 += f.1 * 0.01;
            self.position.2 += f.2 * 0.01;
        }

        if self.moving_backward {
            self.position.0 -= f.0 * 0.01;
            self.position.1 -= f.1 * 0.01;
            self.position.2 -= f.2 * 0.01;
        }
    }

    fn update_spherical(&mut self) {
        if self.moving_up {
            self.spherical_position.2 += 0.01;
        }
        if self.moving_down {
            self.spherical_position.2 -= 0.01;
        }
        if self.moving_left {
            self.spherical_position.0 += 0.01;
        }
        if self.moving_right {
            self.spherical_position.0 -= 0.01;
        }
        if self.moving_forward {
            self.spherical_position.1 += 0.01;
        }
        if self.moving_backward {
            self.spherical_position.1 -= 0.01;
        }

        let (s0, c0) = self.spherical_position.0.sin_cos();
        let (s1, c1) = self.spherical_position.1.sin_cos();
        let r = self.spherical_position.2;

        self.position = (r * s0 * c1, r * s0 * s1, r * c0);
    }

    pub fn update(&mut self) {
        match self.mode {
            Mode::Cartesian => self.update_cartesian(),
            Mode::Spherical => self.update_spherical(),
        }
    }

    pub fn process_input(&mut self, event: &glutin::event::WindowEvent) {
        let input = match *event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
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
            glutin::event::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::event::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::event::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::event::VirtualKeyCode::S => self.moving_backward = pressed,
            _ => (),
        };
    }
}
