mod audit;
mod nodes;
mod tree_def;
pub use audit::*;
pub use nodes::*;
pub use tree_def::*;

#[macro_export]
macro_rules! check_gas {
    ($gas:expr) => {
        if let Some(gas) = $gas {
            if *gas <= 0 {
                return BehaviorTreeState::WaitingForGas;
            }
        }
    };
}
