use glam::{vec3a, Vec3A};
use sph::{fluid::Fluid, sph::Particle};

fn cube_in_box(extend: i32) -> Vec<Particle> {
    let mut particles = vec![];
    for x in -extend..=extend {
        for y in -extend..=extend {
            for z in -extend..=extend {
                if z < 0 || (x.abs() + 1 < extend && y.abs() + 1 < extend) {
                    let movable = x.abs() + 1 < extend && y.abs() + 1 < extend && z - 1 > -extend;
                    particles.push(Particle::new(
                        vec3a(x as f32, y as f32, z as f32),
                        1.,
                        movable,
                    ));
                }
            }
        }
    }
    particles
}

fn avg_movable_pos(particles: &[Particle]) -> Vec3A {
    let moveable_poss = particles
        .iter()
        .filter(|p| p.movable)
        .map(|p| p.pos)
        .collect::<Vec<_>>();
    moveable_poss.iter().fold(Vec3A::ZERO, |a, v| a + *v) / (moveable_poss.len() as f32)
}

fn main() {
    let particles = cube_in_box(10);
    // let particles = vec![
    //     Particle::new(vec3a(0., 0., 0.), 1., false),
    //     Particle::new(vec3a(-4.0, 0., 0.), 1., false),
    // ];

    println!("{}", particles.len());

    let mut fluid = Fluid::new(particles, 1.0, 1., 1.0e-3, 5000.0, -9.81 * Vec3A::Z);

    for _ in 0..100 {
        fluid.step(0.1);
        // fluid.step_slow(0.1);
        println!("{}", avg_movable_pos(&fluid.particles));
    }
}
