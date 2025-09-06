use serde::{Deserialize, Serialize};

use super::{nodes::*, UnpoweredFunction};

/// Enum defining different unpowered tree configurations,
/// supporting various node types.
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy::asset::Asset))]
pub enum UnpoweredTreeDef<
    U: UserNodeDefinition + Sync + Send + 'static,
    W: UserWrapperDefinition<U> + Sync + Send + 'static,
> {
    /// A sequence node that executes children in order until one fails.
    Sequence(Vec<UnpoweredTreeDef<U, W>>),
    /// A selector node that executes children until one succeeds.
    Selector(Vec<UnpoweredTreeDef<U, W>>),
    /// An executor node.
    Executor(Vec<UnpoweredTreeDef<U, W>>),
    /// A repeat node that runs a specified number of times.
    Repeat(Box<UnpoweredTreeDef<U, W>>, usize),
    /// A node that repeats until success is achieved.
    RepeatUntilSuccess(Box<UnpoweredTreeDef<U, W>>),
    /// A node that repeats until failure occurs.
    RepeatUntilFail(Box<UnpoweredTreeDef<U, W>>),
    /// A node that always succeeds.
    Succeeder(Box<UnpoweredTreeDef<U, W>>),
    /// A node that always fails.
    Failer(Box<UnpoweredTreeDef<U, W>>),
    /// An inverter node that inverts the result.
    Inverter(Box<UnpoweredTreeDef<U, W>>),
    /// User-defined node.
    #[serde(untagged)]
    User(U),
    /// Wrapper node allowing additional structure.
    #[serde(untagged)]
    Wrapper(W, Vec<UnpoweredTreeDef<U, W>>),
}

/// Trait for defining user-specific nodes in an unpowered tree.
pub trait UserNodeDefinition {
    type Model: 'static;
    type Controller: 'static;

    /// Creates a node in the unpowered tree context.
    fn create_node(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = Self::Model, Controller = Self::Controller> + Send + Sync>;
}

impl<M: 'static, C: 'static, D: 'static> UserNodeDefinition for D
where
    D: UnpoweredFunction<Model = M, Controller = C> + Clone + Send + Sync,
{
    type Model = M;
    type Controller = C;

    fn create_node(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = Self::Model, Controller = Self::Controller> + Send + Sync>
    {
        Box::new(self.clone())
    }
}

/// Trait for defining wrapper nodes.
pub trait UserWrapperDefinition<U: UserNodeDefinition> {
    /// Wraps a collection of nodes.
    fn create_node_and_wrap(
        &self,
        nodes: Vec<
            Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>,
        >,
    ) -> Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>;
}

#[cfg(feature = "bevy")]
impl<
        U: UserNodeDefinition + Send + Sync + 'static,
        W: UserWrapperDefinition<U> + Send + Sync + 'static,
    > bevy::reflect::TypePath for UnpoweredTreeDef<U, W>
{
    fn type_path() -> &'static str {
        "behavior_bark::unpowered::tree_def::UnpoweredTreeDef"
    }

    fn short_type_path() -> &'static str {
        "UnpoweredTreeDef"
    }
}

impl<U: UserNodeDefinition> UserWrapperDefinition<U> for () {
    fn create_node_and_wrap(
        &self,
        _nodes: Vec<
            Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>,
        >,
    ) -> Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>
    {
        panic!("Cannot create a wrapper with no definition");
    }
}

impl<U: UserNodeDefinition + Send + Sync, W: UserWrapperDefinition<U> + Send + Sync>
    UnpoweredTreeDef<U, W>
{
    /// Creates the tree structure from definitions.
    pub fn create_tree(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>
    {
        match self {
            UnpoweredTreeDef::Sequence(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Sequence::new(nodes))
            }
            UnpoweredTreeDef::Selector(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Selector::new(nodes))
            }
            UnpoweredTreeDef::Executor(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Executor::new(nodes))
            }
            UnpoweredTreeDef::Repeat(node_def, repeats) => {
                let node = node_def.create_tree();
                Box::new(Repeat::new(node, *repeats))
            }
            UnpoweredTreeDef::RepeatUntilFail(node_def) => {
                let node = node_def.create_tree();
                Box::new(RepeatUntilFail::new(node))
            }
            UnpoweredTreeDef::RepeatUntilSuccess(node_def) => {
                let node = node_def.create_tree();
                Box::new(RepeatUntilSuccess::new(node))
            }
            UnpoweredTreeDef::Succeeder(node_def) => {
                let node = node_def.create_tree();
                Box::new(Succeeder::new(node))
            }
            UnpoweredTreeDef::Inverter(node_def) => {
                let node = node_def.create_tree();
                Box::new(Inverter::new(node))
            }
            UnpoweredTreeDef::Failer(node_def) => {
                let node = node_def.create_tree();
                Box::new(Failer::new(node))
            }
            UnpoweredTreeDef::User(node_def) => node_def.create_node(),
            UnpoweredTreeDef::Wrapper(wrapper_def, node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                wrapper_def.create_node_and_wrap(nodes)
            }
        }
    }
}
