use vek::vec3::Vec3;

pub struct Mesh<T> {
    pub indices: Vec<i32>,
    pub vertices: Vec<T>,
    pub offset: Vec3<i32>
}

impl <T> Mesh<T>  {
    pub fn new() -> Self {
        Mesh{
            indices: vec![],
            vertices: vec![],
            offset: Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty() && self.vertices.is_empty()
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
    }

    pub fn add_vertex(&mut self, vertex: T) -> usize {
        self.vertices.push(vertex);

        self.vertices.len() - 1
    }

    pub fn add_triangle(&mut self, vertex_0: i32, vertex_1: i32, vertex_2: i32) {
        self.indices.push(vertex_0);
        self.indices.push(vertex_1);
        self.indices.push(vertex_2);
    }

    pub fn set_offset(&mut self, offset: Vec3<i32>) {
        self.offset = offset;
    }

    pub fn get_offset(self) -> Vec3<i32> {
        self.offset
    }
}