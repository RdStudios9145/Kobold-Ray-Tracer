use std::{f32::consts::PI, time::Duration};

use lib::{
    glfw,
    glm::{vec3, Vec3, Vec4},
    App, Primitive, Quaternion, Scene, Window, WindowOptions,
};

use std::sync::{Arc, Mutex};

fn main() {
    let mut app = App::new(vec![]);

    app.create_window(WindowOptions {
        width: 800,
        height: 600,
        scene: 0,
        title: String::from("test #1"),
    });

    generate_scenes(&mut app);

    // app.create_window(WindowOptions {
    //     width: 900,
    //     height: 700,
    //     scene: 1,
    //     title: String::from("Test #2"),
    // });

    app.run()
}

fn generate_scenes(app: &mut App) {
    let mut scene = Scene::new(600. / 800.);

    scene.add_object(
        Primitive::SPHERE,
        Vec3::new(0., 5., 0.),
        Vec3::new(1., 1., 1.),
        Quaternion::from_euler(0., 0., 0.),
        Vec4::new(1., 1., 1., 1.),
    );

    scene.add_object(
        Primitive::SPHERE,
        Vec3::new(0., -5., 0.),
        Vec3::new(1., 1., 1.),
        Quaternion::from_euler(0., 0., 0.),
        Vec4::new(1., 0., 0., 1.),
    );

    scene.camera.translate(vec3(0., 0., 5.));
    scene.camera.rotate(Quaternion::from_euler(0., 0.1, 0.));

    scene.set_clear_color(0.2, 0.3, 0.3, 1.);
    set_listeners(&mut scene);

    app.register_scene(scene);
}

fn set_listeners(scene: &mut Scene) {
    use glfw::{Key, WindowEvent};

    let mut mouse_locked = false;
    let mut time = Duration::default();
    let mut cursor_pos = (0_f32, 0_f32);
    let mut frames = 0_u32;

    let on_event = move |window: &mut Window, scene: &mut Scene, event: WindowEvent| match event {
        WindowEvent::Key(Key::W, _, _, _) => {
            let mut vec = scene.camera.orientation.as_matrix3() * vec3(0.0, 0.0, -1.0);
            vec.y = 0.0;
            scene.camera.translate(vec);
        }
        WindowEvent::Key(Key::S, _, _, _) => {
            let mut vec = scene.camera.orientation.as_matrix3() * vec3(0.0, 0.0, 1.0);
            vec.y = 0.0;
            scene.camera.translate(vec);
        }
        WindowEvent::Key(Key::A, _, _, _) => {
            let mut vec = scene.camera.orientation.as_matrix3() * vec3(-1.0, 0.0, 0.0);
            vec.y = 0.0;
            scene.camera.translate(vec);
        }
        WindowEvent::Key(Key::D, _, _, _) => {
            let mut vec = scene.camera.orientation.as_matrix3() * vec3(1.0, 0.0, 0.0);
            vec.y = 0.0;
            scene.camera.translate(vec);
        }
        WindowEvent::Key(Key::Space, _, _, _) => {
            scene.camera.translate(vec3(0.0, 1.0, 0.0));
        }
        WindowEvent::Key(Key::LeftShift, _, _, _) => {
            scene.camera.translate(vec3(0.0, -1.0, 0.0));
        }
        WindowEvent::Key(Key::Escape, _, glfw::Action::Release, _) => {
            if mouse_locked {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            } else {
                window.set_should_close(true);
            }

            mouse_locked = false;
        }
        WindowEvent::MouseButton(glfw::MouseButtonLeft, glfw::Action::Release, _) => {
            if !mouse_locked {
                window.set_cursor_mode(glfw::CursorMode::Disabled);
            }

            mouse_locked = true;
        }
        WindowEvent::CursorPos(x, y) => {
            let delta = (cursor_pos.0 - x as f32, cursor_pos.1 - y as f32);
            cursor_pos = (x as f32, y as f32);

            if mouse_locked {
                scene
                    .camera
                    .rotate(Quaternion::from_euler(0., delta.0 / 1800. * PI, 0.));
                let vec = scene.camera.orientation.as_matrix3() * vec3(1.0, 0.0, 0.0);
                scene
                    .camera
                    .rotate(Quaternion::from_two(delta.1 / 1800. * PI, vec));
            }
        }
        _ => {}
    };

    let on_update = move |_: &mut Window, _: &mut Scene, delta: Duration| {
        time += delta;

        if time.as_secs() >= 1 {
            println!("FPS: {}", frames);
            frames = 0;
            time = Duration::ZERO;
        }

        frames += 1;
    };

    scene.on_event = Some(Arc::new(Mutex::new(on_event)));
    scene.on_update = Some(Arc::new(Mutex::new(on_update)));
}
