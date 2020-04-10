use vek::vec3::Vec3;
use crate::volume::{Volume, PositionError};
use crate::region::Region;

pub struct RawVolume<T> where T: Default {
    data: Vec<T>,
    border_value: T,
    valid_region: Region,
    x_pos: i32,
    y_pos: i32,
    z_pos: i32
}

impl <T> RawVolume<T> where T: Default + Copy {
    pub fn new(ref region: Region) -> Self {
        RawVolume{
            data: vec![Default::default(); region.get_volume() as usize],
            border_value: Default::default(),
            valid_region: region.clone(),
            x_pos: 0,
            y_pos: 0,
            z_pos: 0
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

}

impl <T> Volume<T> for RawVolume<T> where T: Default + Copy {
    fn get_region(&self) -> &Region {
        &self.valid_region
    }

    fn get_position(&self) -> Vec3<i32> {
        Vec3{
            x: self.x_pos,
            y: self.y_pos,
            z: self.z_pos
        }
    }

    fn get_voxel(&self) -> T {
        self.get_voxel_at(self.x_pos, self.y_pos, self.z_pos)
    }

    fn set_voxel(&mut self, voxel: T) -> Result<(), PositionError> {
        self.set_voxel_at(self.x_pos, self.y_pos, self.y_pos, voxel)
    }

    fn set_position(&mut self, x: i32, y: i32, z: i32) {
        self.x_pos = x;
        self.y_pos = y;
        self.z_pos = z;
    }

    fn move_positive_x(&mut self) {
        self.x_pos = self.x_pos + 1;
    }

    fn move_positive_y(&mut self) {
        self.y_pos = self.y_pos + 1;
    }

    fn move_positive_z(&mut self) {
        self.z_pos = self.z_pos + 1;
    }

    fn move_negative_x(&mut self) {
        self.x_pos = self.x_pos - 1;
    }

    fn move_negative_y(&mut self) {
        self.y_pos = self.y_pos - 1;
    }

    fn move_negative_z(&mut self) {
        self.z_pos = self.z_pos - 1;
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
}