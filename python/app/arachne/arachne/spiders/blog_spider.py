from pathlib import Path

import scrapy
from scrapy.linkextractors import LinkExtractor
import lektos
from scrapy.exceptions import CloseSpider 
"""Assumtion is is_blog() is smart enough to detect if there is a blog in there 
let me design to get html ->run the extractor on them

"""



class QuotesSpider(scrapy.Spider):
    custom_settings = {
        'DEPTH_LIMIT': 1,
        'FEEDs':{
           'validated_urls.jl':{
               'format': 'jsonlines',
                'overwrite': True 
              
           }
        }
    }
    link_extractor = LinkExtractor()
    result= []


    name = "blogs"
    url_limit=10
    counter=0

    async def start(self):
        urls = [
            "http://quotes.toscrape.com/"
        ]
        for url in urls:
            yield scrapy.Request(url=url, callback=self.parse)

    def parse(self, response):
     for link in self.link_extractor.extract_links(response):
        self.counter+=1
        
        if "speechify" in link.url :
           print("speechify found ")
           continue
        if self.counter>=self.url_limit:
           CloseSpider(reason="Limi reached")
          
 
        if lektos.is_blog(link.url) :
           yield {
              'url':link.url
           }
           
       
        
        #IS BLOG()->EXTRACT()=>Send it to PIPELINE(Kafka or rabbit may be )
        yield scrapy.Request(link.url, callback=self.parse)
    







   




