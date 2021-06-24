use rtc::canvas::Canvas;
use rtc::color::Color;
use rtc::matrix::Matrix;
use rtc::tuple::Tuple;

use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Canvas::new(400, 400);

    let color = Color::new(1.0, 1.0, 1.0);

    let origin = Tuple::point(0.0, 0.0, 0.0);
    let offset = Matrix::translation(3.0 * canvas.width() as f64 / 8.0, 0.0, 0.0);
    let center = Matrix::translation(
        canvas.width() as f64 / 2.0,
        canvas.height() as f64 / 2.0,
        0.0,
    );

    for i in 0..12 {
        let theta = PI / 6.0;
        let rotation = Matrix::rotation_z(i as f64 * theta);

        let point = &(&center * &rotation * &offset) * origin;

        let x = point.x() as usize;
        let y = canvas.height() - point.y() as usize - 1;

        canvas[[x, y]] = color;
    }

    canvas.to_ppm(&mut std::io::stdout())?;

    Ok(())
}
