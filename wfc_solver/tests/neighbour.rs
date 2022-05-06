use bitvec::prelude::*;
use wfc_solver::{
    description::{HorizontalConnection, ProblemDescription, Tile, VerticalConnection},
    solver::naive::{propagate_point, update_initial_sides},
    utils::{Dimensions, FieldGrid, Point},
};

#[test]
fn test_neighbour_1() {
    let desc = basic_straight_air_description();
    let compiled = desc.compile();

    let mut grid = FieldGrid::new(compiled.dimensions(), compiled.all_domain());

    update_initial_sides(&mut grid, &compiled).unwrap();
    print_dimensions(&grid);

    propagate_point(
        &mut grid,
        Point::new(0, 0, 0),
        bitvec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0),
        &compiled,
    )
    .unwrap();

    print_dimensions(&grid);
}

fn print_dimensions(grid: &FieldGrid) {
    println!("Dimensions: {:?}", grid.dimensions());
    for y in 0..grid.dimensions().height() {
        for z in 0..grid.dimensions().depth() {
            for x in 0..grid.dimensions().width() {
                println!(
                    "({}, {}, {}) - {}",
                    x,
                    y,
                    z,
                    grid.get(Point::new(x, y, z)).unwrap()
                );
            }
        }
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
