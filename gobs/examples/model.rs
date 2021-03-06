#[macro_use]
extern crate glium;
extern crate dot_vox;

mod support;

use dot_vox::load_bytes;
use gobs::cubic_surface_extractor::extract_cubic_mesh;
use gobs::raw_volume::RawVolume;
use gobs::raw_volume_sampler::RawVolumeSampler;
use gobs::region::Region;
use gobs::volume::Volume;
use std::time::SystemTime;
use support::main_loop::{run, VoxelVertex};

fn main() {
    run(|display| {
        let vox_file = load_bytes(include_bytes!("chr_super2.vox")).unwrap();
        println!("Loaded vox file. Found {} models", vox_file.models.len());
        let vox_model = vox_file.models.first().unwrap();
        println!(
            "model is {} wide, {} high and {} deep",
            vox_model.size.x, vox_model.size.y, vox_model.size.z
        );
        println!("model contains data for {} voxels", vox_model.voxels.len());

        let model_region = Region::sized(
            vox_model.size.x as i32,
            vox_model.size.y as i32,
            vox_model.size.z as i32,
        );
        let mut volume: RawVolume<u32> = RawVolume::new(model_region.clone());

        let palette: Vec<u32> = vox_file
            .palette
            .iter()
            .map(|rgba| {
                let a = (rgba >> 24) as u8;
                let b = (rgba >> 16) as u8;
                let g = (rgba >> 8) as u8;
                let r = *rgba as u8;

                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect();

        vox_model.voxels.iter().for_each(|voxel| {
            volume
                .set_voxel_at(
                    voxel.x as i32,
                    voxel.y as i32,
                    voxel.z as i32,
                    palette[voxel.i as usize],
                )
                .unwrap();
        });

        println!("generating mesh...");
        let start_time = SystemTime::now();
        let mesh = extract_cubic_mesh(
            &mut RawVolumeSampler::new(&volume),
            &model_region,
            None,
            None,
        )
        .unwrap();
        println!(
            "generated mesh in {}ms",
            SystemTime::now()
                .duration_since(start_time)
                .unwrap()
                .as_millis()
        );

        let x_offset = -model_region.get_width() / 2;
        let y_offset = -model_region.get_height() / 2;
        let z_offset = -model_region.get_depth() / 2;

        let scale_factor: f32 = 1.0 / (model_region.get_width() as f32);

        let vertex_data: Vec<_> = mesh
            .indices()
            .iter()
            .map(|i| {
                let v = &mesh.vertices()[*i as usize];

                let m = v.data;
                let pos = v.decode();

                VoxelVertex {
                    position: [
                        (pos.x as i32 + x_offset) as f32 * scale_factor,
                        (pos.y as i32 + y_offset) as f32 * scale_factor,
                        (pos.z as i32 + z_offset) as f32 * scale_factor,
                        1.0,
                    ],
                    colour: m,
                }
            })
            .collect();

        println!("mesh contains {} vertices", vertex_data.len());

        glium::vertex::VertexBuffer::new(display, &vertex_data)
            .unwrap()
            .into()
    })
}
