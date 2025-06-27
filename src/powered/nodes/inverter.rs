use super::super::*;

/// An Inverter node in a behavior tree that inverts the result of its
/// child node, turning success into failure, and failure into success.
pub struct Inverter<M, C> {
    name: String,
    node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> Inverter<M, C> {
    /// Creates a new Inverter node.
    pub fn new(node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>) -> Self {
        Inverter {
            name: get_bt_id(),
            node,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for Inverter<M, C> {
    type Model = M;
    type Controller = C;
    
    /// Resumes execution with the given model and controller, inverting
    /// the result state of the node it wraps.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(&self.name);
        match self.node.resume_with(model, controller, gas, audit) {
            BehaviorTreeState::Complete => {
                audit.exit(&self.name, BehaviorTreeState::Failed);
                return BehaviorTreeState::Failed;
            }
            BehaviorTreeState::Failed => {
                audit.exit(&self.name, BehaviorTreeState::Complete);
                return BehaviorTreeState::Complete;
            }
            result => {
                audit.exit(&self.name, result);
                // Waiting, NeedsGas
                return result;
            }
        }
    }

    /// Resets the state for a new execution cycle.
    fn reset(self: &mut Self, model: &Self::Model) {
        self.node.reset(model);
    }
}
