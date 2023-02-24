use rand::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

type Node = usize;
type NodeList = Vec<Node>;
type Graph = Vec<NodeList>;
//type AdjMatrix = Vec<Vec<bool>>;

fn generate_graph_list(
    nodes: usize,
    min_neighbors: usize,
    max_neighbors: usize,
    seed: u64,
) -> Graph {
    let neighbor_range = max_neighbors - min_neighbors;
    assert!(neighbor_range > 0);

    let mut rng = XorShiftRng::seed_from_u64(seed);

    (0..nodes)
        .map(|i| {
            let num_neighbors = (rng.next_u64() as usize) % neighbor_range + min_neighbors;
            let mut adj = vec![false; nodes];
            (0..num_neighbors)
                .map(|_| {
                    // feels like a scuffed way to generate n distinct numbers
                    let mut neighbor = (rng.next_u64() as usize) % nodes;
                    while neighbor == i || adj[neighbor] {
                        neighbor = (rng.next_u64() as usize) % nodes;
                    }
                    adj[neighbor] = true;

                    neighbor
                })
                .collect()
        })
        .collect::<Graph>()
}

fn main() {
    dbg!(generate_graph_list(10, 0, 10, 1234));
}
