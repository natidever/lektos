"""
Ray-specific error handling utilities
"""

import ray
import psutil
import logging
import time
from typing import  Callable, Optional
from functools import wraps

from app.errors import (
    RayWorkerError, 
    RayWorkerMemoryError, 
    RayWorkerTimeoutError, 
    RayWorkerCrashError,
    RayClusterError
)



logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def check_ray_cluster():
    """Check if Ray cluster is healthy"""
    try:
        if not ray.is_initialized():
            raise RayClusterError("Ray cluster is not initialized")
        
        # it's wise to check the resource 
        resources = ray.available_resources()
        if resources.get('CPU', 0) == 0:
            raise RayClusterError("No CPU resources available in Ray cluster")
        
        logger.info(f"Ray cluster healthy - Available CPUs: {resources.get('CPU', 0)}")
        return True
        
    except Exception as e:
        raise RayClusterError(f"Ray cluster check failed: {str(e)}")

def handle_ray_worker_error(error: Exception, worker_type: str = "unknown") -> Exception:
    """It converts generic error to ray error for common onces memory timeout worker crash """
    error_str = str(error).lower()
    
    # Memory related errors
    if any(keyword in error_str for keyword in ['memory', 'oom', 'out of memory']):
        return RayWorkerMemoryError(
            f"Ray {worker_type} worker ran out of memory: {str(error)}",
            details={'original_error': str(error), 'worker_type': worker_type}
        )
    
    # Timeout errors
    if any(keyword in error_str for keyword in ['timeout', 'timed out']):
        return RayWorkerTimeoutError(
            f"Ray {worker_type} worker timed out: {str(error)}",
            details={'original_error': str(error), 'worker_type': worker_type}
        )
    
    # Worker crash/connection errors
    if any(keyword in error_str for keyword in ['worker', 'crash', 'died', 'connection']):
        return RayWorkerCrashError(
            f"Ray {worker_type} worker crashed: {str(error)}",
            details={'original_error': str(error), 'worker_type': worker_type}
        )
    
    # Generic Ray worker error
    return RayWorkerError(
        f"Ray {worker_type} worker error: {str(error)}",
        details={'original_error': str(error), 'worker_type': worker_type}
    )

def get_memory_usage():
    """Get current memory usage in MB"""
    process = psutil.Process()
    return process.memory_info().rss / 1024 / 1024

def ray_worker_with_error_handling(worker_type: str, timeout: Optional[int] = None):
    """Decorator for Ray worker functions with error handling"""
    def decorator(func: Callable) -> Callable:
        @wraps(func)
        def wrapper(*args, **kwargs):
            start_time = time.time()
            initial_memory = get_memory_usage()
            
            logger.info(f"Starting {worker_type} worker - Memory: {initial_memory:.1f}MB")
            
            try:
                # Check timeout if specified
                if timeout:
                    result = ray.get(func(*args, **kwargs), timeout=timeout)
                else:
                    result = func(*args, **kwargs)
                
                final_memory = get_memory_usage()
                duration = time.time() - start_time
                
                logger.info(f"Completed {worker_type} worker - Duration: {duration:.2f}s, "
                           f"Memory delta: {final_memory - initial_memory:.1f}MB")
                
                return result
                
            except Exception as e:
                final_memory = get_memory_usage()
                duration = time.time() - start_time
                
                logger.error(f"Failed {worker_type} worker - Duration: {duration:.2f}s, "
                           f"Memory: {final_memory:.1f}MB, Error: {str(e)}")
                
                # Convert to specific Ray error
                ray_error = handle_ray_worker_error(e, worker_type)
                raise ray_error
        
        return wrapper
    return decorator

def safe_ray_get(futures, timeout: Optional[int] = None, worker_type: str = "unknown"):
    """Safely get results from Ray futures with error handling"""
    results = []
    errors = []
    
    for i, future in enumerate(futures):
        try:
            if timeout:
                result = ray.get(future, timeout=timeout)
            else:
                result = ray.get(future)
            results.append(result)
            
        except Exception as e:
            ray_error = handle_ray_worker_error(e, worker_type)
            errors.append({'index': i, 'error': ray_error})
            logger.error(f"Ray future {i} failed: {ray_error.message}")
            results.append(None)  # place holder
    
    return results, errors