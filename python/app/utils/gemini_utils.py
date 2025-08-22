import os
import types

from google import genai
from google.genai import types
from dotenv import load_dotenv


def embed(text: str):
    # if we're going to use gemni for embeding with 768 size, batching
    # has no meaning we'll hit the rate limit wiht just one batch

    load_dotenv()
    GEMNI_API_KEY = os.getenv("GEMINI_API_KEY_6")
    http_options = types.HttpOptions(
        async_client_args={},
    )

    client = genai.Client(api_key=GEMNI_API_KEY, http_options=http_options)

    result = client.models.embed_content(
        model="gemini-embedding-001",
        contents=text,
        config=types.EmbedContentConfig(
            output_dimensionality=768, task_type="SEMANTIC_SIMILARITY"
        ),
    )

    return result.embeddings[0].values
