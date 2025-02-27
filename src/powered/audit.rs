use super::BehaviorTreeState;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub enum BehaviorTreeMarker {
    Enter(String),
    Marker(String),
    Exit(String, BehaviorTreeState),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct BehaviorTreeAudit {
    events: Vec<BehaviorTreeMarker>,
    place: Vec<String>,
}

pub trait BehaviorTreeAuditTrait {
    fn enter<N: ToString>(&mut self, node_name: &N);

    fn mark<N: ToString>(&mut self, node_name: &N);

    fn exit<N: ToString>(&mut self, node_name: &N, state: BehaviorTreeState);
}

impl BehaviorTreeAuditTrait for BehaviorTreeAudit {
    fn enter<N: ToString>(&mut self, node_name: &N) {
        self.events
            .push(BehaviorTreeMarker::Enter(node_name.to_string()));
        self.place.push(node_name.to_string());
    }

    fn mark<N: ToString>(&mut self, node_name: &N) {
        self.events
            .push(BehaviorTreeMarker::Marker(node_name.to_string()));
    }

    fn exit<N: ToString>(&mut self, node_name: &N, state: BehaviorTreeState) {
        self.events
            .push(BehaviorTreeMarker::Exit(node_name.to_string(), state));
        if let Some(current_node_name) = self.place.last() {
            if current_node_name.eq(&node_name.to_string()) {
                self.place.pop();
            }
        }
    }
}

impl<T> BehaviorTreeAuditTrait for &mut Option<T>
where
    T: BehaviorTreeAuditTrait,
{
    fn enter<N: ToString>(&mut self, node_name: &N) {
        if let Some(audit) = self {
            audit.enter(node_name);
        }
    }

    fn mark<N: ToString>(&mut self, node_name: &N) {
        if let Some(audit) = self {
            audit.mark(node_name);
        }
    }

    fn exit<N: ToString>(&mut self, node_name: &N, state: BehaviorTreeState) {
        if let Some(audit) = self {
            audit.exit(node_name, state);
        }
    }
}
