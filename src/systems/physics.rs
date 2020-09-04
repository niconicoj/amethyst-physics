use amethyst::{
    core::SystemDesc,
    ecs::{System, SystemData, World, WriteExpect},
};
use rapier2d::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use rapier2d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier2d::na::Vector2;
use rapier2d::pipeline::PhysicsPipeline;

pub struct PhysicsSystemDesc {
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
}

impl Default for PhysicsSystemDesc {
    fn default() -> Self {
        PhysicsSystemDesc {
            gravity: Vector2::new(0.0, -9.81),
            integration_parameters: IntegrationParameters::default(),
        }
    }
}

impl<'a, 'b> SystemDesc<'a, 'b, PhysicsSystem> for PhysicsSystemDesc {
    fn build(self, world: &mut World) -> PhysicsSystem {
        <PhysicsSystem as System<'_>>::SystemData::setup(world);

        // insert the rigidbodyset and colliderset in the world, init the system with it's required params
        world.insert(RigidBodySet::new());
        world.insert(ColliderSet::new());
        world.insert(JointSet::new());

        PhysicsSystem::new(self.integration_parameters, self.gravity)
    }
}

pub struct PhysicsSystem {
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
}

impl PhysicsSystem {
    pub fn new(integration_parameters: IntegrationParameters, gravity: Vector2<f32>) -> Self {
        PhysicsSystem {
            pipeline: PhysicsPipeline::new(),
            gravity,
            integration_parameters,
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
        }
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteExpect<'a, RigidBodySet>,
        WriteExpect<'a, ColliderSet>,
        WriteExpect<'a, JointSet>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut bodies, mut colliders, mut joints) = data;
        self.pipeline.step(
            &self.gravity, 
            &self.integration_parameters, 
            &mut self.broad_phase, 
            &mut self.narrow_phase, 
            &mut bodies, 
            &mut colliders, 
            &mut joints, 
            &()
        );
    }
}