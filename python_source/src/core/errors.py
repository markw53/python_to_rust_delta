"""
Custom error types for the delta generator.
"""

class DeltaError(Exception):
    """Base class for all delta-related errors."""


class UnsupportedFileType(DeltaError):
    """Raised when encountering an unsupported file type."""


class InvalidPath(DeltaError):
    """Raised when a path cannot be normalized or resolved."""


class SnapshotError(DeltaError):
    """Raised when snapshot creation fails."""


class MetadataError(DeltaError):
    """Raised when metadata extraction fails."""
