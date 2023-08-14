use std::f32::consts::PI;

use glam::Vec3A;

#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vec3A,
    pub velocity: Vec3A,
    pub acceleration: Vec3A,
    pub mass: f32,
    pub density: f32,
    pub pressure: f32,
    pub movable: bool,
}

impl Particle {
    pub fn new(pos: Vec3A, mass: f32, movable: bool) -> Self {
        Self {
            pos,
            velocity: Vec3A::ZERO,
            acceleration: Vec3A::ZERO,
            mass,
            density: f32::NAN,
            pressure: f32::NAN,
            movable,
        }
    }
}

pub struct SPH {
    pub h: f32,
    pub alpha: f32,
}

impl SPH {
    pub fn new(h: f32) -> Self {
        Self {
            h,
            alpha: 1. / (4. * PI * h * h * h),
        }
    }

    fn kernel(&self, xi: Vec3A, xj: Vec3A) -> f32 {
        let q = xi.distance(xj) / self.h;
        let t1 = (1. - q).max(0.);
        let t2 = (2. - q).max(0.);
        self.alpha * (t2 * t2 * t2 - 4. * t1 * t1 * t1)
    }

    fn kernel_grad(&self, xi: Vec3A, xj: Vec3A) -> Vec3A {
        let d = (xi - xj) / self.h;
        let q = d.length();
        if q > 0. {
            let t1 = (1. - q).max(0.);
            let t2 = (2. - q).max(0.);
            self.alpha * (d / q) * (-3. * t2 * t2 + 12. * t1 * t1)
        } else {
            Vec3A::ZERO
        }
    }

    pub fn density<'a, N, A>(&self, neighbours: N, attribute: A, pos: Vec3A) -> f32
    where
        N: IntoIterator<Item = &'a Particle>,
        A: Fn(&Particle) -> f32,
    {
        neighbours
            .into_iter()
            .map(|pj| attribute(pj) * self.kernel(pos, pj.pos))
            .sum()
    }

    pub fn gradient<'a, N, A>(&self, neighbours: N, attribute: A, pi: &Particle) -> Vec3A
    where
        N: IntoIterator<Item = &'a Particle>,
        A: Fn(&Particle) -> f32,
    {
        let ai = attribute(pi);
        pi.density
            * neighbours
                .into_iter()
                .map(|pj| {
                    pj.mass
                        * (ai / (pi.density * pi.density)
                            + attribute(pj) / (pj.density * pi.density))
                        * self.kernel_grad(pi.pos, pj.pos)
                })
                .fold(Vec3A::ZERO, |a, v| a + v)
    }

    pub fn laplace<'a, N, A>(&self, neighbours: N, attribute: A, pi: &Particle) -> Vec3A
    where
        N: IntoIterator<Item = &'a Particle>,
        A: Fn(&Particle) -> Vec3A,
    {
        let ai = attribute(pi);
        2. * neighbours
            .into_iter()
            .map(|pj| {
                let xij = pi.pos - pj.pos;
                (pj.mass / pj.density) * (ai - attribute(pj)).dot(xij)
                    / (xij.length_squared() + 0.01 * self.h * self.h)
                    * self.kernel_grad(pi.pos, pj.pos)
            })
            .fold(Vec3A::ZERO, |a, v| a + v)
    }

    pub fn divergence<'a, N, A>(&self, neighbours: N, attribute: A, pi: &Particle) -> Vec3A
    where
        N: IntoIterator<Item = &'a Particle>,
        A: Fn(&Particle) -> Vec3A,
    {
        let ai = attribute(pi);
        neighbours
            .into_iter()
            .map(|pj| pj.mass * (ai - attribute(pj)) * self.kernel_grad(pi.pos, pj.pos))
            .fold(Vec3A::ZERO, |a, v| a + v)
    }
}
