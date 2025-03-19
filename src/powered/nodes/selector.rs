use super::super::*;

/// A Selector node in a behavior tree that attempts to run its
/// children in sequence until one of them succeeds.
pub struct Selector<M, C> {
    name: String,
    nodes: Vec<Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>>,
    index: Option<usize>,
}

impl<M, C> Selector<M, C> {
    /// Creates a new Selector node.
    pub fn new(nodes: Vec<Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>>) -> Self {
        Selector {
            name: get_bt_id(),
            nodes,
            index: None,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for Selector<M, C> {
    type Model = M;
    type Controller = C;

    /// Resumes execution by trying each child node in order until one succeeds.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(&self.name);
        let mut running_index = self.index.unwrap_or(0);
        loop {
            if let Some(node) = self.nodes.get_mut(running_index) {
                let result = node.resume_with(model, controller, gas, audit);
                match result {
                    BehaviorTreeState::Failed => {
                        // Move on to the next node.
                        running_index += 1;
                    }
                    BehaviorTreeState::Complete => {
                        self.index = None;
                        audit.exit(&self.name, result);
                        return result;
                    }
                    _ => {
                        // Waiting, NeedsGas
                        self.index = Some(running_index);
                        audit.exit(&self.name, result);
                        return result;
                    }
                }
            } else {
                self.index = None;
                audit.exit(&self.name, BehaviorTreeState::Failed);
                return BehaviorTreeState::Failed;
            }
        }
    }

    /// Resets the selector for a new execution cycle.
    fn reset(self: &mut Self, _parameter: &Self::Model) {
        self.index = None;
    }
}
