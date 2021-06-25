use truster::canvas::Canvas;
use truster::color::Color;
use truster::tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn tick(&mut self, e: &Environment) {
        self.position += self.velocity;
        self.velocity += e.gravity + e.wind;
    }
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalized() * 11.25,
    };

    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(0.1, 0.8, 0.2);

    while p.position.y() > 0.0 {
        p.tick(&e);

        let x = p.position.x() as usize;
        let y = canvas.height() - p.position.y() as usize - 1;

        if x > canvas.width() - 1 || y > canvas.height() - 1 {
            continue;
        }

        canvas[[x, y]] = color;
    }

    let mut stdout = std::io::stdout();
    canvas.to_ppm(&mut stdout)?;
    Ok(())
}
