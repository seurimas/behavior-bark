mod failer;
mod inverter;
mod repeat;
mod selector;
mod sequence;
mod succeeder;

pub use failer::*;
pub use inverter::*;
pub use repeat::*;
pub use selector::*;
pub use sequence::*;
pub use succeeder::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

#[cfg(feature = "tracing")]
use lazy_static::lazy_static;

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
    fn enter(&mut self, node_name: &String);

    fn mark(&mut self, node_name: &String);

    fn exit(&mut self, node_name: &String, state: BehaviorTreeState);
}

impl BehaviorTreeAuditTrait for BehaviorTreeAudit {
    fn enter(&mut self, node_name: &String) {
        self.events
            .push(BehaviorTreeMarker::Enter(node_name.clone()));
        self.place.push(node_name.clone());
    }

    fn mark(&mut self, node_name: &String) {
        self.events
            .push(BehaviorTreeMarker::Marker(node_name.clone()));
    }

    fn exit(&mut self, node_name: &String, state: BehaviorTreeState) {
        self.events
            .push(BehaviorTreeMarker::Exit(node_name.clone(), state));
        if let Some(current_node_name) = self.place.last() {
            if current_node_name.eq(node_name) {
                self.place.pop();
            }
        }
    }
}

impl<T> BehaviorTreeAuditTrait for &mut Option<T>
where
    T: BehaviorTreeAuditTrait,
{
    fn enter(&mut self, node_name: &String) {
        if let Some(audit) = self {
            audit.enter(node_name);
        }
    }

    fn mark(&mut self, node_name: &String) {
        if let Some(audit) = self {
            audit.mark(node_name);
        }
    }

    fn exit(&mut self, node_name: &String, state: BehaviorTreeState) {
        if let Some(audit) = self {
            audit.exit(node_name, state);
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BehaviorTreeState {
    Waiting,
    // The powered function could not continue, due to lack of gas.
    WaitingForGas,
    // The powered function failed to complete all work (bad state or negative result).
    Failed,
    // The powered function completed all work.
    Complete,
}

#[cfg(feature = "tracing")]
lazy_static! {
    pub static ref DEFAULT_IDS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
}

#[cfg(feature = "tracing")]
pub fn get_bt_id() -> String {
    format!("<node {}>", DEFAULT_IDS.fetch_add(1, Ordering::SeqCst))
}

pub trait BehaviorTree {
    type Model: 'static;
    type Controller: 'static;
    fn get_name(self: &Self) -> &String;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState;

    fn reset(self: &mut Self, model: &Self::Model);
}
