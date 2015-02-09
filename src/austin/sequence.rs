use austin::result::Result;
use austin::node::Node;
use austin::node_collection::NodeCollection;

pub struct Sequence<'a, T> {
    pub name: &'static str,
    pub children: Vec<Box<Node<T> + 'a>>
}

impl <'a, T> Sequence<'a, T> {

    pub fn new(name: &'static str) -> Sequence<'a, T> {
        Sequence {
            name: name,
            children: vec![]
        }
    }

}

impl <'a, T> Node<T> for Sequence<'a, T> {

    fn evaluate(&mut self, target: &mut T) -> Result {

        for child in self.children.iter_mut() {
            let result = child.evaluate(target);
            match result {
                Result::Success => { /* Null-Op */ },
                _ => { return result; }
            }
        }

        Result::Success
    }

}

impl <'a, T> NodeCollection<'a, T> for Sequence<'a, T> {
    fn add(&mut self, new_child: Box<Node<T> + 'a>) {
        self.children.push(new_child);
    }
}
