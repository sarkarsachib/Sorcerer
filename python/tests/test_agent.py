"""Tests for agent system"""

import pytest
from sorcerer.agent.base import (
    AgentType,
    ExecutionStatus,
    Task,
    AgentResult,
    Memory,
    BaseAgent,
)
from sorcerer.agent.orchestrator import AgentOrchestrator


class MockAgent(BaseAgent):
    """Mock agent for testing"""

    def __init__(self, name: str = "test-agent"):
        super().__init__(name, AgentType.SCOUT)

    async def execute(self, task: Task) -> AgentResult:
        """Simple mock execution"""
        return AgentResult(
            task_id=task.id,
            status=ExecutionStatus.SUCCESS,
            output={"query": task.query, "result": "test-result"},
            confidence=0.95,
        )


@pytest.mark.asyncio
async def test_task_creation():
    """Test that Task objects are created correctly"""
    task = Task("test query")

    assert task.id is not None
    assert task.query == "test query"
    assert task.created_at is not None
    assert isinstance(task.constraints, dict)


def test_agent_types():
    """Test AgentType enum values"""
    assert AgentType.SCOUT == "scout"
    assert AgentType.ANALYST == "analyst"
    assert AgentType.VERIFIER == "verifier"
    assert AgentType.EXECUTOR == "executor"
    assert AgentType.MEMORY == "memory"


def test_memory_creation():
    """Test Memory object creation"""
    memory = Memory()

    assert memory.context == {}
    assert memory.created_at is not None


@pytest.mark.asyncio
async def test_agent_execution():
    """Test basic agent execution"""
    agent = MockAgent()
    task = Task("test query")

    result = await agent.execute(task)

    assert result.task_id == task.id
    assert result.status == ExecutionStatus.SUCCESS
    assert result.confidence == 0.95
    assert result.output["query"] == "test query"


def test_orchestrator_registration():
    """Test agent registration with orchestrator"""
    agent = MockAgent()
    orchestrator = AgentOrchestrator([])

    orchestrator.register_agent(agent)

    assert agent.name in orchestrator.agents


@pytest.mark.asyncio
async def test_orchestrator_task_execution():
    """Test orchestrator task execution"""
    agent = MockAgent()
    orchestrator = AgentOrchestrator([agent])
    task = Task("test query")

    result = await orchestrator.execute_task(task, agent_name="test-agent")

    assert result.task_id == task.id
    assert result.status == ExecutionStatus.SUCCESS
