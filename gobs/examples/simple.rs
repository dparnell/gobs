#[macro_use]
extern crate glium;

use gobs::region::Region;
use gobs::raw_volume::RawVolume;
use gobs::volume::Volume;
use gobs::raw_volume_sampler::RawVolumeSampler;
use gobs::cubic_surface_extractor::extract_cubic_mesh;
use crate::support::main_loop::{run, VoxelVertex};

pub mod support;

const RED: u32   = 0x00ff0000;
const GREEN: u32 = 0x0000ff00;
const BLUE: u32  = 0x000000ff;

fn main() {
    run(|display| {
        let region = Region::cubic(16);
        let mut volume: RawVolume<u32> = RawVolume::new(region);
        volume.set_voxel_at(0, 1, 8, RED).unwrap();
        volume.set_voxel_at(1, 1, 8, GREEN).unwrap();
        volume.set_voxel_at(2, 1, 8, BLUE).unwrap();
        volume.set_voxel_at(3, 1, 8, RED).unwrap();

        let mut sampler = RawVolumeSampler::new(&volume);
        let mesh = extract_cubic_mesh(&mut sampler, &Region::cubic(16)).unwrap();

        let mut vertex_data = Vec::new();

        for i in mesh.indices {
            let v = mesh.vertices.get(i as usize).unwrap();

            let m = v.data;
            let pos = v.decode();

            let scale_factor: f32 = 1.0 / 16.0;
            vertex_data.push(VoxelVertex{
                position: [pos.x as f32 * scale_factor, pos.y as f32 * scale_factor, pos.z as f32 * scale_factor, 1.0],
                colour: m
            })
        }
        glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into()
    })
}