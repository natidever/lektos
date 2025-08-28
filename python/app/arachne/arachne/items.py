# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


class BlogItem(scrapy.Item):
    """Item for storing extracted blog data"""
    url = scrapy.Field()
    title = scrapy.Field()
    author = scrapy.Field()
    date = scrapy.Field()
    publisher = scrapy.Field()
    description = scrapy.Field()
    image_url = scrapy.Field()
    content = scrapy.Field()
    extraction_timestamp = scrapy.Field()
    
    # Metadata about extraction confidence/source
    title_source = scrapy.Field()
    author_source = scrapy.Field()
    date_source = scrapy.Field()
    publisher_source = scrapy.Field()
    description_source = scrapy.Field()


class ArachneItem(scrapy.Item):
    # define the fields for your item here like:
    # name = scrapy.Field()
    pass
