use crate::*;

pub const ROM_SIZE: usize = 1usize << 14;

#[derive(Debug, Clone)]
pub struct Rom {
    pub data: [u8; ROM_SIZE],
}

impl Default for Rom {
    fn default() -> Self {
        Self { data: [0; _] }
    }
}

impl Rom {
    pub fn to_partial_schem(&self, layers: std::ops::Range<usize>) -> Blocks {
        let mut schem = Blocks::new();
        for a in 0..32 {
            for b in 0..32 {
                for c in layers.clone() {
                    assert!(c < 32);
                    let x = -(2 * a as i32 + if b as i32 % 4 == 0 { 2 } else { 0 });
                    let y = -2 * (31 - c) as i32 - 1;
                    let z = -2 * b as i32;

                    let (idx, part) = (a + 32 * c + 32 * 32 * (b / 2), b % 4 == 0 || b % 4 == 3);

                    let ss = if !part {
                        self.data[idx] % 16
                    } else {
                        self.data[idx] / 16
                    } as usize;

                    if ss == 0 {
                        schem.place(
                            (x, y, z),
                            &Block::Plain {
                                id: "minecraft:brown_wool".into(),
                            },
                        );
                    } else {
                        schem.place((x, y, z), &Block::Barrel { ss });
                    }
                }
            }
        }
        schem
    }

    pub fn to_schem(&self) -> Blocks {
        self.to_partial_schem(0..32)
    }
}
