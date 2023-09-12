use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Map;

pub trait Tree<K: Eq + Hash> {
    fn get_parent(&self) -> K;
    fn get_child(&self) -> K;
    fn get_children(&mut self) -> Vec<Self>;
}

pub fn toTree<T: Tree<K>, K: Eq + Hash>(mut list: Vec<T>) -> Vec<T> {
    if (list.is_empty()) {
        list
    } else {
        let mut groups: HashMap<K, Vec<T>> = HashMap::new();
        for x in list {
            let parent = x.get_parent();
            if !groups.contains_key(&parent) {
                groups.insert(parent, Vec::new());
            }
            match groups.get(&parent) {
                None => {}
                Some(g) => {
                    (&g).push(x);
                }
            }
        }
        let map: HashMap<K, &T> = HashMap::new();
        for x in list {
            (&map).insert(x.get_child(), &x);
        }

        for x in groups {
            let option = map.get(&x.0);
            match option {
                None => {
                    list = x.1;
                }
                Some(mut e) => {
                    let mut vec = e.get_children();
                    for x in x.1 {
                        vec.push(x);
                    }
                }
            }
        }
        list
    }
}
