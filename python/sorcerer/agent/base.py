"""Base agent class for SINGULARITY CORE

This module provides the foundation for agent implementations in Sorcerer.
All agent types (Scout, Analyst, Verifier, Executor, Memory) inherit from this base.
"""

from abc import ABC, abstractmethod
from typing import Optional, Dict, Any
from datetime import datetime
from enum import Enum


class AgentType(str, Enum):
    """Different types of agents in the Sorcerer system"""

    SCOUT = "scout"
    ANALYST = "analyst"
    VERIFIER = "verifier"
    EXECUTOR = "executor"
    MEMORY = "memory"


class ExecutionStatus(str, Enum):
    """Status of an agent's execution"""

    PENDING = "pending"
    RUNNING = "running"
    SUCCESS = "success"
    FAILED = "failed"


class Task:
    """A task to be executed by an agent"""

    def __init__(
        self,
        query: str,
        constraints: Optional[Dict[str, Any]] = None,
        parent_id: Optional[str] = None,
    ):
        import uuid

        self.id = str(uuid.uuid4())
        self.query = query
        self.constraints = constraints or {}
        self.parent_id = parent_id
        self.created_at = datetime.utcnow()


class AgentResult:
    """The result of an agent's execution"""

    def __init__(
        self,
        task_id: str,
        status: ExecutionStatus,
        output: Dict[str, Any],
        confidence: float = 0.0,
        execution_time_ms: int = 0,
    ):
        self.task_id = task_id
        self.status = status
        self.output = output
        self.confidence = confidence
        self.execution_time_ms = execution_time_ms


class Memory:
    """Agent memory/context for maintaining state across executions"""

    def __init__(self):
        self.context: Dict[str, Any] = {}
        self.created_at = datetime.utcnow()


class BaseAgent(ABC):
    """Base class that all agent types must implement"""

    def __init__(self, name: str, agent_type: AgentType):
        self.name = name
        self.agent_type = agent_type
        self.memory = Memory()

    @abstractmethod
    async def execute(self, task: Task) -> AgentResult:
        """Execute a task and return a result

        Args:
            task: The task to execute

        Returns:
            AgentResult containing the execution outcome
        """
        pass

    def get_memory(self) -> Memory:
        """Get the agent's current memory/context

        Returns:
            The agent's memory instance
        """
        return self.memory
