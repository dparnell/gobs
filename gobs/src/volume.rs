use crate::region::Region;
use crate::voxel::Voxel;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct PositionError {}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "position is outside volume")
    }
}

impl Error for PositionError {}

pub trait Volume<T>
where
    T: Voxel,
{
    fn get_region(&self) -> &Region;

    fn get_voxel_at(&self, x: i32, y: i32, z: i32) -> T;
    fn set_voxel_at(&mut self, x: i32, y: i32, z: i32, voxel: T) -> Result<(), PositionError>;

    fn calculate_size_in_bytes(&self) -> usize;

    fn get_border_value(&self) -> T;
}
