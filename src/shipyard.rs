use std::hint::black_box;

use shipyard::*;

#[derive(Component)]
struct Vec3(glam::Vec3);

#[derive(Component)]
struct Vec2(glam::Vec2);

pub struct Ecs {
  world: World,
}

impl Ecs {
  pub fn new() -> Ecs { Ecs { world: World::new() } }

  pub fn unbatched_spawn(&mut self, n: usize) {
    for _ in 0..n {
      self.world.add_entity(Vec3(glam::Vec3::new(0.0, 0.0, 0.0)));
    }
  }

  pub fn batched_spawn(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));
  }

  pub fn add_remove(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));
    self.world.run(|vm_vec3: View<Vec3>, mut vm_vec2: ViewMut<Vec2>| {
      for e in vm_vec3.iter().enumerate().map(|(i, _)| vm_vec3.id_at(i).unwrap()) {
        vm_vec2.add_component_unchecked(e, Vec2(glam::Vec2::new(1.0, 1.0)));
      }
    });
    self.world.run(|vm_vec3: View<Vec3>, mut vm_vec2: ViewMut<Vec2>| {
      for e in vm_vec3.iter().enumerate().map(|(i, _)| vm_vec3.id_at(i).unwrap()) {
        vm_vec2.remove(e);
      }
    });
  }

  pub fn iter(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));
    self.world.run(|q: View<Vec3>| {
      for v in q.iter() {
        black_box(v);
      }
    });
  }

  pub fn multiple_iter(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));
    self.world.bulk_add_entity((0..n).map(|_| Vec2(glam::Vec2::new(0.0, 0.0))));

    if !self.world.contains_workload("main") {
      self.world.add_workload(|| {
        (
          |q: View<Vec3>| {
            for v in q.iter() {
              black_box(v);
            }
          },
          |q: View<Vec2>| {
            for v in q.iter() {
              black_box(v);
            }
          },
        )
          .into_workload()
          .rename("main")
      });
    }
    self.world.run_workload("main").unwrap();
  }

  pub fn multiple_iter_same(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));

    if !self.world.contains_workload("main") {
      self.world.add_workload(|| {
        (
          |q: View<Vec3>| {
            for v in q.iter() {
              black_box(v);
            }
          },
          |q: View<Vec3>| {
            for v in q.iter() {
              black_box(v);
            }
          },
        )
          .into_workload()
          .rename("main")
      });
    }
    self.world.run_workload("main").unwrap();
  }

  pub fn iter_mut(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));
    self.world.run(|mut q: ViewMut<Vec3>| {
      for v in (&mut q).iter() {
        black_box(v);
      }
    });
  }

  pub fn multiple_iter_mut(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));
    self.world.bulk_add_entity((0..n).map(|_| Vec2(glam::Vec2::new(0.0, 0.0))));

    if !self.world.contains_workload("main") {
      self.world.add_workload(|| {
        (
          |mut q: ViewMut<Vec3>| {
            for v in (&mut q).iter() {
              black_box(v);
            }
          },
          |mut q: ViewMut<Vec2>| {
            for v in (&mut q).iter() {
              black_box(v);
            }
          },
        )
          .into_workload()
          .rename("main")
      });
    }
    self.world.run_workload("main").unwrap();
  }

  pub fn multiple_iter_mut_same(&mut self, n: usize) {
    self.world.bulk_add_entity((0..n).map(|_| Vec3(glam::Vec3::new(0.0, 0.0, 0.0))));

    if !self.world.contains_workload("main") {
      self.world.add_workload(|| {
        (
          |mut q: ViewMut<Vec3>| {
            for v in (&mut q).iter() {
              black_box(v);
            }
          },
          |mut q: ViewMut<Vec3>| {
            for v in (&mut q).iter() {
              black_box(v);
            }
          },
        )
          .into_workload()
          .rename("main")
      });
    }
    self.world.run_workload("main").unwrap();
  }
}
