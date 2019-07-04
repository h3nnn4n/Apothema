use crate::cube::{get_random_move_sequence, Cube};
use crate::search::bfs_solver;

#[test]
fn test_solve() {
    for _ in 0..5 {
        let mut cube = Cube::new();
        let random_moves = get_random_move_sequence(4);

        assert!(cube.is_solved());

        cube.do_move_sequence(&random_moves);

        assert!(!cube.is_solved());

        let seq = bfs_solver(&cube);

        cube.do_move_sequence(&seq);

        assert!(cube.is_solved());
    }
}
