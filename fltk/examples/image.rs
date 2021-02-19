use fltk::{app, frame::*, image::*, window::*};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 300, "");

    let mut image = SharedImage::load("screenshots/calc.jpg")?;
    image.scale(200, 200, true, true);

    frame.set_image(Some(image));

    // // To remove an image
    // frame.set_image(None::<SharedImage>);

    wind.make_resizable(true);
    wind.show();

    app.run()?;
    Ok(())
}
