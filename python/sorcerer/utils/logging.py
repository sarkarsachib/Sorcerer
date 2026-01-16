"""Logging configuration for Sorcerer

Sets up structured logging with loguru for both development
and production environments.
"""

import sys
from loguru import logger
from typing import Optional


def setup_logging(log_level: str = "info", log_file: Optional[str] = None):
    """Configure logging for Sorcerer

    Args:
        log_level: Logging level (debug, info, warning, error)
        log_file: Optional path to log file
    """
    # Remove default handler
    logger.remove()

    # Add console handler with colors
    logger.add(
        sys.stdout,
        format="<green>{time:YYYY-MM-DD HH:mm:ss}</green> | <level>{level: <8}</level> | <cyan>{name}</cyan>:<cyan>{function}</cyan>:<cyan>{line}</cyan> - <level>{message}</level>",
        level=log_level.upper(),
        colorize=True,
    )

    # Add file handler if specified
    if log_file:
        logger.add(
            log_file,
            format="{time:YYYY-MM-DD HH:mm:ss} | {level: <8} | {name}:{function}:{line} - {message}",
            level=log_level.upper(),
            rotation="10 MB",
            retention="30 days",
        )

    return logger
