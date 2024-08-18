#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{nodes::*, BehaviorTree};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub enum BehaviorTreeDef<U: UserNodeDefinition> {
    Sequence(Vec<BehaviorTreeDef<U>>),
    Selector(Vec<BehaviorTreeDef<U>>),
    Repeat(Box<BehaviorTreeDef<U>>, usize),
    RepeatUntilSuccess(Box<BehaviorTreeDef<U>>),
    RepeatUntilFail(Box<BehaviorTreeDef<U>>),
    Succeeder(Box<BehaviorTreeDef<U>>),
    Failer(Box<BehaviorTreeDef<U>>),
    Inverter(Box<BehaviorTreeDef<U>>),
    User(U),
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

impl<U: UserNodeDefinition> BehaviorTreeDef<U> {
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
        }
    }
}
