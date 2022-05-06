use bitvec::prelude::*;
use wfc_solver::{
    description::{
        CompiledDescription, HorizontalConnection, ProblemDescription, Tile, TileData,
        TileRotation, VerticalConnection,
    },
    utils::Dimensions,
};

#[test]
fn test_compile() {
    let desc = basic_straight_air_description();

    let compiled = desc.compile();

    let ref_compiled = basic_straight_air_compiled();

    // println!("Forward");
    // for x in 0..compiled.len() {
    //     let td = compiled.data(x);
    //     println!("{:?} -- {}", x, compiled.forward(x));
    //     println!("{:?} -- {}", x, ref_compiled.forward(x));
    //     println!();
    // }

    // println!("Backward");
    // for x in 0..compiled.len() {
    //     let td = compiled.data(x);
    //     println!("{:?} -- {}", x, compiled.backward(x));
    //     println!("{:?} -- {}", x, ref_compiled.backward(x));
    //     println!();
    // }

    // println!("Left");
    // for x in 0..compiled.len() {
    //     let td = compiled.data(x);
    //     println!("{:?} -- {}", x, compiled.left(x));
    //     println!("{:?} -- {}", x, ref_compiled.left(x));
    //     println!();
    // }

    // println!("Right");
    // for x in 0..compiled.len() {
    //     let td = compiled.data(x);
    //     println!("{:?} -- {}", x, compiled.right(x));
    //     println!("{:?} -- {}", x, ref_compiled.right(x));
    //     println!();
    // }

    // println!("Up");
    // for x in 0..compiled.len() {
    //     let td = compiled.data(x);
    //     println!("{:?} -- {}", x, compiled.up(x));
    //     println!("{:?} -- {}", x, ref_compiled.up(x));
    //     println!();
    // }

    // println!("Down");
    // for x in 0..compiled.len() {
    //     let td = compiled.data(x);
    //     println!("{:?} -- {}", x, compiled.down(x));
    //     println!("{:?} -- {}", x, ref_compiled.down(x));
    //     println!();
    // }

    // assert_eq!(ref_compiled.dimensions, compiled.dimensions);
    // assert_eq!(ref_compiled.transformation, compiled.transformation);
    // assert_eq!(ref_compiled.up, compiled.up);
    // assert_eq!(ref_compiled.down, compiled.down);
    // assert_eq!(ref_compiled.left, compiled.left);
    // assert_eq!(ref_compiled.right, compiled.right);
    // assert_eq!(ref_compiled.forward, compiled.forward);
    // assert_eq!(ref_compiled.backward, compiled.backward);

    assert_eq!(ref_compiled, compiled);
}

fn basic_straight_air_compiled() -> CompiledDescription {
    CompiledDescription {
        dimensions: Dimensions::new(3, 3, 3),
        transformation: vec![
            TileData::new(0, TileRotation::R0, false),
            TileData::new(0, TileRotation::R90, false),
            TileData::new(0, TileRotation::R180, false),
            TileData::new(0, TileRotation::R270, false),
            TileData::new(1, TileRotation::R0, false),
            TileData::new(1, TileRotation::R90, false),
            TileData::new(1, TileRotation::R180, false),
            TileData::new(1, TileRotation::R270, false),
            TileData::new(2, TileRotation::R0, false),
            TileData::new(2, TileRotation::R90, false),
            TileData::new(2, TileRotation::R180, false),
            TileData::new(2, TileRotation::R270, false),
        ],
        up: vec![
            // Air
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            // Straight
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            // Corner
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0),
        ],
        down: vec![
            // Air
            bitvec!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
            bitvec!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
            bitvec!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
            bitvec!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
            // Straight
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            // Corner
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
        ],
        left: vec![
            // Air
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            // Straight
            bitvec!(0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            bitvec!(0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            // Corner
            bitvec!(0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1),
            bitvec!(0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0),
        ],
        right: vec![
            // Air
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
            // Straight
            bitvec!(0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
            // Corner
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
            bitvec!(0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1),
            bitvec!(1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0),
        ],
        forward: vec![
            // Air
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
            // Straight
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1),
            // Corner
            bitvec!(0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1),
            bitvec!(0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0),
        ],
        backward: vec![
            // Air
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            // Straight
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            bitvec!(0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0),
            // Corner
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            bitvec!(1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1),
            bitvec!(0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0),
            bitvec!(0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0),
        ],
    }
}

fn basic_straight_air_description() -> ProblemDescription<usize> {
    ProblemDescription {
        connections: vec!["air".into(), "half".into(), "full".into()],
        dimensions: Dimensions::new(3, 3, 3),
        tiles: vec![
            Tile {
                id: 0,
                can_flip: false,
                can_rotate: true,
                up: vec![VerticalConnection {
                    connection: "air".into(),
                }],
                down: vec![VerticalConnection {
                    connection: "air".into(),
                }],
                forward: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
                backward: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
                left: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
                right: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
            },
            Tile {
                id: 1, // straight,
                can_flip: false,
                can_rotate: true,
                up: vec![VerticalConnection {
                    connection: "air".into(),
                }],
                down: vec![VerticalConnection {
                    connection: "full".into(),
                }],
                left: vec![HorizontalConnection {
                    connection: "half".into(),
                    flipped: false,
                    symmetry: false,
                }],
                right: vec![HorizontalConnection {
                    connection: "half".into(),
                    flipped: true,
                    symmetry: false,
                }],
                forward: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
                backward: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
            },
            Tile {
                id: 2, // corner
                can_flip: false,
                can_rotate: true,
                up: vec![VerticalConnection {
                    connection: "air".into(),
                }],
                down: vec![VerticalConnection {
                    connection: "full".into(),
                }],
                left: vec![HorizontalConnection {
                    connection: "half".into(),
                    flipped: false,
                    symmetry: false,
                }],
                right: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
                forward: vec![HorizontalConnection {
                    connection: "half".into(),
                    flipped: true,
                    symmetry: false,
                }],
                backward: vec![HorizontalConnection {
                    connection: "air".into(),
                    flipped: false,
                    symmetry: true,
                }],
            },
        ],
    }
}
