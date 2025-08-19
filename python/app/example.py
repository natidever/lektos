import ray
import pyarrow as pa
import io

import os
# local package
import lektos
# 
from pydantic import BaseModel
from datetime import datetime
from typing import List
from google import genai
from google.genai import types
from dotenv import load_dotenv

from qdrant_client import QdrantClient,AsyncQdrantClient
import numpy as np
from qdrant_client.models import PointStruct
from qdrant_client.models import VectorParams, Distance
from qdrant_client.http.exceptions import ApiException



class StoredBlog(BaseModel):
    id:str
    content:List[float]
    url:str
    image_url:str
    title:str
    date:datetime
    author:str
    publisher:str

COLLECTION_NAME="blogs"


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

        
    
    """Embed workder will return data neccessary for upserting which include both the 
    embeding and meta data  """
    return result

def embed(text: str):
    # if we're going to use gemni for embeding with 768 size batching 
    # has no meaning we'll hit the rate limit wiht just one batch(with two size)
    
    load_dotenv()
    GEMNI_API_KEY =os.getenv("GEMINI_API_KEY_6")
    http_options = types.HttpOptions(
    async_client_args={},
    )

    client = genai.Client(
        api_key=GEMNI_API_KEY,
        http_options=http_options
    )
         
    
    result = client.models.embed_content(
        model="gemini-embedding-001",
        contents=text,
        config=types.EmbedContentConfig(output_dimensionality=768,task_type="SEMANTIC_SIMILARITY"),
    )
    

    return result.embeddings[0].values

# --- Step 3: Store ---

@ray.remote
def store_worker_wrapper(blog:StoredBlog):
    import asyncio
    asyncio.run(store_worker(blog))


    
async def store_worker(blogs:StoredBlog):


    load_dotenv()
    
    QDRANT_API_KEY=os.getenv("QUADRANT_API_KEY")
    QDRANT_URL =  os.getenv("QUADRANT_URL")

    try:

        client = AsyncQdrantClient(url=QDRANT_URL,api_key=QDRANT_API_KEY,prefer_grpc=True)

        # point=create_point(blog)
        points = [create_point(blog) for blog in blogs]
        update_resposne = await client.upsert(
            collection_name=COLLECTION_NAME,
            points=points

        )
        return update_resposne
    except ApiException as e :
        print(f"Error while storing points @store_workerfn :{e}")
        raise
    except Exception as e :
        print (f"Unexpected error occur @store_worker :{e}")
        raise
   




def create_point(blog:StoredBlog)->PointStruct:
    print(f"blog_content:{blog.content}")

    return PointStruct(
        id="d30cedb4-ecf2-4a99-bf96-0b9df3c4c328",
        vector=blog.content,
        payload={
            "author":blog.author,
            "title":blog.title,
            "date":blog.date,
            "publisher":blog.publisher,
            "image_url":blog.image_url,
            "url":blog.url

        }
    )


def process_warcs(warc_files):
    # step 1: distributed extraction
    extracted_tables = ray.get([extract_worker.remote(w) for w in warc_files])

    
    # step 2 : embeding
    embeddings = ray.get([embed_worker.remote(t) for t in extracted_tables])
    print(f"Embedings:{embeddings}")
    # for done,fut in embeddings:
    #     # print(f"fut{fut}")
    #     print(f"done:{done.content}")

    
    

    # Step 3: distributed storing
    results = ray.get([store_worker_wrapper.remote(e) for e in embeddings])
    return results

# just a test call 
if __name__ == "__main__":
    # import asyncio
    # blog = StoredBlog(
    #     content=[23,2323,23,32],
    #     id="test",
    #     author="blog.author_test",
    #     title="blog.title_test",
    #     date=datetime.now(),
    #     publisher="blog.publisher_Test",
    #     image_url="blog.image_url_test",
    #     url="blog.url_test"

    # )
    # try:    
    #   asyncio.run(store_worker(blog))
    # except  ApiException as e :
    #     print(f"this was the error :{e}")

    # except Exception as e :
    #     print(f"Unexpected  :{e}")
        


    warc_files = ["warc1.warc", "warc2.warc"]
    results = process_warcs(warc_files)
    print("Pipeline finished:", results)


    
    