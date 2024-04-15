pub fn parse_input(input: &str) -> (i32, Vec<u128>) {
    let mut data = input.lines();
    let nodes = data.next().unwrap().parse().unwrap();
    let edges = data.map(string_to_edge).collect::<Vec<_>>();
    (nodes, edges)
}

fn string_to_edge(s: &str) -> u128 {
    u128::from_str_radix(&s.replace(" ", "").chars().rev().collect::<String>(), 2).unwrap()
}

pub fn r0(state: u128, edges: &Vec<u128>) -> u32 {
    if state == 0 {
        return 0;
    }
    let mut max = 0;
    let mut max_i = 0;
    for (i, e) in edges.iter().enumerate() {
        if (1 << i) & state == 0 {
            continue;
        }
        let deg = (state & e).count_ones();
        if deg == 0 {
            return 1 + r0(state ^ (1 << i), edges);
        }
        if deg > max {
            max = deg;
            max_i = i;
        }
    }
    return r0(state ^ (1 << max_i), edges)
        .max(1 + r0((state ^ (1 << max_i)) & !edges[max_i], edges));
}

pub fn r1(state: u128, edges: &Vec<u128>) -> u32 {
    if state == 0 {
        return 0;
    }
    let mut max = 0;
    let mut max_i = 0;

    for (i, e) in edges.iter().enumerate() {
        if (1 << i) & state == 0 {
            continue;
        }
        let deg = (state & e).count_ones();
        if deg == 1 {
            return 1 + r1((state ^ (1 << i)) & !edges[i], edges);
        }
        if deg == 0 {
            return 1 + r1(state ^ (1 << i), edges);
        }
        if deg > max {
            max = deg;
            max_i = i;
        }
    }
    return r1(state ^ (1 << max_i), edges)
        .max(1 + r1((state ^ (1 << max_i)) & !edges[max_i], edges));
}

pub fn r2(state: u128, edges: &Vec<u128>) -> u32 {
    if state == 0 {
        return 0;
    }
    let mut max = 0;
    let mut max_i = 0;
    for (i, e) in edges.iter().enumerate() {
        if (1 << i) & state == 0 {
            continue;
        }
        let deg = (state & e).count_ones();
        if deg == 1 {
            return 1 + r2((state ^ (1 << i)) & !edges[i], edges);
        }
        if deg == 0 {
            return 1 + r2(state ^ (1 << i), edges);
        }
        if deg > max {
            max = deg;
            max_i = i;
        }
    }
    return r2(state ^ (1 << max_i), edges)
        .max(1 + r1((state ^ (1 << max_i)) & !edges[max_i], edges));
}
