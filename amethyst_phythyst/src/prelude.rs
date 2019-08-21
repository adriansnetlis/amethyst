//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    objects::{
        PhysicsAreaTag, PhysicsGarbageCollector, PhysicsHandle, PhysicsRigidBodyTag,
        PhysicsShapeTag, PhysicsTag,
    },
    servers::{
        AreaDesc, AreaPhysicsServerTrait, BodyMode, PhysicsWorld, RBodyPhysicsServerTrait,
        RigidBodyDesc, ShapeDesc, ShapePhysicsServerTrait, WorldPhysicsServerTrait,
    },
};
