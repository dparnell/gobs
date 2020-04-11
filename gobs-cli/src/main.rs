use gobs_core::raw_volume::RawVolume;
use gobs_core::region::Region;
use gobs_core::volume::Volume;
use gobs_core::raw_volume_sampler::RawVolumeSampler;
use gobs_core::cubic_surface_extractor::extract_cubic_mesh;

fn main() {
    let region = Region::cubic(16);
    let mut volume: RawVolume<i32> = RawVolume::new(region);
    volume.set_voxel_at(8, 8, 8, 1).unwrap();
    //volume.set_voxel_at(9, 8, 8, 1).unwrap();
    //volume.set_voxel_at(10, 8, 8, 1).unwrap();

    let mut sampler = RawVolumeSampler::new(&volume);
    let mesh = extract_cubic_mesh(&mut sampler, &Region::cubic(16)).unwrap();

    let vertex_count = mesh.vertices.len();
    let index_count = mesh.indices.len();

    println!("vertex count = {}, index count = {}", vertex_count, index_count);
    println!("{:?}", mesh.vertices);
}
