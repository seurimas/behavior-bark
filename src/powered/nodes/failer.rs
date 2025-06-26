use super::super::*;

/// A Failer node in a behavior tree which always returns a failure state,
/// regardless of the wrapped node's result.
pub struct Failer<M, C> {
    name: String,
    node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> Failer<M, C> {
    /// Creates a new Failer node.
    pub fn new(node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>) -> Self {
        Failer {
            name: get_bt_id(),
            node,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for Failer<M, C> {
    type Model = M;
    type Controller = C;

    /// Resumes execution with the given model and controller, returning Failed regardless of
    /// the wrapped node's state.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(&self.name);
        match self.node.resume_with(model, controller, gas, audit) {
            BehaviorTreeState::Failed | BehaviorTreeState::Complete => {
                audit.exit(&self.name, BehaviorTreeState::Failed);
                return BehaviorTreeState::Failed;
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
