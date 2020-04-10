use vek::vec3::Vec3;

pub trait Volume<T> where T: Default {
    fn get_position(&self) -> Vec3<i32>;
    fn get_voxel(&self) -> T;
    fn set_position(&mut self, x: i32, y: i32, z: i32);
    fn move_positive_x(&mut self);
    fn move_positive_y(&mut self);
    fn move_positive_z(&mut self);
    fn move_negative_x(&mut self);
    fn move_negative_y(&mut self);
    fn move_negative_z(&mut self);

    fn get_voxel_at(&self, x: i32, y: i32, z: i32) -> T;
    fn set_voxel_at(&mut self, x: i32, y: i32, z: i32, voxel: T);

    fn calculate_size_in_bytes(&self) -> usize;
}