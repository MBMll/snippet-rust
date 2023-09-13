use std::borrow::Borrow;

use crate::collection::Tree;

mod collection;

#[derive(Debug)]
pub struct A {
    id: String,
    parent_id: Option<String>,
    children: Option<Vec<A>>,
}

impl Clone for A {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl collection::Tree<String> for A {
    type Node = A;
    fn get_parent(&self) -> String {
        match &self.parent_id {
            None => {
                String::from("")
            }
            Some(x) => {
                x.clone()
            }
        }
    }

    fn get_child(&self) -> String {
        self.id.clone()
    }

    fn get_node(&self) -> &Self::Node {
        self
    }

    fn put_all(&mut self, children: Vec<Self::Node>) {
        // let mut tmp: Vec<A> = Vec::new();
        // for x in children {
        //     tmp.push(x.b)
        // }
        self.children = Option::from(children);
    }

    // fn get_children(&mut self) -> Vec<Self> {
    //     match &self.children {
    //         None => {
    //             self.children = Option::from(Vec::new());
    //             self.children.expect("")
    //         }
    //         Some(children) => {
    //             children
    //         }
    //     }
    // }
}

// #[derive(Debug)]
fn main() {
    println!("Hello, world!");
    let mut list = Vec::new();
    let parent_id = Option::from(String::from("s"));
    list.push(A { id: String::from("sd"), parent_id: parent_id.clone(), children: None });
    list.push(A { id: String::from("se"), parent_id: parent_id.clone(), children: None });

    let v: Vec<A> = collection::toTree(list);
    for x in v {
        println!("{:?}", x);
    }
    println!("{v:?}")
}
