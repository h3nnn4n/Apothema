fn init_corners() -> Vec<OrientedCorner> {
    vec![
        OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::UBR, o:0},
        OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}
    ]
}

fn init_edges() -> Vec<OrientedEdge> {
    vec![
        OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
        OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}
    ]
}

fn init_corner_cubie_move() -> Vec<Vec<OrientedCorner>> {
    vec![
        vec![OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0},
            OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],  //U
        vec![OrientedCorner{c:Corner::DFR, o:2}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::URF, o:1},
            OrientedCorner{c:Corner::DRB, o:1}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::UBR, o:2}],  //R
        vec![OrientedCorner{c:Corner::UFL, o:1}, OrientedCorner{c:Corner::DLF, o:2}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
            OrientedCorner{c:Corner::URF, o:2}, OrientedCorner{c:Corner::DFR, o:1}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}],  //F
        vec![OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0}, OrientedCorner{c:Corner::UBR, o:0},
            OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}, OrientedCorner{c:Corner::DRB, o:0}, OrientedCorner{c:Corner::DFR, o:0}],  //D
        vec![OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::ULB, o:1}, OrientedCorner{c:Corner::DBL, o:2}, OrientedCorner{c:Corner::UBR, o:0},
            OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::UFL, o:2}, OrientedCorner{c:Corner::DLF, o:1}, OrientedCorner{c:Corner::DRB, o:0}],  //L
        vec![OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::UBR, o:1}, OrientedCorner{c:Corner::DRB, o:2},
            OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::ULB, o:2}, OrientedCorner{c:Corner::DBL, o:1}],  //B
        vec![OrientedCorner{c:Corner::UBR, o:0}, OrientedCorner{c:Corner::URF, o:0}, OrientedCorner{c:Corner::UFL, o:0}, OrientedCorner{c:Corner::ULB, o:0},
            OrientedCorner{c:Corner::DRB, o:0}, OrientedCorner{c:Corner::DFR, o:0}, OrientedCorner{c:Corner::DLF, o:0}, OrientedCorner{c:Corner::DBL, o:0}],  //Us
        vec![OrientedCorner{c:Corner::DFR, o:2}, OrientedCorner{c:Corner::DLF, o:1}, OrientedCorner{c:Corner::UFL, o:2}, OrientedCorner{c:Corner::URF, o:1},
            OrientedCorner{c:Corner::DRB, o:1}, OrientedCorner{c:Corner::DBL, o:2}, OrientedCorner{c:Corner::ULB, o:1}, OrientedCorner{c:Corner::UBR, o:2}],  //Rs
        vec![OrientedCorner{c:Corner::UFL, o:1}, OrientedCorner{c:Corner::DLF, o:2}, OrientedCorner{c:Corner::DBL, o:1}, OrientedCorner{c:Corner::ULB, o:2},
            OrientedCorner{c:Corner::URF, o:2}, OrientedCorner{c:Corner::DFR, o:1}, OrientedCorner{c:Corner::DRB, o:2}, OrientedCorner{c:Corner::UBR, o:1}]   //Fs
    ]
}

fn init_edge_cubie_move() -> Vec<Vec<OrientedEdge>> {
    vec![
        vec![OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //U
        vec![OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UR, o:0}],  //R
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::FL, o:1}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:1},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::UF, o:1}, OrientedEdge{e:Edge::DF, o:1}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //F
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DF, o:0}, OrientedEdge{e:Edge::DL, o:0},
            OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //D
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //L
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::BR, o:1}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BL, o:1}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::UB, o:1}, OrientedEdge{e:Edge::DB, o:1}],  //B
        vec![OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0},
            OrientedEdge{e:Edge::DF, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::BR, o:0}],  //Us
        vec![OrientedEdge{e:Edge::FR, o:0}, OrientedEdge{e:Edge::UF, o:0}, OrientedEdge{e:Edge::FL, o:0}, OrientedEdge{e:Edge::UB, o:0}, OrientedEdge{e:Edge::BR, o:0}, OrientedEdge{e:Edge::DF, o:0},
            OrientedEdge{e:Edge::BL, o:0}, OrientedEdge{e:Edge::DB, o:0}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::UR, o:0}],  //Rs
        vec![OrientedEdge{e:Edge::UR, o:0}, OrientedEdge{e:Edge::FL, o:1}, OrientedEdge{e:Edge::UL, o:0}, OrientedEdge{e:Edge::BL, o:1}, OrientedEdge{e:Edge::DR, o:0}, OrientedEdge{e:Edge::FR, o:1},
            OrientedEdge{e:Edge::DL, o:0}, OrientedEdge{e:Edge::BR, o:1}, OrientedEdge{e:Edge::UF, o:1}, OrientedEdge{e:Edge::DF, o:1}, OrientedEdge{e:Edge::DB, o:1}, OrientedEdge{e:Edge::UB, o:1}]   //Fs
    ]
}
