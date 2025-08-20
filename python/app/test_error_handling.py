"""
test script for ray error ...will be removed/refactored in standard test 
"""

import sys
import os

sys.path.append(os.path.dirname(__file__))

from app.errors import (
    RayWorkerError, RayWorkerMemoryError, RayWorkerTimeoutError,
    RayWorkerCrashError, RayClusterError, ExtractionError,
    WarcFileError, RustExtractionError
)
from ray_error_handler import (
    handle_ray_worker_error, get_memory_usage, check_ray_cluster
)

def test_error_classification():
    print("=" * 40)
    
    # Test memory error
    memory_error = Exception("out of memory error occurred")
    classified = handle_ray_worker_error(memory_error, "test")
    print(f"Memory error classified as: {type(classified).__name__}")
    print(f"Message: {classified.message}")
    print()
    
    timeout_error = Exception("operation timed out after 300 seconds")
    classified = handle_ray_worker_error(timeout_error, "test")
    print(f"Timeout error classified as... {type(classified).__name__}")
    print(f"Message: {classified.message}")
    print()
    
    crash_error = Exception("worker connection lost")
    classified = handle_ray_worker_error(crash_error, "test")
    print(f"Crash error classified as: {type(classified).__name__}")
    print(f"Message: {classified.message}")
    print()
    
    generic_error = Exception("some other error")
    classified = handle_ray_worker_error(generic_error, "test")
    print(f"Generic error classified as: {type(classified).__name__}")
    print(f"Message: {classified.message}")
    print()

def test_memory_monitoring():
    """Test memory monitoring"""
    print("Testing Memory Monitoring")
    print("=" * 25)
    
    current_memory = get_memory_usage()
    print(f"Current memory usage: {current_memory:.2f} MB")
    print()

def test_custom_errors():
    """Test custom error hierarchy"""
    print("Testing Custom Error Hierarchy")
    print("=" * 30)
    
    # Test WarcFileError
    try:
        raise WarcFileError("Test WARC file not found", {"file": "test.warc.gz"})
    except WarcFileError as e:
        print(f"WarcFileError caught: {e.message}")
        print(f"Details: {e.details}")
        print()
    
    try:
        raise RayWorkerMemoryError("Test memory error", {"worker_type": "extract", "memory_usage": "2.5GB"})
    except RayWorkerMemoryError as e:
        print(f"RayWorkerMemoryError caught: {e.message}")
        print(f"Details: {e.details}")
        print()

if __name__ == "__main__":
    print("Ray Error Handling Test Suite")
    print("=" * 50)
    print()
    
    test_error_classification()
    test_memory_monitoring()
    test_custom_errors()
    
    print("All tests completed!")