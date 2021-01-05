pub trait Voxel: Default + Copy + PartialEq + std::fmt::Debug {
    fn is_empty(self) -> bool;
}

impl Voxel for u8 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for u16 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for u32 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for u64 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for u128 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for usize {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for i8 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for i16 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for i32 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for i64 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for i128 {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for isize {
    fn is_empty(self) -> bool {
        self == 0
    }
}

impl Voxel for f32 {
    fn is_empty(self) -> bool {
        self == 0.0
    }
}

impl Voxel for f64 {
    fn is_empty(self) -> bool {
        self == 0.0
    }
}
