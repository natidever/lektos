"""
Custom error taxonomy for Lektos pipeline
Focuses on Ray-specific errors and basic error handling
"""

class LektosError(Exception):
    """Base exception for all Lektos pipeline errors"""
    def __init__(self, message: str, details: dict = None):
        self.message = message
        self.details = details or {}
        super().__init__(self.message)
    
# Ray-specific errors
class RayWorkerError(LektosError):
    """Base class for Ray worker errors"""
    pass

class RayWorkerMemoryError(RayWorkerError):
    """Ray worker ran out of memory"""
    pass

class RayWorkerTimeoutError(RayWorkerError):
    """Ray worker operation timed out"""
    pass

class RayWorkerCrashError(RayWorkerError):
    """Ray worker crashed unexpectedly"""
    pass

class RayClusterError(LektosError):
    """Ray cluster is not available or misconfigured"""
    pass

# Extraction errors
class ExtractionError(LektosError):
    """Base class for extraction errors"""
    pass

class WarcFileError(ExtractionError):
    """WARC file is corrupted or unreadable"""
    pass

class RustExtractionError(ExtractionError):
    """Error in Rust extraction function"""
    pass

# API errors
class APIError(LektosError):
    """Base class for API errors"""
    pass

class GeminiAPIError(APIError):
    """Gemini API related errors"""
    pass

class QdrantAPIError(APIError):
    """Qdrant API related errors"""
    pass

# Data errors
class DataValidationError(LektosError):
    """Data validation failed"""
    pass

class EmbeddingError(LektosError):
    """Embedding generation failed"""
    pass

class StorageError(LektosError):
    """Storage operation failed"""
    pass