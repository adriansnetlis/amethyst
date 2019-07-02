use crate::storage::StoreTag;

use nphysics3d::object::{
    BodyHandle as NpBodyHandle,
    ColliderHandle as NpColliderHandle,
};

use amethyst_phythyst::objects::*;

pub struct RigidBody {
    pub body_handle: NpBodyHandle,
    pub collider_handle: Option<NpColliderHandle>,
    pub world_tag: PhysicsWorldTag,
    pub shape_tag: Option<PhysicsShapeTag>,
}

impl RigidBody {
    pub(crate) fn new(world_tag: PhysicsWorldTag) -> Box<Self> {

        Box::new(RigidBody {
            body_handle: NpBodyHandle::ground(),
            collider_handle: None,
            world_tag,
            shape_tag: None,
        })
    }
}