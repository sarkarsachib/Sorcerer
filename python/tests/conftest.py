"""Pytest configuration and shared fixtures"""

import os
import pytest
from pathlib import Path

# Ensure parent directories are in path
import sys
sys.path.insert(0, str(Path(__file__).parent.parent))

from sorcerer.config.loader import ConfigLoader


@pytest.fixture
def config_loader():
    """Provide a ConfigLoader instance for tests"""
    return ConfigLoader()


@pytest.fixture
def sample_config():
    """Provide sample configuration for tests"""
    return {
        "system": {"name": "Sorcerer", "version": "0.1.0", "log_level": "debug"},
        "database": {
            "host": "localhost",
            "port": 5432,
            "name": "sorcerer_test",
            "pool_size": 5,
            "timeout_secs": 10,
        },
    }
