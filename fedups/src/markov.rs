use crate::transform_input::Graph;
use rulinalg::{matrix::Matrix, vector::Vector};

pub fn markov_algo(input: Graph) -> String {
    let mut data_a = vec![0.0; input.nodes.pow(2)];
    let mut data_b = vec![0.0; input.nodes];
    for edge in input.edges{
        data_a[edge.u * input.nodes + edge.v] = edge.puv;
        data_a[edge.v * input.nodes + edge.u] = edge.pvu;
        data_b[edge.u] += edge.t * edge.puv;
        data_b[edge.v] += edge.t * edge.pvu; 
    }
    let a = Matrix::new(input.nodes, input.nodes, data_a);
    let b = Vector::new(data_b);

    let x = (a - Matrix::<f32>::identity(input.nodes)).solve(b).unwrap();
    "\nfed: ".to_owned() + &(-x.iter().skip(input.fed).next().unwrap()).to_string()
        + "\npost: " + &(-x.iter().skip(input.post).next().unwrap()).to_string()
}