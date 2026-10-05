use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StopNode {
    pub parent: Option<String>,
    pub children: BTreeSet<String>,
    pub stop_durable: bool,
    pub signaled: bool,
    pub settled: bool,
}

#[derive(Clone, Debug, Default)]
pub struct StopTree {
    nodes: BTreeMap<String, StopNode>,
    root: Option<String>,
}

impl StopTree {
    pub fn add_root(&mut self, root: impl Into<String>) {
        let root = root.into();
        self.nodes.entry(root.clone()).or_insert(StopNode {
            parent: None,
            children: BTreeSet::new(),
            stop_durable: false,
            signaled: false,
            settled: false,
        });
        self.root = Some(root);
    }

    pub fn add_child(&mut self, parent: &str, child: impl Into<String>) {
        let child = child.into();
        self.nodes
            .get_mut(parent)
            .expect("parent must be registered first")
            .children
            .insert(child.clone());
        self.nodes.entry(child).or_insert(StopNode {
            parent: Some(parent.to_owned()),
            children: BTreeSet::new(),
            stop_durable: false,
            signaled: false,
            settled: false,
        });
    }

    pub fn mark_stop_durable(&mut self, node: &str) -> bool {
        let Some(current) = self.nodes.get(node) else {
            return false;
        };
        let parent_ready = current.parent.as_ref().is_none_or(|parent| {
            self.nodes
                .get(parent)
                .is_some_and(|value| value.stop_durable)
        });
        if !parent_ready {
            return false;
        }
        self.nodes
            .get_mut(node)
            .expect("checked above")
            .stop_durable = true;
        true
    }

    pub fn mark_signaled(&mut self, node: &str) -> bool {
        let Some(current) = self.nodes.get_mut(node) else {
            return false;
        };
        if !current.stop_durable {
            return false;
        }
        current.signaled = true;
        true
    }

    pub fn mark_settled(&mut self, node: &str) -> bool {
        let children_settled = self
            .nodes
            .get(node)
            .is_some_and(|value| value.children.iter().all(|child| self.is_settled(child)));
        if !children_settled {
            return false;
        }
        let Some(current) = self.nodes.get_mut(node) else {
            return false;
        };
        if !current.stop_durable {
            return false;
        }
        current.settled = true;
        true
    }

    #[must_use]
    pub fn node(&self, node: &str) -> Option<&StopNode> {
        self.nodes.get(node)
    }

    #[must_use]
    pub fn root_settled(&self) -> bool {
        self.root.as_ref().is_some_and(|root| self.is_settled(root))
    }

    fn is_settled(&self, node: &str) -> bool {
        self.nodes.get(node).is_some_and(|value| value.settled)
    }
}
