pub mod vector3;
pub mod view_point;
pub mod color;
pub mod collision;

pub use vector3::Vector3;
pub use color::Color;
pub use view_point::{ViewPoint, Photon};
pub use collision::{ Collider, LightCollider };

#[derive(Debug, Clone)]
pub struct Ray {
    pub o : Vector3,
    pub d : Vector3,
}

impl Ray {
    pub fn new(_o : Vector3, _d : Vector3) -> Self {
        Ray { o : _o, d : _d }
    }
}
