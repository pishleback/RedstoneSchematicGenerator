use crate::Compass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mat4 {
    entries: [[i32; 4]; 4],
}

impl Mat4 {
    #[allow(unused)]
    fn print_rows(&self) {
        for row in self.entries {
            println!("{:?}", row);
        }
    }

    fn identity() -> Self {
        Self {
            entries: [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]],
        }
    }

    fn apply(&self, vec: [i32; 4]) -> [i32; 4] {
        std::array::from_fn(|r| (0usize..4).map(|c| self.entries[r][c] * vec[c]).sum())
    }
}

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Self::Output {
        Mat4 {
            entries: std::array::from_fn(|r| {
                std::array::from_fn(|c| {
                    (0usize..4)
                        .map(|k| self.entries[r][k] * other.entries[k][c])
                        .sum()
                })
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    // Bottom row is 0 0 0 1
    // Act on positions by (x, y, z, 1) -> M (x, y, z, 1).012
    // Act on vectors by (x, y, z, 0) -> M (x, y, z, 0).012
    forward: Mat4,
    backward: Mat4,
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Self::Output {
        Transform::new(self.forward * other.forward, other.backward * self.backward)
    }
}

impl Transform {
    fn new(forward: Mat4, backward: Mat4) -> Self {
        debug_assert_eq!(forward * backward, Mat4::identity());
        Self { forward, backward }
    }

    pub fn inverse(self) -> Self {
        Self::new(self.backward, self.forward)
    }

    pub fn translate((dx, dy, dz): (i32, i32, i32)) -> Self {
        Self::new(
            Mat4 {
                entries: [[1, 0, 0, dx], [0, 1, 0, dy], [0, 0, 1, dz], [0, 0, 0, 1]],
            },
            Mat4 {
                entries: [[1, 0, 0, -dx], [0, 1, 0, -dy], [0, 0, 1, -dz], [0, 0, 0, 1]],
            },
        )
    }

    pub fn rotate() -> Self {
        Self::new(
            Mat4 {
                entries: [[0, 0, -1, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 1]],
            },
            Mat4 {
                entries: [[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]],
            },
        )
    }

    pub fn identity() -> Self {
        Self::new(Mat4::identity(), Mat4::identity())
    }

    pub fn flip_x() -> Self {
        Self::new(
            Mat4 {
                entries: [[-1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]],
            },
            Mat4 {
                entries: [[-1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]],
            },
        )
    }

    pub fn flip_z() -> Self {
        Self::new(
            Mat4 {
                entries: [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]],
            },
            Mat4 {
                entries: [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]],
            },
        )
    }

    pub fn apply_pos(&self, pos: (i32, i32, i32)) -> (i32, i32, i32) {
        let out = self.forward.apply([pos.0, pos.1, pos.2, 1]);
        debug_assert_eq!(out[3], 1);
        (out[0], out[1], out[2])
    }

    pub fn apply_vec(&self, vec: (i32, i32, i32)) -> (i32, i32, i32) {
        let out = self.forward.apply([vec.0, vec.1, vec.2, 0]);
        debug_assert_eq!(out[3], 0);
        (out[0], out[1], out[2])
    }

    pub fn apply_compass(&self, compass: Compass) -> Compass {
        let vec = compass.to_vec();
        let vec = self.apply_vec((vec.0, 0, vec.1));
        assert_eq!(vec.1, 0);
        Compass::from_vec((vec.0, vec.2))
    }
}

pub struct Coords {
    // local -> global
    pub transform: Transform,
}

impl Coords {
    pub fn apply_global_transform(&mut self, transform: Transform) {
        self.transform = transform * self.transform;
    }

    pub fn apply_local_transform(&mut self, transform: Transform) {
        self.transform = self.transform * transform;
    }

    pub fn local_to_global_pos(&self, pos: (i32, i32, i32)) -> (i32, i32, i32) {
        self.transform.apply_pos(pos)
    }

    pub fn local_to_global_vec(&self, vec: (i32, i32, i32)) -> (i32, i32, i32) {
        self.transform.apply_vec(vec)
    }

    pub fn local_to_global_compass(&self, compass: Compass) -> Compass {
        self.transform.apply_compass(compass)
    }
}
