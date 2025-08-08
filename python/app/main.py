from lektos import MetadataPipeline
import asyncio
from app.qdrant.client import get_qdrant_client
from app.qdrant.helpers import default_feed, index
from app.constants import COLLECTION_NAME


async def main():
    client = await get_qdrant_client()
    created_index_result = await index(
        client=client,
          collection_name=COLLECTION_NAME,
            field_schema="datetime",
            field_name = "date"
    )
    # result = await default_feed(client)
    print(f"index result:{created_index_result}")


if __name__ == "__main__":
    asyncio.run(main())
