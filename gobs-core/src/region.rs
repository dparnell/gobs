use vek::vec3::Vec3;

struct Region {
    lower_x: i32,
    lower_y: i32,
    lower_z: i32,
    upper_x: i32,
    upper_y: i32,
    upper_z: i32
}

impl Region {
    fn new(lower: Vec3<i32>, upper: Vec3<i32>) -> Self {
        Region{
            lower_x: lower.x,
            lower_y: lower.y,
            lower_z: lower.z,
            upper_x: upper.x,
            upper_y: upper.y,
            upper_z: upper.z
        }
    }


}
