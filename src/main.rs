use lib::{App, WindowOptions};

fn main() {
    let mut app = App::new(vec![]);
    app.creat_window(WindowOptions {
        width: 800,
        height: 600,
        scene: 0,
        title: String::from("test #1"),
    });

    app.run()
}
