use super::Cube;

impl Cube {
    pub fn has_correct_orientation(&self) -> bool {
        has_correct_orientation(self)
    }

    pub fn has_correct_corner_orientation(&self) -> bool {
        has_correct_corner_orientation(self)
    }

    pub fn has_correct_edge_orientation(&self) -> bool {
        has_correct_edge_orientation(self)
    }

    pub fn has_no_duplicated_pieces(&self) -> bool {
        has_no_duplicates(self)
    }
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
