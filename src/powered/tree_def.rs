#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{nodes::*, BehaviorTree};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(bevy::asset::Asset))]
#[derive(Clone)]
pub enum BehaviorTreeDef<
    U: UserNodeDefinition + Send + Sync + 'static,
    W: UserWrapperDefinition<U> + Send + Sync + 'static,
> {
    Sequence(Vec<BehaviorTreeDef<U, W>>),
    Selector(Vec<BehaviorTreeDef<U, W>>),
    Repeat(Box<BehaviorTreeDef<U, W>>, usize),
    RepeatUntilSuccess(Box<BehaviorTreeDef<U, W>>),
    RepeatUntilFail(Box<BehaviorTreeDef<U, W>>),
    Succeeder(Box<BehaviorTreeDef<U, W>>),
    Failer(Box<BehaviorTreeDef<U, W>>),
    Inverter(Box<BehaviorTreeDef<U, W>>),
    User(U),
    Wrapper(W, Vec<BehaviorTreeDef<U, W>>),
}

impl<
        U: UserNodeDefinition + Send + Sync + 'static,
        W: UserWrapperDefinition<U> + Send + Sync + 'static,
    > Default for BehaviorTreeDef<U, W>
{
    fn default() -> Self {
        BehaviorTreeDef::Sequence(vec![])
    }
}

impl<
        U: UserNodeDefinition + Send + Sync + std::fmt::Debug + 'static,
        W: UserWrapperDefinition<U> + Send + Sync + std::fmt::Debug + 'static,
    > std::fmt::Debug for BehaviorTreeDef<U, W>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BehaviorTreeDef::Sequence(node_defs) => {
                f.debug_tuple("Sequence").field(node_defs).finish()
            }
            BehaviorTreeDef::Selector(node_defs) => {
                f.debug_tuple("Selector").field(node_defs).finish()
            }
            BehaviorTreeDef::Repeat(node_def, repeats) => f
                .debug_tuple("Repeat")
                .field(node_def)
                .field(repeats)
                .finish(),
            BehaviorTreeDef::RepeatUntilSuccess(node_def) => {
                f.debug_tuple("RepeatUntilSuccess").field(node_def).finish()
            }
            BehaviorTreeDef::RepeatUntilFail(node_def) => {
                f.debug_tuple("RepeatUntilFail").field(node_def).finish()
            }
            BehaviorTreeDef::Succeeder(node_def) => {
                f.debug_tuple("Succeeder").field(node_def).finish()
            }
            BehaviorTreeDef::Failer(node_def) => f.debug_tuple("Failer").field(node_def).finish(),
            BehaviorTreeDef::Inverter(node_def) => {
                f.debug_tuple("Inverter").field(node_def).finish()
            }
            BehaviorTreeDef::User(node_def) => f.debug_tuple("User").field(node_def).finish(),
            BehaviorTreeDef::Wrapper(wrapper_def, node_defs) => f
                .debug_tuple("Wrapper")
                .field(wrapper_def)
                .field(node_defs)
                .finish(),
        }
    }
}

impl<
        U: UserNodeDefinition + Send + Sync + PartialEq + 'static,
        W: UserWrapperDefinition<U> + Send + Sync + PartialEq + 'static,
    > PartialEq for BehaviorTreeDef<U, W>
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                BehaviorTreeDef::Sequence(self_node_defs),
                BehaviorTreeDef::Sequence(other_node_defs),
            ) => self_node_defs == other_node_defs,
            (
                BehaviorTreeDef::Selector(self_node_defs),
                BehaviorTreeDef::Selector(other_node_defs),
            ) => self_node_defs == other_node_defs,
            (
                BehaviorTreeDef::Repeat(self_node_def, self_repeats),
                BehaviorTreeDef::Repeat(other_node_def, other_repeats),
            ) => self_node_def == other_node_def && self_repeats == other_repeats,
            (
                BehaviorTreeDef::RepeatUntilSuccess(self_node_def),
                BehaviorTreeDef::RepeatUntilSuccess(other_node_def),
            ) => self_node_def == other_node_def,
            (
                BehaviorTreeDef::RepeatUntilFail(self_node_def),
                BehaviorTreeDef::RepeatUntilFail(other_node_def),
            ) => self_node_def == other_node_def,
            (
                BehaviorTreeDef::Succeeder(self_node_def),
                BehaviorTreeDef::Succeeder(other_node_def),
            ) => self_node_def == other_node_def,
            (BehaviorTreeDef::Failer(self_node_def), BehaviorTreeDef::Failer(other_node_def)) => {
                self_node_def == other_node_def
            }
            (
                BehaviorTreeDef::Inverter(self_node_def),
                BehaviorTreeDef::Inverter(other_node_def),
            ) => self_node_def == other_node_def,
            (BehaviorTreeDef::User(self_node_def), BehaviorTreeDef::User(other_node_def)) => {
                self_node_def == other_node_def
            }
            (
                BehaviorTreeDef::Wrapper(self_wrapper_def, self_node_defs),
                BehaviorTreeDef::Wrapper(other_wrapper_def, other_node_defs),
            ) => self_wrapper_def == other_wrapper_def && self_node_defs == other_node_defs,
            _ => false,
        }
    }
}

pub trait UserNodeDefinition {
    type Model: 'static;
    type Controller: 'static;
    fn create_node(
        &self,
    ) -> Box<dyn BehaviorTree<Model = Self::Model, Controller = Self::Controller> + Send + Sync>;
}

impl<M: 'static, C: 'static, D: 'static> UserNodeDefinition for D
where
    D: BehaviorTree<Model = M, Controller = C> + Clone + Send + Sync,
{
    type Model = M;
    type Controller = C;

    fn create_node(
        &self,
    ) -> Box<dyn BehaviorTree<Model = Self::Model, Controller = Self::Controller> + Send + Sync>
    {
        Box::new(self.clone())
    }
}

pub trait UserWrapperDefinition<U: UserNodeDefinition> {
    fn create_node_and_wrap(
        &self,
        nodes: Vec<
            Box<dyn BehaviorTree<Model = U::Model, Controller = U::Controller> + Send + Sync>,
        >,
    ) -> Box<dyn BehaviorTree<Model = U::Model, Controller = U::Controller> + Send + Sync>;
}

impl<U: UserNodeDefinition> UserWrapperDefinition<U> for () {
    fn create_node_and_wrap(
        &self,
        _nodes: Vec<
            Box<dyn BehaviorTree<Model = U::Model, Controller = U::Controller> + Send + Sync>,
        >,
    ) -> Box<dyn BehaviorTree<Model = U::Model, Controller = U::Controller> + Send + Sync> {
        panic!("Cannot create a wrapper with no definition");
    }
}

#[cfg(feature = "bevy")]
impl<
        U: UserNodeDefinition + Send + Sync + 'static,
        W: UserWrapperDefinition<U> + Send + Sync + 'static,
    > bevy::reflect::TypePath for BehaviorTreeDef<U, W>
{
    fn type_path() -> &'static str {
        "behavior_bark::powered::tree_def::BehaviorTreeDef"
    }

    fn short_type_path() -> &'static str {
        "BehaviorTreeDef"
    }
}

impl<
        U: UserNodeDefinition + Send + Sync + 'static,
        W: UserWrapperDefinition<U> + Send + Sync + 'static,
    > BehaviorTreeDef<U, W>
{
    pub fn create_tree(
        &self,
    ) -> Box<dyn BehaviorTree<Model = U::Model, Controller = U::Controller> + Send + Sync> {
        match self {
            BehaviorTreeDef::Sequence(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Sequence::new(nodes))
            }
            BehaviorTreeDef::Selector(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Selector::new(nodes))
            }
            BehaviorTreeDef::Repeat(node_def, repeats) => {
                let node = node_def.create_tree();
                Box::new(Repeat::new(node, *repeats))
            }
            BehaviorTreeDef::RepeatUntilFail(node_def) => {
                let node = node_def.create_tree();
                Box::new(RepeatUntilFail::new(node))
            }
            BehaviorTreeDef::RepeatUntilSuccess(node_def) => {
                let node = node_def.create_tree();
                Box::new(RepeatUntilSuccess::new(node))
            }
            BehaviorTreeDef::Succeeder(node_def) => {
                let node = node_def.create_tree();
                Box::new(Succeeder::new(node))
            }
            BehaviorTreeDef::Inverter(node_def) => {
                let node = node_def.create_tree();
                Box::new(Inverter::new(node))
            }
            BehaviorTreeDef::Failer(node_def) => {
                let node = node_def.create_tree();
                Box::new(Failer::new(node))
            }
            BehaviorTreeDef::User(node_def) => node_def.create_node(),
            BehaviorTreeDef::Wrapper(wrapper_def, node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                wrapper_def.create_node_and_wrap(nodes)
            }
        }
    }
}
