"""Pytest configuration and shared fixtures"""

import pytest
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
