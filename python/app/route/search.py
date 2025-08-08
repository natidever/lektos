
from fastapi import  APIRouter ,Depends

from app.qdrant.client import get_qdrant_client

search_router = APIRouter(tags=["search"])

@search_router.get("/{query}")
async def similarity_search(query:str,session=Depends(get_qdrant_client)):
    # embeding=http_clinet.get_embeding("query")
    # similar_result = qdrantsearch(embeding)
    # return similar result 
    pass



async def embed_user_query(query:str):
    pass




