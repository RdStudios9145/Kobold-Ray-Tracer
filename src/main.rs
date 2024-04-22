// #[hot_lib_reloader::hot_module(dylib = "lib")]
// mod hot {
//     hot_functions_from_file!("lib/src/lib.rs");
//     pub use lib::*;
// }
// 
// use hot::*;
use std::{ fs, time::Duration };
// use hot::camera::*;
use lib::*;
use lib::camera::*;

const PI: f32 = 3.14159265358979323;

#[derive(Default)]
struct Game {
    mouse_locked: bool,
    time: Duration,
    cursor_pos: (f64, f64),
    frames: u32,
}

impl Program for Game {
    fn init(&mut self, data: &mut context::Context) {
        let scene = create_scene();
        data.scenes = vec![scene];
        data.camera = camera::Camera::new(800. / 600.);
        data.camera.translate(vec3(0.0, 0.0, 5.0));
    }

    fn on_event(&mut self, ev: WindowEvent, context: &mut context::Context) {
        match ev {
            WindowEvent::Key(Key::W, _, _, _) => {
                let mut vec = context.camera.orientation.to_matrix3() * vec3(0.0, 0.0, -1.0);
                vec.y = 0.;
                context.camera.translate(vec);
            },
            WindowEvent::Key(Key::S, _, _, _) => {
                let mut vec = context.camera.orientation.to_matrix3() * vec3(0.0, 0.0, 1.0);
                vec.y = 0.;
                context.camera.translate(vec);
            },
            WindowEvent::Key(Key::A, _, _, _) => {
                let mut vec = context.camera.orientation.to_matrix3() * vec3(-1.0, 0.0, 0.0);
                vec.y = 0.;
                context.camera.translate(vec);
            },
            WindowEvent::Key(Key::D, _, _, _) => {
                let mut vec = context.camera.orientation.to_matrix3() * vec3(1.0, 0.0, 0.0);
                vec.y = 0.;
                context.camera.translate(vec);
            },
            WindowEvent::Key(Key::Space, _, _, _) => {
                context.camera.translate(vec3(0.0, 1.0, 0.0));
            },
            WindowEvent::Key(Key::LeftShift, _, _, _) => {
                context.camera.translate(vec3(0.0, -1.0, 0.0));
            },
            WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                if self.mouse_locked {
                    context.window.set_cursor_mode(CursorMode::Normal);
                } else {
                    context.window.set_should_close(true);
                }

                self.mouse_locked = false;
            },
            WindowEvent::MouseButton(MouseButtonLeft, Action::Release, _) => {
                if !self.mouse_locked {
                    context.window.set_cursor_mode(CursorMode::Disabled);
                }

                self.mouse_locked = true;
            },
            WindowEvent::CursorPos(x, y) => {
                let delta: (f32, f32) = (self.cursor_pos.0 as f32 - x as f32, self.cursor_pos.1 as f32 - y as f32);
                self.cursor_pos = (x, y);

                if self.mouse_locked {
                    context.camera.rotate(quaternion::Quaternion::from_euler(0., delta.0 / 1800. * PI, 0.));
                    let vec = context.camera.orientation.to_matrix3() * vec3(1.0, 0.0, 0.0);
                    context.camera.rotate(quaternion::Quaternion::from_two(delta.1 / 1800. * PI, vec));
                }
            },
            WindowEvent::Key(Key::F12, _, _, _) => {

            },
            WindowEvent::Size(w, h) => {
                let cam = context.camera;
                context.camera = camera::Camera::new(w as f32 / h as f32);
                context.camera.orientation = cam.orientation;
                context.camera.translate(cam.position);
            }
            _ => {}
        };
    }

    fn on_update(&mut self, delta: Duration, _context: &mut context::Context) {
        self.time += delta;

        if self.time.as_secs() >= 1 {
            println!("FPS: {}", self.frames);
            self.frames = 0;
            self.time = Duration::from_secs(0);
        }
        self.frames += 1;
    }
}

fn main() {
    let width = 256;
    let height = 256;

    let mut buff = format!("P3\n{width} {height}\n255\n");

    for i in 0..width {
        for j in 0..height {
            buff += format!("{} {} 0\n", 255. * j as f32 / (width as f32 - 1.), 255. * i as f32 / (width as f32 - 1.)).as_str();
        }
    }
    fs::write("test.ppm", buff).unwrap();
    let mut app = App::new();
    let mut game = Game::default();
    app.run(&mut game, "Test Game!", (800, 600));
}

fn create_scene() -> scene::Scene {
    let mut scene = scene::Scene::new();

    // scene.add_object(object::Object::new_sphere(vec3(0., 0., 0.), vec3(2., 2., 2.)));
    scene.add_object(object::Object::new_sphere(vec3(0., 0., 0.), 1.));
    // scene.add_object(object::Object::new_sphere(vec3(0., 0., 0.), 2.));
    scene.add_object(object::Object::new(
        vec![[0.0, 1.0, -10.0], [1.0, 1.0, -10.0], [1.0, 0.0, -10.0]],
        vec![[0, 1, 2]],
    ));

    scene.set_refresh_color((0.2, 0.3, 0.3, 1.0));

    scene
}
