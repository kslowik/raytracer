use serde::{Deserialize, Serialize};

use crate::camera::Camera;
use crate::hittable::ObjectList;
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub camera: Camera,
    pub object_list: ObjectList, // right now the only object it sphere
}
