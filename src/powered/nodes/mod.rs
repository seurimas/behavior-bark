/// Module for behavior tree nodes.
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

use lazy_static::lazy_static;

use super::BehaviorTreeAudit;

/// Enum representing different possible states of a behavior tree node.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BehaviorTreeState {
    /// Indicates the tree is waiting for completion.
    Waiting,
    /// Indicates the powered function could not continue due to lack of resources.
    WaitingForGas,
    /// Indicates the function failed to complete all work.
    Failed,
    /// Indicates the function completed all work successfully.
    Complete,
}

lazy_static! {
    /// Provides a unique identifier for nodes.
    pub static ref DEFAULT_IDS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
}

/// Generates a unique behavior tree node identifier.
pub fn get_bt_id() -> String {
    format!("<node {}>", DEFAULT_IDS.fetch_add(1, Ordering::SeqCst))
}

/// Trait defining the behavior tree operations.
pub trait BehaviorTree {
    type Model: 'static;
    type Controller: 'static;

    /// Resumes the behavior tree execution.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState;

    /// Resets the behavior tree to its initial state.
    fn reset(self: &mut Self, model: &Self::Model);
}
