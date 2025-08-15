# there will function that is able to process a warc file then i will call ray.process() use it 3 ocre

from pathlib import Path
# import lektos
import ray
import socket
import pyarrow as pa
import pyarrow.ipc as ipc
from tqdm import tqdm


#
# @ray.remote
# def process_single_warc(warc_path):
#     """Ray task to process one WARC file"""
#     try:
#         value = lektos.extractor_runner(warc_path)
#         print(f"Lektos Runner returns :{value}")
#         return value
#     except Exception as e:
#         print(f"Exception:P{e}")
#         return None, str(e)


def batch_process_warcs(warc_directory, max_workers=8):
    warc_files = sorted(Path(warc_directory).glob("*.warc.gz"))
    print(f"Found {len(warc_files)} WARC files to process")

    results = []
    with tqdm(total=len(warc_files)) as pbar:
        chunk_size = max_workers * 5
        for i in range(0, len(warc_files), chunk_size):
            chunk = warc_files[i : i + chunk_size]
            # procees_extract_blog("her")

            futures = [process_single_warc.remote(str(f.absolute())) for f in chunk]

        while futures:
            done, futures = ray.wait(futures, timeout=5.0)
            for result in ray.get(done):
                print(f"rresult:{result}")
                results.extend(result)
                pbar.update(1)

    return results


# if __name__ == "__main__":
#     all_urls = batch_process_warcs(
#         "/home/natnael/projects/lektos/src/common_crawl_2025-26_warcfiles/",
#         max_workers=8,  # Adjust based on your cluster
#     )
#     print(f"Total URLs extracted: {len(all_urls)}")





HOST="127.0.0.1"
PORT=4000
import lektos
import pyarrow as pa
import io

def main ():
    # Step 1: Call Rust function
    arrow_bytes = lektos.core_extractor_runner("path")

    # Step 2: Read in-memory Arrow data
    buf = io.BytesIO(arrow_bytes)
    reader = pa.ipc.open_stream(buf)
    table = reader.read_all()

    print(table)

    # with socket.create_connection((HOST,PORT)) as sock :
    #     print("Connected to Rust server!")

    #     # Wrap socket as file-like object for pyarrow
    #     reader = sock.makefile('rb')
    #     stream = ipc.open_stream(reader)

    #     for batch in stream:
    #         print("Received RecordBatch:")
    #         print(batch)

if __name__ == "__main__":
    main()


