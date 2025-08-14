# there will function that is able to process a warc file then i will call ray.process() use it 3 ocre

from pathlib import Path
import lektos
import ray

from tqdm import tqdm


#
@ray.remote
def process_single_warc(warc_path):
    """Ray task to process one WARC file"""
    try:
        value = lektos.extractor_runner(warc_path)
        print(f"Lektos Runner returns :{value}")
        return value
    except Exception as e:
        print(f"Exception:P{e}")
        return None, str(e)


def batch_process_warcs(warc_directory, max_workers=8):
    warc_files = sorted(Path(warc_directory).glob("*.warc.gz"))
    print(f"Found {len(warc_files)} WARC files to process")

    results = []
    with tqdm(total=len(warc_files)) as pbar:
        chunk_size = max_workers * 5
        for i in range(0, len(warc_files), chunk_size):
            chunk = warc_files[i : i + chunk_size]

            futures = [process_single_warc.remote(str(f.absolute())) for f in chunk]

        # while futures:
        #     done, futures = ray.wait(futures, timeout=5.0)
        #     for result in ray.get(done):
        #         print(f"rresult:{result}")
        #         results.extend(result)
        #         pbar.update(1)

    return results


if __name__ == "__main__":
    all_urls = batch_process_warcs(
        "/home/natnael/projects/lektos/src/common_crawl_2025-26_warcfiles/",
        max_workers=8,  # Adjust based on your cluster
    )
    print(f"Total URLs extracted: {len(all_urls)}")
