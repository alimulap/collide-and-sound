#![allow(dead_code)]

use rand::Rng;
use rapier2d::prelude::*;
use sfml::{
    graphics::{CircleShape, Color, Drawable, RenderStates, RenderTarget, Shape, Transformable},
    system::Vector2f,
};

use crate::util::ToNaMat2x1;
use crate::{
    physics::{Physics, PhysicsObject},
    util,
};

pub struct Ball<'s> {
    pub shape: CircleShape<'s>,
    rb_handle: Option<RigidBodyHandle>,
}

impl Ball<'_> {
    pub fn new<P: Into<Vector2f>>(pos: P) -> Self {
        let mut shape = CircleShape::default();
        shape.set_position(pos);
        shape.set_radius(100.0);
        shape.set_origin((100.0, 100.0));
        shape.set_fill_color(sfml::graphics::Color::TRANSPARENT);
        shape.set_outline_color(sfml::graphics::Color::WHITE);
        shape.set_outline_thickness(5.0);
        shape.set_point_count(100);
        Self {
            shape,
            rb_handle: None,
        }
    }

    pub fn new_with_size<P: Into<Vector2f>>(pos: P, size: BallSize) -> Self {
        let mut shape = CircleShape::default();
        let radius = match size {
            BallSize::Small => 15.0,
            BallSize::Medium => 100.0,
            BallSize::Large => 150.0,
        };
        shape.set_position(pos);
        shape.set_radius(radius);
        shape.set_origin((radius, radius));
        shape.set_fill_color(sfml::graphics::Color::TRANSPARENT);
        shape.set_outline_color(sfml::graphics::Color::WHITE);
        shape.set_outline_thickness(5.0);
        shape.set_point_count(100);
        Self {
            shape,
            rb_handle: None,
        }
    }

    pub fn update(&mut self, physics: &mut Physics) {
        if let Some(rbhandle) = self.rb_handle {
            if let Some(rb) = physics.rigidbody_set.get(rbhandle) {
                let pos = rb.position().translation.vector;
                self.shape.set_position((pos.x, pos.y));
            }
        }
    }

    pub fn set_gravity_scale(&mut self, scale: f32, physics: &mut Physics) {
        if let Some(rbhandle) = self.rb_handle {
            if let Some(rb) = physics.rigidbody_set.get_mut(rbhandle) {
                rb.set_gravity_scale(scale, false);
            }
        }
    }

    pub fn set_mass(&mut self, mass: f32, physics: &mut Physics) {
        if let Some(rbhandle) = self.rb_handle {
            if let Some(rb) = physics.rigidbody_set.get_mut(rbhandle) {
                let chandle = rb.colliders().iter().next().unwrap();
                if let Some(collider) = physics.collider_set.get_mut(*chandle) {
                    collider.set_density(mass);
                    collider.set_mass(mass);
                }
            }
        }
    }

    pub fn is_obj_with_handle(&self, handle: RigidBodyHandle) -> bool {
        if let Some(rbhandle) = self.rb_handle {
            rbhandle == handle
        } else {
            false
        }
    }

    pub fn set_outline_color(&mut self, color: Color) {
        self.shape.set_outline_color(color);
    }

    pub fn rand_outline_color(&mut self) {
        let mut rng = rand::thread_rng();
        self.shape.set_outline_color(Color::rgb(
            rng.gen_range(10..255),
            rng.gen_range(10..255),
            rng.gen_range(10..255),
        ));
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.shape.set_radius(radius);
        self.shape.set_origin((radius, radius));
    }

    pub fn radius(&self) -> f32 {
        self.shape.radius()
    }

    pub fn create_collider(&mut self) -> Collider {
        ColliderBuilder::ball(self.shape.radius() + self.shape.outline_thickness())
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .restitution(1.035)
            .build()
    }
}

impl util::Drawable for Ball<'_> {
    fn draw(&mut self, target: &mut dyn RenderTarget, states: &RenderStates) {
        self.shape.draw(target, states);
    }
}

impl PhysicsObject for Ball<'_> {
    fn insert_into_physics(&mut self, rbtype: RigidBodyType, physics: &mut Physics) {
        let rb = RigidBodyBuilder::new(rbtype)
            .ccd_enabled(true)
            .translation(self.shape.position().to_na_mat2x1())
            .build();
        let collider = self.create_collider();
        let rbhandle = physics.insert_body(rb, collider);
        self.rb_handle = Some(rbhandle);
    }
}

pub enum BallSize {
    Small,
    Medium,
    Large,
}
