#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot {
    hot_functions_from_file!("lib/src/lib.rs");
    pub use lib::*;
}

use hot::*;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct Game {
    mouse_locked: bool,
}

impl Program for Game {
    fn init(&mut self, data: &mut context::Context) {
        data.scenes[0].set_refresh_color((0.2, 0.3, 0.3, 1.0));
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
            WindowEvent::Key(Key::Escape, _, _, _) => {
                self.mouse_locked = false;
                
            },
            WindowEvent::MouseButton(MouseButtonLeft, Action::Release, _) => {
                self.mouse_locked = true;
            }
            _ => {}
        };
    }
}

fn main() {
    let mut app = App::new();
    let mut scene = scene::Scene::new();
    // scene.add_object(object::Object::new(object::ObjectType::Sphere {
    //     center: point::Point(1, 1, 1),
    //     radius: 1,
    // }));
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
    
    let mut context = context::Context {
        scenes: vec![scene],
        current_scene: 0,
        camera: camera::Camera::new(800. / 600.)
    };

    let mut game = Game::default();
    app.run(&mut game, context, "Test Game!", (800, 600));
}
