use std::{
    collections::{HashMap, HashSet},
    iter::Sum,
    ops::Add,
};

#[derive(Debug)]
struct Node<T, R>
where
    T: PartialEq,
    R: PartialEq,
{
    idx: usize,
    name: R,
    val: T,
    parent: Option<usize>,
    children: HashSet<usize>,
}

impl<T, R> Node<T, R>
where
    T: PartialEq,
    R: PartialEq,
{
    fn new(idx: usize, val: T, name: R) -> Self {
        Self {
            idx,
            name,
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
    arena: Vec<Node<T, String>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + Add<Output = T> + Sum + Copy,
{
    fn node(&mut self, val: T, name: &str) -> usize {
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }

        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val, name.to_string()));
        idx
    }

    fn node_size(&self, idx: usize) -> T {
        let node = &self.arena[idx];

        // Add a local cache to save the stack from exploding
        let mut node_cache: HashMap<usize, T> = HashMap::new();

        println!("node_size for: {}-{}", idx, node.name);

        let child_sum = node
            .children
            .iter()
            .map(|i| match node_cache.get(i) {
                Some(cached) => *cached,
                None => {
                    let sz = &self.node_size(*i);
                    node_cache.insert(*i, *sz);
                    *sz
                }
            })
            .sum();

        node.val + child_sum
    }
}

fn build_tree(lines: Vec<String>) -> ArenaTree<u32> {
    let mut tree: ArenaTree<u32> = ArenaTree::default();
    let mut current_dir: Option<usize> = None;

    // Populate arena tree
    // With sizes only stored in leaves
    for line in lines {
        let mut splits = line.split(" ");

        match splits.next() {
            Some("$") => match splits.next() {
                Some("cd") => match splits.next() {
                    Some("..") => {
                        println!(
                            "cd ..: {:?} -> {:?}",
                            current_dir,
                            tree.arena[current_dir.unwrap()].parent
                        );
                        current_dir = tree.arena[current_dir.unwrap()].parent
                    }
                    Some(dirname) => {
                        let dir_node = Some(tree.node(0, dirname));
                        println!("cd {}: {:?} -> {:?}", dirname, current_dir, dir_node);
                        current_dir = dir_node
                    }

                    None => panic!(),
                },
                Some("ls") => {
                    println!(
                        "ls: {:?}-{:?}",
                        current_dir,
                        tree.arena[current_dir.unwrap()].name
                    );
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
                let leaf = tree.node(size, splits.next().unwrap());
                tree.arena[leaf].parent = current_dir;
                tree.arena[current_dir.unwrap()].children.insert(leaf);
            }
            None => panic!(),
        }
    }

    tree
}

pub fn task_one(lines: Vec<String>) -> String {
    let tree = build_tree(lines);

    println!("{:?}", tree);

    tree.arena
        .iter()
        .filter_map(|n| {
            let size = tree.node_size(n.idx);

            if size < 10000 {
                Some(size)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}
