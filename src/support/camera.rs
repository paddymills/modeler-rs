
pub const UPDATE_DISTANCE: f32 = 0.01;
const UP: (f32, f32, f32) = (0.0, 1.0, 0.0);

#[derive(Debug)]
pub struct CameraState {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32), 
    direction: (f32, f32, f32),

    moving: (i8, i8, i8),
    rotating: (i8, i8, i8),
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            position: (0.1, 0.1, 1.0),
            rotation: (0.5, 1.0, 0.0),
            direction: (0.0, 0.0, -1.0),

            moving: (0, 0, 0),
            rotating: (0, 0, 0),
        }
    }

    pub fn set_aspect_ratio(&mut self, x: f32, y: f32) {
        self.aspect_ratio = x / y
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio,    0.0,                 0.0           ,   0.0],
            [         0.0         ,     f ,                 0.0           ,   0.0],
            [         0.0         ,    0.0,  (    zfar+znear)/(zfar-znear),   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let s = (
            f.1 * UP.2 - f.2 * UP.1,
            f.2 * UP.0 - f.0 * UP.2,
            f.0 * UP.1 - f.1 * UP.0
        );

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s_norm.1 * f.2 - s_norm.2 * f.1,
            s_norm.2 * f.0 - s_norm.0 * f.2,
            s_norm.0 * f.1 - s_norm.1 * f.0
        );

        let p = (
            -self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
            -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
            -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2
        );

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [     p.0, p.1, p.2, 1.0],
        ]
    }

    pub fn get_x_rotation(&self) -> [[f32; 4]; 4] {
        let r = self.rotation.0;
        [
            [1.0,      0.0,     0.0, 0.0],
            [0.0,  r.cos(), r.sin(), 0.0],
            [0.0, -r.sin(), r.cos(), 0.0],
            [0.0,      0.0,     0.0, 1.0],
        ]
    }

    pub fn get_y_rotation(&self) -> [[f32; 4]; 4] {
        let r = self.rotation.1;
        [
            [r.cos(), 0.0, -r.sin(), 0.0],
            [    0.0, 1.0,      0.0, 0.0],
            [r.sin(), 0.0,  r.cos(), 0.0],
            [    0.0, 0.0,      0.0, 1.0],
        ]
    }

    pub fn get_z_rotation(&self) -> [[f32; 4]; 4] {
        let r = self.rotation.2;
       [
            [ r.cos(), r.sin(), 0.0, 0.0],
            [-r.sin(), r.cos(), 0.0, 0.0],
            [    0.0,      0.0, 1.0, 0.0],
            [    0.0,      0.0, 0.0, 1.0],
        ]
    }

    pub fn update(&mut self, dist: f32) {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let s = (
            f.1 * UP.2 - f.2 * UP.1,
            f.2 * UP.0 - f.0 * UP.2,
            f.0 * UP.1 - f.1 * UP.0
        );

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s.1 * f.2 - s.2 * f.1,
            s.2 * f.0 - s.0 * f.2,
            s.0 * f.1 - s.1 * f.0
        );

        // remember
        //  s depends on f
        //  u depends on s, and therefore f

        // x-axis: left/right
        if self.moving.0 != 0 {
            self.position.0 += s.0 * dist * (self.moving.0 as f32);
            self.position.1 += s.1 * dist * (self.moving.0 as f32);
            self.position.2 += s.2 * dist * (self.moving.0 as f32);
        }

        // y-axis: front/back
        if self.moving.1 != 0 {
            self.position.0 += f.0 * dist * (self.moving.1 as f32);
            self.position.1 += f.1 * dist * (self.moving.1 as f32);
            self.position.2 += f.2 * dist * (self.moving.1 as f32);
        }

        // z-axis: up/down
        if self.moving.2 != 0 {
            self.position.0 += u.0 * dist * (self.moving.2 as f32);
            self.position.1 += u.1 * dist * (self.moving.2 as f32);
            self.position.2 += u.2 * dist * (self.moving.2 as f32);
        }

        self.rotation.0 += (self.rotating.0 as f32) * dist;
        self.rotation.1 += (self.rotating.1 as f32) * dist;
        self.rotation.2 += (self.rotating.2 as f32) * dist;
    }

    pub fn process_input(&mut self, event: &winit::event::WindowEvent) {
        use winit::{
            event::{WindowEvent, ElementState, MouseScrollDelta},
            keyboard::{PhysicalKey, KeyCode}
        };
        
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = (event.state == ElementState::Pressed) as i8;
                match &event.physical_key {
                    // movement
                    PhysicalKey::Code(KeyCode::ArrowUp)    => self.moving.2 =  pressed,
                    PhysicalKey::Code(KeyCode::ArrowDown)  => self.moving.2 = -pressed,
                    PhysicalKey::Code(KeyCode::KeyA)       => self.moving.0 = -pressed,
                    PhysicalKey::Code(KeyCode::KeyD)       => self.moving.0 =  pressed,
                    PhysicalKey::Code(KeyCode::KeyW)       => self.moving.1 =  pressed,
                    PhysicalKey::Code(KeyCode::KeyS)       => self.moving.1 = -pressed,

                    // rotation
                    PhysicalKey::Code(KeyCode::ArrowLeft)  => self.rotating.1 = -pressed,
                    PhysicalKey::Code(KeyCode::ArrowRight) => self.rotating.1 =  pressed,
                    PhysicalKey::Code(KeyCode::Digit1)     => self.rotating.0 =  pressed,
                    PhysicalKey::Code(KeyCode::Digit2)     => self.rotating.1 =  pressed,
                    PhysicalKey::Code(KeyCode::Digit3)     => self.rotating.2 =  pressed,

                    // reset rotation
                    PhysicalKey::Code(KeyCode::KeyR) => self.rotation = (0.5, 1.0, 0.0),

                    _ => (),
                }
            },
            // WindowEvent::CursorMoved { position, .. } => todo!(),
            // WindowEvent::MouseInput { button, state, .. } => todo!(),
            WindowEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (*x, *y),
                    MouseScrollDelta::PixelDelta(size) => (size.x as f32, size.y as f32)
                };

                // TODO: refactor to impl this better
                self.moving.0 = (x / x.abs()) as i8;
                self.update(x.abs());
                self.moving.0 = 0;
                self.moving.1 = (y / y.abs()) as i8;
                self.update(y.abs());
                self.moving.1 = 0;
            },
            WindowEvent::Resized(size) => self.set_aspect_ratio( size.width as f32, size.height as f32 ),
            _ => ()
        }
    }
}
