use vek::vec3::Vec3;

pub trait Sampler<T> where T: Default + Copy  {

    fn get_position(&self) -> Vec3<i32>;
    fn get_voxel(&self) -> T;
    fn set_position(&mut self, x: i32, y: i32, z: i32);

    fn move_positive_x(&mut self);
    fn move_positive_y(&mut self);
    fn move_positive_z(&mut self);
    fn move_negative_x(&mut self);
    fn move_negative_y(&mut self);
    fn move_negative_z(&mut self);

    fn is_current_position_valid(&self) -> bool;

    fn peek_voxel_1nx1ny1nz(&self) -> T;
    fn peek_voxel_1nx1ny0pz(&self) -> T;
    fn peek_voxel_1nx1ny1pz(&self) -> T;
    fn peek_voxel_1nx0py1nz(&self) -> T;
    fn peek_voxel_1nx0py0pz(&self) -> T;
    fn peek_voxel_1nx0py1pz(&self) -> T;
    fn peek_voxel_1nx1py1nz(&self) -> T;
    fn peek_voxel_1nx1py0pz(&self) -> T;
    fn peek_voxel_1nx1py1pz(&self) -> T;

    fn peek_voxel_0px1ny1nz(&self) -> T;
    fn peek_voxel_0px1ny0pz(&self) -> T;
    fn peek_voxel_0px1ny1pz(&self) -> T;
    fn peek_voxel_0px0py1nz(&self) -> T;
    fn peek_voxel_0px0py0pz(&self) -> T;
    fn peek_voxel_0px0py1pz(&self) -> T;
    fn peek_voxel_0px1py1nz(&self) -> T;
    fn peek_voxel_0px1py0pz(&self) -> T;
    fn peek_voxel_0px1py1pz(&self) -> T;

    fn peek_voxel_1px1ny1nz(&self) -> T;
    fn peek_voxel_1px1ny0pz(&self) -> T;
    fn peek_voxel_1px1ny1pz(&self) -> T;
    fn peek_voxel_1px0py1nz(&self) -> T;
    fn peek_voxel_1px0py0pz(&self) -> T;
    fn peek_voxel_1px0py1pz(&self) -> T;
    fn peek_voxel_1px1py1nz(&self) -> T;
    fn peek_voxel_1px1py0pz(&self) -> T;
    fn peek_voxel_1px1py1pz(&self) -> T;
}