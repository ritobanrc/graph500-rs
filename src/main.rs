use rand::{seq::SliceRandom, Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

type Node = usize;
type NodeList = Vec<Node>;
type Graph = Vec<NodeList>;
type Weight = f32;

/// An edge in the graph
#[derive(Debug, Clone, Default)]
pub struct Edge {
    pub start: Node,
    pub end: Node,
    // The weight of this edge
    // TODO: don't need to store this if only running kernel 2
    pub weight: Weight,
}

/// Step 1 of the benchmark, generates edge list
///
/// `scale` is the logarithm base 2 of the unmber of vertices.
/// `edge_factor` must be 16 for the actual benchmarks (but can be lower for testing)
fn generate_edge_list(scale: u32, edge_factor: usize) -> Vec<Edge> {
    let num_verts: usize = 1 << scale;
    let mut rng = XorShiftRng::from_entropy();

    /// The ratio of the graph’s edge count to its vertex count (i.e., half the average degree of a vertex in the graph).
    const A: f32 = 0.57;
    const B: f32 = 0.19;
    const C: f32 = 0.19;

    let ab = A + B;
    let c_norm = C / (1. - (A + B));
    let a_norm = A / (A + B);

    let num_edges = edge_factor * num_verts;

    let mut edge_list = vec![Edge::default(); num_edges];

    for ib in 1..scale {
        for edge in edge_list.iter_mut() {
            let ii = rng.gen::<f32>() > ab;
            let cmp = if ii { c_norm } else { a_norm };
            let jj = rng.gen::<f32>() > cmp;

            if ii {
                edge.start += 1 << (ib - 1);
            }
            if jj {
                edge.end += 1 << (ib - 1);
            }
        }
    }

    for edge in edge_list.iter_mut() {
        edge.weight = rng.gen::<f32>();
    }

    let mut vert_perm = (0..num_verts).collect::<Vec<_>>();
    vert_perm.shuffle(&mut rng);
    for edge in edge_list.iter_mut() {
        edge.start = vert_perm[edge.start];
        edge.end = vert_perm[edge.end];
    }

    edge_list.shuffle(&mut rng);

    edge_list
}

fn main() {
    //dbg!(generate_edge_list(5, 16));
    println!("{:?}", generate_edge_list(4, 4));
}
