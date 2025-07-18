use once_cell::sync::Lazy;
use regex::Regex;


static BLOG_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| vec!
    [
    // Platform-specific patterns
    Regex::new(r"(?:^|\.)wordpress\.com(/[^/]+)?$").unwrap(),
    Regex::new(r"/\d{4}/\d{2}/[a-z0-9-]+\.html?").unwrap(),  // Date-based slugs
    Regex::new(r"/(?:post|article)s?/\d+/[a-z0-9-]+").unwrap(),
    Regex::new(r"\.blogspot\.[a-z]{2,3}/").unwrap(),
    
    // Content indicators
    Regex::new(r"/(?:wp-content|ghost)/").unwrap(),  // CMS fingerprints
]



);



static ANTI_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| vec![
    // Platform-agnostic anti-patterns
    Regex::new(r"/(?:account|profile|user|login|signin)/").unwrap(),  // User management
    Regex::new(r"/(?:search|explore|discover)/").unwrap(),            // Discovery pages
    Regex::new(r"/(?:docs|help|support|faq)/").unwrap(),              // Documentation
    Regex::new(r"/(?:ads?|tracking|promo)/").unwrap(),                // Ads/marketing
    Regex::new(r"/(?:api|ws|cgi-bin)/").unwrap(),                     // Technical endpoints
    Regex::new(r"/\d{8}/?$").unwrap(),                               // Date-only (e.g., /20240115/)
    Regex::new(r"\?[^=]+=(?:[0-9]+|session_id)").unwrap(),            // Tracking params
    
    // Platform-specific anti-patterns (e.g., Substack/Medium)
    Regex::new(r"/@[^/]+/(?:stats|analytics)/").unwrap(),            // Creator dashboards
    Regex::new(r"/p/\d+/edit").unwrap(),                             // Post-editing pages
]);

static BLOG_PATTEsRNS: Lazy<Vec<(&str, Regex)>> = Lazy::new(|| vec![
    // Medium: @username/post-title OR p/post-id
    ("medium.com", Regex::new(r"/(?:@[^/]+/[^/]+|p/[^/]+)$").unwrap()),
    
    // Substack: /p/post-title
    ("substack.com", Regex::new(r"/p/[^/]+$").unwrap()),
    
    // Blogger: /YYYY/MM/post-title.html
    ("blogspot.com", Regex::new(r"/\d{4}/\d{2}/[^/]+\.html$").unwrap()),
    
    // WordPress: /YYYY/MM/DD/post-title/ OR ?p=post_id
    ("wordpress.com", Regex::new(r"/(?:\d{4}/\d{2}/\d{2}/[^/]+/|\?p=\d+)").unwrap()),
    
    // Ghost: /post-title/ OR /blog/post-title
    ("ghost.io", Regex::new(r"/(?:blog/)?[^/]+/$").unwrap()),
    
    // Tumblr: /post/post-id/slug
    ("tumblr.com", Regex::new(r"/post/\d+/[^/]+$").unwrap()),
    
    // Hashnode: /post-title-{hash}
    ("hashnode.dev", Regex::new(r"/[^/]+-\w{8,}$").unwrap()),
    
    // Dev.to: /username/slug
    ("dev.to", Regex::new(r"/[^/]+/[^/]+$").unwrap()),
]);


pub fn is_blog_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    
   

    // 2. Anti-pattern safety net
    if ANTI_PATTERNS.iter().any(|re| re.is_match(&url_lower)) {
        return false;
    }

    // 3. Positive pattern matching
    let positive_match = BLOG_PATTERNS.iter().any(|re| re.is_match(&url_lower));
    
    // 4. Semantic path analysis
    let path_segments: Vec<&str> = url_lower.split('/').collect();
    let last_segment = path_segments.last().unwrap_or(&"");
    let is_blog_like_path = path_segments.contains(&"blog") ||
        last_segment.ends_with(".html") ||
        (last_segment.contains('-') && !last_segment.contains('.'));

    // 5. TLD reinforcement
    let is_common_blog_tld = url_lower.ends_with(".blog") || 
        url_lower.contains(".blogspot.");

    positive_match || (is_blog_like_path && !is_common_blog_tld) || is_common_blog_tld
}



// fn is_blog_url(url: &str) -> bool {
//     let url_lower = url.to_lowercase();
    
//     // Domain-based matching (more reliable than path-based)
//     let is_target_domain = 
//         url_lower.contains(".medium.com") ||
//         url_lower.contains(".substack.com") ;
//         // url_lower.contains(".tumblr.com") ||
//         url_lower.contains(".blogspot.com") ;
//         // // url_lower.contains(".wordpress.com") ||
//         // url_lower.contains(".ghost.io") ||
//         url_lower.contains("blogger.com") ;
//         // // url_lower.contains(".hashnode.dev") ||
//         // // url_lower.contains(".dev.to") ||
//         // // url_lower.contains(".wixsite.com") ||
//         // // url_lower.contains(".weebly.com");
//         // url_lower.contains("/blog/");


//     // Path-based exclusions
//     let is_excluded_path = 
//         url_lower.contains("/search") ||
//         url_lower.contains("/tag/") ||
//         url_lower.contains("/category/") ||
//         url_lower.contains("/author/") ||
//         url_lower.contains("/page/") ||
//         url_lower.contains("/wp-json") ||
//         url_lower.contains("/feed")||
//         url_lower.contains("/comments")||
//         url_lower.contains("/support")||
//         url_lower.contains("/home");
//         url_lower.contains("/account");


       




//     is_target_domain && !is_excluded_path
//     //    is_target_domain
// }