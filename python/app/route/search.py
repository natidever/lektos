import os
from fastapi import APIRouter, Depends
from google import genai
from dotenv import load_dotenv
from google.genai import types
from google import genai

from app.qdrant.client import get_qdrant_client

search_router = APIRouter(tags=["search"])


@search_router.get("/{query}")
async def similarity_search(query: str, client=Depends(get_qdrant_client)):
    user_query = clean_user_query(query=query)
    embeded_query = embed_user_query(query=user_query)

    return embeded_query

    



def embed_user_query(query:str):
    load_dotenv()
    gemi_api_key = os.getenv("GEMINI_API_KEY_1")
    gemini_client = genai.Client(
        api_key=gemi_api_key
    )
    result = gemini_client.models.embed_content(
        model="gemini-embedding-001",
        contents=query,
        config=types.EmbedContentConfig(output_dimensionality=768),
    )

    return result

   

def clean_user_query(query: str) -> str:
    q = query.strip()
    q = " ".join(q.split())
    q = q[:100]
    return q



