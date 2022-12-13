use std::{cell::RefCell, rc::Rc, str::SplitWhitespace};

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    value: T,
    children: Vec<NodeRef<T>>,
    parent: Option<NodeRef<T>>,
}

impl<T> From<Node<T>> for NodeRef<T> {
    fn from(n: Node<T>) -> Self {
        Rc::new(RefCell::new(n))
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            children: vec![],
            parent: None,
        }
    }

    fn add_child(&mut self, value: T) -> NodeRef<T> {
        let n: NodeRef<T> = Node::new(value).into();
        self.children.push(n.clone());
        n
    }

    fn traverse(&self, func: &dyn Fn(&Self)) {
        func(self);
        for c in self.children.iter() {
            c.borrow().traverse(func);
        }
    }

    #[allow(dead_code)]
    fn traverse_breadth_first(&mut self, func: &dyn Fn(&mut Self)) {
        func(self);
        for c in self.children.iter() {
            c.borrow_mut().traverse_breadth_first(func);
        }
    }

    fn traverse_depth_first(&mut self, func: &dyn Fn(&mut Self)) {
        for c in self.children.iter() {
            c.borrow_mut().traverse_depth_first(func);
        }
        func(self);
    }
}

#[derive(PartialEq)]
enum InodeKind {
    File,
    Directory,
}

struct Inode {
    name: String,
    size: u32,
    kind: InodeKind,
    level: u32,
}

impl Inode {
    fn new(name: String, kind: InodeKind, size: u32, level: u32) -> Self {
        if kind == InodeKind::Directory {
            assert!(size == 0);
        }

        Inode {
            name,
            kind,
            size,
            level,
        }
    }
}

fn parse_command(context: NodeRef<Inode>, cmd_line: &mut SplitWhitespace) -> NodeRef<Inode> {
    let cmd = cmd_line.next().unwrap();

    if cmd == "cd" {
        let ctx = context.borrow();
        let arg = cmd_line.next().unwrap();
        if arg == ".." {
            return match &ctx.parent {
                Some(ctx) => ctx.clone(),
                None => {
                    drop(ctx);
                    context
                }
            };
        }

        let c = ctx.children.iter().find(|x| {
            let node = x.borrow();
            node.value.name == arg
        });
        if let Some(nc) = c {
            return nc.clone();
        }
    }

    context
}

pub fn aoc(lines: &[String]) {
    let root: NodeRef<_> =
        Node::new(Inode::new("root".to_string(), InodeKind::Directory, 0, 0)).into();
    let mut context = root.clone();

    // First line is just cd to /
    for line in lines.iter().skip(1) {
        let mut split = line.split_whitespace();
        let p1 = split.next().unwrap();
        if let "$" = p1 {
            context = parse_command(context.clone(), &mut split);
        } else {
            let mut b = context.borrow_mut();
            let lvl = b.value.level;

            let child = match p1 {
                "dir" => {
                    let name = split.next().unwrap();
                    Inode::new(name.to_owned(), InodeKind::Directory, 0, lvl + 1)
                }
                &_ => {
                    let size = p1.parse().unwrap();
                    let name = split.next().unwrap();
                    Inode::new(name.to_owned(), InodeKind::File, size, lvl + 1)
                }
            };

            let node = b.add_child(child);
            node.borrow_mut().parent = Some(context.clone());
        }
    }

    root.borrow_mut().traverse_depth_first(&|n| {
        let inode = &mut n.value;
        let size = n
            .children
            .iter()
            .fold(inode.size, |acc, n| acc + n.borrow().value.size);
        inode.size = size;
    });

    let node = root.borrow();
    p1(&node);
    p2(&node);
}

fn p1(root: &Node<Inode>) {
    let sum = RefCell::new(0);
    root.traverse(&|n| {
        let node = &n.value;
        if node.kind == InodeKind::Directory && node.size < 100000 {
            // println!("{}", node.name);
            *sum.borrow_mut() += node.size;
        }
    });
    println!("{}", sum.borrow());
}

fn p2(root: &Node<Inode>) {
    let fs = 70_000_000;
    let target = fs - 30_000_000;
    let usage = root.value.size;

    let smallest = RefCell::new(u32::MAX);
    root.traverse(&|n| {
        let node = &n.value;
        if node.kind == InodeKind::Directory
            && usage - node.size <= target
            && *smallest.borrow() > node.size
        {
            // println!("{}", node.name);
            *smallest.borrow_mut() = node.size;
        }
    });

    println!("{}", smallest.borrow());
}
