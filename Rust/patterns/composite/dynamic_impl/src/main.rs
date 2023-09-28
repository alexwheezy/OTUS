use dynamic_impl::{Component, Composite, Edge, Leaf};
use rand::random;

// For simplicity, we'll let the client code simply output a
// list of the result of the component tree.
fn client_func(component: Box<dyn Component>) {
    println!("Result: {}", component.operation().unwrap_or_default());
}

fn main() {
    let leaf = Leaf::new(random::<u32>());
    println!("Client: I've got a simple component:");
    client_func(Box::new(leaf));

    let mut tree = Composite::default();
    let mut branch1 = Composite::default();

    let leaf1 = Leaf::new(random::<u32>());
    let leaf2 = Leaf::new(random::<u32>());
    let leaf3 = Leaf::new(random::<u32>());

    let edge1 = Edge::new(random::<u32>());

    branch1.add(Box::new(leaf1));
    branch1.add(Box::new(leaf2));

    let mut branch2 = Composite::default();
    branch2.add(Box::new(leaf3));
    branch2.add(Box::new(edge1));

    tree.add(Box::new(branch1));
    tree.add(Box::new(branch2));

    let mut root = Composite::default();
    root.add(Box::new(tree));

    println!("Client: Now I've got a composite tree:");
    client_func(Box::new(root));
}
