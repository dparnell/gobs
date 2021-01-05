use vek::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FaceArity {
    Three,
    Four,
}

/// A polygon mesh.
pub struct Mesh<T> {
    pub(crate) indices: Vec<i32>,
    pub(crate) vertices: Vec<T>,
    pub(crate) offset: Vec3<i32>,
    pub(crate) face_arity: FaceArity,
}

impl<T> Mesh<T> {
    /// Creates a new mesh. The `face_arity` argument determines if this is a triangle or quad mesh.
    pub fn new(face_arity: FaceArity) -> Self {
        Mesh {
            indices: vec![],
            vertices: vec![],
            offset: Default::default(),
            face_arity,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty() && self.vertices.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
    }

    #[inline]
    pub fn add_vertex(&mut self, vertex: T) -> usize {
        self.vertices.push(vertex);

        self.vertices.len() - 1
    }

    #[inline]
    pub fn add_triangle(&mut self, vertex_0: i32, vertex_1: i32, vertex_2: i32) {
        assert!(FaceArity::Three == self.face_arity);
        self.indices.push(vertex_0);
        self.indices.push(vertex_1);
        self.indices.push(vertex_2);
    }

    #[inline]
    pub fn add_quad(&mut self, vertex_0: i32, vertex_1: i32, vertex_2: i32, vertex_3: i32) {
        assert!(FaceArity::Four == self.face_arity);
        self.indices.push(vertex_0);
        self.indices.push(vertex_1);
        self.indices.push(vertex_2);
        self.indices.push(vertex_3);
    }

    #[inline]
    pub fn set_offset(&mut self, offset: Vec3<i32>) {
        self.offset = offset;
    }

    pub fn indices(&self) -> &[i32] {
        &self.indices
    }

    pub fn vertices(&self) -> &[T] {
        &self.vertices
    }

    #[inline]
    pub fn offset(&self) -> &[i32] {
        &self.offset
    }

    #[inline]
    pub fn face_arity(&self) -> FaceArity {
        self.face_arity.clone()
    }

    pub fn remove_unused_vertices(&mut self) {
        let vert_count = self.vertices.len();
        let mut is_vertex_used = vec![false; vert_count];
        for i in &self.indices {
            is_vertex_used[*i as usize] = true;
        }

        let mut used_count = 0;
        let mut new_pos = vec![0; vert_count];
        for i in 0..vert_count {
            if is_vertex_used[i] {
                if used_count != i {
                    self.vertices.swap(used_count, i);
                }
                new_pos[i] = used_count;
                used_count = used_count + 1
            }
        }

        self.vertices.drain(used_count..vert_count);
        for i in 0..self.indices.len() {
            self.indices[i] = new_pos[self.indices[i] as usize] as i32;
        }
    }
}
