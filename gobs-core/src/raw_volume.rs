use vek::vec3::Vec3;
use crate::volume::Volume;
use crate::region::Region;

pub struct RawVolume<T> where T: Default {
    data: Vec<T>,
    border_value: T,
    valid_region: Region
}

impl <T> RawVolume<T> where T: Default + Clone {
    pub fn new(ref region: Region) -> Self {
        RawVolume{
            data: vec![Default::default(); (region.get_width() * region.get_height() * region.get_depth()) as usize],
            border_value: Default::default(),
            valid_region: region.clone()
        }
    }
}

impl <T> Volume<T> for RawVolume<T> where T: Default {
    fn get_position(&self) -> Vec3<i32> {
        unimplemented!()
    }

    fn get_voxel(&self) -> T {
        unimplemented!()
    }

    fn set_position(&mut self, x: i32, y: i32, z: i32) {
        unimplemented!()
    }

    fn move_positive_x(&mut self) {
        unimplemented!()
    }

    fn move_positive_y(&mut self) {
        unimplemented!()
    }

    fn move_positive_z(&mut self) {
        unimplemented!()
    }

    fn move_negative_x(&mut self) {
        unimplemented!()
    }

    fn move_negative_y(&mut self) {
        unimplemented!()
    }

    fn move_negative_z(&mut self) {
        unimplemented!()
    }

    fn get_voxel_at(&self, x: i32, y: i32, z: i32) -> T {
        unimplemented!()
    }

    fn set_voxel_at(&mut self, x: i32, y: i32, z: i32, voxel: T) {
        unimplemented!()
    }

    fn calculate_size_in_bytes(&self) -> usize {
        unimplemented!()
    }
}