use crate::volume::{Volume, PositionError};
use crate::region::Region;
use crate::voxel::Voxel;

pub struct RawVolume<T> where T: Voxel {
    pub data: Vec<T>,
    pub border_value: T,
    pub valid_region: Region,
}

impl <T> RawVolume<T> where T: Voxel {
    pub fn new(ref region: Region) -> Self {
        RawVolume{
            data: vec![Default::default(); region.get_volume() as usize],
            border_value: Default::default(),
            valid_region: region.clone(),
        }
    }

    fn get_offset(&self, x: i32, y: i32, z: i32) -> Option<usize> {
        if self.valid_region.contains_point(x, y, z) {
            let corner = self.valid_region.get_lower_corner();
            let local_x = x - corner.x;
            let local_y = y - corner.y;
            let local_z = z - corner.z;

            let width = self.valid_region.get_width();
            let height = self.valid_region.get_height();

            Some((local_x + local_y * width + local_z * width * height) as usize)
        } else {
            None
        }
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn set_border_value(&mut self, value: T) {
        self.border_value = value;
    }
}

impl <T> Volume<T> for RawVolume<T> where T: Voxel {
    fn get_region(&self) -> &Region {
        &self.valid_region
    }

    fn get_voxel_at(&self, x: i32, y: i32, z: i32) -> T {
        self.get_offset(x, y, z).map_or(self.border_value, |offset| { self.data[offset] })
    }

    fn set_voxel_at(&mut self, x: i32, y: i32, z: i32, voxel: T) -> Result<(), PositionError> {
        self.get_offset(x, y, z).map_or(Err(PositionError{}),|offset| { self.data[offset] = voxel; Ok(()) })
    }

    fn calculate_size_in_bytes(&self) -> usize {
        std::mem::size_of::<T>() * self.valid_region.get_volume() as usize
    }

    fn get_border_value(&self) -> T {
        self.border_value
    }
}