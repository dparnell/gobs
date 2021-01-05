use vek::vec3::Vec3;

pub struct Vertex<T> {
    pub position: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub data: T,
}

impl<T> Vertex<T> {
    pub fn new(position: Vec3<f32>, normal: Vec3<f32>, data: T) -> Self {
        Vertex {
            position,
            normal,
            data,
        }
    }
}
