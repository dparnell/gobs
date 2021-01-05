use dot_vox::load_bytes;
use gobs::cubic_surface_extractor::extract_cubic_mesh;
use gobs::mesh::FaceArity;
use gobs::raw_volume::RawVolume;
use gobs::raw_volume_sampler::RawVolumeSampler;
use gobs::region::Region;
use gobs::volume::Volume;
use std::{fs::File, io::Write, time::SystemTime};

use itertools::Itertools;

fn main() -> Result<(), std::io::Error> {
    let name = "chr_super2";

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

    vox_model.voxels.iter().for_each(|voxel| {
        volume
            .set_voxel_at(voxel.x as i32, voxel.y as i32, voxel.z as i32, 1)
            .unwrap();
    });

    println!("generating mesh...");
    let start_time = SystemTime::now();
    let mesh = extract_cubic_mesh(
        &mut RawVolumeSampler::new(&volume),
        &model_region,
        Some(FaceArity::Four),
        Some(false),
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

    let path = format!("{}.obj", name);
    let mut file = File::create(path)?;

    writeln!(file, "o {}", name)?;

    for vertex in mesh.vertices() {
        let pos = vertex.decode();
        writeln!(
            file,
            "v {} {} {}",
            (pos.x as i32 + x_offset) as f32 * scale_factor,
            (pos.y as i32 + y_offset) as f32 * scale_factor,
            (pos.z as i32 + z_offset) as f32 * scale_factor,
        )?;
    }

    for quad in mesh.indices().iter().tuples::<(_, _, _, _)>() {
        writeln!(
            file,
            "f {} {} {} {}",
            quad.0 + 1,
            quad.1 + 1,
            quad.2 + 1,
            quad.3 + 1
        )?;
    }

    file.flush()?;

    println!("exported mesh as {}.obj", name);

    Ok(())
}
