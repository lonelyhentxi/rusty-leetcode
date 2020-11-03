/*
 * @lc app=leetcode.cn id=588 lang=rust
 *
 * [588] 设计内存文件系统
 */

// @lc code=start
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug)]
enum FSNode {
    Directory {
        children: BTreeMap<String, Rc<RefCell<FSNode>>>,
    },
    File {
        address: usize,
    },
}

impl FSNode {
    pub fn new_directory() -> Self {
        FSNode::Directory {
            children: BTreeMap::new(),
        }
    }

    pub fn new_file(address: usize) -> Self {
        FSNode::File { address }
    }
}

#[derive(Debug)]
struct FileSystem {
    vfs: Rc<RefCell<FSNode>>,
    files: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSystem {
    pub fn new() -> Self {
        Self {
            vfs: Rc::new(RefCell::new(FSNode::new_directory())),
            files: vec![],
        }
    }

    fn vcd(&self, ps: &[&str]) -> Option<(String, Rc<RefCell<FSNode>>)> {
        let mut curr = self.vfs.clone();
        let mut key = String::new();
        for p in ps {
            curr = {
                let mut curr_node = curr.borrow_mut();
                match *curr_node {
                    FSNode::Directory { ref mut children } => children[*p].clone(),
                    _ => {
                        return None;
                    },
                }
            };
            key = p.to_string();
        }
        Some((key, curr))
    }

    pub fn ls(&self, path: String) -> Vec<String> {
        let ps = path.split("/").skip(1).filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let directory_op = self.vcd(&ps[0..ps.len()]);
        if let Some((key, directory)) = directory_op {
            let directory_node = directory.borrow_mut();
            match *directory_node {
                FSNode::Directory { ref children } => children.iter().map(|(k, _)| k.clone()).collect::<Vec<String>>(),
                _ => vec![key],
            }
        } else {
            vec![]
        }
    }

    pub fn mkdir(&mut self, path: String) {
        let ps = path.split("/").skip(1).filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let mut curr = self.vfs.clone();
        for p in ps {
            curr = {
                let mut curr_node = curr.borrow_mut();
                match *curr_node {
                    FSNode::Directory { ref mut children } => {
                        if children.contains_key(p) {
                            children[p].clone()
                        } else {
                            let new_node = Rc::new(RefCell::new(FSNode::new_directory()));
                            children.insert(String::from(p), new_node.clone());
                            new_node
                        }
                    }
                    _ => unreachable!(),
                }
            };
        }
    }

    pub fn add_content_to_file(&mut self, file_path: String, content: String) {
        let ps = file_path.split("/").skip(1).filter(|s| !s.is_empty()).collect::<Vec<_>>();
        if ps.len() == 0 {
            return;
        }
        let directory = self.vcd(&ps[0..ps.len() - 1]).unwrap().1;
        let file_name = ps[ps.len() - 1];
        let mut directory_node = directory.borrow_mut();
        match *directory_node {
            FSNode::Directory { ref mut children } => {
                let file_ref = if children.contains_key(file_name) {
                    children[file_name].clone()
                } else {
                    let new_address = self.files.len();
                    self.files.push(String::new());
                    let new_node = Rc::new(RefCell::new(FSNode::new_file(new_address)));
                    children.insert(String::from(file_name), new_node.clone());
                    new_node
                };
                let file_node = file_ref.borrow();
                match *file_node {
                    FSNode::File { address } => { self.files[address] += &content; },
                    _ => unreachable!()
                };
            }
            _ => unreachable!(),
        }
    }

    fn read_content_from_file(&self, file_path: String) -> String {
        let ps = file_path.split("/").skip(1).filter(|s| !s.is_empty()).collect::<Vec<_>>();
        if ps.len() == 0 {
            return String::new();
        }
        let directory = self.vcd(&ps[0..ps.len() - 1]).unwrap().1;
        let file_name = ps[ps.len() - 1];
        let directory_node = directory.borrow_mut();
        match *directory_node {
            FSNode::Directory { ref children } => {
                let file_ref = children[file_name].clone();
                let file_node = file_ref.borrow();
                match *file_node {
                    FSNode::File { address } => { self.files[address].clone() },
                    _ => unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}

// @lc code=end


#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    fn test_file_system_1() {
        let mut fs = FileSystem::new();
        assert_eq!(fs.ls(String::from("/")), Vec::<String>::new());
        fs.mkdir(String::from("/a/b/c"));
        fs.add_content_to_file(String::from("/a/b/c/d"), String::from("hello"));
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["a"]);
        assert_eq!(fs.read_content_from_file(String::from("/a/b/c/d")), String::from("hello"));
    }

    #[test]
    fn test_file_system_2() {
        let mut fs = FileSystem::new();
        fs.mkdir(String::from("/goowmfn"));
        assert_eq!(fs.ls(String::from("/goowmfn")), Vec::<String>::new());
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["goowmfn"]);
        fs.mkdir(String::from("/z"));
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["goowmfn","z"]);
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["goowmfn","z"]);
        fs.add_content_to_file(String::from("/goowmfn/c"), String::from("shetopcy"));
        assert_eq!(fs.ls(String::from("/z")), Vec::<String>::new());
        assert_eq!(fs.ls(String::from("/goowmfn/c")), lc_vec_s!["c"]);
        assert_eq!(fs.ls(String::from("/goowmfn")), lc_vec_s!["c"]);
    }
}