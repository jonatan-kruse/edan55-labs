use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::transform_input::Graph;

pub fn monte_carlo_algo(input: &Graph, iterations: i32) -> (f32, f32) {
    (simulate(input, iterations, input.fed), simulate(input, iterations, input.post))
}

fn simulate(input: &Graph, iterations: i32, start_node: usize) -> f32 {
    let total_time: f32 = (0..iterations).into_par_iter().map(|_| {
        let mut current_node = start_node;
        let mut time = 0.0;
        while current_node != input.home {
            let possible_next_nodes = input.edges.iter()
                .filter(|e| e.u == current_node || e.v == current_node)
                .map(|e| if e.u == current_node { (e.v, e.puv, e.t) } else { (e.u, e.pvu, e.t) })
                .collect::<Vec<_>>();

            let mut rng = rand::thread_rng();
            let random_value: f32 = rng.gen();
            let mut cumulative_probability = 0.0;

            for &(item, probability, t) in &possible_next_nodes {
                cumulative_probability += probability;
                if cumulative_probability >= random_value {
                    current_node = item;
                    time += t;
                    break;
                }
            }
        }
        time
    }).sum();
    total_time / (iterations as f32)
}
