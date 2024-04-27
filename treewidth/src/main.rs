use treewidth::{print_bag_tree, run_with_spinner, the_algorithm};

fn main() {
    // Start All Computable with 10 or higher width --------------------------------------------------------------------
    // let path = "./data/TutteGraph"; // width: 5
    // let path = "./data/EllinghamHorton78Graph"; // width: 6
    // let path = "./data/NonisotropicOrthogonalPolarGraph_3_5"; // width: 10
    // let path = "./data/PaleyGraph_17"; // width: 11
    // let path = "./data/GoethalsSeidelGraph_2_3"; // width: 11
    // let path = "./data/OddGraph_4"; // width: 12
    // let path = "./data/Klein7RegularGraph"; // width: 13
    // let path = "./data/HanoiTowerGraph_4_3"; // width: 13
    // let path = "./data/WellsGraph"; // width: 14
    // let path = "./data/NKStarGraph_5_3"; // width: 14
    // let path = "./data/CompleteGraph_15"; // width: 14
    // let path = "./data/SylvesterGraph"; // width: 15
    // let path = "./data/TaylorTwographDescendantSRG_3"; // width: 17
    // let path = "./data/AhrensSzekeresGeneralizedQuadrangleGraph_3"; // width: 17
    // let path = "./data/JohnsonGraph_8_2"; // width: 18
    // let path = "./data/SchlaefliGraph"; // width: 21
    let path = "./data/TaylorTwographSRG_3"; // width: 22
    // End Computable -------------------------------------------------------------------------------------------------
    
    // Start All Not Yet Computable -----------------------------------------------------------------------------------
    // let path = "./data/CompleteBipartiteGraph_25_20"; // width: 20
    // let path = "./data/HoffmanSingletonGraph"; // width: 25 
    // let path = "./data/SymplecticDualPolarGraph_4_3"; // width: 27
    // let path = "./data/SymplecticPolarGraph_4_3"; // width: 28
    // let path = "./data/KneserGraph_8_3"; // width: 29
    // let path = "./data/KneserGraph_10_2"; // width: 35
    // let path = "./data/PasechnikGraph_2"; // width: 36
    // let path = "./data/SwitchedSquaredSkewHadamardMatrixGraph_2"; // width: 38
    // let path = "./data/SquaredSkewHadamardMatrixGraph_2"; // width: 40
    // let path = "./data/GossetGraph"; // width: 43
    // let path = "./data/T2starGeneralizedQuadrangleGraph_4"; // width: 46
    // let path = "./data/NonisotropicUnitaryPolarGraph_3_3"; // width: 53
    // End Not Yet Computable -----------------------------------------------------------------------------------------

    print_bag_tree(path);
    println!("------ Maximum Independent Set ------");
    let answer = run_with_spinner(|| the_algorithm(path));
    println!("{answer}");
}
