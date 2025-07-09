#[cfg(feature = "serde")]
use std::sync::{Arc, Mutex};

/// Audit structure and methods for behavior trees
use super::BehaviorTreeState;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Marker for different states in behavior tree execution, used for auditing.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum BehaviorTreeMarker {
    /// Enter marker for nodes
    Enter(String),
    /// Marker indicator
    Marker(String),
    /// Exit marker for nodes with state information
    Exit(String, BehaviorTreeState),
}

pub trait DataLogger: std::fmt::Debug + Send + Sync {
    /// Logs data with a label and associated data.
    fn log_data(&self, name: &str, label: &str, data: &str);
}

/// Structure to store audit information of behavior trees.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct BehaviorTreeAudit {
    events: Vec<BehaviorTreeMarker>,
    place: Vec<String>,
    #[cfg(feature = "serde")]
    #[serde(skip)]
    data_logger: Option<Arc<Mutex<dyn DataLogger>>>,
    data_only: bool,
}

impl BehaviorTreeAudit {
    /// Creates a new BehaviorTreeAudit instance.
    pub fn new() -> Self {
        BehaviorTreeAudit {
            events: Vec::new(),
            place: Vec::new(),
            #[cfg(feature = "serde")]
            data_logger: None,
            data_only: false,
        }
    }

    pub fn data_only<T: DataLogger + 'static>(data_logger: T) -> Self {
        BehaviorTreeAudit {
            events: Vec::new(),
            place: Vec::new(),
            #[cfg(feature = "serde")]
            data_logger: Some(Arc::new(Mutex::new(data_logger))),
            data_only: true,
        }
    }
}

/// Trait defining audit operations for behavior trees.
pub trait BehaviorTreeAuditTrait {
    /// Records the entry of a node.
    fn enter<N: ToString>(&mut self, node_name: &N);

    /// Marks a node with additional information.
    fn mark<N: ToString>(&mut self, node_name: &N);

    /// Audits data used during the execution of a behavior tree.
    #[cfg(feature = "serde")]
    fn data<N: ToString, L: ToString, D: Serialize>(&mut self, node_name: &N, label: &L, data: &D);

    /// Records the exit of a node including its state.
    fn exit<N: ToString>(&mut self, node_name: &N, state: BehaviorTreeState);
}

impl BehaviorTreeAuditTrait for BehaviorTreeAudit {
    fn enter<N: ToString>(&mut self, node_name: &N) {
        if self.data_only {
            return;
        }
        self.events
            .push(BehaviorTreeMarker::Enter(node_name.to_string()));
        self.place.push(node_name.to_string());
    }

    fn mark<N: ToString>(&mut self, node_name: &N) {
        if self.data_only {
            return;
        }
        self.events
            .push(BehaviorTreeMarker::Marker(node_name.to_string()));
    }

    #[cfg(feature = "serde")]
    fn data<N: ToString, L: ToString, D: Serialize>(
        &mut self,
        _node_name: &N,
        _label: &L,
        _data: &D,
    ) {
        #[cfg(feature = "serde")]
        if let Some(logger) = &self.data_logger {
            match logger.lock() {
                Ok(logger) => {
                    logger.log_data(
                        &_node_name.to_string(),
                        &_label.to_string(),
                        &serde_json::to_string(_data).unwrap_or_default(),
                    );
                }
                Err(e) => {
                    eprintln!("Failed to lock data logger: {}", e);
                }
            }
        }
    }

    fn exit<N: ToString>(&mut self, node_name: &N, state: BehaviorTreeState) {
        if self.data_only {
            return;
        }
        self.events
            .push(BehaviorTreeMarker::Exit(node_name.to_string(), state));
        if let Some(current_node_name) = self.place.last() {
            if current_node_name.eq(&node_name.to_string()) {
                self.place.pop();
            }
        }
    }
}

#[cfg(feature = "serde")]
#[derive(Debug, Clone)]
pub struct DataPrintAudit;

#[cfg(feature = "serde")]
impl DataLogger for DataPrintAudit {
    fn log_data(&self, name: &str, label: &str, data: &str) {
        println!("Data for {} - {}: {}", name, label, data);
    }
}

impl<T> BehaviorTreeAuditTrait for &mut Option<T>
where
    T: BehaviorTreeAuditTrait,
{
    fn enter<N: ToString>(&mut self, node_name: &N) {
        if let Some(audit) = self {
            audit.enter(node_name);
        }
    }

    fn mark<N: ToString>(&mut self, node_name: &N) {
        if let Some(audit) = self {
            audit.mark(node_name);
        }
    }

    #[cfg(feature = "serde")]
    fn data<N: ToString, L: ToString, D: Serialize>(&mut self, node_name: &N, label: &L, data: &D) {
        if let Some(audit) = self {
            audit.data(node_name, label, data);
        }
    }

    fn exit<N: ToString>(&mut self, node_name: &N, state: BehaviorTreeState) {
        if let Some(audit) = self {
            audit.exit(node_name, state);
        }
    }
}
