use lektos::utils::find_blog_url::is_blog_url;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_substack_url() {
        let accepted = [
            "https://example.substack.com/p/rust-tips",
            "http://newsletter.substack.com/p/web3-future/",
            "https://blog.example.com/p/this-is-a-post", // Custom domain
            "https://tech.substack.com/s/short-note",    // Note format
            "https://example.substack.com/?p=12345",     // Query param
            "https://subdomain.example.com/p/post-slug", // Nested subdomain
            "https://example.substack.com/p/post-title-with-numbers-123",
            "https://example.substack.com/p/post_title_with_underscores",
            "https://example.substack.com/p/post-with-hyphens",
            "https://example.substack.com/s/note-slug", // Short note
        ];

        for url in accepted {
            assert!(
                is_blog_url(url),
                "Expected {} to be a valid Substack URL",
                url
            )
        }
    }

    #[test]
    fn test_invalid_substack_urls() {
        let invalid_urls = vec![
            "https://substack.com/dashboard/",
            "https://example.substack.com/account/",
        ];

        for url in invalid_urls {
            assert!(!is_blog_url(url), "Failed invalid URL: {}", url);
        }
    }

    #[test]
    fn test_valid_wordpress_urls() {
        let valid_urls = vec![
            // Date-based formats
            "https://example.com/2024/07/18/sample-post/",
            "http://blog.example.com/2024/07/sample-post",
            // "https://news.example.com/2023/12/25/holiday-special/",

            // Custom base prefixes
            "https://example.com/blog/rust-tips/",
            "https://example.com/articles/wordpress-security",
            "https://example.com/journal/creative-writing/",
            // Query parameters
            "https://example.com/?p=12345",
            "http://example.com/?page_id=6789",
            // Numeric archives
            "https://example.com/archives/123",
            // Category-inclusive (valid when not matching anti-patterns)
            "https://example.com/technology/rust-guide/",
            "https://example.com/news/politics/update/",
            // Root-level posts
            "https://example.com/sample-post-title/",
            "http://example.com/another_post",
        ];

        for url in valid_urls {
            assert!(is_blog_url(url), "Failed valid URL: {}", url);
        }
    }

    #[test]
    fn test_invalid_wordpress_urls() {
        let invalid_urls = vec![
            // Admin and technical paths
            "https://example.com/wp-admin/",
            "https://example.com/wp-json/",
            "https://example.com/wp-login.php",
            // Archive listings
            "https://example.com/category/technology/",
            "https://example.com/tag/rust/",
            "https://example.com/author/john-doe/",
            // Search and feeds
            "https://example.com/search/?s=query",
            "https://example.com/feed/",
            "https://example.com/comments/feed/",
            // E-commerce
            "https://example.com/shop/",
            "https://example.com/product/ebook/",
            "https://example.com/checkout/",
            // User accounts
            "https://example.com/account/",
            "https://example.com/profile/",
            "https://example.com/dashboard/",
            "https://example.com/?p=",
            // Media attachments
            "https://example.com/?attachment_id=123",
            "https://example.com/wp-content/uploads/file.pdf",
        ];

        for url in invalid_urls {
            assert!(!is_blog_url(url), "Failed invalid URL: {}", url);
        }
    }

    #[test]
    fn test_valid_medium_urls() {
        let valid_urls = vec![
            // Standard user posts
            "https://medium.com/@username/rust-tips-123abc",
            "http://medium.com/@another_user/this-is-a-post",
            "https://medium.com/@user/post-title-with-hyphens",
            // Publication posts
            "https://medium.com/publication-name/post-title-123def",
            "https://medium.com/tech-blog/why-rust-is-awesome-456ghi",
            // Short-form posts (p/ format)
            "https://medium.com/p/1a2b3c4d5e",
            "http://medium.com/p/9f61864ab26",
            // Custom domains
            "https://blog.example.com/@user/my-post",
            "http://news.example.com/p/12345",
            // Posts with hex IDs
            "https://medium.com/swlh/why-i-stopped-using-x-a3340bb23c1d",
            "https://medium.com/geek-culture/the-problem-with-y-ef2f72f0eb4a",
        ];

        for url in valid_urls {
            assert!(is_blog_url(url), "Valid Medium URL failed: {}", url);
        }
    }
    #[test]
    fn test_invalid_medium_urls() {
        let invalid_urls = vec![
            // Admin and technical paths
            "httsps://medium.com/me",
            // "https://medium.com/me/stories",
            "https://medium.com/m/?source=main_menu",
            // Listing pages
            "https://medium.com/tag/technology",
            "https://medium.com/search?q=rust",
            // // User profiles
            "https://medium.com/@username",
            // Publication pages
            "https://medium.com/publication-name",
            "https://medium.com/publication-name/about",
            // Incomplete URLs
            "https://medium.com/p/",
            "https://medium.com/p/?source=featured",
            "https://medium.com/@username/",
            // Media and other non-post pages
            "https://medium.com/media/123abc",
            "https://medium.com/help",
            "https://mediums.com/policy",
            "https://medium.com/subscribe",
        ];

        for url in invalid_urls {
            assert!(!is_blog_url(url), "Invalid Medium URL passed: {}", url);
        }
    }
}
