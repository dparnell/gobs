use gobs_core::cubic_surface_extractor;
use gobs_core::region::Region;
use gobs_core::raw_volume::RawVolume;
use gobs_core::raw_volume_sampler::RawVolumeSampler;
use gobs_core::cubic_surface_extractor::extract_cubic_mesh;
use gobs_core::volume::Volume;

#[test]
fn basic_case() {
    let region = Region::cubic(16);
    let mut volume: RawVolume<i32> = RawVolume::new(region);
    volume.set_voxel_at(8, 8, 8, 1).unwrap();


    let mut sampler = RawVolumeSampler::new(&volume);
    let mesh = extract_cubic_mesh(&mut sampler, &Region::cubic(16));

    assert_eq!(8, mesh.vertices.len());
    assert_eq!(36, mesh.indices.len());
}