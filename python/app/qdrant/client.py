import os
from qdrant_client import AsyncQdrantClient

from dotenv import load_dotenv
from qdrant_client.http.exceptions import UnexpectedResponse


_client = None


async def get_qdrant_client() -> AsyncQdrantClient:
    try:
        global _client

        if _client is not None:
            return _client
        load_dotenv()
        qdrant_url = os.getenv("QUADRANT_URL")
        qdrant_api_key = os.getenv("QUADRANT_API_KEY")

        _client = AsyncQdrantClient(
            url=qdrant_url, api_key=qdrant_api_key, prefer_grpc=True
        )

        return _client
    except UnexpectedResponse as e:
        print(f"unexpected response :{e.status_code}")
        raise
    except Exception as e:
        print(f"Failed to initialize client: {type(e).__name__}: {e}")
        raise
