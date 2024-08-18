use super::*;

pub struct Failer<M, C> {
    name: String,
    node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> Failer<M, C> {
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
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(self.get_name());
        match self.node.resume_with(model, controller, gas, audit) {
            BehaviorTreeState::Failed | BehaviorTreeState::Complete => {
                audit.exit(self.get_name(), BehaviorTreeState::Failed);
                return BehaviorTreeState::Failed;
            }
            result => {
                audit.exit(self.get_name(), result);
                // Waiting, NeedsGas
                return result;
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.node.reset(model);
    }

    fn get_name(self: &Self) -> &String {
        &self.name
    }
}
