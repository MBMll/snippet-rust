mod collection;

#[derive(Debug)]
pub struct A {
    id: String,
    parent_id: Option<String>,
    children: Option<Vec<A>>,
}

impl collection::Tree<String> for A {
    fn get_parent(&self) -> String {
        match &self.parent_id {
            None => {
                String::from("")
            }
            Some(x) => {
                *x
            }
        }
    }

    fn get_child(&self) -> String {
        self.id.clone()
    }

    fn get_children(&mut self) -> Vec<Self> {
        match &self.children {
            None => {
                self.children = Option::from(Vec::new());
                self.children.expect("")
            }
            Some(children) => {
                children
            }
        }
    }
}

// #[derive(Debug)]
fn main() {
    println!("Hello, world!");
    let mut list = Vec::new();
    let parent_id = Option::from(String::from("s"));
    list.push(A { id: String::from("sd"), parent_id: parent_id.clone(), children: None });
    list.push(A { id: String::from("se"), parent_id: parent_id.clone(), children: None });

    let v = collection::toTree(list);
    for x in v {
        println!("{:?}", x);
    }
    println!("{v:?}")
}
