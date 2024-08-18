use super::super::*;

pub struct Sequence<M, C> {
    name: String,
    nodes: Vec<Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>>,
    index: Option<usize>,
}

impl<M, C> Sequence<M, C> {
    pub fn new(nodes: Vec<Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>>) -> Self {
        Sequence {
            name: get_bt_id(),
            nodes,
            index: None,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for Sequence<M, C> {
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
        let mut running_index = self.index.unwrap_or(0);
        loop {
            if let Some(node) = self.nodes.get_mut(running_index) {
                let result = node.resume_with(model, controller, gas, audit);
                match result {
                    BehaviorTreeState::Complete => {
                        // Move on to the next node.
                        running_index += 1;
                    }
                    BehaviorTreeState::Failed => {
                        self.index = None;
                        audit.exit(self.get_name(), result);
                        return result;
                    }
                    _ => {
                        // Waiting, NeedsGas
                        self.index = Some(running_index);
                        audit.exit(self.get_name(), result);
                        return result;
                    }
                }
            } else {
                self.index = None;
                return BehaviorTreeState::Complete;
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.index = None;
    }

    fn get_name(self: &Self) -> &String {
        &self.name
    }
}
