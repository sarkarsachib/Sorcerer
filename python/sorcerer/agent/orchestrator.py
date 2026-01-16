"""Agent Orchestrator for SINGULARITY CORE

This module coordinates multiple agents and manages task decomposition
for complex search and reasoning operations.
"""

from typing import List, Optional
from .base import BaseAgent, Task, AgentResult


class AgentOrchestrator:
    """Coordinates agent execution and task decomposition"""

    def __init__(self, agents: List[BaseAgent]):
        self.agents = {agent.name: agent for agent in agents}

    async def execute_task(self, task: Task, agent_name: Optional[str] = None) -> AgentResult:
        """Execute a task using a specific agent or auto-select one

        Args:
            task: The task to execute
            agent_name: Optional agent name to use (auto-selects if None)

        Returns:
            AgentResult from execution
        """
        agent = self._select_agent(task, agent_name)
        if agent is None:
            from .base import AgentResult, ExecutionStatus

            return AgentResult(
                task_id=task.id,
                status=ExecutionStatus.FAILED,
                output={"error": "No suitable agent found"},
                confidence=0.0,
            )

        return await agent.execute(task)

    def _select_agent(self, task: Task, agent_name: Optional[str]) -> Optional[BaseAgent]:
        """Select an agent for task execution

        Args:
            task: The task to execute
            agent_name: Optional specific agent name

        Returns:
            Selected agent or None
        """
        if agent_name:
            return self.agents.get(agent_name)

        # Simple auto-selection logic - will be enhanced in Phase 2
        # For now, return the first available agent
        return next(iter(self.agents.values()), None)

    def register_agent(self, agent: BaseAgent) -> None:
        """Register a new agent with the orchestrator

        Args:
            agent: The agent to register
        """
        self.agents[agent.name] = agent
