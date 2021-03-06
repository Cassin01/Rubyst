use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub root: Op<String>,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op<T> {
    Nil,
    Add,
    Neg,
    Mul,
    Div,
    Rem,
    Pow,
    Asi,    // Assignment operator
    Fun(T),
    ROp(T), // Relational operator
    Num(T), // Num
    Str(T), // String
    Val(T),
    STMT,   // Statement
}

pub trait TreeInsert<T> {
    fn left(self, T) -> Tree;
    fn right(self, T) -> Tree;
}

impl TreeInsert<Option<Box<Tree>>> for Tree {
    fn left(mut self, leaf: Option<Box<Tree>>) -> Tree {
        self.left = leaf;
        self
    }
    fn right(mut self, leaf: Option<Box<Tree>>) -> Tree {
        self.right = leaf;
        self
    }
}

impl TreeInsert<Tree> for Tree {
    fn left(mut self, leaf: Tree) -> Tree {
        self.left = Some(Box::new(leaf));
        self
    }
    fn right(mut self, leaf: Tree) -> Tree {
        self.right = Some(Box::new(leaf));
        self
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree({:?})", self.root)
    }
}

impl Tree {
    pub fn new(root: Op<String>) -> Tree {
        Tree {
            root: root,
            left: None,
            right: None,
        }
    }

    pub fn root(mut self, root: Op<String>) -> Self {
        self.root = root;
        self
    }

    pub fn enum_op(op: String) -> Op<String> {
        match op.as_str() {
            "*"  => Op::Mul,
            "/"  => Op::Div,
            "+"  => Op::Add,
            "-"  => Op::Neg,
            "%"  => Op::Rem,
            "**" => Op::Pow,
            "==" => Op::ROp(String::from("==")),
            "!=" => Op::ROp(String::from("!=")),
            "<"  => Op::ROp(String::from("<")),
            "<=" => Op::ROp(String::from("<=")),
            ">=" => Op::ROp(String::from(">=")),
            ">"  => Op::ROp(String::from(">")),
            _    => panic!("not operator"),
        }
    }
}

pub trait PushBack<T> {
    fn push_back(&mut self, new_node: T);
}

impl PushBack<Tree> for Tree {
    fn push_back(&mut self, new_node: Tree) {
        let _node = Self::last_node(&mut self.right);
        *_node = Some(Box::new(new_node));
    }
}

impl PushBack<Op<String>> for Tree {
    fn push_back(&mut self, new_node: Op<String>) {
        let _node = Self::last_node(&mut self.right);
        *_node = Some(Box::new(Tree::new(new_node)));
    }
}

impl Tree {
    // 最終要素探索関数
    fn last_node(tree: &mut Option<Box<Tree>>) -> &mut Option<Box<Tree>> {
        if let Some(ref mut _n) = *tree {
            Self::last_node(&mut _n.right)
        }
        else {
            tree
        }
    }
}