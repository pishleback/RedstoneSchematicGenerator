use redstone_schem::{Block, Blocks};

fn main() {
    let mut schem = Blocks::new();

    schem.place(
        (-2, -1, -2),
        &Block::Plain {
            id: "minecraft:dirt".into(),
        },
    );
    schem.place(
        (2, -1, 2),
        &Block::Plain {
            id: "minecraft:dirt".into(),
        },
    );
    schem.place(
        (2, -1, -2),
        &Block::Plain {
            id: "minecraft:stone".into(),
        },
    );
    schem.place(
        (-2, -1, 2),
        &Block::Plain {
            id: "minecraft:glass".into(),
        },
    );
    schem.place((0, -1, 0), &Block::Barrel { ss: 0 });

    let mut file = std::fs::File::create("example.schem").unwrap();
    schem.finish(&mut file).unwrap();
}
