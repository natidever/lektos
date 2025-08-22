from typing import List
from dotenv import load_dotenv
from qdrant_client import AsyncQdrantClient
import os
from app.models import StoredBlog
from app.constants import COLLECTION_NAME
from qdrant_client import AsyncQdrantClient
from qdrant_client.models import PointStruct
from qdrant_client.http.exceptions import ApiException
from app.utils.logging_utils import logger


async def store_worker(blogs: List[StoredBlog]):
    """Store blogs in Qdrant with error handling"""
    load_dotenv()

    QDRANT_API_KEY = os.getenv("QUADRANT_API_KEY")
    QDRANT_URL = os.getenv("QUADRANT_URL")

    try:
        client = AsyncQdrantClient(
            url=QDRANT_URL, api_key=QDRANT_API_KEY, prefer_grpc=True
        )

        # Convert blogs to points
        points = [create_point(blog) for blog in blogs]

        # Upsert to Qdrant
        update_response = await client.upsert(
            collection_name=COLLECTION_NAME, points=points
        )

        return update_response

    except ApiException as e:
        logger.error(f"Qdrant API error while storing points: {e}")
        raise RuntimeError(f"Qdrant ApiException: {str(e)}")
    except Exception as e:
        logger.error(f"Unexpected error in store_worker: {e}")
        raise


def create_point(blog: StoredBlog) -> PointStruct:
    return PointStruct(
        id=blog.id,
        vector=blog.content,
        payload={
            "author": blog.author,
            "title": blog.title,
            "date": blog.date,
            "publisher": blog.publisher,
            "image_url": blog.image_url,
            "url": blog.url,
        },
    )
