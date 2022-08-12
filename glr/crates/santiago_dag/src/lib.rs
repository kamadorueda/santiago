#[derive(Debug, Default)]
pub struct LayeredDAG<T> {
    pub layers: Vec<DAGLayer<T>>,
}

impl<T> LayeredDAG<T> {
    pub fn clone_last_layer(&mut self)
    where
        T: Clone,
    {
        let last_layer = self.layers.last().unwrap();

        let nodes = last_layer.nodes.clone();
        let predecessors = (0..nodes.len()).map(|i| Vec::from([i])).collect();

        self.layers.push(DAGLayer { nodes, predecessors });
    }
}

#[derive(Debug, Default)]
pub struct DAGLayer<T> {
    /// Nodes in the current layer.
    pub nodes: Vec<T>,
    /// `connectivity[node_index] = [node indexes in previous layer]`.
    pub predecessors: Vec<Vec<usize>>,
}

// "I saw a man on the bed in the apartment with a telescope"

// shift 0 [0] *n *v *d *n *p *d *n *p *d *n |

// shift 4 [0 4] *v *d *n *p *d *n *p *d *n | n
// reduce 3 [0] NP *v *d *n *p *d *n *p *d *n | NP(n)

// shift 2 [0 2] *v *d *n *p *d *n *p *d *n | NP(n)

// shift 7 [0 2 7] *d *n *p *d *n *p *d *n | v NP(n)
// shift 3 [0 2 7 3] *n *p *d *n *p *d *n | d v NP(n)
// shift 10 [0 2 7 3 10] *p *d *n *p *d *n | n d v NP(n)
// reduce 4 [0 2 7] NP *p *d *n *p *d *n | NP(d,n) v NP(n)

// shift 12 [0 2 7 12] *p *d *n *p *d *n | NP(d,n) v NP(n)
// \ shift 6 [0 2 7 12 6] *d *n *p *d *n | p NP(d,n) v NP(n)
// \ reduce 7 [0 2] VP *p *d *n *p *d *n | VP(v,NP(d,n)) NP(n)
//   shift 8 [0 2 8] *p *d *n *p *d *n | VP(v,NP(d,n)) NP(n)
//   reduce 1 [0] S *p *d *n *p *d *n | VP(v,NP(d,n)) NP(n)
//   shift 1 [0 1] *p *d *n *p *d *n
//   shift 6 [0 1 6] *d *n *p *d *n
//   shift 3 [0 1 6 3] *n *p *d *n
//   shift 10 [0 1 6 3 10] *p *d *n
//   reduce 4 [0 1 6] NP *p *d *n
//   shift 11 [0 1 6 11] *p *d *n
//   \ reduce 6 [0 1] PP *p *d *n
//   \ shift 6 [0 1 6 11 6] *d *n
