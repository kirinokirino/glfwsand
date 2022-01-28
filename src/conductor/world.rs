use crate::automata;
use crate::automata::{Automata, Destination};
use crate::common::Position;
use hecs::PreparedQuery;
use hecs::World as Ecs;
use pixelbuffer::{Event, Pixel, PixelBuffer, Resolution, Window};
pub struct World {
    window: Window,
    resolution: Resolution,

    ecs: Ecs,
    mouse: (f64, f64),
    selection: Automata,
}

impl World {
    pub fn new(resolution: Resolution, title: &str) -> Self {
        Self {
            window: Window::new(resolution, title),
            resolution,
            ecs: Ecs::new(),
            mouse: (0.0, 0.0),
            selection: Automata::Sand,
        }
    }

    fn add_walkers(&mut self, pos: Position) {
        let to_spawn = (0..50).map(|_| (self.selection, pos, Destination::from(pos)));
        self.ecs.spawn_batch(to_spawn);
    }

    fn update_automata_destination_system(
        &mut self,
        query: &mut PreparedQuery<(&mut Destination, &Position, &Automata)>,
    ) {
        for (_id, (dest, pos, automata)) in query.query_mut(&mut self.ecs) {
            match automata {
                Automata::RandomWalker => *dest = automata::random_walker::update(pos),
                Automata::Water => *dest = automata::water::update(pos),
                Automata::Sand => *dest = automata::sand::update(pos),
            }
        }
    }

    fn resolve_movement_and_draw_point_automata_system(
        &mut self,
        buffer: &mut PixelBuffer,
        query: &mut PreparedQuery<(&mut Position, &Destination, &Automata)>,
    ) {
        for (_id, (pos, dest, automata)) in query.query_mut(&mut self.ecs) {
            if (dest.x < 0
                || dest.y < 0
                || dest.x >= i64::from(self.resolution.width)
                || dest.y >= i64::from(self.resolution.height))
                && (pos.x < 0
                    || pos.y < 0
                    || pos.x >= i64::from(self.resolution.width)
                    || pos.y >= i64::from(self.resolution.height))
            {
                continue;
            }

            // Only if destination is far away:
            /* let to_check = Position::new(dest.x, dest.y).straight_line(*pos);
            let free = to_check
                .iter()
                .find(|pos| buffer.free((pos.x as u16, pos.y as u16).into()));
                */
            let mut dest = Position::from(*dest);
            let free = buffer.free((dest.x as u16, dest.y as u16).into());
            if !free {
                dest = *pos
                    .straight_line(Position::new(pos.x, pos.y - 100))
                    .iter()
                    .find(|pos| buffer.free((pos.x as u16, pos.y as u16).into()))
                    .unwrap_or(pos);
            }
            *pos = dest;
            let pixel = match automata {
                Automata::RandomWalker => Pixel::new(170, fastrand::u8(180..220), 220),
                Automata::Water => Pixel::new(100, 100, fastrand::u8(180..255)),
                Automata::Sand => Pixel::new(fastrand::u8(120..200), 90, 70),
            };
            buffer.set_pixel((dest.x as u16, dest.y as u16).into(), pixel);
        }
    }

    fn run_update_systems(&mut self, buffer: &mut PixelBuffer) {
        let mut q = PreparedQuery::<(&mut Destination, &Position, &Automata)>::default();
        self.update_automata_destination_system(&mut q);
        let mut q = PreparedQuery::<(&mut Position, &Destination, &Automata)>::default();
        self.resolve_movement_and_draw_point_automata_system(buffer, &mut q);
    }

    fn draw_sprites_system(
        &mut self,
        buffer: &mut PixelBuffer,
        query: &mut PreparedQuery<(&mut Position, &Automata)>,
    ) {
        for (id, (pos, automata)) in query.query_mut(&mut self.ecs) {
            if pos.x < 0
                || pos.y < 0
                || pos.x >= i64::from(self.resolution.width)
                || pos.y >= i64::from(self.resolution.height)
            {
                continue;
            }
            let clear = Pixel::alpha();
            let white = Pixel::new(255, 255, 255);
            let mut pixels = vec![
                white, white, white, white, clear, white, white, clear, white, white, clear, white,
                white, clear, white, white, white, white, clear, white, white, white, white,
            ];
            buffer.blit(
                (pos.x as u16, pos.y as u16).into(),
                Resolution::new(3, 6),
                &mut pixels,
            );
        }
        todo!();
    }

    fn run_pure_draw_systems(&mut self, buffer: &mut PixelBuffer) {
        todo!();
        //let mut q = PreparedQuery::<(&mut Position, &Automata)>::default();
        //self.draw_point_automata_system(buffer, &mut q);
        //self.draw_sprites_system(buffer, &mut q);
    }

    pub fn start(&mut self) {
        'running: loop {
            let mut buffer: PixelBuffer = PixelBuffer::new(self.resolution);
            let events = self.window.shown();
            for event in events {
                match event {
                    Event::Close => break 'running,
                    Event::Key(key) => match key {
                        glfw::Key::W => self.selection = Automata::Water,
                        glfw::Key::S => self.selection = Automata::Sand,
                        glfw::Key::A => (),
                        glfw::Key::D => (),
                        glfw::Key::Space => self.selection = Automata::RandomWalker,
                        _ => println!("Pressed unhandled key {:?}", key),
                    },
                    Event::MouseButton(btn) => match btn {
                        glfw::MouseButton::Button1 => self
                            .add_walkers(Position::new(self.mouse.0 as i64, self.mouse.1 as i64)),
                        _ => println!("Pressed unhandled mouse button {:?}", btn),
                    },
                    Event::Cursor((x, y)) => self.mouse = (x, y),
                }
            }
            self.run_update_systems(&mut buffer);
            //self.run_pure_draw_systems(&mut buffer);

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
