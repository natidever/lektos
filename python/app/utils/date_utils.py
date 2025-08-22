from dateutil import parser
from datetime import datetime, timezone
from zoneinfo import ZoneInfo
import re


def normalize_date(raw_date: str) -> str:
    """
    Here I'll Normalize any human readable or machine date string to RFC 3339 with timezone info.
    sicne qdrant accept RFC3339  there is another date normalization you can in the doc(Blog/date/normalization)

    Handles:
    - Naive datetime → assumes UTC
    - Datetime with numeric offset → preserved
    - UTC 'Z' → preserved
    - IANA timezone names → localized properly
    - Common human-readable formats
    NOTE:date is crutial but the blog is more crutial this implementation let date to be empty if we are unable to parse it to RFC3339
         because here we are assuming everything  reachs here is a blog .
    returns:
        RFC 3339 string (example '2024-08-19T14:00:00Z' or '2024-08-19T14:00:00-06:00')
    """
    raw_date = raw_date.strip()

    # First, check for IANA timezone names attached without separator
    # Example: '2023-01-26 12:06:13America/Costa_Rica'
    iana_match = re.match(r"(.+?)([A-Za-z]+/[A-Za-z_]+)$", raw_date)
    if iana_match:
        dt_str, tz_str = iana_match.groups()
        try:
            dt = parser.parse(dt_str)  # naive datetime
            dt = dt.replace(tzinfo=ZoneInfo(tz_str))  # apply timezone
        except Exception as e:
            return "Unkown"
    else:
        # Normal parse for other formats
        try:
            dt = parser.parse(raw_date)
            # If naive, assume UTC
            if dt.tzinfo is None:
                dt = dt.replace(tzinfo=timezone.utc)
        except Exception as e:
            return "Unknown"

    # Convert to RFC 3339 string, use 'Z' if UTC
    try:
        rfc3339_str = dt.isoformat().replace("+00:00", "Z")
    except Exception as e:
        dt = "Unknown"
    return rfc3339_str
