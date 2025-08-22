import ray
import pyarrow as pa
import io
import os
import asyncio
import logging
import glob


# local package
import lektos


# Error handlings
from app.errors import (
    RayWorkerError,
    RayWorkerMemoryError,
    RayWorkerTimeoutError,
    RayWorkerCrashError,
    RayClusterError,
    ExtractionError,
    WarcFileError,
    RustExtractionError,
)
from app.ray_error_handler import (
    check_ray_cluster,
    handle_ray_worker_error,
    safe_ray_get,
)

from typing import List
from app.models import StoredBlog
from app.utils.qdrant_utils import store_worker
from app.utils.gemini_utils import embed

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


# Initialize Ray with error handling
try:
    if not ray.is_initialized():
        ray.init(num_cpus=6)
        logger.info("Ray cluster initialized successfully")

    # checkinn cluster health
    check_ray_cluster()

except Exception as e:
    raise RayClusterError(f"Failed to initialize Ray cluster: {str(e)}")


def rust_extract_warc(warc_file: str) -> bytes:
    """
    Calling rust extractor: it takes a WARC file path and
    returns  arrow IPC bytes currently we're reading from file
    still some error handling opt lefet here

    """
    try:
        if not os.path.exists(warc_file):
            raise WarcFileError(f"WARC file not found: {warc_file}")

        result = lektos.core_extractor_runner(warc_file)

        if not result:
            raise RustExtractionError(
                f"Rust extraction returned empty result for {warc_file}"
            )

        return result

    except Exception as e:
        if isinstance(e, (WarcFileError, RustExtractionError)):
            raise
        else:
            raise RustExtractionError(
                f"Rust extraction failed for {warc_file}: {str(e)}"
            )


@ray.remote(num_cpus=1, memory=2 * 1024 * 1024 * 1024)
def extract_worker(warc_file: str):
    """remote call for rust extractor"""
    try:
        logger.info(f"Starting extraction for {warc_file}")

        # calling rust fun
        arrow_bytes = rust_extract_warc(warc_file)

        # Convert to Arrow table
        buf = io.BytesIO(arrow_bytes)
        reader = pa.ipc.open_stream(buf)
        table = reader.read_all()

        logger.info(f"Successfully extracted {len(table)} rows from {warc_file}")
        return table

    except Exception as e:
        logger.error(f"Extraction failed for {warc_file}: {str(e)}")
        # Convert to specific Ray error
        ray_error = handle_ray_worker_error(e, "extract")
        raise ray_error


@ray.remote(num_cpus=1, memory=1024 * 1024 * 1024)  # 1GB memory limit ...locally
def embed_worker(table: pa.Table) -> List[StoredBlog]:
    """
    Embed worker will return data necessary for upserting which includes both the
    embedding and meta data. The result will be blog meta-data + embedding.
    """
    try:
        logger.info(f"Starting embedding for {len(table)} items")

        # Extract data from Arrow table
        content = table["content"].to_pylist()
        id = table["id"].to_pylist()
        urls = table["url"].to_pylist()
        image_urls = table["image_url"].to_pylist()
        titles = table["title"].to_pylist()
        dates = table["date"].to_pylist()
        authors = table["author"].to_pylist()
        publishers = table["publisher"].to_pylist()

        result: List[StoredBlog] = []

        for i in range(len(id)):
            try:
                embedding = embed(content[i])

                blog = StoredBlog(
                    id=id[i],
                    content=embedding,
                    url=urls[i],
                    image_url=image_urls[i],
                    title=titles[i],
                    date=dates[i],
                    author=authors[i],
                    publisher=publishers[i],
                )

                result.append(blog)

            except Exception as e:
                logger.error(f"Failed to embed item {i}: {str(e)}")
                # Continue with other items instead of failing the whole batch
                continue

        logger.info(f"Successfully embedded {len(result)} out of {len(id)} items")
        return result

    except Exception as e:
        logger.error(f"Embedding worker failed: {str(e)}")
        ray_error = handle_ray_worker_error(e, "embed")
        raise ray_error


@ray.remote(num_cpus=1, memory=512 * 1024 * 1024)  # 512MB memory limit locally
def store_worker_wrapper(blogs: List[StoredBlog]):
    """Wrapper for async store worker"""
    try:
        logger.info(f"Starting storage for {len(blogs)} blogs")
        result = asyncio.run(store_worker(blogs))
        logger.info(f"Successfully stored {len(blogs)} blogs")
        return result

    except Exception as e:
        logger.error(f"Storage worker failed: {str(e)}")
        ray_error = handle_ray_worker_error(e, "store")
        raise ray_error


def process_warcs(warc_files):
    """Process WARC files with Ray error handling"""
    try:
        # Check Ray cluster health before processing
        check_ray_cluster()

        logger.info(f"Starting processing of {len(warc_files)} WARC files")

        # the first step extractoin
        logger.info("Step 1: Starting distributed extraction")
        extract_futures = [extract_worker.remote(w) for w in warc_files]
        extracted_tables, extract_errors = safe_ray_get(
            extract_futures,
            timeout=600,
            worker_type="extract",  # 10 minute timeout
        )

        # Filter out None results from failed extractions
        valid_tables = [table for table in extracted_tables if table is not None]
        logger.info(
            f"Extraction completed: {len(valid_tables)} successful, {len(extract_errors)} failed"
        )

        if not valid_tables:
            logger.error("All extractions failed, stopping pipeline")
            return []

        # Step 2: Distributed embedding with error handling
        logger.info("Step 2: Starting distributed embedding")
        embed_futures = [embed_worker.remote(t) for t in valid_tables]
        embeddings, embed_errors = safe_ray_get(
            embed_futures,
            timeout=1800,
            worker_type="embed",  # 30 minute timeout
        )

        # Filter out None results and flatten the list
        valid_embeddings = []
        for embedding_batch in embeddings:
            if embedding_batch is not None:
                valid_embeddings.extend(embedding_batch)

        logger.info(
            f"Embedding completed: {len(valid_embeddings)} blogs embedded, {len(embed_errors)} batches failed"
        )

        if not valid_embeddings:
            logger.error("All embeddings failed, stopping pipeline")
            return []

        # finally step 4 Distributed storing with error handling
        logger.info("Step 3: Starting distributed storage")
        # Group embeddings into smaller batches for storage
        batch_size = 10
        storage_batches = [
            valid_embeddings[i : i + batch_size]
            for i in range(0, len(valid_embeddings), batch_size)
        ]

        store_futures = [
            store_worker_wrapper.remote(batch) for batch in storage_batches
        ]
        results, store_errors = safe_ray_get(
            store_futures,
            timeout=300,
            worker_type="store",  # 5 minute timeout
        )

        # Filter out None results
        valid_results = [result for result in results if result is not None]
        logger.info(
            f"Storage completed: {len(valid_results)} batches stored, {len(store_errors)} failed"
        )

        # Log summary of all errors
        total_errors = len(extract_errors) + len(embed_errors) + len(store_errors)
        if total_errors > 0:
            logger.warning(f"Pipeline completed with {total_errors} total errors")
            logger.warning(
                f"Extract errors: {len(extract_errors)}, "
                f"Embed errors: {len(embed_errors)}, "
                f"Store errors: {len(store_errors)}"
            )

        return valid_results

    except RayClusterError as e:
        logger.error(f"Ray cluster error: {e.message}")
        raise
    except Exception as e:
        logger.error(f"Unexpected error in process_warcs: {str(e)}")
        raise


# just a test call


if __name__ == "__main__":
    """
    main execution with comprehensive Ray error handling
    """
    try:
        # Local warc file
        warc_files = glob.glob(
            "/home/natnael/projects/lektos/src/common_crawl_2025-26_warcfiles/*.warc.gz"
        )

        if not warc_files:
            logger.error("No WARC files found in the specified directory")
            exit(1)

        # testing with one warc file
        test_files = warc_files[0:1]
        logger.info(f"Processing {len(test_files)} WARC files: {test_files}")

        # with error handling
        results = process_warcs(test_files)

        if results:
            logger.info(
                f"Pipeline completed successfully! Processed {len(results)} batches"
            )
            print("Pipeline finished successfully:", len(results), "batches processed")
        else:
            logger.warning("Pipeline completed but no results were produced")
            print("Pipeline finished with no results")

    except RayClusterError as e:
        logger.error(f"Ray cluster error: {e.message}")
        print(f"Ray cluster error: {e.message}")
        exit(1)

    except (
        RayWorkerError,
        RayWorkerMemoryError,
        RayWorkerTimeoutError,
        RayWorkerCrashError,
    ) as e:
        logger.error(f"Ray worker error: {e.message}")
        print(f"Ray worker error: {e.message}")
        exit(1)

    except (ExtractionError, WarcFileError, RustExtractionError) as e:
        logger.error(f"Extraction error: {e.message}")
        print(f"Extraction error: {e.message}")
        exit(1)

    except Exception as e:
        logger.error(f"Unexpected error: {str(e)}")
        print(f"Unexpected error: {str(e)}")
        exit(1)

    finally:
        # Clean up ray resources
        if ray.is_initialized():
            logger.info("Shutting down Ray cluster")
            ray.shutdown()
