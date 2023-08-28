use std::any::Any;
use std::collections::HashMap;
use std::iter::Map;
use std::marker::Tuple;

pub trait Tree<T, K> {
    fn get_parent(e: &T) -> Option<K>;
    fn get_child(e: &T) -> K;
    fn put_all(e: &T, list: Vec<T>);

    fn from<>(list: Vec<T>) -> Vec<T> {
        if (list.is_empty()) {
            list
        } else {
            let mut groups:HashMap<Option<K>,Vec<T>> = HashMap::new();
            for x in list {
                let parent: Option<K> = self::get_parent(&x);
                if !groups.contains_key(&parent) {
                    groups.insert(parent, Vec::new());
                }
                groups.get(&parent).insert(x);
            }
            let map: HashMap<K, &T> = HashMap::new();
            for x in list {
                (&map).insert(get_child(x), &x);
            }
            for x in groups {
                let option = map.get(&x.0);
                match option {
                    None => {}
                    Some(e) => {
                        put_all(e.,)
                    }
                }
            }
            list
        }
    }
}

struct A {
    id: String,
    parent_id: Option<String>,
}

impl ATree for dyn Tree<A, String> {
    fn get_parent(e: &A) -> Option<String> {
        e.parent_id.clone()
    }
    fn get_child(e: &A) -> String {
        e.id.clone()
    }
}