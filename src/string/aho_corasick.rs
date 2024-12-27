use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::{Rc, Weak};

#[derive(Default)]
#[derive(Debug)]
struct ACNode {
    trans: BTreeMap<char, Rc<RefCell<ACNode>>>,
    suffix: Weak<RefCell<ACNode>>,
    lengths: Vec<usize>
}

#[derive(Default)]
#[derive(Debug)]
pub struct AhoCorasick {
    root: Rc<RefCell<ACNode>>
}

impl AhoCorasick {
    pub fn new(words: &[&str]) -> Self {
        let root: Rc<RefCell<ACNode>> = Rc::new(RefCell::new(ACNode::default()));

        for word in words{
            let mut current_node = Rc::clone(&root);

            for char in word.chars() {
                current_node = Rc::clone(Rc::clone(&current_node).borrow_mut().trans.entry(char).or_default());
            }

            current_node.borrow_mut().lengths.push(word.len());
        }

        Self::build_suffix(Rc::clone(&root));
        Self {root}
    }

    fn build_suffix(root: Rc<RefCell<ACNode>>){
        let mut vec_queue: VecDeque<Rc<RefCell<ACNode>>> = VecDeque::new();

        vec_queue.push_back(Rc::clone(&root));

        while let Some(parent) = vec_queue.pop_front() {
            let parent_borrow = parent.borrow();

            for (c, child) in &parent_borrow.trans {
                vec_queue.push_back(Rc::clone(child));
                let mut child = child.borrow_mut();
                let mut suffix: Option<Rc<RefCell<ACNode>>> = parent_borrow.suffix.upgrade();

                loop {
                    match &suffix {
                        None => {
                            child.lengths.extend(root.borrow().lengths.clone());
                            child.suffix = Rc::downgrade(&root);
                            break;
                        }
                        Some(node) => {
                            if node.borrow().trans.contains_key(c) {
                                let node = &node.borrow().trans[c];

                                child.lengths.extend(node.borrow().lengths.clone());
                                child.suffix = Rc::downgrade(node);
                                break;
                            } else {
                                suffix = suffix.unwrap().borrow().suffix.upgrade();
                            }
                        }
                    }
                }

            }
        }
    }

    pub fn search<'a>(&self, s: &'a str) -> Vec<&'a str> {
        let mut ans = vec![];
        let mut cur = Rc::clone(&self.root);
        let mut position: usize = 0;

        for char in s.chars() {
            loop {
                if let Some(child) = Rc::clone(&cur).borrow().trans.get(&char){
                    cur = Rc::clone(child);
                    break;
                }

                let suffix = cur.borrow().suffix.clone();

                match suffix.upgrade() {
                    Some(node) => cur = node,
                    None => break,
                }
            }

            position += char.len_utf8();

            for &length in &cur.borrow().lengths {
                ans.push(&s[position-length..position]);
            }
        }
        
        ans
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let dict = ["abc", "abcd", "xyz", "acxy", "efg", "123", "678", "6543"];
        let ac = AhoCorasick::new(&dict);
        let res = ac.search("ababcxyzacxy12678acxy6543");

        assert_eq!(res, ["abc", "xyz", "acxy", "678", "acxy", "6543",]);
    }

    #[test]
    fn test_aho_corasick_with_utf8(){
        let dict = ["abc",
            "中文",
            "abc中",
            "abcd",
            "xyz",
            "acxy",
            "efg",
            "123",
            "678",
            "6543",
            "ハンバーガー",];

        let ac = AhoCorasick::new(&dict);

        let res = ac.search("ababc中xyzacxy12678acxyハンバーガー6543中文");

        assert_eq!(res, [
            "abc",
            "abc中",
            "xyz",
            "acxy",
            "678",
            "acxy",
            "ハンバーガー",
            "6543",
            "中文"
        ]);
    }
}