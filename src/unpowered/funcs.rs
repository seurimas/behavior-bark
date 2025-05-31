/// Enumeration of possible states in unpowered functions.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnpoweredFunctionState {
    /// Function is waiting for completion.
    Waiting,
    /// The function failed.
    Failed,
    /// The function completed successfully.
    Complete,
}

/// Trait defining operations for unpowered functions.
pub trait UnpoweredFunction {
    type Model: 'static;
    type Controller: 'static;

    /// Resumes execution with the specified model and controller.
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState;

    /// Resets the function to its initial state.
    fn reset(self: &mut Self, model: &Self::Model);
}
