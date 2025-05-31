use super::super::*;

/// A Repeat node in a behavior tree that repeats its child node a
/// specified number of times, or indefinitely if set to repeat until
/// failure or success.
pub struct Repeat<M, C> {
    name: String,
    node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
    runs: usize,
    runs_left: usize,
}

impl<M, C> Repeat<M, C> {
    /// Creates a new Repeat node.
    pub fn new(
        node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
        runs: usize,
    ) -> Self {
        Repeat {
            name: get_bt_id(),
            node,
            runs,
            runs_left: runs,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for Repeat<M, C> {
    type Model = M;
    type Controller = C;

    /// Resumes execution, decrementing the number of remaining runs
    /// until completion or failure.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(&self.name);
        while self.runs_left > 0 {
            let result = self.node.resume_with(model, controller, gas, audit);
            match result {
                BehaviorTreeState::Failed => {
                    self.runs_left = self.runs;
                    audit.exit(&self.name, result);
                    return result;
                }
                BehaviorTreeState::Complete => {
                    self.runs_left -= 1;
                }
                _ => {
                    audit.exit(&self.name, result);
                    // Waiting
                    return result;
                }
            }
        }
        self.runs_left = self.runs;
        audit.exit(&self.name, BehaviorTreeState::Complete);
        return BehaviorTreeState::Complete;
    }

    /// Resets the repeat counter to its initial value for a new cycle.
    fn reset(self: &mut Self, _model: &Self::Model) {
        self.runs_left = self.runs;
    }
}

/// A RepeatUntilFail node that executes its child node until it fails.
pub struct RepeatUntilFail<M, C> {
    name: String,
    node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> RepeatUntilFail<M, C> {
    /// Creates a new RepeatUntilFail node.
    pub fn new(node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>) -> Self {
        RepeatUntilFail {
            name: get_bt_id(),
            node,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for RepeatUntilFail<M, C> {
    type Model = M;
    type Controller = C;

    /// Resumes execution until the node fails, signifying completion.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(&self.name);
        loop {
            let result = self.node.resume_with(model, controller, gas, audit);
            match result {
                BehaviorTreeState::Failed => {
                    audit.exit(&self.name, BehaviorTreeState::Complete);
                    return BehaviorTreeState::Complete;
                }
                BehaviorTreeState::Complete => {
                    // We'll be stepping the current node again.
                    continue;
                }
                _ => {
                    // Waiting, NeedsGas
                    audit.exit(&self.name, result);
                    return result;
                }
            }
        }
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        // Nothing to do.
    }
}

/// A RepeatUntilSuccess node that executes its child node until it succeeds.
pub struct RepeatUntilSuccess<M, C> {
    name: String,
    node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> RepeatUntilSuccess<M, C> {
    /// Creates a new RepeatUntilSuccess node.
    pub fn new(node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>) -> Self {
        RepeatUntilSuccess {
            name: get_bt_id(),
            node,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for RepeatUntilSuccess<M, C> {
    type Model = M;
    type Controller = C;

    /// Resumes execution until the node succeeds, marking completion.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(&self.name);
        loop {
            let result = self.node.resume_with(model, controller, gas, audit);
            match result {
                BehaviorTreeState::Complete => {
                    audit.exit(&self.name, BehaviorTreeState::Complete);
                    return BehaviorTreeState::Complete;
                }
                BehaviorTreeState::Failed => {
                    // We'll be stepping the current node again.
                    continue;
                }
                _ => {
                    // Waiting, NeedsGas
                    audit.exit(&self.name, result);
                    return result;
                }
            }
        }
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        // Nothing to do.
    }
}
