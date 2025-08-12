# from lektos import MetadataPipeline
import asyncio
from contextlib import asynccontextmanager

from fastapi import Depends, FastAPI
import httpx
from app.qdrant.client import get_qdrant_client
from app.qdrant.helpers import default_feed, index
from app.constants import COLLECTION_NAME
from app.qdrant.embedings import TEST_EMBEDING
from app.route.search import embed_user_query, search_router


@asynccontextmanager
async def lifespan(app: FastAPI):
    app.state.http_client = httpx.AsyncClient(timeout=30)
    yield

    await app.state.http_client.aclose()


app = FastAPI(lifespan=lifespan)

app.include_router(search_router,prefix="/search")


@app.get("/")
async def root(qdrant_client=Depends(get_qdrant_client)):
    return await default_feed(client = qdrant_client)
    # return "Server running"



@app.get("/health-check")
def health_check():
    embed_user_query("hy")
    return "Server running"

""" Accept user prompt then embed it then pass it to as qury"""
