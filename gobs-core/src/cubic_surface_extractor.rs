use vek::vec3::Vec3;

use crate::vertex::Vertex;
use crate::mesh::Mesh;
use crate::volume::Volume;
use crate::region::Region;
use std::ops::Fn;
use crate::voxel::Voxel;

pub fn extract_cubic_mesh_custom<T, F>(volume: &dyn Volume<T>, region: &Region, result: &mut Mesh<T>, is_quad_needed: F, merge_quads: bool)
    where T: Voxel, F: Fn(&T, &T) -> Option<T> {

    result.clear();
}


pub fn extract_cubic_mesh<T>(volume: &dyn Volume<T>, region: &Region) -> Mesh<T> where T: Voxel {
    let mut mesh : Mesh<T> = Mesh::new();

    extract_cubic_mesh_custom(volume, region, &mut mesh,|back, front| {
        if !back.is_empty() && front.is_empty() {
            Some(back.clone())
        } else {
            None
        }
    }, true);

    mesh
}