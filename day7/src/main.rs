use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::fs;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Cmd<'a> {
    cmd: &'a str,
    output: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    fn new(cmd: &'a str) -> Self {
        return Cmd {
            cmd: cmd,
            output: Vec::new(),
        };
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

type NodeRef = Rc<RefCell<Node>>;
type ParentRef = Weak<RefCell<Node>>;

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
        let parent_target = self.parent.as_ref();

        return if parent_target.is_some() {
            parent_target.unwrap().upgrade()
        } else {
            println!("[!!!] get_parent, no parent ref from: {}", self.name);
            None
        };
    }

    fn add_child(parent: &mut NodeRef, child: NodeRef) {
        child.borrow_mut().parent = Some(Rc::downgrade(&parent.clone()));
        parent.borrow_mut().children.push(child);
    }

    fn get_child(&self, name: &str) -> Option<NodeRef> {
        for child in &self.children {
            if child.borrow().name == name {
                return Some(child.clone());
            }
        }

        None
    }

    fn init_tree(root: NodeRef, contents: &str) -> NodeRef {
        let cmd_lines: Vec<&str> = contents.split("\n").collect();
        let cmds = get_cmds(cmd_lines);
        let groot = root.clone();
        let mut cur_node = root;

        for cmd in &cmds[1..] {
            // eprintln!("cmd = {:?}, cur_node = {:?}", cmd, cur_node.borrow().name);

            match &cmd.cmd.split(" ").collect::<Vec<&str>>()[..] {
                ["ls"] => {
                    for out in &cmd.output {
                        match out.split(" ").collect::<Vec<&str>>()[..] {
                            ["dir", directory] => {
                                // Create child
                                Node::add_child(
                                    &mut cur_node,
                                    Node::new_noderef(Node::new(directory)),
                                );
                            }
                            [size, filename] => {
                                // Create file
                                cur_node
                                    .borrow_mut()
                                    .add_file(filename, size.parse().unwrap());
                            }
                            _ => {
                                println!("!!!");
                            }
                        }
                    }
                }

                ["cd", ".."] => {
                    let parent = cur_node.borrow().get_parent();
                    if parent.is_some() {
                        cur_node = parent.unwrap();
                    } else {
                        println!("[!!!] cd .. error -- no parent");
                    }
                }

                ["cd", directory] => {
                    // Find child
                    let child = cur_node.borrow().get_child(directory);
                    if child.is_some() {
                        cur_node = child.unwrap();
                    } else {
                        println!("[!!!] get_child error, no child");
                    }

                    // Update cur_node
                }

                _ => {
                    println!("!!!");
                }
            }
        }

        groot
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

    fn fmt_internal(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        let res = write!(f, "{} - {}\n", " ".repeat(depth * 2), self.name);
        for file in &self.files {
            let _ = write!(
                f,
                "{} * {} {}\n",
                " ".repeat((depth + 1) * 2),
                file.size,
                file.name
            );
        }
        for child in &self.children {
            let t = child.borrow();
            let _ = t.fmt_internal(f, depth + 1);
        }
        res
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_internal(f, 0)
    }
}

fn get_cmds(cmd_lines: Vec<&str>) -> Vec<Cmd> {
    let mut cmds: Vec<Cmd> = Vec::new();

    let mut _tmpcmd = Cmd::new("");

    for cmd in cmd_lines {
        if &cmd[0..1] == "$" {
            _tmpcmd = Cmd::new(&cmd[2..]);
            cmds.push(_tmpcmd); // moving tmpcmd into cmds
        } else {
            let curcmd = cmds.len() - 1;
            cmds[curcmd].output.push(cmd);
        }
    }

    cmds
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
