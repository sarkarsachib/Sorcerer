"""Configuration loader for Sorcerer

Loads configuration from TOML files with support for
environment variable overrides.
"""

import os
import sys
from pathlib import Path
from typing import Dict, Any, Optional
from dataclasses import dataclass

# Use stdlib tomllib on Python 3.11+, fall back to toml
if sys.version_info >= (3, 11):
    import tomllib
else:
    try:
        import toml as tomllib
    except ImportError:
        raise ImportError(
            "toml package required for Python <3.11. Install with: pip install toml"
        )

@dataclass
class SystemConfig:
    """System-level configuration"""

    name: str = "Sorcerer"
    version: str = "0.1.0"
    log_level: str = "info"


@dataclass
class DatabaseConfig:
    """Database configuration"""

    host: str = "localhost"
    port: int = 5432
    name: str = "sorcerer_index"
    pool_size: int = 20
    timeout_secs: int = 30

    def get_url(self, user: str, password: str) -> str:
        """Get database connection URL"""
        return f"postgresql://{user}:{password}@{self.host}:{self.port}/{self.name}"


@dataclass
class IndexingConfig:
    """Indexing configuration"""

    vector_dimensions: int = 1536
    batch_size: int = 100
    auto_commit_interval_secs: int = 300


@dataclass
class CrawlerConfig:
    """Crawler configuration"""

    user_agent: str = "Sorcerer/0.1.0 (+http://sorcerer.ai)"
    max_concurrent_crawls: int = 10
    timeout_secs: int = 30


@dataclass
class AgentsConfig:
    """Agent configuration"""

    max_sub_agents: int = 5
    memory_ttl_hours: int = 24
    default_timeout_secs: int = 300


@dataclass
class ApiConfig:
    """API configuration"""

    host: str = "0.0.0.0"
    port: int = 8080
    grpc_port: int = 50051


class ConfigLoader:
    """Load and manage Sorcerer configuration"""

    def __init__(self, config_path: Optional[Path] = None):
        self.config_path = config_path
        self._config: Optional[Dict[str, Any]] = None

    def load(self) -> Dict[str, Any]:
        """Load configuration from file or use defaults

        Returns:
            Configuration dictionary
        """
        if self._config is None:
            if self.config_path and self.config_path.exists():
                # Handle binary vs text mode
                if sys.version_info >= (3, 11):
                    with open(self.config_path, 'rb') as f:
                        self._config = tomllib.load(f)
                else:
                    with open(self.config_path, 'r') as f:
                        self._config = tomllib.load(f)
            else:
                self._config = self._get_defaults()

        return self._config

    def get_system_config(self) -> SystemConfig:
        """Get system configuration"""
        config = self.load().get("system", {})
        return SystemConfig(**config)

    def get_database_config(self) -> DatabaseConfig:
        """Get database configuration"""
        config = self.load().get("database", {})
        return DatabaseConfig(**config)

    def get_indexing_config(self) -> IndexingConfig:
        """Get indexing configuration"""
        config = self.load().get("indexing", {})
        return IndexingConfig(**config)

    def get_crawler_config(self) -> CrawlerConfig:
        """Get crawler configuration"""
        config = self.load().get("crawler", {})
        return CrawlerConfig(**config)

    def get_agents_config(self) -> AgentsConfig:
        """Get agents configuration"""
        config = self.load().get("agents", {})
        return AgentsConfig(**config)

    def get_api_config(self) -> ApiConfig:
        """Get API configuration"""
        config = self.load().get("api", {})
        return ApiConfig(**config)

    def _get_defaults(self) -> Dict[str, Any]:
        """Get default configuration"""
        return {
            "system": {"name": "Sorcerer", "version": "0.1.0", "log_level": "info"},
            "database": {
                "host": "localhost",
                "port": 5432,
                "name": "sorcerer_index",
                "pool_size": 20,
                "timeout_secs": 30,
            },
            "indexing": {
                "vector_dimensions": 1536,
                "batch_size": 100,
                "auto_commit_interval_secs": 300,
            },
            "crawler": {
                "user_agent": "Sorcerer/0.1.0 (+http://sorcerer.ai)",
                "max_concurrent_crawls": 10,
                "timeout_secs": 30,
            },
            "agents": {
                "max_sub_agents": 5,
                "memory_ttl_hours": 24,
                "default_timeout_secs": 300,
            },
            "api": {"host": "0.0.0.0", "port": 8080, "grpc_port": 50051},
        }
