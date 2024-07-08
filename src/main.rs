use lib::{glm::Vec3, App, Primitive, Quaternion, Scene, WindowOptions};

fn main() {
    let mut app = App::new(vec![]);

    generate_scenes(&mut app);

    app.create_window(WindowOptions {
        width: 800,
        height: 600,
        scene: 0,
        title: String::from("test #1"),
    });

    app.create_window(WindowOptions {
        width: 900,
        height: 700,
        scene: 1,
        title: String::from("Test #2"),
    });

    app.run()
}

fn generate_scenes(app: &mut App) {
    let mut scene = Scene::new(600. / 800.);

    scene.add_object(
        Primitive::Sphere.into(),
        Vec3::zeros(),
        Vec3::new(1., 1., 1.),
        Quaternion::from_euler(0., 0., 0.),
    );

    scene.set_clear_color(0.2, 0.3, 0.3, 1.);

    app.register_scene(scene);
}
