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

    pub fn get_volume(&self) -> i32 {
        self.get_depth() * self.get_height() * self.get_width()
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

    pub fn get_area(&self) -> i32 {
        self.get_width() * self.get_height()
    }

    pub fn contains_point(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.lower_x && x <= self.upper_x && y >= self.lower_y && y <= self.upper_y && z >= self.lower_z && z <= self.upper_z
    }

    pub fn contains_point_excluding_boundary(&self, x: i32, y: i32, z: i32, boundary: i32) -> bool {
        x >= (self.lower_x + boundary) && x <= (self.upper_x - boundary) && y >= (self.lower_y + boundary) && y <= (self.upper_y - boundary) && z >= (self.lower_z + boundary) && z <= (self.upper_z - boundary)
    }

    pub fn get_lower_corner(&self) -> Vec3<i32> {
        Vec3 {
            x: self.lower_x,
            y: self.lower_y,
            z: self.lower_z
        }
    }

    pub fn contains_point_in_x(&self, x:i32) -> bool {
        x >= self.lower_x && x <= self.upper_x
    }

    pub fn contains_point_in_y(&self, y:i32) -> bool {
        y >= self.lower_y && y <= self.upper_y
    }

    pub fn contains_point_in_z(&self, z:i32) -> bool {
        z >= self.lower_z && z <= self.upper_z
    }
}
