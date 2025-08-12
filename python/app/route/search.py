import os
from fastapi import APIRouter, Depends,HTTPException
from google import genai
from dotenv import load_dotenv
from google.genai import types
from google import genai


from app.qdrant.client import get_qdrant_client
from app.constants import COLLECTION_NAME
from app.qdrant.helpers import similarity_search_service

search_router = APIRouter(tags=["search"])




load_dotenv()

GEMINI_API_KEY = os.getenv("GEMINI_API_KEY_1")
if not GEMINI_API_KEY :
    raise RuntimeError("Missing Gemini Api Key")

gemini_client = genai.Client(
            api_key=GEMINI_API_KEY
        )




@search_router.get("/")
async def similarity_search(query: str, qdrantclient=Depends(get_qdrant_client)):
    # user_query = clean_user_query(query=query)
    print(f"Query from api:{query}")
    embeded_query = embed_user_query(query=query,gemni_client=gemini_client)

    value=embeded_query.embeddings[0].values
    
    # print(f"Embedding result: {embeded_query}")
    return await similarity_search_service(client=qdrantclient,collection_name=COLLECTION_NAME,query=value)
     

    



def embed_user_query(query:str,gemni_client):

    try:
      
        print(f"Query from embeding function:{query}")

        result = gemini_client.models.embed_content(
            model="gemini-embedding-001",
            contents=query,
            config=types.EmbedContentConfig(output_dimensionality=768),
        )
        # print(f"Embedding result-x: {result}")

        return result
    except genai.errors.APIError as e :
    #    log here
       print(f"gemin api error has occured detail :{e}")
       raise HTTPException(status_code=502, detail="Gemini API error occured")
    except Exception as e :
        print(f"Unexpected error: {e}")
        raise HTTPException(status_code=500, detail="Internal server error")
    



   

def clean_user_query(query: str) -> str:
    q = query.strip()
    q = " ".join(q.split())
    if not q :
        raise HTTPException(status_code=400 ,detail="Search query can not  be empty")
    if len( q) > 100 :
       raise HTTPException(status_code=400 ,detail="Search token limit exceds")
    
    return q



