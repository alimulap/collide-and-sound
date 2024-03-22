use crossbeam::channel::Receiver;
use rapier2d::{na::Matrix2x1, prelude::*};

pub struct Physics {
    gravity: Matrix2x1<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub rigidbody_set: RigidBodySet,
    pub collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    //physics_hooks: (),
    event_handler: ChannelEventCollector,
    event_receiver: (Receiver<CollisionEvent>, Receiver<ContactForceEvent>),
}

impl Physics {
    pub fn new() -> Self {
        let gravity = vector![0.0, 9.81 * 25.];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let rigidbody_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        //let physics_hooks = ();
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
        let event_receiver = (collision_recv, contact_force_recv);

        Self {
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            rigidbody_set,
            collider_set,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            //physics_hooks,
            event_handler,
            event_receiver,
        }
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigidbody_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &self.event_handler,
        );
    }

    pub fn insert_body(&mut self, rb: RigidBody, collider: Collider) -> RigidBodyHandle {
        let rbhandle = self.rigidbody_set.insert(rb);
        self.collider_set
            .insert_with_parent(collider, rbhandle, &mut self.rigidbody_set);
        rbhandle
    }

    pub fn get_collision_events(&mut self) -> Vec<CollisionEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_receiver.0.try_recv() {
            events.push(event);
        }
        events
    }
}

pub trait PhysicsObject {
    fn insert_into_physics(&mut self, rbtype: RigidBodyType, physics: &mut Physics);
}
