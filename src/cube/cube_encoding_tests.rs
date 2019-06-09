use super::*;

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
