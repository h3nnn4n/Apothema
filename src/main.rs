mod cube;


fn main() {
    let mut cube = cube::Cube::new();

    cube.do_move(cube::Move::Fx2);

    println!("Apothema!");
}
