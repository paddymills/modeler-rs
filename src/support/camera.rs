use winit::{
    dpi::PhysicalPosition,
    event::{
        WindowEvent, ElementState, KeyboardInput, VirtualKeyCode, MouseButton, MouseScrollDelta
    }
};


pub const UPDATE_DISTANCE: f32 = 0.01;
const UP: (f32, f32, f32) = (0.0, 1.0, 0.0);

#[derive(Debug)]
pub struct CameraState {
    width: f32,
    height: f32,

    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32), 
    direction: (f32, f32, f32),

    moving: (i8, i8, i8),
    rotating: (i8, i8, i8),

    lmouse_held: bool,
    mouse_pos: PhysicalPosition<f32>,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            width: 1024.0,
            height: 768.0,

            position: (0.1, 0.1, 1.0),
            rotation: (0.5, 1.0, 0.0),
            direction: (0.0, 0.0, -1.0),

            moving: (0, 0, 0),
            rotating: (0, 0, 0),

            lmouse_held: false,
            mouse_pos: PhysicalPosition::default()
        }
    }

    fn normalize(&mut self) {
        let (min, max) = (0.0, 2.0 * 3.14159);

        let norm = |val| {
                match val {
                x if x < min => val + max,
                x if x > max => val - max,
                _ => val
            }
        };

        self.rotation.0 = norm(self.rotation.0);
        self.rotation.1 = norm(self.rotation.1);
        self.rotation.2 = norm(self.rotation.2);

        self.position.0 = self.position.0.clamp(-1.0, 1.0);
        self.position.1 = self.position.1.clamp(-1.0, 1.0);
        self.position.2 = self.position.2.clamp(-1.0, 1.0);
    }

    pub fn set_aspect_ratio(&mut self, x: f32, y: f32) {
        self.width = x;
        self.height = y;
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.width / self.height
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.get_aspect_ratio(), 0.0,                 0.0           , 0.0],
            [            0.0            ,  f ,                 0.0           , 0.0],
            [            0.0            , 0.0,  (    zfar+znear)/(zfar-znear), 1.0],
            [            0.0            , 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
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

        self.normalize();
    }

    pub fn process_input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input: KeyboardInput { state, virtual_keycode: Some(keycode), .. }, .. } => {
                let pressed = (state == &ElementState::Pressed) as i8;
                match &keycode {
                    // movement
                    VirtualKeyCode::Up   => self.moving.2 =  pressed,
                    VirtualKeyCode::Down => self.moving.2 = -pressed,
                    VirtualKeyCode::A      => self.moving.0 = -pressed,
                    VirtualKeyCode::D      => self.moving.0 =  pressed,
                    VirtualKeyCode::W      => self.moving.1 =  pressed,
                    VirtualKeyCode::S      => self.moving.1 = -pressed,

                    // rotation
                    VirtualKeyCode::Left  => self.rotating.1 = -pressed,
                    VirtualKeyCode::Right => self.rotating.1 =  pressed,
                    VirtualKeyCode::Key1  => self.rotating.0 =  pressed,
                    VirtualKeyCode::Key2  => self.rotating.1 =  pressed,
                    VirtualKeyCode::Key3  => self.rotating.2 =  pressed,

                    // reset rotation
                    VirtualKeyCode::T => self.rotation = (0.5, 1.0, 0.0),
                    VirtualKeyCode::R => self.rotation = (0.0, 0.0, 0.0),

                    _ => (),
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                if self.lmouse_held {
                    let x = (position.x as f32 - self.mouse_pos.x) / self.width;
                    let y = (position.y as f32 - self.mouse_pos.y) / self.height;

                    // TODO: refactor to impl this better
                    // TODO: rotation about axes, given current camera position
                    // x-rotation about the y-axis
                    self.rotating.1 = (x / x.abs()) as i8;
                    self.update(x.abs());
                    self.rotating.1 = 0;
                    // y-rotation about the x-axis
                    self.rotating.0 = (y / y.abs()) as i8;
                    self.update(y.abs());
                    self.rotating.0 = 0;
                }
                
                self.mouse_pos.x = position.x as f32;
                self.mouse_pos.y = position.y as f32;
            },
            WindowEvent::MouseInput { button, state, .. } => {
                match button {
                    MouseButton::Left => self.lmouse_held = state == &ElementState::Pressed,
                    _ => ()
                }
            },
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
