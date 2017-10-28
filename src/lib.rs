pub extern crate bulletrs_sys as sys;
extern crate mint;

mod bullet;
mod errors;
mod physics_client;
mod command;
mod status;
mod shape;
mod dynamicsinfo;
mod rigidbody;

pub use mint::{Point3, Vector3, Vector4};
pub use errors::Error;

pub use bullet::{Bullet, ConnectMethod};
pub use physics_client::PhysicsClientHandle;
pub use shape::{Shape, ShapeType};
pub use dynamicsinfo::DynamicsInfo;
pub use rigidbody::{RigidBodyHandle};

