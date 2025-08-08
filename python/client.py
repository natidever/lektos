
import os
from qdrant_client import AsyncQdrantClient

from dotenv import load_dotenv


_client = None

def get_qdrant_clinet()->AsyncQdrantClient:
    global _client
    load_dotenv()
    qdrant_url=os.getenv("QUADRANT_URL")
    qdrant_api_key = os.getenv("QUADRANT_API_KEY")
    _client = AsyncQdrantClient(url=qdrant_url,api_key=qdrant_api_key)

    return _client

    