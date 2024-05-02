use rand::Rng;

use crate::transform_input::Graph;

pub fn monte_carlo_algo(input: Graph) -> String {
    let iterations = 10000;
    let mut total_time_fed = 0.0;
    let mut total_time_post = 0.0;
    for _ in 0..iterations {
        let mut current_node = input.fed;
        let mut time = 0.0;
        while current_node != input.home {
            let possible_next_nodes1 = input
                .edges
                .iter()
                .filter(|e| e.u == current_node)
                .map(|e| (e.v, e.puv, e.t))
                .collect::<Vec<_>>();
            let possible_next_nodes2 = input
                .edges
                .iter()
                .filter(|e| e.v == current_node)
                .map(|e| (e.u, e.pvu, e.t))
                .collect::<Vec<_>>();
            let possible_next_nodes = possible_next_nodes1
                .into_iter()
                .chain(possible_next_nodes2)
                .collect::<Vec<_>>();

            let mut rng = rand::thread_rng();
            let mut cumulative_probability = 0.0;
            let random_value: f32 = rng.gen(); // Generate a random f32 number between 0 and 1

            for &(item, probability, t) in possible_next_nodes.iter() {
                cumulative_probability += probability;
                if cumulative_probability >= random_value {
                    current_node = item;
                    time += t;
                    break;
                }
            }
        }
        total_time_fed += time;
    }
    for _ in 0..iterations {
        let mut current_node = input.post;
        let mut time = 0.0;
        while current_node != input.home {
            let possible_next_nodes1 = input
                .edges
                .iter()
                .filter(|e| e.u == current_node)
                .map(|e| (e.v, e.puv, e.t))
                .collect::<Vec<_>>();
            let possible_next_nodes2 = input
                .edges
                .iter()
                .filter(|e| e.v == current_node)
                .map(|e| (e.u, e.pvu, e.t))
                .collect::<Vec<_>>();
            let possible_next_nodes = possible_next_nodes1
                .into_iter()
                .chain(possible_next_nodes2)
                .collect::<Vec<_>>();

            let mut rng = rand::thread_rng();
            let mut cumulative_probability = 0.0;
            let random_value: f32 = rng.gen(); // Generate a random f32 number between 0 and 1

            for &(item, probability, t) in possible_next_nodes.iter() {
                cumulative_probability += probability;
                if cumulative_probability >= random_value {
                    current_node = item;
                    time += t;
                    break;
                }
            }
        }
        total_time_post += time;
    }
    "\nfed: ".to_owned() + &(total_time_fed / iterations as f32).to_string()
        + "\npost: " + &(total_time_post / iterations as f32).to_string()
}
