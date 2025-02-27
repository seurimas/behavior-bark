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

lazy_static! {
    pub static ref DEFAULT_IDS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
}

pub fn get_bt_id() -> String {
    format!("<node {}>", DEFAULT_IDS.fetch_add(1, Ordering::SeqCst))
}

pub trait BehaviorTree {
    type Model: 'static;
    type Controller: 'static;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState;

    fn reset(self: &mut Self, model: &Self::Model);
}
