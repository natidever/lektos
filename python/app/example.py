import ray
import pyarrow as pa
import io

import lektos
from pydantic import BaseModel
from datetime import datetime
from typing import List
class StoredBlog(BaseModel):
    id:str
    content:List[float]
    url:str
    image_url:str
    title:str
    date:datetime
    author:str
    publisher:str


ray.init()

# --- Step 1: Rust extraction via FFI (simulated here as a function returning bytes) ---
# You'd actually bind this with pyo3/maturin, etc.
def rust_extract_warc(warc_file: str) -> bytes:
    return lektos.core_extractor_runner("")
    

    """
    letktos
    I'll call into Rust extractor: it takes a WARC file path and
    returns serialized Arrow IPC bytes representing the blogs.
    """



@ray.remote
def extract_worker(warc_file: str):
    """Distributed extraction: one WARC file -> Arrow table"""
    arrow_bytes = rust_extract_warc(warc_file)
    buf = io.BytesIO(arrow_bytes)
    reader = pa.ipc.open_stream(buf)
    table = reader.read_all()
    # print(f"Table::{table}")
    return table

@ray.remote
def embed_worker(table: pa.Table)->List[StoredBlog]:
    """i'll take Arrow table rows and return embeddings """
    content = table["content"].to_pylist()  # assuming 'content' column
    id=table["id"].to_pylist()
    urls = table["url"].to_pylist()
    image_urls = table["image_url"].to_pylist()
    titles = table["title"].to_pylist()
    dates = table["date"].to_pylist()
    authors = table["author"].to_pylist()
    publishers = table["publisher"].to_pylist()
    result:List[StoredBlog] = []

    for i in range(len(id)):
        embedding = fake_embed(content[i])

        
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

        
    
    """Embed workder will return data neccessary for upserting which include both the 
    embeding and meta data  """
    return result

def fake_embed(text: str):
    # placeholder: real embedding call to Gemini or OpenAI
    return [0.1, 0.2, 0.3]

# --- Step 3: Store ---
@ray.remote
def store_worker(embeddings, metadata=None):
    """Store embeddings in Qdrant (or elsewhere)."""
    # mock 
    print(f"Storing {len(embeddings)} embeddings")
    return True


def process_warcs(warc_files):
    # step 1: distributed extraction
    extracted_tables = ray.get([extract_worker.remote(w) for w in warc_files])

    
    # step 2 : embeding
    embeddings = ray.get([embed_worker.remote(t) for t in extracted_tables])
    for done,fut in embeddings:
        # print(f"fut{fut}")
        print(f"done:{done.content}")

    
    


    # Step 3: distributed storing
    results = ray.get([store_worker.remote(e) for e in embeddings])
    return results

# just a test call 
if __name__ == "__main__":
    warc_files = ["warc1.warc", "warc2.warc"]
    results = process_warcs(warc_files)
    print("Pipeline finished:", results)
