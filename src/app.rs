use rapier2d::{dynamics::RigidBodyType, geometry::CollisionEvent};
use sfml::{
    graphics::{Color, RenderStates, RenderTarget, RenderWindow},
    window::{Event, Key},
};

use crate::{
    ball::{Ball, BallSize},
    physics::{Physics, PhysicsObject},
    ring::{Ring, RingSize},
    sounds::{SoundList, SoundType, Sounds},
    util::Drawable,
};

pub struct App<'s> {
    window: RenderWindow,
    physics: Physics,
    balls: Vec<Ball<'s>>,
    rings: Vec<Ring<'s>>,
    soundlist: SoundList<'s>,
    sounds: Sounds<'s>,
}

impl<'s> App<'s> {
    pub fn new(title: &str) -> Self {
        let mut window = RenderWindow::new(
            (1280, 720),
            title,
            sfml::window::Style::CLOSE,
            &sfml::window::ContextSettings {
                antialiasing_level: 8,
                ..Default::default()
            },
        );
        window.set_vertical_sync_enabled(true);
        let mut physics = Physics::new();

        let mut balls = vec![
            Ball::new_with_size((580.0, 180.0), BallSize::Small),
            Ball::new_with_size((700.0, 180.0), BallSize::Small),
        ];

        balls.iter_mut().enumerate().for_each(|(_i, ball)| {
            ball.insert_into_physics(RigidBodyType::Dynamic, &mut physics);
        });

        let mut rings = vec![Ring::new_with_size((640.0, 360.0), RingSize::Large)];

        rings.iter_mut().for_each(|ring| {
            ring.insert_into_physics(RigidBodyType::Fixed, &mut physics);
        });

        let mut soundlist = SoundList::new();
        soundlist.preload();
        let sounds = Sounds::new();

        Self {
            window,
            physics,
            balls,
            rings,
            soundlist,
            sounds,
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => self.window.close(),
                    Event::KeyPressed { code, .. } => {
                        if code == Key::Q {
                            self.window.close();
                        }
                    }
                    _ => {}
                }
            }
            self.window.clear(Color::BLACK);

            self.update();
            self.draw();

            self.window.display();
        }
    }

    fn update(&mut self) {
        for ball in &mut self.balls {
            ball.update(&mut self.physics);
        }
        self.physics.step();
        self.physics
            .get_collision_events()
            .iter()
            .for_each(|event| {
                if event.stopped() && !event.removed() {
                    self.react_to_collision(*event);
                }
            });
        self.sounds.update();
        self.physics.cleanup();
    }

    fn draw(&mut self) {
        let states = RenderStates::default();

        for ball in &mut self.balls {
            ball.draw(&mut self.window, &states);
        }
        for ring in &mut self.rings {
            ring.draw(&mut self.window, &states);
        }
    }

    fn react_to_collision(&mut self, event: CollisionEvent) {
        if !self.physics.is_collider_removed(event.collider1())
            && !self.physics.is_collider_removed(event.collider2())
        {
            let rb1_handle = self
                .physics
                .collider_set
                .get(event.collider1())
                .unwrap()
                .parent()
                .unwrap();
            let rb1 = self.physics.rigidbody_set.get(rb1_handle).unwrap();
            let rb2_handle = self
                .physics
                .collider_set
                .get(event.collider2())
                .unwrap()
                .parent()
                .unwrap();
            let rb2 = self.physics.rigidbody_set.get(rb2_handle).unwrap();
            let combined_velocity_magnitude = rb1.linvel().norm() + rb2.linvel().norm();
            self.sounds.play(
                self.soundlist.get(SoundType::Bounce),
                pitch(combined_velocity_magnitude),
            );
            let mut found_obj1 = false;
            let mut found_obj2 = false;
            for ball in &mut self.balls {
                if found_obj1 && found_obj2 {
                    break;
                }
                if ball.is_obj_with_handle(rb1_handle) && !found_obj1 {
                    ball.rand_outline_color();
                    ball.set_radius(ball.radius() * 1.01);
                    self.physics.replace_collider(
                        rb1_handle,
                        event.collider1(),
                        ball.create_collider(),
                    );
                    found_obj1 = true;
                } else if ball.is_obj_with_handle(rb2_handle) && !found_obj2 {
                    ball.rand_outline_color();
                    ball.set_radius(ball.radius() * 1.01);
                    self.physics.replace_collider(
                        rb2_handle,
                        event.collider2(),
                        ball.create_collider(),
                    );
                    found_obj2 = true;
                }
            }
            for ring in &mut self.rings {
                if found_obj1 && found_obj2 {
                    break;
                }
                if ring.is_obj_with_handle(rb1_handle) && !found_obj1 {
                    ring.rand_outline_color();
                    found_obj1 = true;
                } else if ring.is_obj_with_handle(rb2_handle) && !found_obj2 {
                    ring.rand_outline_color();
                    found_obj2 = true;
                }
            }
        }
    }
}

fn pitch(magnitude: f32) -> f32 {
    let magnitude = magnitude as u32;
    match magnitude {
        0..=200 => magnitude as f32 / 200.0,
        201..=1000 => 1.0 + (magnitude - 200) as f32 / 800.0,
        1001..=2000 => 2.0 + (magnitude - 1000) as f32 / 1000.0,
        2001..=4000 => 3.0 + (magnitude - 2000) as f32 / 2000.0,
        _ => 4.0,
    }
}
