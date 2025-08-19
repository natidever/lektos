import ray
import pyarrow as pa
import io

import os
# local package
import lektos
# 
from pydantic import BaseModel, field_validator
from typing import List
from google import genai
from google.genai import types
from dotenv import load_dotenv

from qdrant_client import QdrantClient,AsyncQdrantClient
import numpy as np
from qdrant_client.models import PointStruct
from qdrant_client.models import VectorParams, Distance
from qdrant_client.http.exceptions import ApiException
from dateutil import parser
from datetime import datetime, timezone
from zoneinfo import ZoneInfo
import re

class StoredBlog(BaseModel):
    id:str
    content:List[float]
    url:str
    image_url:str
    title:str
    date:str
    author:str
    publisher:str

    @field_validator('date', mode='before')
    def parse_date(cls, v):
        if isinstance(v, str):
            try :
              return  normalize_date(v)
            except Exception as e :
                return "Unknown"
        return v

COLLECTION_NAME="blogs"


# ray.init(
#     num_cpus=7
# )

# --- Step 1: Rust extraction via FFI (simulated here as a function returning bytes) ---
# You'd actually bind this with pyo3/maturin, etc.
def rust_extract_warc(warc_file: str) -> bytes:
    return lektos.core_extractor_runner(warc_file)
    

    """
    letktos
    I'll call into Rust extractor: it takes a WARC file path and
    returns serialized Arrow IPC bytes representing the blogs.
    """



@ray.remote
def extract_worker(warc_file: str):
    """Distributed extraction: one WARC file -> Arrow table"""
    print(f"Parsing{warc_file}")
    arrow_bytes = rust_extract_warc(warc_file)
    buf = io.BytesIO(arrow_bytes)
    reader = pa.ipc.open_stream(buf)
    table = reader.read_all()
    # print(f"Table::{table}")
    return table

@ray.remote
def embed_worker(table: pa.Table)->List[StoredBlog]:
   
    """i'll take Arrow table rows and return embeddings there is a bit optimization left here  """
    content = table["content"].to_pylist()  
    id=table["id"].to_pylist()
    urls = table["url"].to_pylist()
    image_urls = table["image_url"].to_pylist()
    titles = table["title"].to_pylist()
    dates = table["date"].to_pylist()
    print(f"dates:{dates}")
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
    embeding and meta data  the result will blog meta-data + embeding
    
    
    """
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
    return asyncio.run(store_worker(blog))



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
    return PointStruct(
        id=blog.id,
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

    
    # step 2: distributed embedin

    embeddings = ray.get([embed_worker.remote(t) for t in extracted_tables])   
    
    

    # Step 3: distributed storing
    results = ray.get([store_worker_wrapper.remote(e) for e in embeddings])
    return results



def normalize_date(raw_date: str) -> str:
    """
    Here I'll Normalize any human-readable or machine date string to RFC 3339 with timezone info.
    sicne qdrant accept RFC3339  there is another date normalization you can in the doc(Blog/date/normalization)
    
    Handles:
    - Naive datetime → assumes UTC
    - Datetime with numeric offset → preserved
    - UTC 'Z' → preserved
    - IANA timezone names → localized properly
    - Common human-readable formats
    NOTE:date is crutial but the blog is more crutial this implementation let date to be empty if we are unable to pare it to RFC3339
         because here we are assuming everything  reachs here is a blog .
    returns:
        RFC 3339 string (e.g., '2024-08-19T14:00:00Z' or '2024-08-19T14:00:00-06:00')
    """
    raw_date = raw_date.strip()
    
    # First, check for IANA timezone names attached without separator
    # Example: '2023-01-26 12:06:13America/Costa_Rica'
    iana_match = re.match(r"(.+?)([A-Za-z]+/[A-Za-z_]+)$", raw_date)
    if iana_match:
        dt_str, tz_str = iana_match.groups()
        try:
            dt = parser.parse(dt_str)  # naive datetime
            dt = dt.replace(tzinfo=ZoneInfo(tz_str))  # apply timezone
        except Exception as e:    
            return "Unkown"
            # raise ValueError(f"Cannot parse IANA timezone date '{raw_date}': {e}")
    else:
        # Normal parse for other formats
        try:
            dt = parser.parse(raw_date)
            # If naive, assume UTC
            if dt.tzinfo is None:
                dt = dt.replace(tzinfo=timezone.utc)
        except Exception as e :
           
           return "Unknown"

    # Convert to RFC 3339 string, use 'Z' if UTC
    try:
      rfc3339_str = dt.isoformat().replace("+00:00", "Z")
    except Exception as e :
        dt = "Unknown"
    return rfc3339_str

# just a test call 


if __name__ == "__main__":
#     dates=['2016-06-20T08:02:25+0900', '2017-11-25T09:22:15+0900', '2025-06-10 18:27:04', '2024/07/01', '2025-01-26T12:19:00+0900', '2025-01-16T08:00:00+08:00', '2025-06-10 09:34:56', '2013-07-01', '2017-06-01T21:39:50+0900', '2018-07-08T20:56:00-04:00', '2018-06-23T08:59:00+07:00', '2007-05-20T18:45:00+02:00', '2009-06-16T04:10:00+09:00', '2012-12-22T15:58:00+07:00', '۱۴۰۱/۵/۳ ۸:۰۰:۰۲', '2015-05-06T19:07:31-05:00', '2019-11-09', '2024-07-31', '2024-12-03T09:34:48+09:00', '2016-05-01T03:30:54+03:00', '2023-11-03T06:57:23-03:00', '2024-04-01T09:18:51-03:00', '2024-09-05T04:00:00.000Z', '2024-02-27T18:41:41', '2023-06-26T22:20:52+02:00', '2017-06-19T23:50:23Z', '2024-04-18T06:17:45.290Z', '2019-05-11T07:11:00+02:00', '2025-05-20T19:43:00-05:00', '2020-01-16T00:05:57+02:00', '2025-05-16T01:36:00+02:00', '2023-01-26 12:06:13America/Costa_Rica', '2023-12-22T23:30:00+09:00', '2019-07-27T22:53:27+09:00', '2022-11-22T02:00:02+00:00', '2023-12-31T12:22:59+03:30', '2023-07-18T12:44:04Z', '2011-11-07T07:11:00Z', '2024-8-17T9:20:00', '2009-03-22T00:00:00+01:00', '2014-01-15T00:00:32+01:00', '2019-06-25T11:20:11+00:00', '2025-06-10T11:40:25+03:30', '', 'December, 20 2019 17:41:56 -0300', '2022-04-26T17:49:08+0000', '2022-10-02T18:06:13+09:00', '2025-05-30T23:42:00+05:30', '2025-04-02T10:14:24+01:00', '2024-08-30T11:33:35+00:00', '2025-06-12T15:26:55+03:00', '2016-11-18T09:44:27+01:00', '2025-04-15T04:36:05+00:00', '2025-04-20T04:12:06+00:00', '2019-12-30T14:17:40+06:00', '2020-02-27T06:00:00+01:00', '2025-05-17T06:50:56+00:00', '2012-04-27T17:47:31-03:00', '2023-08-22 17:59:59+05:00', '2024-07-06T05:01:00+09:00', '2016-01-19T00:02:04+01:00', '2022-09-29T17:04:07-07:00', '2022-06-30T07:25:59+09:00', '2020-03-04 08:53:51Europe/Warsaw', '2019-04-20T00:00:00+00:00', '2023-10-15T17:35:58+09:00', '2025-05-20T17:47:32+08:00', '2012-12-16', '2024-08-08T13:21:07+00:00', '2024-05-15T16:01:25+08:00', '2025-05-18T08:05:00+0800', '2020-01-31T10:14:22-05:00', '2023-07-13T03:22:51+00:00', '2024-04-11T14:12:00-03:00', '2014-07-23T18:04:00+09:00', '2025-05-14', '2018-05-28T15:26:00+0900', '2024-08-13T17:00:02Z', '2016-05-21T20:02:39+09:00', '2008-07-22T09:35:47+09:00', '2022-11-18T10:42:12-03:00', '2022-12-09 13:46', '2025-04-16T18:59:00', '2022-06-29T02:47:00+03:00', '2014-03-19T16:40:21+01:00', '2015-06-13T14:09:07+09:00', '2023-12-21', '2024-04-30T18:02:16+09:00', '2022-10-14T11:29:46.000Z', '2022-03-24 15:15:51', '2023-04-30 15:17:49', '2022-05-29 18:59:19', '2022-07-02 08:04:11', '2022-05-28 10:15:06', '2023-01-13 15:22:03', '2016-10-21T11:12:56+02:00', '2011-01-07T19:23:04Z', '2023-01-18T15:45:18+00:00', '2021-08-19T20:01:59+05:30', '2023-01-17T01:08:00-08:00', '', '2025-05-17T07:11:36+09:00', '2025-05-29T20:00:05+09:00', '2022-12-19T16:58:03+00:00', '2023-10-23', '2020-01-22T15:52:55+01:00', '2020-08-02T01:14:46+02:00', '2010-06-24 13:03:00', '2024-10-30T13:37:00+05:30', '2020-10-20T06:55:00-07:00', '2020-03-20T21:32:00-07:00', '2022-04-27T09:33:00+07:00', '2018-09-18T05:00:00+02:00', '2017-07-25T11:15:29Z', '2025-05-16T11:15:17Z', '2024-02-03T11:56:00+02:00', '2025-02-17T00:40:00+09:00', '2017-12-10T23:17:00-08:00', '2018-03-29T13:19:00-07:00', '2020-03-27T18:40:00-07:00', '2022-06-18T09:11:00-07:00', '2022-07-05T17:53:00-07:00', '2025-05-01T11:17:10.565235+08:00', '2021-03-11T10:58:38+01:00', '2023-03-13T08:20:16.239Z', '2015-03-11T21:30:00+07:00', '2024-01-25 14:19:22', '2019-11-11T07:00:00+01:00', '2020-08-19T12:47:26Z', '2019-01-29T22:13:19.841Z', '2021-09-27T17:00:00+00:00', '2013-05-07T09:25:00-03:00', 'Jun 28, 2023', 'Jun 28, 2023', '2022-04-27T18:14:22+5:30', '2023-11-24T11:53:00.0000000', '', '2023-10-28T04:32:00.209Z', 'Jun 08, 2023', '2021-07-12T11:37:00-07:00', '2020-01-02T07:58:00+01:00', '2019-09-04 10:06+0400', '2024-06-04T19:20:03-04:00', '2019-11-26T09:52:26+00:00', '2024-05-13T15:55:00+00:00', '2023-01-13T09:32:47+00:00', '2019-12-06T12:13:48.0000000-05:00', '2013-04-01T00:00:00', '2024-02-14', '2019-04-16T19:09:40.613Z', '2022-02-10', '2023-08-10T03:00:00+02:00', '2024-10-29T07:25:43+09:00', '2022-08-13T23:41:00-07:00', '2016-08-10T06:55:07+09:00', '2025-03-22T06:00:00Z', '2018-05-01T07:43:00+07:00', '2020-10-15', '2023-03-17', '2021-06-19T23:20:00-07:00', '2023-08-29T02:30:00-04:00', '2024-10-05T11:48:00+05:30', '2020-10-19T21:49:00-07:00', '2024-07-29 20:29:25Asia/Karachi', '2025-02-08T21:40:00+01:00', '2021-09-22T17:08:09+01:00', '2019-03-18T07:53:47.764Z', '2024-01-23T14:59:14.000Z', '2024-02-20 18:43:34', '2017-01-06T08:34:00.000Z', '2017-05-18T16:19', '2022-10-12T20:15:29.000Z', '2012-10-26T22:03:00+02:00', '2016-04-20T08:33:22.711Z', '2015-12-23T00:14:00.000Z', '2021-09-12T17:25:22.682Z', '2014-06-26', '2025-05-17T04:43:35+05:30', '2018-08-29T11:31:03', '2023-04-15T20:56:00-07:00', '2024-02-26T10:46:52.685Z', '2025-06-07T13:58:26.519272', '2024-01-30T09:00:13+01:00', '2020-12-23T19:34:50-03:00', '2020-11-05', '2014-05-09T15:30:43Z', '2017-02-25T08:18:07.496Z', '2016-01-01T04:57:00+02:00', "<!--Can't find substitution for tag [post.timestamp]-->", '2025-05-14T11:36:00.000Z', '2023-03-29T06:13:00+02:00', '2025-05-17T03:00:47-07:00', '2024-06-17T15:45:32+02:00', '2016-02-10T12:07:00+01:00', '2025-06-10T06:00:00Z', '2025-05-22T05:00:00Z', '2017-07-06T10:20:50-0400', '2018-09-29T17:48:00+02:00', '2025-01-21 18:16:00 +0300', '2024-09-01T02:29:26+09:00']

# for d in dates:
#     try:
#      print(d, "→", normalize_date(d))
#     except Exception as e :
#         print(f"error:{e}")

        

    import glob
    warc_files = glob.glob("/home/natnael/projects/lektos/src/common_crawl_2025-26_warcfiles/*.warc.gz")
    print(f"{len(warc_files[0:1])} warc file in the pipeline")
    results = process_warcs(warc_files[0:1])
    print("Pipeline finished:", results)
    


    

