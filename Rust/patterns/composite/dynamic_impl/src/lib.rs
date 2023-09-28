// Composite pattern is a partitioning design pattern and describes a group of objects that is treated
// the same way as a single instance of the same type of object. The intent of a composite is to
// “compose” objects into tree structures to represent part-whole hierarchies. It allows you to have a
// tree structure and ask each node in the tree structure to perform a task.

use dyn_eq::DynEq;
dyn_eq::eq_trait_object!(Component);

pub trait Component: DynEq {
    /// The method adds a new component to the structure tree.
    fn add(&mut self, _: Box<dyn Component>) {}
    /// The method removes a component from the component tree.
    fn remove(&mut self, _: Box<dyn Component>) {}
    /// Whether the component is of complex or simple type.
    fn is_composite(&self) -> bool {
        false
    }
    /// An operation that can be performed by the corresponding component in the tree.
    fn operation(&self) -> Option<String>;
}

/// Composite structure integrates all other components.
#[derive(Default, PartialEq, Eq)]
pub struct Composite {
    components: Vec<Box<dyn Component>>,
}

impl Component for Composite {
    fn add(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    fn remove(&mut self, component: Box<dyn Component>) {
        self.components.retain(|item| *item == component);
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

macro_rules! new_components {
    () => {};
    ($name:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub struct $name {
            id: u32,
        }

        impl $name {
            pub fn new(id: u32) -> Self {
                Self { id }
            }
        }

        impl Component for $name {
            fn operation(&self) -> Option<String> {
                Some(String::from(stringify!($name)))
            }
        }
    };
}

new_components!(Leaf);
new_components!(Edge);

#[test]
fn test_simple_operation() {
    use rand::random;
    // We create unique identifiers for each component.
    let leaf = Leaf::new(random::<u32>());
    let edge = Edge::new(random::<u32>());

    assert_eq!(leaf.operation(), Some(String::from("Leaf")));
    assert_eq!(edge.operation(), Some(String::from("Edge")));

    let root = Composite::default();
    assert_eq!(root.operation(), None);
}

#[test]
fn test_composite_operation() {
    use rand::random;

    let mut comp = Composite::default();
    let leaf = Leaf::new(random::<u32>());

    comp.add(Box::new(leaf));
    comp.add(Box::new(Leaf::new(random::<u32>())));

    assert!(comp.is_composite());
    assert_eq!(comp.operation(), Some(String::from("Branch(Leaf+Leaf)")));

    comp.remove(Box::new(leaf));
    assert_eq!(comp.operation(), Some(String::from("Branch(Leaf)")));
}
