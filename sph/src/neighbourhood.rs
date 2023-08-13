use glam::{i64vec3, u64vec3, I64Vec3, U64Vec3, Vec3A};

use crate::sph::Particle;

#[derive(Debug)]
pub struct Neighbourhood(Vec<usize>);

impl Neighbourhood {
    pub fn particle_refs<'s, 'a: 's>(
        &'s self,
        particles: &'a [Particle],
    ) -> impl Iterator<Item = &'a Particle> + 's {
        self.0.iter().map(|i| &particles[*i])
    }
}

struct ModCube<T> {
    cube: Vec<Vec<Vec<Vec<T>>>>,
    size: I64Vec3,
}

impl<T> ModCube<T> {
    pub fn new(size: U64Vec3) -> ModCube<T> {
        let mut cube = vec![];
        for _ in 0..size.x {
            let mut vx = vec![];
            for _ in 0..size.y {
                let mut vy = vec![];
                for _ in 0..size.z {
                    vy.push(vec![]);
                }
                vx.push(vy);
            }
            cube.push(vx);
        }
        Self {
            cube,
            size: size.as_i64vec3(),
        }
    }

    fn loc(&self, pos: I64Vec3) -> (usize, usize, usize) {
        (
            pos.x.rem_euclid(self.size.x) as usize,
            pos.y.rem_euclid(self.size.y) as usize,
            pos.z.rem_euclid(self.size.z) as usize,
        )
    }

    pub fn get(&self, pos: I64Vec3) -> &Vec<T> {
        let loc = self.loc(pos);
        &self.cube[loc.0][loc.1][loc.2]
    }

    pub fn get_mut(&mut self, pos: I64Vec3) -> &mut Vec<T> {
        let loc = self.loc(pos);
        &mut self.cube[loc.0][loc.1][loc.2]
    }
}

pub struct Index {
    // grid: BTreeMap<(i32, i32, i32), Vec<usize>>,
    grid: ModCube<usize>,
    grid_scale: f32,
}

impl Index {
    pub fn new<'a, P>(particles: P, grid_scale: f32, grid_size: u64) -> Self
    where
        P: IntoIterator<Item = &'a Particle>,
    {
        let mut index = Self {
            grid: ModCube::new(u64vec3(grid_size, grid_size, grid_size)),
            grid_scale,
        };
        for (i, pi) in particles.into_iter().enumerate() {
            index.grid.get_mut(index.grid_position(pi.pos)).push(i);
        }
        index
    }

    pub fn grid_position(&self, pos: Vec3A) -> I64Vec3 {
        let t = pos / self.grid_scale;
        i64vec3(t.x as i64, t.y as i64, t.z as i64)
    }

    pub fn neighbourhood(&self, pos: Vec3A, radius: f32) -> Neighbourhood {
        let gradius = (radius / self.grid_scale).ceil() as i64;
        let gpos = self.grid_position(pos);
        let mut neighbourhood = Neighbourhood(vec![]);
        for x in (gpos.x - gradius)..=(gpos.x + gradius) {
            for y in (gpos.y - gradius)..=(gpos.y + gradius) {
                for z in (gpos.z - gradius)..=(gpos.z + gradius) {
                    neighbourhood
                        .0
                        .append(&mut self.grid.get(i64vec3(x, y, z)).clone())
                }
            }
        }
        // dbg!(&neighbourhood);
        neighbourhood
    }
}
