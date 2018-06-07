pub trait Tree<K,V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

#[derive(Debug)]
pub struct BinaryTreeNode<K,V> {
    key: K,
    value: V,
    left: Option<Box<BinaryTreeNode<K,V>>>,
    right: Option<Box<BinaryTreeNode<K,V>>>,
}

impl <K: Ord + Clone, V: Clone> BinaryTreeNode<K, V> {
    pub fn new(key: K, value: V) -> BinaryTreeNode<K, V> {
        BinaryTreeNode {
            key,
            value,
            left: None,
            right: None,
        }
    }

    fn remove_descendent(&mut self, key: &K) -> Option<V> {
        if self.left.is_some() && self.left.as_ref().unwrap().key == *key {
            return self.remove_left();
        }

        if self.left.is_some() && *key < self.key {
            return self.left.as_mut().unwrap().remove_descendent(key);
        }

        if self.right.is_some() && self.right.as_ref().unwrap().key == *key {
            return self.remove_right();
        }

        if self.right.is_some() && *key < self.key {
            return self.right.as_mut().unwrap().remove_descendent(key);
        }

        None
    }

    fn remove_left(&mut self) -> Option<V> {
        let mut removed_left = self.left.take().unwrap();
        self.left = removed_left.take_subtree();
        Some(removed_left.value)
    }

    fn remove_right(&mut self) -> Option<V> {
        let mut removed_right = self.right.take().unwrap();
        self.right = removed_right.take_subtree();
        Some(removed_right.value)
    }

    fn take_subtree(&mut self) -> Option<Box<BinaryTreeNode<K,V>>> {
        match (self.left.as_ref(), self.right.as_ref()) {
            (None, None) => None,
            (Some(_), None) => self.left.take(),
            (None, Some(_)) => self.right.take(),
            (Some(_), Some(_)) => Some(self.take_right_min_subtree())
        }
    }

    fn take_right_min_subtree(&mut self) -> Box<BinaryTreeNode<K,V>> {
        let (min_key, min_value) = self.right.as_ref().unwrap().min_key_value();
        let copied_key = min_key.clone();
        let mut new_node = BinaryTreeNode{
            key: min_key,
            value: min_value,
            left: self.left.take(),
            right: self.right.take()
        };
        new_node.remove_descendent(&copied_key);
        Box::new(new_node)
    }

    fn min_key_value(&self) -> (K, V) {
        match self.left {
            None => (self.key.clone(), self.value.clone()),
            Some(ref left) => left.min_key_value(),
        }
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        if key == &self.key {
            Some(&self.value)
        } else if key < &self.key {
            match self.left {
                None => None,
                Some(ref left) => left.find(key),
            }
        } else {
            match self.right {
                None => None,
                Some(ref right) => right.find(key),
            }
        }
    }

    pub fn append(&mut self, node: BinaryTreeNode<K,V>) {
        if node.key < self.key {
            self.insert_left(node);
        } else if node.key == self.key {
            self.value = node.value;
        } else{
            self.insert_right(node);
        }
    }

    fn insert_left(&mut self, node: BinaryTreeNode<K,V>) {
        match self.left {
            None => self.left = Some(Box::new(node)),
            Some(ref mut left) => left.append(node)
        }
    }

    fn insert_right(&mut self, node: BinaryTreeNode<K,V>) {
        match self.right {
            None => self.right = Some(Box::new(node)),
            Some(ref mut right) => right.append(node)
        }
    }
}

pub struct BinaryTree<K,V> {
    root: Option<BinaryTreeNode<K,V>>,
}


impl <K: Ord + Clone, V: Clone> BinaryTree<K, V> {
    pub fn new() -> BinaryTree<K,V> {
        BinaryTree{
            root: None,
        }
    }
}

impl<K: Ord + Clone,V: Clone> Tree<K,V> for BinaryTree<K,V> {
    fn get(&self, key: &K) -> Option<&V> {
        match self.root {
            None => None,
            Some(ref root) => root.find(key),
        }
    }

    fn put(&mut self, key: K, value: V) {
        match self.root {
            None => self.root = Some(BinaryTreeNode::new(key, value)),
            Some(ref mut node) => node.append(BinaryTreeNode::new(key, value))
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if self.root.is_none() {
            return None;
        }

        if self.root.as_ref().unwrap().key != *key {
            return self.root.as_mut().unwrap().remove_descendent(key);
        }

        let removed_value = self.root.as_ref().unwrap().value.clone();
        self.root = self.root.as_mut().unwrap().take_subtree().map(|r| *r);

        Some(removed_value)
    }

    fn is_empty(&self) -> bool {
        match self.root {
            None => true,
            Some(_) => false,
        }
    }

    fn clear(&mut self){
        self.root = None
    }
}

#[cfg(test)]
mod tests {
    use super::{Tree, BinaryTree};

    #[test]
    fn basic_put_get() {
        let mut bin_tree: BinaryTree<String,String> = BinaryTree::new();
        bin_tree.put("2".to_string(), "goodbye".to_string());
        bin_tree.put("1".to_string(), "hello".to_string());
        bin_tree.put("3".to_string(), "cherry".to_string());

        assert_eq!(&"hello".to_string(), bin_tree.get(&"1".to_string()).unwrap());
        assert_eq!(&"goodbye".to_string(), bin_tree.get(&"2".to_string()).unwrap());
        assert_eq!(&"cherry".to_string(), bin_tree.get(&"3".to_string()).unwrap());
    }

    #[test]
    fn put_get_remove() {
        let mut bin_tree: BinaryTree<String,String> = BinaryTree::new();
        bin_tree.put("2".to_string(), "goodbye".to_string());
        bin_tree.put("1".to_string(), "hello".to_string());
        bin_tree.put("3".to_string(), "cherry".to_string());

        assert_eq!(&"hello".to_string(), bin_tree.get(&"1".to_string()).unwrap());

        assert_eq!(
            Some("hello".to_string()),
            bin_tree.remove(&"1".to_string()));
    }

    #[test]
    fn remove_root() {
        let mut bin_tree: BinaryTree<String,String> = BinaryTree::new();
        bin_tree.put("2".to_string(), "goodbye".to_string());
        bin_tree.put("1".to_string(), "hello".to_string());
        bin_tree.put("3".to_string(), "cherry".to_string());

        assert_eq!(
            Some("goodbye".to_string()),
            bin_tree.remove(&"2".to_string()));

        assert_eq!(
            None,
            bin_tree.get(&"2".to_string()));
        assert_eq!(&"hello".to_string(), bin_tree.get(&"1".to_string()).unwrap());
        assert_eq!(&"cherry".to_string(), bin_tree.get(&"3".to_string()).unwrap());
    }

    #[test]
    fn remove_subtree() {
        let mut bin_tree: BinaryTree<String,String> = BinaryTree::new();
        bin_tree.put("2".to_string(), "goodbye".to_string());
        bin_tree.put("1".to_string(), "hello".to_string());
        bin_tree.put("7".to_string(), "cherry".to_string());
        bin_tree.put("4".to_string(), "doot".to_string());
        bin_tree.put("9".to_string(), "uber".to_string());

        assert_eq!(
            Some("cherry".to_string()),
            bin_tree.remove(&"7".to_string()));
        assert_eq!(
            None,
            bin_tree.get(&"7".to_string()));

        assert_eq!(&"hello".to_string(), bin_tree.get(&"1".to_string()).unwrap());
        assert_eq!(&"goodbye".to_string(), bin_tree.get(&"2".to_string()).unwrap());
        assert_eq!(&"doot".to_string(), bin_tree.get(&"4".to_string()).unwrap());
        assert_eq!(&"uber".to_string(), bin_tree.get(&"9".to_string()).unwrap());
    }
}
