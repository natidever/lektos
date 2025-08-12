from qdrant_client import AsyncQdrantClient
from qdrant_client.http import models

from app.constants import COLLECTION_NAME


async def default_feed(client: AsyncQdrantClient):
    date_filter = models.FieldCondition(
        key="date",
        range=models.DatetimeRange(
            gt="2021-11-28T21:10:33+00:00",
            gte=None,
            lt=None,
            lte="2022-11-22T15:25:41.533Z",
        ),
    )

    result = await client.scroll(
        collection_name=COLLECTION_NAME,
        scroll_filter=models.Filter(must=[date_filter]),
        limit=5,
    )

    return result


async def index(
    client: AsyncQdrantClient, collection_name: str, field_name: str, field_schema: str
):
    return await client.create_payload_index(
        collection_name=collection_name,
        field_name=field_name,
        field_schema=field_schema,
    )


async def similarity_search_service(
    client: AsyncQdrantClient, collection_name: str, query: list[float]
):
    return await client.query_points(
        collection_name=collection_name,
        query=query,
    )
