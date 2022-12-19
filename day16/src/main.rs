use std::{fs, collections::{HashMap, HashSet, BinaryHeap}, cmp::min};

fn build_graph_from_input() -> (HashMap<String, Vec<String>>, HashMap<String, i32>) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut flow_rates: HashMap<String, i32> = HashMap::new();

    fs::read_to_string("input")
        .unwrap()
        .lines()
        .for_each(|line| {
            let label = &line[6..8];
            let rest: Vec<&str> = line[23..].split(";").collect();
            let flow: i32 = rest[0].parse().unwrap();
            let adjacent: Vec<String> = rest[1].replace("s", "")[22..]
                .split(", ")
                .map(|str| str.to_string())
                .collect();
            graph.insert(label.to_string(), adjacent);
            flow_rates.insert(label.to_string(), flow);
        });

    (graph, flow_rates)
}

impl Ord for QueuedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for QueuedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

// impl PartialEq for QueuedNode {
//     fn eq(&self, other: &Self) -> bool {
//         self.node == other.node
//     }
// }

// impl Hash for QueuedNode {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.node.hash(state);
//     }
// }

#[derive(Eq, PartialEq)]
struct QueuedNode {
    node: String,
    distance: i32
}

fn dijkstras_from(from: &str, graph: &HashMap<String, Vec<String>>) -> HashMap<String, i32> {
    let mut dists: HashMap<String, i32> = HashMap::new();
    dists.insert(from.to_owned(), 0);

    let mut visited: HashSet<String> = HashSet::new();
    
    let mut next: BinaryHeap<QueuedNode> = BinaryHeap::new();
    next.push(QueuedNode {node: from.to_string(), distance: 0});


    while next.len() > 0 {
        let current = next.pop().unwrap();
        visited.insert(current.node.to_string());
        for neighbor in graph.get(&current.node).unwrap() {
            let candidate_dist = current.distance + 1;
            let new_dist = match dists.get(neighbor) {
                Some(dist) => min(*dist, candidate_dist),
                None => candidate_dist  
            };
            dists.insert(neighbor.to_string(), new_dist);

            if !visited.contains(neighbor) {
                next.push(QueuedNode {
                    node: neighbor.to_string(),
                    distance: min(new_dist, candidate_dist)
                })
            }
        }
    }

    dists
}

fn part_1() -> i32 {
    let (graph, flow_rates) = build_graph_from_input();

    let mut helpful_valves: Vec<String> = graph.keys().filter(|&node| {
        flow_rates.get(node).unwrap() > &0 || node == "AA"
    }).map(|str| str.to_string()).collect();
    helpful_valves.sort();
    let mut id_to_valve: HashMap<i32, String> = HashMap::new();
    let mut valve_to_id: HashMap<String, i32> = HashMap::new();
    let mut valve_flow_rates: HashMap<i32, i32> = HashMap::new();
    
    let mut cur_valve_id = 0;
    for valve in &helpful_valves {
        id_to_valve.insert(cur_valve_id, valve.to_string());
        valve_to_id.insert(valve.to_string(), cur_valve_id);
        valve_flow_rates.insert(cur_valve_id, *flow_rates.get(valve).unwrap());
        cur_valve_id += 1;
    }

    let mut valve_distances: HashMap<i32, Vec<i32>> = HashMap::new();

    for valve in &helpful_valves {
        let mut distances = vec![];
        let distances_from_valve = dijkstras_from(valve, &graph);
        
        for valve_id in 0..cur_valve_id {
            distances.push(*distances_from_valve.get(id_to_valve.get(&valve_id).unwrap()).unwrap())
        }

        valve_distances.insert(*valve_to_id.get(valve).unwrap(), distances);
    }
    
    /*
        map each string to a number
        
        OPT(n, v, m) = for each node not visited in v, max( pressure_released + OPT(n, v - n, m - dist + 1))
    */

    let max_pressure: i32 = find_max_pressure_release(*valve_to_id.get("AA").unwrap(), (1 << cur_valve_id) - 1, 30, &valve_distances, &valve_flow_rates);

    max_pressure
}

fn find_max_pressure_release(starting_node: i32, visitable_bitmask: i32, starting_time: i32, distances: &HashMap<i32, Vec<i32>>, valve_flow_rates: &HashMap<i32, i32>) -> i32 {
    let mut dp_table = vec![vec![vec![0; distances.len()]; (visitable_bitmask + 1) as usize]; starting_time as usize + 1];

    for current_time in 1..=(starting_time as usize) {
        for visitable in 1..=(visitable_bitmask as usize) {
            for node in 0..distances.len() {
                let mut max_pressure = 0;
                for (destination, distance) in distances[&(node as i32)].iter().enumerate() {
                    let destination_mask = 1 << destination;
                    if (destination_mask & visitable) > 0 {
                        let time_after_opening = (current_time as i32) - distance - 1;
                        if time_after_opening < 0 {
                            continue
                        }
                        let possible_pressure = time_after_opening * valve_flow_rates[&(destination as i32)] + dp_table[time_after_opening as usize][visitable - destination_mask][destination];
                        if possible_pressure > max_pressure {
                            max_pressure = possible_pressure
                        }
                    }
                }
                dp_table[current_time][visitable][node] = max_pressure;
            }
        }
    }

    dp_table[starting_time as usize][visitable_bitmask as usize][starting_node as usize]
}

fn part_2() -> i32 {
    5
}

fn main() {

    println!("Part 1");
    let part_1 = part_1();
    println!("{part_1}");

    println!("Part 2");
    println!("TODO!");
    let part_2 = part_2();
    println!("{part_2}");
}