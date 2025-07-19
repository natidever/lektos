
use lektos::utils::find_blog_url::is_blog_url;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
   fn test_valid_substack_url(){
    let accepted = [
"https://example.substack.com/p/rust-tips",
"http://newsletter.substack.com/p/web3-future/",
"https://blog.example.com/p/this-is-a-post" , // Custom domain
"https://tech.substack.com/s/short-note"  ,  // Note format
"https://example.substack.com/?p=12345"  ,   // Query param
"https://subdomain.example.com/p/post-slug" ,// Nested subdomain
"https://example.substack.com/p/post-title-with-numbers-123",
"https://example.substack.com/p/post_title_with_underscores",
"https://example.substack.com/p/post-with-hyphens",
"https://example.substack.com/s/note-slug"  ,// Short note


];

for url in accepted {
    assert!(is_blog_url(url), "Expected {} to be a valid Substack URL", url)
}


}
}
