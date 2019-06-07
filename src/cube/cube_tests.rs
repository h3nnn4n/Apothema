use super::*;

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
}

mod r_moves {
    use super::*;

    #[test]
    fn test_rx1_has_diameter_4() {
        let mut cube = Cube::new();
        let before = cube.edges;

        for _ in 0..4 {
            cube.do_move(Move::Rx1);
        }

        let after = cube.edges;

        for index in 0..12 {
            assert_eq!(before[index], after[index]);
        }
    }

    #[test]
    fn test_rx1_move_affects_the_cube() {
        let mut cube = Cube::new();
        let before = cube.edges[Edge::UR as usize];

        cube.do_move(Move::Rx1);

        let after = cube.edges[Edge::UR as usize];

        assert_ne!(before, after);
    }

    #[test]
    fn test_rx1_keeps_cube_valid() {
        let mut cube = Cube::new();

        assert!(has_no_duplicates(&cube));
        println_edges(cube.edges);

        cube.do_move(Move::Rx1);

        println_edges(cube.edges);
        assert!(has_no_duplicates(&cube));
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
