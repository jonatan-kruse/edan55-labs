use fedups::{run_markov, run_monte_carlo};

fn main() {
    run_markov("toy");
    run_monte_carlo("toy");
    run_markov("small");
    run_monte_carlo("small");
    run_markov("rnd1");
    run_monte_carlo("rnd1");
    run_markov("rnd2");
    run_monte_carlo("rnd2");
    run_markov("rnd3");
    run_monte_carlo("rnd3");
    run_markov("strange1");
    run_monte_carlo("strange1");
    run_markov("strange2");
    run_monte_carlo("strange2");
}
