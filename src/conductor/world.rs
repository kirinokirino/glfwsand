use crate::automata::random_walker::Walker;
use crate::automata::Automata;
use crate::common::Position;
use hecs::PreparedQuery;
use hecs::World as Ecs;
use pixelbuffer::{Event, Pixel, PixelBuffer, Resolution, Window};
pub struct World {
    window: Window,
    resolution: Resolution,

    ecs: Ecs,
    walkers: Vec<Walker>,
}

impl World {
    pub fn new(resolution: Resolution, title: &str) -> Self {
        Self {
            window: Window::new(resolution, title),
            resolution,
            walkers: Vec::new(),
            ecs: Ecs::new(),
        }
    }
    fn add_walkers(&mut self) {
        let to_spawn = (0..50).map(|_| {
            let pos = Position::new(fastrand::i64(50..400), fastrand::i64(50..400));
            (Automata::RandomWalker, pos)
        });
        self.ecs.spawn_batch(to_spawn);
    }

    fn random_walkers_system(&mut self, query: &mut PreparedQuery<(&mut Position, &Automata)>) {
        for (id, (pos, automata)) in query.query_mut(&mut self.ecs) {
            *pos += Position::new(fastrand::i64(-1..2), fastrand::i64(-1..2));
        }
    }

    fn run_systems(&mut self) {
        let mut q = PreparedQuery::<(&mut Position, &Automata)>::default();
        self.random_walkers_system(&mut q);
    }

    pub fn start(&mut self) {
        loop {
            if let Some(event) = self.window.shown() {
                match event {
                    Event::Close => break,
                    Event::Key(key) => match key {
                        glfw::Key::W => (),
                        glfw::Key::S => (),
                        glfw::Key::A => (),
                        glfw::Key::D => (),
                        glfw::Key::Space => self.add_walkers(),
                        _ => println!("Pressed unhandled key {:?}", key),
                    },
                }
            }
            let mut buffer: PixelBuffer = PixelBuffer::new(self.resolution);
            self.run_systems();

            for (id, pos) in &mut self.ecs.query::<&Position>() {
                if pos.x < 0
                    || pos.y < 0
                    || pos.x >= i64::from(self.resolution.width)
                    || pos.y >= i64::from(self.resolution.height)
                {
                    continue;
                }
                buffer.set_pixel(
                    (pos.x as u16, pos.y as u16).into(),
                    Pixel::new(255, 255, 255),
                );
            }

            self.window.set_frame(buffer.get_buffer());
        }
    }
}
/*
fn system_remove_dead(world: &mut World) {
    // Here we query entities with 0 or less hp and despawn them
    let mut to_remove: Vec<Entity> = Vec::new();
    for (id, hp) in &mut world.query::<&Health>() {
        if hp.0 <= 0 {
            to_remove.push(id);
        }
    }

    for entity in to_remove {
        world.despawn(entity).unwrap();
    }
}
*/
