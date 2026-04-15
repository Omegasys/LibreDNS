pub struct KademliaNode {
    pub id: String,
    pub known_nodes: Vec<String>,
}

impl KademliaNode {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            known_nodes: vec![],
        }
    }

    pub fn find_closest(&self, target: &str) -> Option<String> {
        self.known_nodes
            .iter()
            .min_by_key(|node| levenshtein(node, target))
            .cloned()
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let mut matrix = vec![vec![0; b.len() + 1]; a.len() + 1];

    for i in 0..=a.len() {
        matrix[i][0] = i;
    }

    for j in 0..=b.len() {
        matrix[0][j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };

            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[a.len()][b.len()]
}
