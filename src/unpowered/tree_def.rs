use serde::{Deserialize, Serialize};

use super::{nodes::*, UnpoweredFunction};

#[derive(Serialize, Deserialize, Clone)]
pub enum UnpoweredTreeDef<
    U: UserNodeDefinition + Sync + Send,
    W: UserWrapperDefinition<U> + Sync + Send,
> {
    Sequence(Vec<UnpoweredTreeDef<U, W>>),
    Selector(Vec<UnpoweredTreeDef<U, W>>),
    Executor(Vec<UnpoweredTreeDef<U, W>>),
    Repeat(Box<UnpoweredTreeDef<U, W>>, usize),
    RepeatUntilSuccess(Box<UnpoweredTreeDef<U, W>>),
    RepeatUntilFail(Box<UnpoweredTreeDef<U, W>>),
    Succeeder(Box<UnpoweredTreeDef<U, W>>),
    Failer(Box<UnpoweredTreeDef<U, W>>),
    Inverter(Box<UnpoweredTreeDef<U, W>>),
    User(U),
    Wrapper(W, Vec<UnpoweredTreeDef<U, W>>),
}

pub trait UserNodeDefinition {
    type Model: 'static;
    type Controller: 'static;

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

pub trait UserWrapperDefinition<U: UserNodeDefinition> {
    fn create_node_and_wrap(
        &self,
        nodes: Vec<
            Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>,
        >,
    ) -> Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>;
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
