use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::fs;
use std::rc::{Rc, Weak};

type NodeRef = Rc<RefCell<Node>>;
type ParentRef = Weak<RefCell<Node>>;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<NodeRef>,
    parent: Option<ParentRef>,
    files: Vec<File>,
}

impl Node {
    fn new(name: &str) -> Node {
        return Node {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            files: Vec::new(),
        };
    }

    fn new_noderef(node: Node) -> NodeRef {
        return Rc::new(RefCell::new(node));
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(File {
            name: name.to_string(),
            size: size,
        });
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.as_ref().map_or(None, |p| p.upgrade())
    }

    fn add_child(parent: &mut NodeRef, child: NodeRef) {
        child.borrow_mut().parent = Some(Rc::downgrade(&parent.clone()));
        parent.borrow_mut().children.push(child);
    }

    fn get_child(&self, name: &str) -> Option<NodeRef> {
        self.children
            .iter()
            .filter(|c| c.borrow().name == name)
            .map(|c| c.clone())
            .last()
    }

    fn init_tree(root: NodeRef, contents: &str) -> NodeRef {
        let lines: Vec<&str> = contents.split("\n").collect();
        let root = Node::new_noderef(Node::new("/"));
        let mut cur: NodeRef = root.clone();
        for line in &lines[1..] {
            match line.split(" ").collect::<Vec<&str>>()[..] {
                ["$", "ls"] => {}
                ["$", "cd", ".."] => {
                    cur = cur.clone().borrow().get_parent().unwrap();
                }
                ["$", "cd", directory] => {
                    cur = cur.clone().borrow().get_child(directory).unwrap();
                }
                ["dir", name] => {
                    Node::add_child(&mut cur, Node::new_noderef(Node::new(name)));
                }
                [size, name] => {
                    cur.borrow_mut().add_file(name, size.parse().unwrap());
                }
                _ => {}
            }
        }
        root.clone()
    }

    fn dirsize(&self, size: usize) -> usize {
        self.children
            .iter()
            .map(|c| c.borrow().dirsize(size))
            .sum::<usize>()
            + self.files.iter().map(|f| f.size).sum::<usize>()
    }

    // Sum all directories under `limit`
    fn sum_under_limit(&self, limit: usize) -> usize {
        let mut size = self
            .children
            .iter()
            .map(|c| c.borrow().sum_under_limit(limit))
            .sum();

        if self.dirsize(0) <= limit {
            size += self.dirsize(0);
        }

        size
    }

    // Find dir with largest size < limit
    fn largest_under_limit(&self, limit: usize) -> usize {
        let mut size = self
            .children
            .iter()
            .map(|c| c.borrow().largest_under_limit(limit))
            .filter(|s| s >= &limit)
            .min()
            .unwrap_or(usize::MAX);

        let dirsize = self.dirsize(0);

        if dirsize >= limit && dirsize < size {
            size = dirsize
        }

        size
    }
}

fn part1(root: NodeRef) -> usize {
    let sum_sizes = root.borrow().sum_under_limit(100000);
    println!("part1: sum_sizes: {}", sum_sizes);
    return sum_sizes;
}

fn part2(root: NodeRef) -> usize {
    let disk_size: usize = 70000000;
    let update_size_needed: usize = 30000000;
    let total_size = root.borrow().dirsize(0); // Disk space used
    let need_space = update_size_needed - (disk_size - total_size); // File to delete size
    let smallest = root.borrow().largest_under_limit(need_space);
    println!("smallest dir size to delete: {}", smallest);
    smallest
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;
    let root = Node::init_tree(Node::new_noderef(Node::new("/")), contents.as_str());
    part1(root.clone());
    part2(root.clone());
    Ok(())
}
