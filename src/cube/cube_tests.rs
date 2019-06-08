use super::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use test_case_derive::test_case;

const MOVES: [Move; 18] = [
    Move::Rx1,
    Move::Rx2,
    Move::Rx3,
    Move::Lx1,
    Move::Lx2,
    Move::Lx3,
    Move::Fx1,
    Move::Fx2,
    Move::Fx3,
    Move::Bx1,
    Move::Bx2,
    Move::Bx3,
    Move::Ux1,
    Move::Ux2,
    Move::Ux3,
    Move::Dx1,
    Move::Dx2,
    Move::Dx3,
];

#[test_case(Move::Rx1, 4 :: "Move::Rx1")]
#[test_case(Move::Rx2, 2 :: "Move::Rx2")]
#[test_case(Move::Rx3, 4 :: "Move::Rx3")]
#[test_case(Move::Lx1, 4 :: "Move::Lx1")]
#[test_case(Move::Lx2, 2 :: "Move::Lx2")]
#[test_case(Move::Lx3, 4 :: "Move::Lx3")]
#[test_case(Move::Ux1, 4 :: "Move::Ux1")]
#[test_case(Move::Ux2, 2 :: "Move::Ux2")]
#[test_case(Move::Ux3, 4 :: "Move::Ux3")]
#[test_case(Move::Dx1, 4 :: "Move::Dx1")]
#[test_case(Move::Dx2, 2 :: "Move::Dx2")]
#[test_case(Move::Dx3, 4 :: "Move::Dx3")]
#[test_case(Move::Fx1, 4 :: "Move::Fx1")]
#[test_case(Move::Fx2, 2 :: "Move::Fx2")]
#[test_case(Move::Fx3, 4 :: "Move::Fx3")]
#[test_case(Move::Bx1, 4 :: "Move::Bx1")]
#[test_case(Move::Bx2, 2 :: "Move::Bx2")]
#[test_case(Move::Bx3, 4 :: "Move::Bx3")]
fn move_diameter(m: Move, diameter: u32) {
    let mut cube = Cube::new();
    let before_edges = cube.edges;
    let before_corners = cube.corners;

    for _ in 0..diameter {
        cube.do_move(m);
    }

    assert_eq!(
        before_edges, cube.edges,
        "edges dont match diameter {} with move {}",
        diameter, m
    );

    assert_eq!(
        before_corners, cube.corners,
        "corners dont match diameter {} with move {}",
        diameter, m
    );
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

        assert!(cube.has_correct_edge_orientation());
    }

    #[test]
    fn test_correct_corder_orientation() {
        let cube = Cube::new();

        assert!(cube.has_correct_corner_orientation());
    }
}

mod parametrized_moves {
    use super::*;

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
    fn test_move_has_oposite() {
        for m in MOVES.iter() {
            let mut cube = Cube::new();

            cube.do_move(*m);
            cube.do_move(move_utils::reverse_move(*m));

            assert!(cube.is_solved(), "failed for move {}", *m);
        }
    }

    #[test]
    fn test_moves_keep_cube_valid() {
        for m in MOVES.iter() {
            let mut cube = Cube::new();

            assert!(cube.has_no_duplicated_pieces());
            assert!(cube.has_correct_orientation());

            for _ in 0..12 {
                cube.do_move(*m);
            }

            assert!(
                cube.has_no_duplicated_pieces(),
                "cube has duplicated elements after move {}",
                m
            );
            assert!(
                cube.has_correct_orientation(),
                "cube has incorrect orientation after move {}",
                m
            );
        }
    }

    #[test]
    fn random_moves_keeps_cube_valid() {
        let mut cube = Cube::new();
        let mut rng = thread_rng();

        for _ in 0..1000 {
            let m = MOVES.choose(&mut rng).unwrap();

            cube.do_move(*m);

            assert!(cube.is_valid());
        }
    }
}

mod move_sequence {
    use super::*;

    #[test]
    fn move_sequence_is_reversible() {
        let mut rng = thread_rng();
        let mut cube = Cube::new();

        let sequence: Moves = MOVES.choose_multiple(&mut rng, 15).cloned().collect();
        let reverse_sequence = reverse_move_sequence(sequence.clone());

        cube.do_move_sequence(&sequence);

        assert!(!cube.is_solved());

        cube.do_move_sequence(&reverse_sequence);

        assert!(cube.is_solved());
    }

    #[test]
    fn move_sequence_diameter() {
        let mut sequence_book: HashMap<Moves, u32> = HashMap::new();
        sequence_book.insert(vec![Move::Rx1, Move::Ux1, Move::Rx3, Move::Ux3], 6);
        sequence_book.insert(vec![Move::Rx2], 2);
        sequence_book.insert(vec![Move::Ux1], 4);
        sequence_book.insert(
            vec![
                Move::Rx1,
                Move::Ux1,
                Move::Rx3,
                Move::Ux3,
                Move::Rx3,
                Move::Fx1,
                Move::Rx2,
                Move::Ux3,
                Move::Rx3,
                Move::Ux3,
                Move::Rx1,
                Move::Ux1,
                Move::Rx3,
                Move::Fx3,
            ],
            2,
        );

        for (seq, diameter) in sequence_book {
            let diameter_found = find_move_sequence_diameter(&seq);

            assert_eq!(
                diameter_found, diameter,
                "failed for sequence {:?} with expected diameter of {}, but had {}",
                seq, diameter, diameter_found
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
