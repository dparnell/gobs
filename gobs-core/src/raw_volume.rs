use crate::volume::{Volume, PositionError};
use crate::region::Region;
use crate::voxel::Voxel;

pub struct RawVolume<T> where T: Voxel {
    data: Vec<T>,
    border_value: T,
    valid_region: Region,

}

impl <T> RawVolume<T> where T: Voxel {
    pub fn new(ref region: Region) -> Self {
        RawVolume{
            data: vec![Default::default(); region.get_volume() as usize],
            border_value: Default::default(),
            valid_region: region.clone(),
        }
    }

    fn get_offset(&self, x: i32, y: i32, z: i32) -> Result<usize, PositionError> {
        if self.valid_region.contains_point(x, y, z) {
            let corner = self.valid_region.get_lower_corner();
            let local_x = x - corner.x;
            let local_y = y - corner.y;
            let local_z = z - corner.z;

            let width = self.valid_region.get_width();
            let height = self.valid_region.get_height();

            Ok((local_x + local_y * width + local_z * width * height) as usize)
        } else {
            Err(PositionError{})
        }
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

}

impl <T> Volume<T> for RawVolume<T> where T: Voxel {
    fn get_region(&self) -> &Region {
        &self.valid_region
    }

    fn get_voxel_at(&self, x: i32, y: i32, z: i32) -> T {
        match self.get_offset(x, y, z) {
            Ok(offset) => self.data[offset],
            _ => self.border_value
        }
    }

    fn set_voxel_at(&mut self, x: i32, y: i32, z: i32, voxel: T) -> Result<(), PositionError> {
        let offset = self.get_offset(x,y,z)?;
        self.data[offset] = voxel;

        Ok(())
    }

    fn calculate_size_in_bytes(&self) -> usize {
        std::mem::size_of::<T>() * self.valid_region.get_volume() as usize
    }

    fn get_border_value(&self) -> T {
        self.border_value
    }
}