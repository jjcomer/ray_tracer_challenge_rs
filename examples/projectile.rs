use anyhow::Result;
use raytrace::tuples::Tuple;

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
fn main() -> Result<()> {
    let mut proj = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    };
    let env = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_vector(-0.01, 0.0, 0.0),
    };
    let mut counter = 0;
    while proj.position.y > 0.0 {
        println!(
            "Current position {} {} {}",
            proj.position.x, proj.position.y, proj.position.z
        );
        proj = tick(&env, &proj);
        counter += 1;
    }
    println!("It took {} ticks to hit the ground", counter);
    Ok(())
}
