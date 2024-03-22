use app::App;

mod app;
mod ball;
mod physics;
mod ring;
mod sounds;
mod util;

fn main() {
    App::new("Collide and Sound").run();
}
