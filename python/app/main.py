from lektos import MetadataPipeline
import asyncio
from app.qdrant.client import get_qdrant_client



async def main():

        client =await get_qdrant_client()
        result = await client.collection_exists("lblogs")

        print(f"does lblog exists:{result}")
 
    
  



  

if __name__ == "__main__":
    asyncio.run(main())
