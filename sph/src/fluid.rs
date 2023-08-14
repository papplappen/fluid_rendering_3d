use glam::Vec3A;

use crate::{
    neighbourhood::Index,
    sph::{Particle, SPH},
};

pub struct Fluid {
    pub particles: Vec<Particle>,
    pub sph: SPH,
    pub rest_density: f32,
    pub kinematic_viscosity: f32,
    pub stiffness: f32,
    pub gravity: Vec3A,
}

impl Default for Fluid {
    fn default() -> Self {
        Self {
            particles: Default::default(),
            sph: SPH::new(1.),
            rest_density: 1.,
            kinematic_viscosity: 1.0e-3,
            stiffness: 5000.0,
            gravity: 9.81 * Vec3A::NEG_Y,
        }
    }
}

impl Fluid {
    pub fn new(
        particles: Vec<Particle>,
        h: f32,
        rest_density: f32,
        kinematic_viscosity: f32,
        stiffness: f32,
        gravity: Vec3A,
    ) -> Self {
        Self {
            particles,
            sph: SPH::new(h),
            rest_density,
            kinematic_viscosity,
            stiffness,
            gravity,
        }
    }

    pub fn step(&mut self, dt: f32) {
        let index = Index::new(&self.particles, 2. * self.sph.h, 64);
        let neighbourhoods = self
            .particles
            .iter()
            .map(|pi| index.neighbourhood(pi.pos, 2. * self.sph.h))
            .collect::<Vec<_>>();

        let densities = self
            .particles
            .iter()
            .zip(neighbourhoods.iter())
            .map(|(pi, nb)| {
                self.sph
                    .density(nb.particle_refs(&self.particles), |p| p.mass, pi.pos)
            })
            .collect::<Vec<_>>();

        let pressures = densities
            .iter()
            .map(|d| (self.stiffness * (d / self.rest_density - 1.)).max(0.))
            .collect::<Vec<_>>();

        for ((pi, d), p) in self.particles.iter_mut().zip(densities).zip(pressures) {
            pi.density = d;
            pi.pressure = p;
        }

        let accelerations_non_pressure = self
            .particles
            .iter()
            .zip(neighbourhoods.iter())
            .map(|(pi, nb)| {
                self.gravity
                    + self.kinematic_viscosity
                        * self
                            .sph
                            .laplace(nb.particle_refs(&self.particles), |p| p.velocity, pi)
            })
            .collect::<Vec<_>>();
        let accelerations_pressure = self
            .particles
            .iter()
            .zip(neighbourhoods.iter())
            .map(|(pi, nb)| {
                (-1. / pi.density)
                    * self
                        .sph
                        .gradient(nb.particle_refs(&self.particles), |p| p.pressure, pi)
            })
            .collect::<Vec<_>>();

        for ((pi, a_nonp), a_p) in self
            .particles
            .iter_mut()
            .zip(accelerations_non_pressure)
            .zip(accelerations_pressure)
        {
            pi.acceleration = a_nonp + a_p;
        }

        for pi in &mut self.particles {
            if pi.movable {
                pi.velocity += dt * pi.acceleration;
                pi.pos += dt * pi.velocity;
            }
        }
    }

    pub fn step_slow(&mut self, dt: f32) {
        let densities = self
            .particles
            .iter()
            .map(|pi| self.sph.density(&self.particles, |p| p.mass, pi.pos))
            .collect::<Vec<_>>();

        let pressures = densities
            .iter()
            .map(|d| (self.stiffness * (d / self.rest_density - 1.)).max(0.))
            .collect::<Vec<_>>();

        for ((pi, d), p) in self.particles.iter_mut().zip(densities).zip(pressures) {
            pi.density = d;
            pi.pressure = p;
        }

        let accelerations_non_pressure = self
            .particles
            .iter()
            .map(|pi| {
                self.gravity
                    + self.kinematic_viscosity
                        * self.sph.laplace(&self.particles, |p| p.velocity, pi)
            })
            .collect::<Vec<_>>();
        let accelerations_pressure = self
            .particles
            .iter()
            .map(|pi| (-1. / pi.density) * self.sph.gradient(&self.particles, |p| p.pressure, pi))
            .collect::<Vec<_>>();

        for ((pi, a_nonp), a_p) in self
            .particles
            .iter_mut()
            .zip(accelerations_non_pressure)
            .zip(accelerations_pressure)
        {
            pi.acceleration = a_nonp + a_p;
        }

        for pi in &mut self.particles {
            if pi.movable {
                pi.velocity += dt * pi.acceleration;
                pi.pos += dt * pi.velocity;
            }
        }
    }
}
