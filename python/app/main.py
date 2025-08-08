# from lektos import MetadataPipeline
import asyncio

from fastapi import FastAPI
from app.qdrant.client import get_qdrant_client
from app.qdrant.helpers import default_feed, index, similarity_search
from app.constants import COLLECTION_NAME
from app.qdrant.embedings import TEST_EMBEDING
from app.route.search import search_router

app = FastAPI()  

app.include_router(search_router)



async def main():
    try:
        client = await get_qdrant_client()
        created_index_result = await similarity_search(
            client=client,
            collection_name=COLLECTION_NAME,
            query=TEST_EMBEDING
        
        )
    # result = await default_feed(client)
        print(f"index result:{created_index_result}")
    except Exception as e :
      print(f"Error occured :{e}")
      raise 


if __name__ == "__main__":
    asyncio.run(main())



""" Accept user prompt then embed it then pass it to as qury"""