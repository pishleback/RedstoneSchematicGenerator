use mc_schem::fastnbt;
use mc_schem::{Block as PlainBlock, region::Region};
use std::collections::HashMap;

pub mod transforms;

#[derive(Debug, Clone, Copy)]
pub enum Compass {
    North,
    East,
    South,
    West,
}

impl Compass {
    // To (dx, dy) of length 1 pointing in the direction of self
    fn to_vec(self) -> (i32, i32) {
        match self {
            Compass::North => (0, -1),
            Compass::East => (1, 0),
            Compass::South => (0, 1),
            Compass::West => (-1, 0),
        }
    }

    // From (dx, dy) of length 1 pointing in the direction of self
    fn from_vec(vec: (i32, i32)) -> Self {
        match vec {
            (0, -1) => Self::North,
            (1, 0) => Self::East,
            (0, 1) => Self::South,
            (-1, 0) => Self::West,
            _ => panic!(),
        }
    }
}

fn barrel_ss(ss: usize) -> HashMap<String, fastnbt::Value> {
    let mut n = ((ss * 27).div_ceil(14) - 2).max(ss);

    if ss == 14 {
        n += 1;
    }

    let mut items = Vec::with_capacity(n);

    for i in 0..n {
        let mut item = HashMap::new();

        item.insert("Count".to_string(), fastnbt::Value::Byte(64));
        item.insert("Slot".to_string(), fastnbt::Value::Byte(i as i8));
        item.insert(
            "id".to_string(),
            fastnbt::Value::String("minecraft:redstone".to_string()),
        );
        item.insert("tag".to_string(), fastnbt::Value::Compound(HashMap::new()));

        items.push(fastnbt::Value::Compound(item));
    }

    let mut result = HashMap::new();

    result.insert(
        "Id".to_string(),
        fastnbt::Value::String("minecraft:barrel".to_string()),
    );

    result.insert("Items".to_string(), fastnbt::Value::List(items));

    result
}

#[derive(Debug, Clone)]
pub enum Block {
    Plain {
        id: String,
    },
    Barrel {
        ss: usize,
    },
    Dust {
        power: u16,
    },
    Torch {
        lit: bool,
    },
    WallTorch {
        lit: bool,
        facing: Compass,
    },
    Repeater {
        powered: bool,
        facing: Compass,
        delay: u16,
    },
}

pub struct Blocks {
    blocks: HashMap<(i32, i32, i32), Block>,
}

impl Blocks {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn place(&mut self, pos: (i32, i32, i32), block: &Block) {
        if self.blocks.insert(pos, block.clone()).is_some() {
            panic!("Pos {:?} taken.", pos);
        }
    }

    #[allow(clippy::result_unit_err)]
    pub fn finish<W: std::io::Write>(self, writer: &mut W) -> Result<(), ()> {
        if self.blocks.is_empty() {
            println!("No blocks!");
            return Err(());
        }

        let min_x = self.blocks.iter().map(|((x, _, _), _)| *x).min().unwrap();
        let max_x = self.blocks.iter().map(|((x, _, _), _)| *x).max().unwrap();
        let size_x = max_x - min_x + 1;

        let min_y = self.blocks.iter().map(|((_, y, _), _)| *y).min().unwrap();
        let max_y = self.blocks.iter().map(|((_, y, _), _)| *y).max().unwrap();
        let size_y = max_y - min_y + 1;

        let min_z = self.blocks.iter().map(|((_, _, z), _)| *z).min().unwrap();
        let max_z = self.blocks.iter().map(|((_, _, z), _)| *z).max().unwrap();
        let size_z = max_z - min_z + 1;

        let mut region = Region::with_shape([size_x, size_y, size_z]);

        for ((x, y, z), block) in self.blocks {
            let (x, y, z) = ((x - min_x), (y - min_y), (z - min_z));
            match block {
                Block::Plain { id } => {
                    region
                        .set_block([x, y, z], &PlainBlock::from_id(id.as_str()).unwrap())
                        .unwrap();
                }
                Block::Dust { power } => {
                    region
                        .set_block(
                            [x, y, z],
                            &mc_schem::Block::from_id(
                                format!("minecraft:redstone_wire[power={power}]").as_str(),
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }

                Block::Torch { lit } => {
                    region
                        .set_block(
                            [x, y, z],
                            &mc_schem::Block::from_id(
                                format!("minecraft:redstone_torch[lit={lit}]").as_str(),
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }

                Block::WallTorch { lit, facing } => {
                    region
                        .set_block(
                            [x, y, z],
                            &mc_schem::Block::from_id(
                                format!(
                                    "minecraft:redstone_wall_torch[lit={lit},facing={}]",
                                    match facing {
                                        Compass::North => "north",
                                        Compass::East => "east",
                                        Compass::South => "south",
                                        Compass::West => "west",
                                    }
                                )
                                .as_str(),
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }

                Block::Repeater {
                    powered,
                    facing,
                    delay,
                } => {
                    region
                        .set_block(
                            [x, y, z],
                            &mc_schem::Block::from_id(
                                format!(
                                    "minecraft:repeater[facing={},powered={powered},delay={delay}]",
                                    match facing {
                                        Compass::North => "south",
                                        Compass::East => "west",
                                        Compass::South => "north",
                                        Compass::West => "east",
                                    }
                                )
                                .as_str(),
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }

                Block::Barrel { ss } => {
                    region
                        .set_block(
                            [x, y, z],
                            &mc_schem::Block::from_id("minecraft:barrel[facing=up,open=false]")
                                .unwrap(),
                        )
                        .unwrap();
                    if ss != 0 {
                        region.set_block_entity_at(
                            [x, y, z],
                            mc_schem::region::BlockEntity {
                                tags: barrel_ss(ss),
                            },
                        );
                    }
                }
            };
        }

        let mut schem = mc_schem::Schematic::new();
        schem.metadata.mc_data_version =
            mc_schem::schem::mc_version::DataVersion::Java_1_18_2 as i32;
        schem.metadata.schem_we_offset = Some([min_x, min_y, min_z]);
        schem.regions.push(region);

        schem
            .save_world_edit_13_writer(writer, &mc_schem::WorldEdit13SaveOption::default())
            .map_err(|_| ())
    }
}
