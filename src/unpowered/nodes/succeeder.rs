use super::super::*;
pub struct Succeeder<M, C> {
    node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> Succeeder<M, C> {
    pub fn new(node: Box<dyn UnpoweredFunction<Model = M, Controller = C> + Send + Sync>) -> Self {
        Succeeder { node }
    }
}

impl<M: 'static, C: 'static> UnpoweredFunction for Succeeder<M, C> {
    type Model = M;
    type Controller = C;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self.node.resume_with(model, controller) {
            UnpoweredFunctionState::Failed | UnpoweredFunctionState::Complete => {
                return UnpoweredFunctionState::Complete;
            }
            result => {
                // Waiting, NeedsGas
                return result;
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.node.reset(model);
    }
}
