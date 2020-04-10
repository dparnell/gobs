use vek::vec3::Vec3;

#[derive(Clone,Debug)]
pub struct Region {
    pub lower_x: i32,
    pub lower_y: i32,
    pub lower_z: i32,
    pub upper_x: i32,
    pub upper_y: i32,
    pub upper_z: i32
}

impl Region {
    pub fn new(lower: Vec3<i32>, upper: Vec3<i32>) -> Self {
        Region{
            lower_x: lower.x,
            lower_y: lower.y,
            lower_z: lower.z,
            upper_x: upper.x,
            upper_y: upper.y,
            upper_z: upper.z
        }
    }

    pub fn get_width(&self) -> i32 {
        self.upper_x - self.lower_x + 1
    }

    pub fn get_height(&self) -> i32 {
        self.upper_y - self.lower_y + 1
    }

    pub fn get_depth(&self) -> i32 {
        self.upper_z - self.lower_z + 1
    }
}
