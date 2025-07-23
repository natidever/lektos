use ammonia::{Builder, Url, UrlRelative};
use scraper::{Html, Selector};
use std::collections::HashSet;

// Todo:Implement base url for link
pub struct BlogProcessor;

impl BlogProcessor {
    pub fn extract_and_sanitize(html: &str) -> String {
        let content_html = Self::extract_main_content(html);

        Self::sanitize_content(&content_html)
    }

    fn extract_main_content(html: &str) -> String {
        let document = Html::parse_document(html);

        let semantic_selectors = [
            "article",
            "main",
            "[role='main']",
            ".post-content",
            ".article-body",
            ".entry-content",
        ];

        for selector in &semantic_selectors {
            if let Ok(sel) = Selector::parse(selector) {
                if let Some(content) = document.select(&sel).next() {
                    return content.html();
                }
            }
        }

        if let Ok(body_sel) = Selector::parse("body") {
            if let Some(body) = document.select(&body_sel).next() {
                return body.html();
            }
        }

        html.to_string()
    }

    fn sanitize_content(html: &str) -> String {
        let allowed_tags: HashSet<&str> = [
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "p",
            "blockquote",
            "ul",
            "ol",
            "li",
            "strong",
            "em",
            "b",
            "i",
            "u",
            "a",
            "br",
            "hr",
        ]
        .iter()
        .cloned()
        .collect();

        let allowed_attrs: HashSet<&str> = ["href", "title"].iter().cloned().collect();

        Builder::new()
            .tags(allowed_tags)
            .generic_attributes(allowed_attrs)
            .link_rel(Some("noopener nofollow"))
            .url_relative(UrlRelative::RewriteWithBase(
                Url::parse("https://default-base.com/").unwrap(),
            ))
            .clean(html)
            .to_string()
    }
}
