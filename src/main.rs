#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot {
    hot_functions_from_file!("lib/src/lib.rs");
    pub use lib::*;
}

fn main() {
    let mut app = hot::App::new("Window Test".to_string(), (800, 600));
    let mut scene = hot::scene::Scene::new();
    // scene.add_object(hot::object::Object::new(hot::object::ObjectType::Sphere {
    //     center: hot::point::Point(1, 1, 1),
    //     radius: 1,
    // }));
    let verts = &vec![
        [-0.5,  0.5, -9.0],
        [ 0.5,  0.5, -9.0],
        [ 0.5, -0.5, -9.0],
        [-0.5, -0.5, -9.0],
    ];
    let indicies = &vec![[0, 1, 3], [1, 2, 3]];
    scene.add_object(hot::object::Object::new(verts.to_vec(), indicies.to_vec()));
    scene.add_object(hot::object::Object::new(
        vec![[0.0, 1.0, -10.0], [1.0, 1.0, -10.0], [1.0, 0.0, -10.0]],
        vec![[0, 1, 2]],
    ));
    scene.set_refresh_color((0.2, 0.3, 0.3, 1.0));

    let mut listeners = hot::listener::Listener::default();
    listeners.attach_on_event(move_camera);
    
    let mut context = hot::context::Context {
        scenes: vec![scene],
        current_scene: 0,
        camera: hot::camera::Camera::new(800. / 600.)
    };

    app.run(&mut context, listeners)
}

fn move_camera(context: &mut hot::context::Context, ev: hot::WindowEvent) -> &mut hot::context::Context {
    match ev {
        hot::WindowEvent::Key(hot::Key::W, _, _, _) => {
            context.camera.translate(hot::vec3(0.0, 0.0, -1.0));
        },
        hot::WindowEvent::Key(hot::Key::S, _, _, _) => {
            context.camera.translate(hot::vec3(0.0, 0.0, 1.0));
        },
        hot::WindowEvent::Key(hot::Key::A, _, _, _) => {
            context.camera.translate(hot::vec3(-1.0, 0.0, 0.0));
        },
        hot::WindowEvent::Key(hot::Key::D, _, _, _) => {
            context.camera.translate(hot::vec3(1.0, 0.0, 0.0));
        },
        _ => {}
    };
    context
}
