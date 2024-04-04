// #[hot_lib_reloader::hot_module(dylib = "lib")]
// mod hot {
//     hot_functions_from_file!("lib/src/lib.rs");
//     pub use lib::*;
// }
// 
// use hot::*;
use std::time::Duration;
// use hot::camera::*;
use lib::*;
use lib::camera::*;

const PI: f64 = 3.14159265358979323;

#[derive(Default)]
struct Game {
    mouse_locked: bool,
    time: Duration,
    cursor_pos: (f64, f64),
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
                context.camera.translate(vec3(0.0, 0.0, -1.0));
            },
            WindowEvent::Key(Key::S, _, _, _) => {
                context.camera.translate(vec3(0.0, 0.0, 1.0));
            },
            WindowEvent::Key(Key::A, _, _, _) => {
                context.camera.translate(vec3(-1.0, 0.0, 0.0));
            },
            WindowEvent::Key(Key::D, _, _, _) => {
                context.camera.translate(vec3(1.0, 0.0, 0.0));
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
                let delta = (self.cursor_pos.0 - x, self.cursor_pos.1 - y);
                self.cursor_pos = (x, y);

                if self.mouse_locked {
                    context.camera.rotate(quaternion::Quaternion::from_euler(0., (delta.0 / 100.0 / PI) as f32, 0.));
                    let vec = context.camera.orientation.to_matrix3() * vec3(1.0, 0.0, 0.0);
                    context.camera.rotate(quaternion::Quaternion::from_two(-(delta.1 / 100.0 / PI) as f32, vec));
                }

                println!("{:?}, {:?}", delta, context.camera.orientation);
                // println!("{:?}", context.camera.orientation.to_matrix());
            }
            _ => {}
        };
    }

    fn on_update(&mut self, delta: Duration, data: &mut context::Context) {
        self.time += delta;
        let secs = self.time.as_millis() as f32 / 1000.0;
        // data.camera.orientation = quaternion::Quaternion::from_euler(0.0, 0.0, secs);
    }
}

fn main() {
    let q1 = quaternion::Quaternion::from_two((0.01 / PI) as f32, vec3(0., 1., 0.));
    let q2 = q1 * q1;
    println!("1: {:?}, 2: {:?}", q1, q2);
    // return;

    let mut app = App::new();
    let mut game = Game::default();
    app.run(&mut game, "Test Game!", (800, 600));
}

fn create_scene() -> scene::Scene {
    let mut scene = scene::Scene::new();

    let verts = &vec![
        [-1.0,  1.0, -1.0],
        [ 1.0,  1.0, -1.0],
        [ 1.0, -1.0, -1.0],
        [-1.0, -1.0, -1.0],
        [-1.0,  1.0,  1.0],
        [ 1.0,  1.0,  1.0],
        [ 1.0, -1.0,  1.0],
        [-1.0, -1.0,  1.0],
    ];
    let indicies = &vec![[0, 1, 3], [1, 2, 3], [0, 7, 4], [0, 3, 7], [5, 4, 6], [4, 7, 6], [1, 5, 2], [5, 6, 2], [4, 5, 0], [5, 1, 0], [3, 2, 7], [2, 6, 7]];
    scene.add_object(object::Object::new(verts.to_vec(), indicies.to_vec()));
    scene.add_object(object::Object::new(
        vec![[0.0, 1.0, -10.0], [1.0, 1.0, -10.0], [1.0, 0.0, -10.0]],
        vec![[0, 1, 2]],
    ));

    scene.set_refresh_color((0.2, 0.3, 0.3, 1.0));

    scene
}
