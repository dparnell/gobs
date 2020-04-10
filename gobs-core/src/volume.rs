use vek::vec3::Vec3;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::region::Region;

#[derive(Debug)]
pub struct PositionError {
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "position is outside volume")
    }
}

impl Error for PositionError {
}


pub trait Volume<T> where T: Default {
    fn get_region(&self) -> &Region;
    fn get_position(&self) -> Vec3<i32>;
    fn get_voxel(&self) -> T;
    fn set_voxel(&mut self, voxel: T) -> Result<(), PositionError>;
    fn set_position(&mut self, x: i32, y: i32, z: i32);
    fn move_positive_x(&mut self);
    fn move_positive_y(&mut self);
    fn move_positive_z(&mut self);
    fn move_negative_x(&mut self);
    fn move_negative_y(&mut self);
    fn move_negative_z(&mut self);

    fn get_voxel_at(&self, x: i32, y: i32, z: i32) -> T;
    fn set_voxel_at(&mut self, x: i32, y: i32, z: i32, voxel: T)  -> Result<(), PositionError>;

    fn calculate_size_in_bytes(&self) -> usize;
}