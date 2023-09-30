use sfml::{
    graphics::{
        Color, RenderTarget, RenderWindow
    },
    window::{ContextSettings, Event, Style},
};

fn main() {
    let mut rw = RenderWindow::new(
        (800, 600),
        "Chip8",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_vertical_sync_enabled(true);

    while rw.is_open() {
        rw.clear(Color::BLACK);
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                _ => {}
            }
        }
        rw.display();
    }
}