use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: HashSet<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: HashSet::new(),
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn build_node(&mut self, val: T) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn node_size(&self, idx: usize, get_val: fn(&T) -> u32) -> u32 {
        let node = &self.arena[idx];

        // Add a local cache to save the stack from exploding
        let mut node_cache: HashMap<usize, u32> = HashMap::new();

        let child_sum: u32 = node
            .children
            .iter()
            .map(|i| match node_cache.get(i) {
                Some(cached) => *cached,
                None => {
                    let sz = &self.node_size(*i, get_val);
                    node_cache.insert(*i, *sz);
                    *sz
                }
            })
            .sum();

        get_val(&node.val) + child_sum
    }
}

fn build_tree(lines: Vec<String>) -> ArenaTree<(u32, String)> {
    let mut tree: ArenaTree<(u32, String)> = ArenaTree::default();
    let mut current_dir: Option<usize> = None;

    // Populate arena tree
    // With sizes only stored in leaves
    for line in lines {
        let mut splits = line.split(" ");

        match splits.next() {
            Some("$") => match splits.next() {
                Some("cd") => {
                    current_dir = match splits.next() {
                        Some("..") => tree.arena[current_dir.unwrap()].parent,
                        Some(dirname) => match current_dir {
                            None => Some(tree.build_node((0, dirname.to_string()))),
                            Some(dir) => tree.arena[dir].children.clone().into_iter().find(|&n| {
                                let (_, name) = &tree.arena[n].val;
                                name == dirname
                            }),
                        },

                        None => panic!(),
                    }
                }
                Some("ls") => {
                    continue;
                }
                None => panic!(),
                _ => panic!(),
            },
            Some(size_or_dir) => {
                // Find/create a dir node and:
                // - set its parent to current dir.
                // - add this dir to current dir node's children.
                let size = match size_or_dir {
                    "dir" => 0,
                    s => s.parse().unwrap(),
                };
                let name = splits.next().unwrap().to_string();
                let current_node = &tree.arena[current_dir.unwrap()];
                if current_node
                    .children
                    .iter()
                    .find(|&&c| tree.arena[c].val.1 == name)
                    .is_none()
                {
                    let leaf = tree.build_node((size, name));
                    tree.arena[leaf].parent = current_dir;
                    tree.arena[current_dir.unwrap()].children.insert(leaf);
                }
            }
            None => panic!(),
        }
    }

    tree
}

fn get_node_size<T>(val: &(u32, T)) -> u32 {
    val.0
}

pub fn task_one(lines: Vec<String>) -> String {
    let tree = build_tree(lines);

    tree.arena
        .iter()
        .filter_map(|n| {
            let size = tree.node_size(n.idx, get_node_size);

            // Only count directories
            if size < 100000 && !n.children.is_empty() {
                Some(size)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn task_two(lines: Vec<String>) -> String {
    let tree = build_tree(lines);
    let max_size = 70000000;
    let required_size = 30000000;
    let current = max_size - tree.node_size(0, get_node_size);
    let threshold = required_size - current;

    tree.arena
        .iter()
        .filter_map(|n| {
            let size = tree.node_size(n.idx, get_node_size);

            // Only count directories
            if size > threshold && !n.children.is_empty() {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}
