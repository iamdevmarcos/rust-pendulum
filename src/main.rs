use speedy2d::Window;
use window_handler::MyWindowHandler;

mod vector;
mod pendulum;
mod window_handler;

fn main() {
    let window: Window = Window::new_centered("Pendulum", (800, 480)).unwrap();
    let win = MyWindowHandler {
        p: pendulum::Pendulum::new(400.0, 0.0, 200.0),
    };

    window.run_loop(win);
}
