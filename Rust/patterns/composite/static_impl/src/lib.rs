// Composite pattern is a partitioning design pattern and describes a group of objects that is treated
// the same way as a single instance of the same type of object. The intent of a composite is to
// “compose” objects into tree structures to represent part-whole hierarchies. It allows you to have a
// tree structure and ask each node in the tree structure to perform a task.

pub trait Component {
    /// The method adds a new component to the structure tree.
    fn add(&mut self, _: &Components) {}
    /// The method removes a component from the component tree.
    fn remove(&mut self, _: &Components) {}
    /// Whether the component is of complex or simple type.
    fn is_composite(&self) -> bool {
        false
    }
    /// An operation that can be performed by the corresponding component in the tree.
    fn operation(&self) -> Option<String>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Components {
    Leaf(u32),
    Edge(u32),
    Complex(Composite),
}

impl Component for Components {
    fn is_composite(&self) -> bool {
        matches!(self, Components::Complex(_))
    }
    fn operation(&self) -> Option<String> {
        match self {
            Components::Leaf(_) => Some(String::from("Leaf")),
            Components::Edge(_) => Some(String::from("Edge")),
            Components::Complex(component) => component.operation(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Composite {
    components: Vec<Components>,
}

impl Component for Composite {
    fn add(&mut self, component: &Components) {
        self.components.push(component.clone());
    }

    fn remove(&mut self, component: &Components) {
        self.components.retain(|item| item == component);
    }

    fn is_composite(&self) -> bool {
        true
    }

    fn operation(&self) -> Option<String> {
        if self.components.is_empty() {
            return None;
        }
        // The result can be unwrap since the list of components is not empty.
        // We will add a "+" sign between the components characterizing our operation.
        let last = self.components.last().unwrap();
        let result: String = self
            .components
            .iter()
            .map(|item| match item.eq(last) {
                true => item.operation().unwrap(),
                false => item.operation().unwrap() + "+",
            })
            .collect();
        Some(format!("Branch({result})"))
    }
}

#[test]
fn test_simple_operation() {
    use crate::Components::{Edge, Leaf};
    use rand::random;

    let leaf = Leaf(random::<u32>());
    let edge = Edge(random::<u32>());

    assert_eq!(leaf.operation(), Some(String::from("Leaf")));
    assert_eq!(edge.operation(), Some(String::from("Edge")));
}

#[test]
fn test_composite_operation() {
    use crate::Components::{Complex, Leaf};
    use rand::random;

    let mut composite = Composite::default();
    let leaf = Leaf(random::<u32>());
    composite.add(&leaf);
    composite.add(&Leaf(random::<u32>()));

    let mut comp = Complex(composite);

    assert!(comp.is_composite());
    assert_eq!(comp.operation(), Some(String::from("Branch(Leaf+Leaf)")));

    if let Complex(ref mut data) = comp {
        data.remove(&leaf)
    }

    assert_eq!(comp.operation(), Some(String::from("Branch(Leaf)")));
}
