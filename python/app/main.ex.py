# there will function that is able to process a warc file then i will call ray.process() use it 3 ocre

from pathlib import Path

# import lektos
import ray
import socket
import pyarrow as pa
import pyarrow.ipc as ipc
from tqdm import tqdm


import lektos
import pyarrow as pa
import io


@ray.remote
def process_single_warc(warc_path):
    """Ray task to process one WARC file"""
    try:
        value = lektos.core_extractor_runner(warc_path)
        buf = io.BytesIO(value)
        reader = pa.ipc.open_stream(buf)
        table = reader.read_all()

        # for

        # Step 2: Read in-memory Arrow data
        # chang all table list of mad thi
        urls = table.column("url")
        urls.to_pylist()
        result = []
        for batch in table.to_batches():
            print(f"Table to batch :{batch}")
            # for row in batch.to_pylist():
            #     # print(row)
            #     result.extend(row)

        # arrow_bytes = lektos.core_extractor_runner("path")

        # print(f"Lektos Runner returns :{dir(table)}")
        return result
    except Exception as e:
        print(f"Exception:P{e}")
        return None, str(e)


def batch_process_warcs(warc_directory, max_workers=8):
    warc_files = sorted(Path(warc_directory).glob("*.warc.gz"))
    print(f"Found {len(warc_files)} WARC files to process")

    results = []
    with tqdm(30) as pbar:
        chunk_size = max_workers * 2
        for i in range(0, 30, chunk_size):
            chunk = warc_files[i : i + chunk_size]
            # procees_extract_blog("her")

            futures = [process_single_warc.remote(str(f.absolute())) for f in chunk]

            while futures:
                done, futures = ray.wait(futures, timeout=5.0)
                for result in ray.get(done):
                    # print(f"Result:{result}")
                    results.extend(result)
                    pbar.update(1)

    return results


# if __name__ == "__main__":
#     all_urls = batch_process_warcs(
#         "/home/natnael/projects/lektos/src/common_crawl_2025-26_warcfiles/",
#         max_workers=7,  # Adjust based on your cluster
#     )
#     print(f"Total URLs extracted: {len(all_urls)}")


# HOST="127.0.0.1"
# PORT=4000

# def main ():
#     # Step 1: Call Rust function
#     arrow_bytes = lektos.core_extractor_runner("path")

#     # Step 2: Read in-memory Arrow data
#     buf = io.BytesIO(arrow_bytes)
#     reader = pa.ipc.open_stream(buf)
#     table = reader.read_all()
#     # chang all table list of mad thi
#     urls = table.column("url")
#     urls.to_pylist()
#     for batch in table.to_batches():
#         for row in batch.to_pylist():
#             print(row)


#     # with socket.create_connection((HOST,PORT)) as sock :
#     #     print("Connected to Rust server!")

#     #     # Wrap socket as file-like object for pyarrow
#     #     reader = sock.makefile('rb')
#     #     stream = ipc.open_stream(reader)

#     #     for batch in stream:
#     #         print("Received RecordBatch:")
#     #         print(batch)

# if __name__ == "__main__":
#     main()


# Here we hae the data what left with is

# 1.Batch process in one local machine in python
# 2.


def main():
    import lektos
    print(lektos.is_blog("https://medium.com/how-starting-an-investment-firm-almost-landed-me-in-a-federal-prison-652ef7f222a2"))
    pass

main()