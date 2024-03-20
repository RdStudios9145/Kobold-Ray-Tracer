# Kobold Ray Tracer

A continuation of my Kobold game engine. This one focuses on ray tracing. If this continues well, I may make the Kobold engine standalone with ray tracing an additional feature.

## Usage

examples can be found in the example folder when I clean up the project. Right now, the main application is in src and contains the useage of the Kobold engine.

---

The Kobold Engine is compiled into a dll, which the program includes and calls into. Create a `Game` struct that implements `Program`, then create a new `App` and pass in the `Game` struct.

`Program` will define empty event and update listeners, which you can override with on_update and on_event.

Example:

```rust
#[derive(Default)]
struct Game {}

impl Program for Game {
  fn on_event(...) {
    println!("Event!");
  }
}

fn main() {
  let mut game = Game { ..default() }
  App::new(&mut game, "Example Title", (800, 600)).run();
}
```
