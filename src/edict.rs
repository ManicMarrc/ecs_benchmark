use std::hint::black_box;

pub use edict::prelude::*;
use edict::query::With;

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
      self.world.spawn((Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),));
    }
  }

  pub fn batched_spawn(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));
  }

  pub fn add_remove(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));

    let mut schedule_add = Scheduler::new();
    schedule_add.add_system(|world: &mut World, q: QueryRef<&EntityId, With<Vec3>>| {
      for e in &q {
        world.insert(*e, Vec2(glam::Vec2::new(1.0, 1.0))).unwrap();
      }
    });

    schedule_add.run_threaded(&mut self.world);

    let mut schedule_remove = Scheduler::new();
    schedule_remove.add_system(|world: &mut World, q: QueryRef<&EntityId, With<Vec3>>| {
      for e in &q {
        world.remove::<Vec2>(*e).unwrap();
      }
    });

    schedule_remove.run_threaded(&mut self.world);
  }

  pub fn iter(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));

    let mut schedule = Scheduler::new();
    schedule.add_system(|q: QueryRef<&Vec3>| {
      for v in &q {
        black_box(v);
      }
    });

    schedule.run_threaded(&mut self.world);
  }

  pub fn multiple_iter(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));
    self.world.spawn_batch((0..n).map(|_| (Vec2(glam::Vec2::new(0.0, 0.0)),)));

    let mut schedule = Scheduler::new();
    schedule.add_system(|q: QueryRef<&Vec3>| {
      for v in &q {
        black_box(v);
      }
    });
    schedule.add_system(|q: QueryRef<&Vec2>| {
      for v in &q {
        black_box(v);
      }
    });

    schedule.run_threaded(&mut self.world);
  }

  pub fn multiple_iter_same(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));

    let mut schedule = Scheduler::new();
    schedule.add_system(|q: QueryRef<&Vec3>| {
      for v in &q {
        black_box(v);
      }
    });
    schedule.add_system(|q: QueryRef<&Vec3>| {
      for v in &q {
        black_box(v);
      }
    });

    schedule.run_threaded(&mut self.world);
  }

  pub fn iter_mut(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));

    let mut schedule = Scheduler::new();
    schedule.add_system(|mut q: QueryRef<&mut Vec3>| {
      for v in &mut q {
        black_box(v);
      }
    });

    schedule.run_threaded(&mut self.world);
  }

  pub fn multiple_iter_mut(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));
    self.world.spawn_batch((0..n).map(|_| (Vec2(glam::Vec2::new(0.0, 0.0)),)));

    let mut schedule = Scheduler::new();
    schedule.add_system(|mut q: QueryRef<&mut Vec3>| {
      for v in &mut q {
        black_box(v);
      }
    });
    schedule.add_system(|mut q: QueryRef<&mut Vec2>| {
      for v in &mut q {
        black_box(v);
      }
    });

    schedule.run_threaded(&mut self.world);
  }

  pub fn multiple_iter_mut_same(&mut self, n: usize) {
    self.world.spawn_batch((0..n).map(|_| (Vec3(glam::Vec3::new(0.0, 0.0, 0.0)),)));

    let mut schedule = Scheduler::new();
    schedule.add_system(|mut q: QueryRef<&mut Vec3>| {
      for v in &mut q {
        black_box(v);
      }
    });
    schedule.add_system(|mut q: QueryRef<&mut Vec3>| {
      for v in &mut q {
        black_box(v);
      }
    });

    schedule.run_threaded(&mut self.world);
  }
}
