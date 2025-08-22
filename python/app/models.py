from typing import List
from pydantic import BaseModel, field_validator

from app.utils.date_utils import normalize_date


class StoredBlog(BaseModel):
    id: str
    content: List[float]
    url: str
    image_url: str
    title: str
    date: str
    author: str
    publisher: str

    @field_validator("date", mode="before")
    def parse_date(cls, v):
        if isinstance(v, str):
            try:
                return normalize_date(v)
            except Exception as e:
                return "Unknown"
        return v
