use super::*;
use std::collections::HashMap;
use test_case_derive::test_case;

#[test_case(Move::Rx1,  4 :: "Move::Rx1")]
#[test_case(Move::Lx1,  4 :: "Move::Lx1")]
#[test_case(Move::Ux1,  4 :: "Move::Ux1")]
#[test_case(Move::Dx1,  4 :: "Move::Dx1")]
#[test_case(Move::Fx1,  4 :: "Move::Fx1")]
#[test_case(Move::Bx1,  4 :: "Move::Bx1")]
fn move_diameter(m: Move, diameter: u32) {
    let mut cube = Cube::new();
    let before_edges = cube.edges;
    let before_corners = cube.corners;

    for _ in 0..diameter {
        cube.do_move(m);
    }

    assert_eq!(before_edges, cube.edges, "edges dont match");
    assert_eq!(before_corners, cube.corners, "corners dont match");
}

mod initial_state {
    use super::*;

    #[test]
    fn test_no_repeated_edges() {
        let cube = Cube::new();
        let edges = cube.edges;

        for i in 0..10 {
            for j in i + 1..11 {
                assert_ne!(edges[i], edges[j]);
            }
        }
    }

    #[test]
    fn test_no_repeated_corners() {
        let cube = Cube::new();
        let corners = cube.corners;

        for i in 0..7 {
            for j in i + 1..8 {
                assert_ne!(corners[i], corners[j]);
            }
        }
    }

    #[test]
    fn test_correct_edge_orientation() {
        let cube = Cube::new();

        assert!(has_correct_edge_orientation(&cube));
    }

    #[test]
    fn test_correct_corder_orientation() {
        let cube = Cube::new();

        assert!(has_correct_corner_orientation(&cube));
    }
}

mod parametrized_moves {
    use super::*;

    const MOVES: [Move; 6] = [
        Move::Rx1,
        Move::Lx1,
        Move::Fx1,
        Move::Bx1,
        Move::Ux1,
        Move::Dx1,
    ];

    #[test]
    fn test_move_affects_the_cube() {
        for m in MOVES.iter() {
            let mut cube = Cube::new();
            let edges_before = cube.edges;
            let corners_before = cube.corners;

            cube.do_move(*m);

            assert_ne!(edges_before, cube.edges);
            assert_ne!(corners_before, cube.corners);
        }
    }

    #[test]
    fn test_moves_keep_cube_valid() {
        for m in MOVES.iter() {
            let mut cube = Cube::new();

            assert!(has_no_duplicates(&cube));
            assert!(has_correct_orientation(&cube));

            for _ in 0..12 {
                cube.do_move(*m);
            }

            assert!(
                has_no_duplicates(&cube),
                "cube has duplicated elements after move {}",
                m
            );
            assert!(
                has_correct_orientation(&cube),
                "cube has incorrect orientation after move {}",
                m
            );
        }
    }
}

fn println_edges(edges: Edges) {
    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {}",
        edges[0],
        edges[1],
        edges[2],
        edges[3],
        edges[4],
        edges[5],
        edges[6],
        edges[7],
        edges[8],
        edges[9],
        edges[10],
        edges[11],
    );
}

fn has_correct_orientation(cube: &Cube) -> bool {
    has_correct_edge_orientation(cube) && has_correct_corner_orientation(cube)
}

fn has_correct_edge_orientation(cube: &Cube) -> bool {
    let edges = cube.edges;
    let mut orientation = 0;

    for edge in edges.iter() {
        orientation += edge.o;
    }

    orientation % 2 == 0
}

fn has_correct_corner_orientation(cube: &Cube) -> bool {
    let corners = cube.corners;
    let mut orientation = 0;

    for corner in corners.iter() {
        orientation += corner.o;
    }

    orientation % 3 == 0
}

fn has_no_duplicates(cube: &Cube) -> bool {
    let corners = cube.corners;
    let edges = cube.edges;

    for i in 0..11 {
        for j in i + 1..12 {
            if edges[i] == edges[j] {
                return false;
            }
        }
    }

    for i in 0..7 {
        for j in i + 1..8 {
            if corners[i] == corners[j] {
                return false;
            }
        }
    }

    true
}
