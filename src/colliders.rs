use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle{
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle{
    fn from(entity_instance: &EntityInstance) -> Self {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        
        match (entity_instance.identifier.as_ref()){
            "Player" => {
                Self{
                    collider: Collider::cuboid(8., 11.5),
                    rigid_body: RigidBody::Dynamic,
                    friction: Friction{
                        coefficient: 0.,
                        combine_rule: CoefficientCombineRule::Average,
                    },
                    rotation_constraints,
                    ..Default::default()
                }
            },
            _ => ColliderBundle::default()
        }
    }
}


#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle{
    pub collider : Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}