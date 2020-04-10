use vek::vec3::Vec3;
use crate::region::Region;
use crate::sampler::Sampler;
use crate::volume::{Volume, PositionError};
use crate::raw_volume::RawVolume;

struct RawVolumeSampler<'a, T> where T: Default + Copy {
    data: &'a Vec<T>,
    valid_region: Region,
    x_pos: i32,
    y_pos: i32,
    z_pos: i32,
    current_voxel: Option<usize>,
    current_x_valid: bool,
    current_y_valid: bool,
    current_z_valid: bool,
    border_value: T
}

impl <'a, T> RawVolumeSampler<'a, T> where T: Default + Copy {
    pub fn new(volume: &'a RawVolume<T>) -> Self {
        let region = volume.get_region().clone();
        let x = region.lower_x;
        let y = region.lower_y;
        let z = region.lower_z;

        RawVolumeSampler {
            data: volume.get_data(),
            valid_region: region,
            x_pos: x,
            y_pos: y,
            z_pos: z,
            current_voxel: Some(0),
            current_x_valid: true,
            current_y_valid: true,
            current_z_valid: true,
            border_value: volume.get_border_value()
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

impl<'a, T> Sampler<T> for RawVolumeSampler<'a, T> where T: Default + Copy {

    fn get_position(&self) -> Vec3<i32> {
        Vec3{
            x: self.x_pos,
            y: self.y_pos,
            z: self.z_pos
        }
    }

    fn get_voxel(&self) -> T {
        match self.current_voxel {
            Some(offset) => self.data[offset],
            _ => self.border_value
        }
    }

    fn set_position(&mut self, x: i32, y: i32, z: i32) {
        self.x_pos = x;
        self.y_pos = y;
        self.z_pos = z;

        self.current_x_valid = self.valid_region.contains_point_in_x(x);
        self.current_y_valid = self.valid_region.contains_point_in_y(y);
        self.current_z_valid = self.valid_region.contains_point_in_z(z);

        match self.get_offset(x, y, z) {
            Ok(offset) => self.current_voxel = Some(offset),
            _ => self.current_voxel = None
        }
    }

    fn move_positive_x(&mut self) {
        let was_valid = self.is_current_position_valid();
        self.x_pos = self.x_pos + 1;
        self.current_x_valid = self.valid_region.contains_point_in_x(self.x_pos);
        if was_valid && self.is_current_position_valid() {
            self.current_voxel = Some(self.current_voxel.unwrap() + 1);
        } else {
            self.current_voxel = None
        }
    }

    fn move_positive_y(&mut self) {
        let was_valid = self.is_current_position_valid();
        self.y_pos = self.y_pos + 1;
        self.current_y_valid = self.valid_region.contains_point_in_y(self.y_pos);
        if was_valid && self.is_current_position_valid() {
            self.current_voxel = Some(self.current_voxel.unwrap() + self.valid_region.get_width() as usize);
        } else {
            self.current_voxel = None
        }
    }

    fn move_positive_z(&mut self) {
        let was_valid = self.is_current_position_valid();
        self.z_pos = self.z_pos + 1;
        self.current_z_valid = self.valid_region.contains_point_in_z(self.z_pos);
        if was_valid && self.is_current_position_valid() {
            self.current_voxel = Some(self.current_voxel.unwrap() + self.valid_region.get_area() as usize)
        } else {
            self.current_voxel = None
        }
    }

    fn move_negative_x(&mut self) {
        let was_valid = self.is_current_position_valid();
        self.x_pos = self.x_pos - 1;
        self.current_x_valid = self.valid_region.contains_point_in_x(self.x_pos);
        if was_valid && self.is_current_position_valid() {
            self.current_voxel = Some(self.current_voxel.unwrap() - 1);
        } else {
            self.current_voxel = None
        }
    }

    fn move_negative_y(&mut self) {
        let was_valid = self.is_current_position_valid();
        self.y_pos = self.y_pos - 1;
        self.current_y_valid = self.valid_region.contains_point_in_y(self.y_pos);
        if was_valid && self.is_current_position_valid() {
            self.current_voxel = Some(self.current_voxel.unwrap() - self.valid_region.get_width() as usize);
        } else {
            self.current_voxel = None
        }
    }

    fn move_negative_z(&mut self) {
        let was_valid = self.is_current_position_valid();
        self.z_pos = self.z_pos - 1;
        self.current_z_valid = self.valid_region.contains_point_in_z(self.z_pos);
        if was_valid && self.is_current_position_valid() {
            self.current_voxel = Some(self.current_voxel.unwrap() - self.valid_region.get_area() as usize)
        } else {
            self.current_voxel = None
        }
    }

    fn is_current_position_valid(&self) -> bool {
        self.current_x_valid && self.current_y_valid && self.current_z_valid
    }

    fn peek_voxel_1nx1ny1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx1ny0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx1ny1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx0py1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx0py0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx0py1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx1py1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx1py0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1nx1py1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px1ny1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px1ny0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px1ny1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px0py1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px0py0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px0py1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px1py1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px1py0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_0px1py1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px1ny1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px1ny0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px1ny1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px0py1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px0py0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px0py1pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px1py1nz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px1py0pz(&self) -> T {
        unimplemented!()
    }

    fn peek_voxel_1px1py1pz(&self) -> T {
        unimplemented!()
    }
}