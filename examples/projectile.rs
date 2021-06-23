use rtc::tuple::Tuple;

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

fn main() {
	let mut p = Projectile {
		position: Tuple::point(0.0, 1.0, 0.0),
		velocity: Tuple::vector(1.0, 1.0, 0.0).normalized(),
	};

	let e = Environment {
		gravity: Tuple::vector(0.0, -0.1, 0.0),
		wind: Tuple::vector(-0.01, 0.0, 0.0),
	};

	while p.position.y() > 0.0 {
		println!("{:?}", p.position);
		p.tick(&e);
	}
}
