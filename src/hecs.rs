use std::hint::black_box;

use hecs::*;

pub struct Ecs {
  world: World,
}

impl Ecs {
  pub fn new() -> Ecs { Ecs { world: World::new() } }

  pub fn unbatched_spawn(&mut self, n: usize) {
    for _ in 0..n {
      self.world.spawn((glam::Vec3::new(0.0, 0.0, 0.0),));
    }
  }

  pub fn batched_spawn(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));
  }

  pub fn add_remove(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));
    let entities: Vec<Entity> = self.world.query::<()>().iter().map(|(e, _)| e).collect();
    for e in &entities {
      self.world.insert(*e, (glam::Vec2::new(1.0, 1.0),)).unwrap();
    }
    for e in &entities {
      self.world.remove::<(glam::Vec2,)>(*e).unwrap();
    }
  }

  pub fn iter(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));
    for v in self.world.query::<&glam::Vec3>().iter() {
      black_box(v);
    }
  }

  pub fn multiple_iter(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));
    self.world.spawn_batch((0..n).map(|_| (glam::Vec2::new(0.0, 0.0),)));

    for v in self.world.query::<&glam::Vec3>().iter() {
      black_box(v);
    }
    for v in self.world.query::<&glam::Vec2>().iter() {
      black_box(v);
    }
  }

  pub fn multiple_iter_same(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));

    for v in self.world.query::<&glam::Vec3>().iter() {
      black_box(v);
    }
    for v in self.world.query::<&glam::Vec3>().iter() {
      black_box(v);
    }
  }

  pub fn iter_mut(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));
    for v in self.world.query_mut::<&mut glam::Vec3>() {
      black_box(v);
    }
  }

  pub fn multiple_iter_mut(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));
    self.world.spawn_batch((0..n).map(|_| (glam::Vec2::new(0.0, 0.0),)));

    for v in self.world.query_mut::<&mut glam::Vec3>() {
      black_box(v);
    }
    for v in self.world.query_mut::<&mut glam::Vec2>() {
      black_box(v);
    }
  }

  pub fn multiple_iter_mut_same(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (glam::Vec3::new(0.0, 0.0, 0.0),)));

    for v in self.world.query_mut::<&mut glam::Vec3>() {
      black_box(v);
    }
    for v in self.world.query_mut::<&mut glam::Vec3>() {
      black_box(v);
    }
  }
}
