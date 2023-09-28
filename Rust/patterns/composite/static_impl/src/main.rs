use rand::random;
use static_impl::{Component, Components, Composite};

fn client_func(component: &Components) {
    println!("Result: {}", component.operation().unwrap());
}

fn main() {
    use Components::{Complex, Edge, Leaf};

    let leaf = Leaf(random::<u32>());
    println!("Client: I've got a simple component:");
    client_func(&leaf);

    let mut tree = Composite::default();
    let mut branch1 = Composite::default();

    let leaf1 = Leaf(random::<u32>());
    let leaf2 = Leaf(random::<u32>());
    let leaf3 = Leaf(random::<u32>());
    let edge1 = Edge(random::<u32>());

    branch1.add(&leaf1);
    branch1.add(&leaf2);

    let mut branch2 = Composite::default();
    branch2.add(&leaf3);
    branch2.add(&edge1);

    tree.add(&Complex(branch1));
    tree.add(&Complex(branch2));

    let mut root = Composite::default();
    root.add(&Complex(tree));

    println!("Client: Now I've got a composite tree:");
    client_func(&Complex(root));
}
