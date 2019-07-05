use super::*;

mod as_u64 {
    use super::*;

    #[test]
    fn solved_edge_orientation_is_zero() {
        let mut cube = Cube::new();

        assert_eq!(cube.edge_orientation_as_u64(), 0);

        let random_moves = get_random_move_sequence(10);
        cube.do_move_sequence(&random_moves);

        assert!(!cube.is_solved());
        assert_ne!(cube.edge_orientation_as_u64(), 0);
    }

    #[test]
    fn solved_corner_orientation_is_zero() {
        let mut cube = Cube::new();

        assert_eq!(cube.corner_orientation_as_u64(), 0);

        let random_moves = get_random_move_sequence(10);
        cube.do_move_sequence(&random_moves);

        assert!(!cube.is_solved());
        assert_ne!(cube.corner_orientation_as_u64(), 0);
    }

    #[test]
    fn test_edge_encoding() {
        let cube = Cube::new();

        let edge_tuple = cube.edges_to_tuple();
        let edge_orientation = cube.edge_orientation_as_u64();
        let edge_permutation = cube.edge_permutation_as_u64();

        assert_eq!(edge_tuple, (edge_permutation, edge_orientation));
    }

    #[test]
    fn test_corner_encoding() {
        let cube = Cube::new();

        let corner_tuple = cube.corners_to_tuple();
        let corner_orientation = cube.corner_orientation_as_u64();
        let corner_permutation = cube.corner_permutation_as_u64();

        assert_eq!(corner_tuple, (corner_permutation, corner_orientation));
    }
}

mod tuple {
    use super::*;

    #[test]
    fn test_edge_encoding() {
        let cube = Cube::new();

        let edge_tuple = cube.edges_to_tuple();

        let mut new_cube = Cube::new();
        new_cube.edges_from_tuple(edge_tuple);

        assert!(new_cube.is_valid());
        assert_eq!(cube, new_cube);
    }

    #[test]
    fn test_corner_encoding() {
        let cube = Cube::new();

        let corner_tuple = cube.corners_to_tuple();

        let mut new_cube = Cube::new();
        new_cube.corners_from_tuple(corner_tuple);

        assert!(new_cube.is_valid());
        assert_eq!(cube, new_cube);
    }

    #[test]
    fn test_edge_encoding_with_random_cubes() {
        for _ in 0..50 {
            let mut cube = Cube::new();
            let random_moves = get_random_move_sequence(15);

            cube.do_move_sequence(&random_moves);

            let edge_tuple = cube.edges_to_tuple();

            let mut new_cube = Cube::new();
            new_cube.edges_from_tuple(edge_tuple);

            assert!(new_cube.is_valid());
            assert_eq!(cube.edges, new_cube.edges);
        }
    }

    #[test]
    fn test_corner_encoding_with_random_cubes() {
        for _ in 0..50 {
            let mut cube = Cube::new();
            let random_moves = get_random_move_sequence(15);

            cube.do_move_sequence(&random_moves);

            let corner_tuple = cube.corners_to_tuple();

            let mut new_cube = Cube::new();
            new_cube.corners_from_tuple(corner_tuple);

            assert!(new_cube.is_valid());
            assert_eq!(cube.corners, new_cube.corners);
        }
    }

    #[test]
    fn test_cube_encoding_with_random_cubes() {
        for _ in 0..50 {
            let mut cube = Cube::new();
            let random_moves = get_random_move_sequence(15);

            cube.do_move_sequence(&random_moves);

            let cube_tuple = cube.cube_to_tuple();

            let mut new_cube = Cube::new();
            new_cube.cube_from_tuple(cube_tuple);

            assert!(new_cube.is_valid());
            assert_eq!(cube, new_cube);
        }
    }
}

mod int {
    use super::*;

    #[test]
    fn test_edge_encoding() {
        let cube = Cube::new();

        let edge_i = cube.edges_to_i();

        let mut new_cube = Cube::new();
        new_cube.edges_from_i(edge_i);

        assert!(new_cube.is_valid());
        assert_eq!(cube, new_cube);
    }

    #[test]
    fn test_edge_encoding_with_random_cubes() {
        for _ in 0..50 {
            let mut cube = Cube::new();
            let random_moves = get_random_move_sequence(15);

            cube.do_move_sequence(&random_moves);

            let edge_i = cube.edges_to_i();

            let mut new_cube = Cube::new();
            new_cube.edges_from_i(edge_i);

            assert!(new_cube.is_valid());
            assert_eq!(cube.edges, new_cube.edges);
        }
    }

    #[test]
    fn test_corner_encoding() {
        let cube = Cube::new();

        let corner_i = cube.corners_to_i();

        let mut new_cube = Cube::new();
        new_cube.corners_from_i(corner_i);

        assert!(new_cube.is_valid());
        assert_eq!(cube, new_cube);
    }

    #[test]
    fn test_corner_encoding_with_random_cubes() {
        for _ in 0..50 {
            let mut cube = Cube::new();
            let random_moves = get_random_move_sequence(15);

            cube.do_move_sequence(&random_moves);

            let corner_i = cube.corners_to_i();

            let mut new_cube = Cube::new();
            new_cube.corners_from_i(corner_i);

            assert!(new_cube.is_valid());
            assert_eq!(cube.corners, new_cube.corners);
        }
    }
}

mod ud_slice {
    use super::*;

    #[test]
    fn solved_is_zero() {
        let mut cube = Cube::new();

        assert_eq!(cube.ud_slice_as_u64(), 0);

        let random_moves = get_random_move_sequence(10);
        cube.do_move_sequence(&random_moves);

        assert!(!cube.is_solved());
        assert_ne!(cube.ud_slice_as_u64(), 0);
    }

    #[test]
    fn kociemba_g1_is_zero() {
        let mut cube = Cube::new();

        cube.do_move_sequence(&vec![Move::Ux1, Move::Rx2, Move::Lx2]);
        assert_eq!(cube.ud_slice_as_u64(), 0);

        cube.do_move_sequence(&vec![Move::Dx3, Move::Fx2, Move::Bx2]);
        assert_eq!(cube.ud_slice_as_u64(), 0);
    }
}

mod sorted_ud_slice {
    use super::*;

    #[test]
    fn solved_is_zero() {
        let mut cube = Cube::new();

        assert_eq!(cube.sorted_ud_slice_as_u64(), 0);

        let random_moves = get_random_move_sequence(10);
        cube.do_move_sequence(&random_moves);

        assert!(!cube.is_solved());
        assert_ne!(cube.sorted_ud_slice_as_u64(), 0);
    }

    #[test]
    fn kociemba_g1_is_not_zero() {
        let mut cube = Cube::new();

        cube.do_move_sequence(&vec![Move::Ux1, Move::Rx2, Move::Lx2]);
        assert_ne!(cube.sorted_ud_slice_as_u64(), 0);

        cube.do_move_sequence(&vec![Move::Dx3, Move::Fx2, Move::Bx2]);
        assert_ne!(cube.sorted_ud_slice_as_u64(), 0);
    }
}
