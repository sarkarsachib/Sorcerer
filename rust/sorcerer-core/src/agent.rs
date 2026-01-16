//! Agent abstractions and types for the agentic reasoning system

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core agent trait that all agent types must implement
pub trait Agent: Send + Sync {
    /// Returns the agent's unique name
    fn name(&self) -> &str;

    /// Returns the type of this agent
    fn agent_type(&self) -> AgentType;

    /// Executes a task and returns a result
    async fn execute(&mut self, task: Task) -> Result<AgentResult, AgentError>;

    /// Returns the agent's current memory/context
    fn memory(&self) -> &Memory;
}

/// Different types of agents in the Sorcerer system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    /// Discovery and exploration agents
    Scout,
    /// Deep reading and analysis agents
    Analyst,
    /// Fact-checking and verification agents
    Verifier,
    /// Action execution agents
    Executor,
    /// Long-term context management agents
    Memory,
}

/// A task to be executed by an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier
    pub id: String,
    /// The query or command to execute
    pub query: String,
    /// Constraints on the query execution
    pub constraints: QueryConstraints,
    /// Optional parent task ID for task hierarchies
    pub parent_id: Option<String>,
    /// When this task was created
    pub created_at: DateTime<Utc>,
}

impl Task {
    /// Create a new task with a generated ID
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            query: query.into(),
            constraints: QueryConstraints::default(),
            parent_id: None,
            created_at: Utc::now(),
        }
    }
}

/// The result of an agent's execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    /// ID of the task that produced this result
    pub task_id: String,
    /// Execution status
    pub status: ExecutionStatus,
    /// Output data (flexible JSON structure)
    pub output: serde_json::Value,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Time taken to execute in milliseconds
    pub execution_time_ms: u64,
}

/// Status of an agent's execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// Task is waiting to be executed
    Pending,
    /// Task is currently running
    Running,
    /// Task completed successfully
    Success,
    /// Task failed with an error message
    Failed(String),
}

/// Agent memory/context for maintaining state across executions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Memory {
    /// Key-value context data
    pub context: HashMap<String, serde_json::Value>,
    /// When this memory was created
    pub created_at: DateTime<Utc>,
}

impl Memory {
    /// Create a new empty memory
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
            created_at: Utc::now(),
        }
    }
}

/// Errors that can occur during agent execution
#[derive(Debug, Clone)]
pub enum AgentError {
    /// The agent's execution failed
    ExecutionFailed(String),
    /// The provided task was invalid
    InvalidTask(String),
    /// The operation timed out
    Timeout,
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AgentError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            AgentError::InvalidTask(msg) => write!(f, "Invalid task: {}", msg),
            AgentError::Timeout => write!(f, "Agent execution timeout"),
        }
    }
}

impl std::error::Error for AgentError {}

/// Import QueryConstraints from query module
use crate::query::QueryConstraints;
