use rapier2d::dynamics::RigidBodyType;
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
            (600, 400),
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
            Ball::new_with_size((270.0, 200.0), BallSize::Small),
            Ball::new_with_size((330.0, 200.0), BallSize::Small),
        ];

        balls.iter_mut().enumerate().for_each(|(_i, ball)| {
            ball.insert_into_physics(RigidBodyType::Dynamic, &mut physics);
        });

        let mut rings = vec![Ring::new_with_size((300.0, 200.0), RingSize::Large)];

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
                if event.started() {
                    self.sounds.play(self.soundlist.get(SoundType::Bounce));
                }
            });
        self.sounds.update();
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
}
