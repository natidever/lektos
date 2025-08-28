import json
import logging
from datetime import datetime
from pathlib import Path
from typing import Generator, Optional

import scrapy
from scrapy.linkextractors import LinkExtractor
from scrapy.exceptions import CloseSpider
from scrapy.http import Response

import lektos
from ..items import BlogItem

logger = logging.getLogger(__name__)

class BlogSpider(scrapy.Spider):
    name = "blogs"
    
    custom_settings = {
        'DEPTH_LIMIT': 4,
        'CONCURRENT_REQUESTS': 8,
        'CONCURRENT_REQUESTS_PER_DOMAIN': 4,
        'DOWNLOAD_DELAY': 2,
        'RANDOMIZE_DOWNLOAD_DELAY': True,
        'FEEDS': {
            'crawl_extracted_blogs.json': {
                'format': 'json',
                'overwrite': True,
                'indent': 2
            }
        },
        'HTTPERROR_ALLOW_ALL': False,
        'RETRY_TIMES': 3,
        'RETRY_HTTP_CODES': [500, 502, 503, 504, 408, 429],
        'DUPEFILTER_DEBUG': True,
        'LOG_LEVEL': 'INFO'
    }
    
    def __init__(self, start_url=None, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.start_url = start_url or "https://medium.com/vaticle/what-is-an-ontology-c5baac4a2f6c"
        self.link_extractor = LinkExtractor(
            allow_domains=None,
            deny_extensions=['pdf', 'doc', 'docx', 'xls', 'xlsx', 'ppt', 'pptx', 'zip', 'rar'],
            unique=True,
            canonicalize=True,
            strip=True,
            allow=(),
            deny=(),
            restrict_xpaths=None,
            tags=('a', 'area'),
            attrs=('href',),
            process_value=None
        )
        self.url_limit = int(getattr(self, 'url_limit', 100))
        self.counter = 0
        self.processed_blogs = 0
        self.metadata_pipeline = lektos.MetadataPipeline()
        
        self.stats = {
            'urls_crawled': 0,
            'blogs_found': 0,
            'blogs_processed': 0,
            'extraction_errors': 0
        }
        
        logger.info(f"BlogSpider initialized with URL limit: {self.url_limit}")

    def start_requests(self):
        yield scrapy.Request(
            url=self.start_url,
            callback=self.parse,
            meta={'is_seed': True},
            errback=self.handle_error
        )

    def parse(self, response: Response) -> Generator:
        self.stats['urls_crawled'] += 1
        current_url = response.url
        
        logger.debug(f"Parsing URL: {current_url}")
        
        try:
            if lektos.is_blog(current_url):
                self.stats['blogs_found'] += 1
                logger.info(f"Blog detected: {current_url}")
                
                blog_item = self.extract_blog_data(response)
                if blog_item:
                    self.stats['blogs_processed'] += 1
                    self.processed_blogs += 1
                    yield blog_item
                    
                    logger.info(f"Successfully extracted blog #{self.processed_blogs}: {current_url}")
                else:
                    logger.warning(f"Failed to extract blog data from: {current_url}")
                    self.stats['extraction_errors'] += 1
                    
        except Exception as e:
            logger.error(f"Error checking if URL is blog ({current_url}): {e}")
            self.stats['extraction_errors'] += 1

        if self.counter < self.url_limit:
            links = self.link_extractor.extract_links(response)
            logger.info(f"Found {len(links)} links on {current_url}")
            
            if links:
                logger.info(f"Sample links: {[link.url for link in links[:5]]}")
            else:
                logger.warning(f"No links found on {current_url}. Response length: {len(response.text)}")
                logger.debug(f"Response preview: {response.text[:500]}")
            
            for link in links:
                if self.counter >= self.url_limit:
                    logger.info(f"URL limit ({self.url_limit}) reached. Stopping crawl.")
                    raise CloseSpider(reason="URL limit reached")
                
                self.counter += 1
                
                if self.should_skip_url(link.url):
                    logger.debug(f"Skipping URL: {link.url}")
                    continue
                
                logger.info(f"Following link #{self.counter}: {link.url}")
                yield scrapy.Request(
                    url=link.url,
                    callback=self.parse,
                    errback=self.handle_error,
                    meta={'depth': response.meta.get('depth', 0) + 1},
                    dont_filter=False
                )
        else:
            logger.info("URL limit reached, stopping further requests")

    def extract_blog_data(self, response: Response) -> Optional[BlogItem]:
        try:
            html_content = response.text
            current_url = response.url
            
            metadata = self.metadata_pipeline.run(html_content)
            blog_content = lektos.BlogProcessor.extract_and_sanitize(html_content)
            
            item = BlogItem()
            
            item['url'] = current_url
            item['content'] = blog_content
            item['extraction_timestamp'] = datetime.now().isoformat()
            
            if metadata.title:
                item['title'] = metadata.title.value
                item['title_source'] = metadata.title.source
            else:
                item['title'] = self.extract_fallback_title(response)
                item['title_source'] = 'fallback'
                
            if metadata.author:
                item['author'] = metadata.author.value
                item['author_source'] = metadata.author.source
            else:
                item['author'] = 'Unknown'
                item['author_source'] = 'default'
                
            if metadata.date:
                item['date'] = metadata.date.value
                item['date_source'] = metadata.date.source
            else:
                item['date'] = ''
                item['date_source'] = 'none'
                
            if metadata.publisher:
                item['publisher'] = metadata.publisher.value
                item['publisher_source'] = metadata.publisher.source
            else:
                item['publisher'] = self.extract_domain_from_url(current_url)
                item['publisher_source'] = 'domain'
                
            if metadata.description:
                item['description'] = metadata.description.value
                item['description_source'] = metadata.description.source
            else:
                item['description'] = self.extract_fallback_description(blog_content)
                item['description_source'] = 'content_excerpt'
                
            if hasattr(metadata, 'image_url') and metadata.image_url:
                item['image_url'] = metadata.image_url.value
            else:
                item['image_url'] = ''
            
            if not item.get('title') or not item.get('content'):
                logger.warning(f"Insufficient data extracted from {current_url}")
                return None
                
            return item
            
        except Exception as e:
            logger.error(f"Error extracting blog data from {response.url}: {e}")
            return None

    def should_skip_url(self, url: str) -> bool:
        # here we can again use the rust is_url implementation 
        skip_patterns = [
            'speechify',
            '/about',
            '/contact',
            '/privacy',
            '/terms',
    
        ]
        
        url_lower = url.lower()
        
        for pattern in skip_patterns:
            if pattern in url_lower:
                return True
                
        if len(url) > 200:
            return True
            
        return False

    def extract_fallback_title(self, response: Response) -> str:
        try:
            title = response.css('title::text').get()
            return title.strip() if title else 'Untitled'
        except:
            return 'Untitled'

    def extract_domain_from_url(self, url: str) -> str:
        try:
            from urllib.parse import urlparse
            parsed = urlparse(url)
            return parsed.netloc
        except:
            return 'Unknown'

    def extract_fallback_description(self, content: str, max_length: int = 200) -> str:
        try:
            import re
            text = re.sub(r'<[^>]+>', '', content)
            text = re.sub(r'\s+', ' ', text).strip()
            
            if len(text) <= max_length:
                return text
            
            sentences = text.split('. ')
            description = sentences[0]
            
            for sentence in sentences[1:]:
                if len(description + '. ' + sentence) <= max_length:
                    description += '. ' + sentence
                else:
                    break
                    
            return description + ('...' if len(text) > len(description) else '')
            
        except:
            return ''

    def handle_error(self, failure):
        logger.error(f"Request failed: {failure.request.url} - {failure.value}")
        self.stats['extraction_errors'] += 1

    def closed(self, reason):
        logger.info(f"Spider closed: {reason}")
        logger.info(f"Final statistics: {self.stats}")
        logger.info(f"Total blogs processed: {self.processed_blogs}")
        
        stats_file = Path('crawl_statistics.json')
        try:
            with open(stats_file, 'w') as f:
                json.dump({
                    'close_reason': reason,
                    'statistics': self.stats,
                    'blogs_processed': self.processed_blogs,
                    'timestamp': datetime.now().isoformat()
                }, f, indent=2)
            logger.info(f"Statistics saved to {stats_file}")
        except Exception as e:
            logger.error(f"Failed to save statistics: {e}")


QuotesSpider = BlogSpider
