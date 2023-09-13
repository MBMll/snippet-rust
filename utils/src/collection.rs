use std::any::Any;
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;
use std::iter::Map;

pub trait Tree<K: Eq + Hash> {
    type Node;
    fn get_parent(&self) -> K;
    fn get_child(&self) -> K;
    fn get_node(&self) -> &Self::Node;
    // fn get_children(&mut self) -> Vec<&dyn Self>;
    fn put_all(&mut self, children: Vec<Self::Node>);
}

pub fn toTree<T: Tree<K> + Clone, K: Eq + Hash + Clone>(mut list: Vec<T>) -> Vec<T> {
    if (list.is_empty()) {
        list
    } else {
        let mut groups: HashMap<K, Vec<&T>> = HashMap::new();
        for x in &list {
            let parent = x.get_parent();
            // if !groups.contains_key(&parent) {
            //     groups.insert(parent.clone(), Vec::new());
            // }
            let mut v: Vec<&T> = Vec::new();
            let mut group = groups.get_mut(&parent);
            group.get_or_insert(&mut v).push(x);
            // match groups.get(&parent) {
            //     None => {}
            //     Some( g) => {
            //         g.push(x);
            //     }
            // }
        }
        let map: HashMap<K, &T> = HashMap::new();
        for x in &list {
            (&map).insert(x.get_child(), &x);
        }

        for x in groups {
            let option = map.get(&x.0);
            match option {
                None => {
                    list = x.1;
                }
                Some(mut e) => {
                    // let mut vec = e.get_children();
                    // for x in x.1 {
                    //     vec.push(x);
                    // }
                    let mut tmp = Vec::new();
                    let vec: Vec<T> = (*groups.get(&x.0).expect("")).clone();
                    for mut v in vec {
                        tmp.push(*v.get_node());
                    }
                    e.put_all(tmp);
                }
            }
        }
        list
    }
}
