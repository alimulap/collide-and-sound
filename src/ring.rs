#![allow(unused)]

use std::f32::consts::PI;

use rapier2d::{
    dynamics::{RigidBodyBuilder, RigidBodyHandle, RigidBodyType},
    geometry::ColliderBuilder,
    na::Point2,
    pipeline::ActiveEvents,
};
use sfml::{
    graphics::{CircleShape, Drawable, Shape, Transformable},
    system::Vector2f,
};

use crate::{
    physics::{Physics, PhysicsObject},
    util::{self, ToNaMat2x1},
};

pub struct Ring<'s> {
    pub shape: CircleShape<'s>,
    rb_handle: Option<RigidBodyHandle>,
}

impl<'s> Ring<'s> {
    pub fn new<P: Into<Vector2f>>(pos: P) -> Self {
        let mut shape = CircleShape::default();
        shape.set_position(pos);
        shape.set_radius(100.0);
        shape.set_origin((100.0, 100.0));
        shape.set_fill_color(sfml::graphics::Color::TRANSPARENT);
        shape.set_outline_color(sfml::graphics::Color::WHITE);
        shape.set_outline_thickness(5.0);
        shape.set_point_count(256);
        Self {
            shape,
            rb_handle: None,
        }
    }

    pub fn new_with_size<P: Into<Vector2f>>(pos: P, size: RingSize) -> Self {
        let mut shape = CircleShape::default();
        let radius = match size {
            RingSize::Small => 25.0,
            RingSize::Medium => 100.0,
            RingSize::Large => 175.0,
        };
        shape.set_position(pos);
        shape.set_radius(radius);
        shape.set_origin((radius, radius));
        shape.set_fill_color(sfml::graphics::Color::TRANSPARENT);
        shape.set_outline_color(sfml::graphics::Color::WHITE);
        shape.set_outline_thickness(5.0);
        shape.set_point_count(256);
        Self {
            shape,
            rb_handle: None,
        }
    }
}

impl util::Drawable for Ring<'_> {
    fn draw(
        &mut self,
        target: &mut dyn sfml::graphics::RenderTarget,
        states: &sfml::graphics::RenderStates,
    ) {
        self.shape.draw(target, states);
    }
}

impl PhysicsObject for Ring<'_> {
    fn insert_into_physics(&mut self, rbtype: RigidBodyType, physics: &mut Physics) {
        let rb = RigidBodyBuilder::new(rbtype)
            .translation(self.shape.position().to_na_mat2x1())
            .rotation(PI / 2.0)
            .ccd_enabled(true)
            .build();
        let rb_handle = physics.rigidbody_set.insert(rb);
        let (radius, thickness, point_count) = {
            (
                self.shape.radius(),
                self.shape.outline_thickness(),
                self.shape.point_count(),
            )
        };
        let (vertices, indices) = {
            assert!(point_count > 2, "A ring must have at least 3 points.");
            assert!(radius > 0.0, "A ring must have a radius greater than 0.");
            assert!(
                thickness > 0.0,
                "A ring must have a thickness greater than 0."
            );
            let vertices_len = point_count * 2;
            let mut vertices = Vec::with_capacity(vertices_len);
            let mut indices = Vec::with_capacity(vertices_len);

            for i in 0..vertices_len {
                let angle = i as f32 / vertices_len as f32 * PI * 2.0;
                let radius = if i % 2 == 0 {
                    radius + thickness
                } else {
                    radius
                };
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;
                vertices.push(Point2::new(x, y));
            }
            for i in 0..vertices_len - 2 {
                indices.push([i as u32, (i + 1) as u32, (i + 2) as u32]);
            }
            indices.push([vertices_len as u32 - 2, vertices_len as u32 - 1, 0]);
            indices.push([vertices_len as u32 - 1, 1, 0]);

            (vertices, indices)
        };
        let collider = ColliderBuilder::trimesh(vertices, indices)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .restitution(1.02)
            .build();
        physics
            .collider_set
            .insert_with_parent(collider, rb_handle, &mut physics.rigidbody_set);
        self.rb_handle = Some(rb_handle);
    }
}

pub enum RingSize {
    Small,
    Medium,
    Large,
}
