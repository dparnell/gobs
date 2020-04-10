use vek::vec3::Vec3;

use crate::mesh::Mesh;
use crate::region::Region;
use crate::voxel::Voxel;
use crate::sampler::Sampler;

const MAX_VERTICES_PER_POSITION: usize = 8;

pub struct CubicVertex<T> where T: Voxel {
    pub position: u32,
    pub data: T
}

impl <T> CubicVertex<T> where T: Voxel {
    pub fn new(x: u8, y: u8, z: u8, data: T) -> Self {
        CubicVertex{
            position: x as u32 + (y as u32 * 0x100) + (z as u32 * 0x10000),
            data
        }
    }

    pub fn decode(&self) -> Vec3<u8> {
        Vec3{
            x: (self.position & 0xff) as u8,
            y: ((self.position >> 8) & 0xff) as u8,
            z: ((self.position >> 16) & 0xff) as u8
        }
    }
}

#[derive(Default, Clone)]
struct Quad {
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32
}

impl Quad {
    fn new(v0: i32, v1: i32, v2: i32, v3: i32) -> Self {
        Quad{
            v0, v1, v2, v3
        }
    }

    fn maybe_merge<T>(&mut self, other: &Self, mesh: &Mesh<CubicVertex<T>>) -> bool where T: Voxel {
        //All four vertices of a given quad have the same data,
        //so just check that the first pair of vertices match.
        if mesh.get_vertex(self.v0 as usize).data == mesh.get_vertex(other.v0 as usize).data {

            if self.v0 == other.v1 && self.v3 == other.v2 {
                self.v0 = other.v0;
                self.v3 = other.v3;

                return true
            }

            if self.v3 == other.v0 && self.v2 == other.v1 {
                self.v3 = other.v3;
                self.v2 = other.v2;

                return true;
            }

            if self.v1 == other.v0 && self.v2 == other.v3 {
                self.v1 = other.v1;
                self.v2 = other.v2;

                return true;
            }

            if self.v0 == other.v3 && self.v1 == other.v2 {
                self.v0 = other.v0;
                self.v1 = other.v1;

                return true;
            }
        }

        false
    }
}

#[derive(Clone)]
struct IndexAndMaterial<T> where T: Voxel {
    pub index: i32,
    pub material: T
}

impl <T> Default for IndexAndMaterial<T> where T: Voxel {
    fn default() -> Self {
        IndexAndMaterial{
            index: -1,
            material: Default::default()
        }
    }
}

struct Array3<T> where T: Default + Clone {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub area: usize
}

impl <'a, T> Array3<T> where T: Default + Clone {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        Array3{
            data: vec![Default::default(); width * height * depth],
            width,
            height,
            depth,
            area: width * height
        }
    }
}

fn add_vertex<T>(x: u32, y: u32, z: u32, material: T, existing_vertices: &mut Array3<IndexAndMaterial<T>>, result: &mut Mesh<CubicVertex<T>>) -> i32
    where T: Voxel {

    let width = existing_vertices.width;
    let area = existing_vertices.area;

    let data = existing_vertices.data.as_mut_slice();
    for ct in 0 .. MAX_VERTICES_PER_POSITION {
        let idx = x as usize + (y as usize) * width + ct * area;
        if let Some(item) = data.get_mut(idx) {
           if item.index == -1 {
                let vert: CubicVertex<T> = CubicVertex::new(x as u8, y as u8, z as u8, material);

               item.index = result.add_vertex(vert) as i32;
               item.material = material;

               return item.index;
           } else {
               if item.material == material {
                   return item.index
               }
           }
        }
    }

    return -1;
}

pub fn extract_cubic_mesh_custom<T, F>(sampler: &mut dyn Sampler<T>, region: &Region, result: &mut Mesh<CubicVertex<T>>, is_quad_needed: F, merge_quads: bool)
    where T: Voxel, F: Fn(&T, &T) -> Option<T> {

    result.clear();

    let width = (region.get_width() + 2) as usize;
    let height = (region.get_height() + 2) as usize;
    let depth = (region.get_depth() + 2) as usize;

    let mut prev_slice_vertices: Array3<IndexAndMaterial<T>> = Array3::new(width, height, MAX_VERTICES_PER_POSITION);
    let mut current_slice_vertices: Array3<IndexAndMaterial<T>> = Array3::new(width, height, MAX_VERTICES_PER_POSITION);

    let mut neg_x_quads: Vec<Vec<Quad>> = vec![Default::default(); width];
    let mut pos_x_quads: Vec<Vec<Quad>> = vec![Default::default(); width];
    let mut neg_y_quads: Vec<Vec<Quad>> = vec![Default::default(); height];
    let mut pos_y_quads: Vec<Vec<Quad>> = vec![Default::default(); height];
    let mut neg_z_quads: Vec<Vec<Quad>> = vec![Default::default(); depth];
    let mut pos_z_quads: Vec<Vec<Quad>> = vec![Default::default(); depth];

    for z in region.lower_z ..= region.upper_z {
        let reg_z = (z - region.lower_z) as u32;

        for y in region.lower_y ..= region.upper_y {
            let reg_y = (y - region.lower_y) as u32;
            sampler.set_position(region.lower_x, y, z);

            for x in region.lower_x ..= region.upper_x {
                let reg_x = (x - region.lower_x) as u32;

                let current_voxel = sampler.get_voxel();
                let neg_x_voxel = sampler.peek_voxel_1nx0py0pz();
                let neg_y_voxel = sampler.peek_voxel_0px1ny0pz();
                let neg_z_voxel = sampler.peek_voxel_0px0py1nz();

                // X
                if let Some(material) = is_quad_needed(&current_voxel, &neg_x_voxel) {
                    let v0 = add_vertex(reg_x, reg_y, reg_z, material, &mut prev_slice_vertices, result);
                    let v1 = add_vertex(reg_x, reg_y, reg_z + 1, material, &mut current_slice_vertices, result);
                    let v2 = add_vertex(reg_x, reg_y + 1, reg_z + 1, material, &mut current_slice_vertices, result);
                    let v3 = add_vertex(reg_x, reg_y + 1, reg_z, material, &mut prev_slice_vertices, result);

                    if let Some(v) = neg_x_quads.get_mut(reg_x as usize) {
                        v.push(Quad::new(v0, v1, v2, v3));
                    }
                }

                if let Some(material) = is_quad_needed(&neg_x_voxel, &current_voxel) {
                    let v0 = add_vertex(reg_x, reg_y, reg_z, material, &mut prev_slice_vertices, result);
                    let v1 = add_vertex(reg_x, reg_y, reg_z + 1, material, &mut current_slice_vertices, result);
                    let v2 = add_vertex(reg_x, reg_y + 1, reg_z + 1, material, &mut current_slice_vertices, result);
                    let v3 = add_vertex(reg_x, reg_y + 1, reg_z, material, &mut prev_slice_vertices, result);

                    if let Some(v) = pos_x_quads.get_mut(reg_x as usize) {
                        v.push(Quad::new(v0, v1, v2, v3));
                    }
                }

                // Y
                if let Some(material) = is_quad_needed(&current_voxel, &neg_y_voxel) {
                    let v0 = add_vertex(reg_x, reg_y, reg_z, material, &mut prev_slice_vertices, result);
                    let v1 = add_vertex(reg_x + 1, reg_y, reg_z, material, &mut current_slice_vertices, result);
                    let v2 = add_vertex(reg_x + 1, reg_y, reg_z + 1, material, &mut current_slice_vertices, result);
                    let v3 = add_vertex(reg_x, reg_y, reg_z + 1, material, &mut prev_slice_vertices, result);

                    if let Some(v) = neg_y_quads.get_mut(reg_y as usize) {
                        v.push(Quad::new(v0, v1, v2, v3));
                    }
                }

                if let Some(material) = is_quad_needed(&neg_y_voxel, &current_voxel) {
                    let v0 = add_vertex(reg_x, reg_y, reg_z, material, &mut prev_slice_vertices, result);
                    let v1 = add_vertex(reg_x + 1, reg_y, reg_z, material, &mut current_slice_vertices, result);
                    let v2 = add_vertex(reg_x + 1, reg_y, reg_z + 1, material, &mut current_slice_vertices, result);
                    let v3 = add_vertex(reg_x, reg_y, reg_z + 1, material, &mut prev_slice_vertices, result);

                    if let Some(v) = pos_y_quads.get_mut(reg_y as usize) {
                        v.push(Quad::new(v0, v1, v2, v3));
                    }
                }

                // Z
                if let Some(material) = is_quad_needed(&current_voxel, &neg_z_voxel) {
                    let v0 = add_vertex(reg_x, reg_y, reg_z, material, &mut prev_slice_vertices, result);
                    let v1 = add_vertex(reg_x, reg_y + 1, reg_z, material, &mut current_slice_vertices, result);
                    let v2 = add_vertex(reg_x + 1, reg_y + 1, reg_z, material, &mut current_slice_vertices, result);
                    let v3 = add_vertex(reg_x + 1, reg_y, reg_z, material, &mut prev_slice_vertices, result);

                    if let Some(v) = neg_z_quads.get_mut(reg_y as usize) {
                        v.push(Quad::new(v0, v1, v2, v3));
                    }
                }

                if let Some(material) = is_quad_needed(&neg_z_voxel, &current_voxel) {
                    let v0 = add_vertex(reg_x, reg_y, reg_z, material, &mut prev_slice_vertices, result);
                    let v1 = add_vertex(reg_x, reg_y + 1, reg_z, material, &mut current_slice_vertices, result);
                    let v2 = add_vertex(reg_x + 1, reg_y + 1, reg_z, material, &mut current_slice_vertices, result);
                    let v3 = add_vertex(reg_x + 1, reg_y, reg_z, material, &mut prev_slice_vertices, result);

                    if let Some(v) = pos_z_quads.get_mut(reg_y as usize) {
                        v.push(Quad::new(v0, v1, v2, v3));
                    }
                }
            }
        }

        std::mem::swap(&mut current_slice_vertices, &mut prev_slice_vertices);
        current_slice_vertices.data.iter_mut().for_each(|item| { item.index = -1 })
    }

    for face in vec![pos_x_quads, neg_x_quads, pos_y_quads, neg_y_quads, pos_z_quads, neg_z_quads] {
        for quads in face {

        }
    }
}


pub fn extract_cubic_mesh<T>(sampler: &mut dyn Sampler<T>, region: &Region) -> Mesh<CubicVertex<T>> where T: Voxel {
    let mut mesh : Mesh<CubicVertex<T>> = Mesh::new();

    extract_cubic_mesh_custom(sampler, region, &mut mesh,|back, front| {
        if !back.is_empty() && front.is_empty() {
            Some(*back)
        } else {
            None
        }
    }, true);

    mesh
}