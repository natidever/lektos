use once_cell::sync::Lazy;
use regex::Regex;

static BLOG_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // Platform-specific patterns
        Regex::new(r"^/(?:@[^/]+/[^/]+|[^/]+/[^/]+(?:-[\da-f]{12})?|p/[^/]+)$").unwrap(), // Medium: @username/post-title OR p/post-id
        Regex::new(r"(?:^|\.)wordpress\.com(/[^/]+)?$").unwrap(),
        Regex::new(r"/\d{4}/\d{2}/[a-z0-9-]+\.html?").unwrap(), // Date-based slugs
        Regex::new(r"/(?:post|article)s?/\d+/[a-z0-9-]+").unwrap(),
        Regex::new(r"\.blogspot\.[a-z]{2,3}/").unwrap(),
        // Content indicators
        Regex::new(r"/(?:wp-content|ghost)/").unwrap(), // CMS fingerprints
    ]
});

static ANTI_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // Platform-agnostic anti-patterns
        Regex::new(r"/(?:account|profile|user|login|signin)/").unwrap(), // User management
        Regex::new(r"/(?:search|explore|discover)/").unwrap(),           // Discovery pages
        Regex::new(r"/(?:docs|help|support|faq)/").unwrap(),             // Documentation
        Regex::new(r"/(?:ads?|tracking|promo)/").unwrap(),               // Ads/marketing
        Regex::new(r"/(?:api|ws|cgi-bin)/").unwrap(),                    // Technical endpoints
        Regex::new(r"/\d{8}/?$").unwrap(), // Date-only (e.g., /20240115/)
        Regex::new(r"\?[^=]+=(?:[0-9]+|session_id)").unwrap(), // Tracking params
        // Platform-specific anti-patterns (e.g., Substack/Medium)
        Regex::new(r"/@[^/]+/(?:stats|analytics)/").unwrap(), // Creator dashboards
        Regex::new(r"/p/\d+/edit").unwrap(),                  // Post-editing pages
    ]
});

static BLOG_PATTEsRNS: Lazy<Vec<(&str, Regex)>> = Lazy::new(|| {
    vec![
        // Medium: @username/post-title OR p/post-id
        (
            "medium.com",
            Regex::new(r"^/(?:@[^/]+/[^/]+|[^/]+/[^/]+(?:-[\da-f]{12})?|p/[^/]+)$").unwrap(),
        ),
        // Substack: /p/post-title
        // ("substack.com", Regex::new(r"/p/[^/]+$").unwrap()),

        // // Blogger: /YYYY/MM/post-title.html
        // ("blogspot.com", Regex::new(r"/\d{4}/\d{2}/[^/]+\.html$").unwrap()),

        // // WordPress: /YYYY/MM/DD/post-title/ OR ?p=post_id
        // ("wordpress.com", Regex::new(r"/(?:\d{4}/\d{2}/\d{2}/[^/]+/|\?p=\d+)").unwrap()),

        // // Ghost: /post-title/ OR /blog/post-title
        // ("ghost.io", Regex::new(r"/(?:blog/)?[^/]+/$").unwrap()),

        // // Tumblr: /post/post-id/slug
        // ("tumblr.com", Regex::new(r"/post/\d+/[^/]+$").unwrap()),

        // // Hashnode: /post-title-{hash}
        // ("hashnode.dev", Regex::new(r"/[^/]+-\w{8,}$").unwrap()),

        // // Dev.to: /username/slug
        // ("dev.to", Regex::new(r"/[^/]+/[^/]+$").unwrap()),
    ]
});

// pub fn is_blog_url(url: &str) -> bool {
//     let url_lower = url.to_lowercase();

//     // 2. Anti-pattern safety net
//     // if ANTI_PATTERNS.iter().any(|re| re.is_match(&url_lower)) {
//     //     return false;
//     // }

//     // 3. Positive pattern matching
//     let positive_match = BLOG_PATTERNS.iter().any(|re| re.is_match(&url_lower));

//     positive_match
// }

// static MEDIUM_REGEX: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r"(?x)
//         ^https://medium\.com/
//         (?:
//             @[^/]+/[\w-]+                # User posts (@username/slug)
//             |[\w-]+/[\w-]+(?:-[\da-f]{12})? # Publication posts (pub/slug or pub/slug-hexid)
//             |p/[\w-]+                     # Short-form posts (p/postid)
//         )$
//     ").unwrap()
// });

static WORDPRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?xi)
        ^https?://
        [^/]+
        (?:/[^/]+)*
        /
        (?:
            \d{4}/\d{1,2}(?:/\d{1,2})?/[^/?]+
            |
            (?:blog|news|articles|journal)/[^/?]+
            |
            archives/\d+
            |
            \?(?:p|page_id)=\d+[^/?]*
            |
            [^/?]+/[^/?]+
            |
            [^/?]+
        )
        /?$
    "#,
    )
    .expect("Invalid WordPress regex pattern")
});

static WORDPRESS_ANTI_PATTERNS: [&str; 18] = [
    "wp-admin",
    "wp-json",
    "wp-login.php",
    "/category/",
    "/tag/",
    "/author/",
    "/search/",
    "/feed/",
    "/comments/feed",
    "/shop/",
    "/product/",
    "/checkout/",
    "/account/",
    "/profile/",
    "/dashboard/",
    "?attachment_id=",
    "wp-content/uploads",
    "/wp-",
];

pub fn is_wordpress_blog(url: &str) -> bool {
    WORDPRESS_REGEX.is_match(url)
        && !WORDPRESS_ANTI_PATTERNS
            .iter()
            .any(|pattern| url.contains(pattern))
}

static SUBSTACK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?xi)
        ^https?://
        (?: 
            [^/]+\.substack\.com  # Standard subdomains
            |
            [^/]+                # Custom domains
        )
        /
        (?:
            p/[^/?]+[^/]          # Publication posts (must have slug)
            |
            s/[^/?]+[^/]          # Notes (must have slug)
            |
            \?p=\d+[^/]*          # Query parameters (must have ID)
        )
        /?$
    "#,
    )
    .expect("Invalid Substack regex pattern")
});

static SUBSTACK_ANTI_PATTERNS: [&str; 8] = [
    "/dashboard",
    "/account",
    "/search",
    "/archive",
    "/podcast",
    "/p/", // Empty post slug
    "/s/", // Empty note slug
    "?p=", // Empty post ID
];

pub fn is_substack_blog(url: &str) -> bool {
    SUBSTACK_REGEX.is_match(url)
        && !SUBSTACK_ANTI_PATTERNS
            .iter()
            .any(|pattern| url.contains(pattern))
}

static MEDIUM_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?xi)
        ^https?://
        (?:[^/]+\.)?medium\.com  # Main domain or subdomain
        (?:/[^/]+)*              # Optional path segments
        /
        (?:
            @[^/]+/[^/]+         # User posts (@username/slug)
            |
            [^/]+/[^/]+(?:-[a-f0-9]{12})?  # Publication posts (optional hex ID)
            |
            p/[a-f0-9]+          # Short-form posts (hex ID only)
        )
        /?$
    "#,
    )
    .expect("Invalid Medium regex pattern")
});

static MEDIUM_ANTI_PATTERNS: [&str; 16] = [
    // Added @username case
    "/me",
    "/me/",
    "/m/",
    "/_/",
    "/tag/",
    "/topic/",
    "/search",
    "/@",
    "/lists",
    "/about",
    "/media/",
    "/help",
    "/policy",
    "/subscribe",
    "/p/$",
    "/@[^/]+$",
];

pub fn is_medium_blog(url: &str) -> bool {
    // First check if it matches Medium post pattern
    let is_medium_post = MEDIUM_REGEX.is_match(url);

    // Then verify it doesn't contain any anti-patterns
    let has_anti_pattern = MEDIUM_ANTI_PATTERNS.iter().any(|pat| url.contains(pat));

    is_medium_post && !has_anti_pattern
}

pub fn is_blog_url(url: &str) -> bool {
    is_wordpress_blog(url) || is_substack_blog(url) || is_medium_blog(url)
    // MEDIUM_REGEX.is_match(url)
}

// Intelegent URL Filtering for blogs how to intellegentlyfilter url that are used for blogs we are int

//
