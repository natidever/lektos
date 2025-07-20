#[derive(Debug, Default)]
pub struct Blog {
    pub title: String,
    pub author: String,

    pub date: String,
    pub publisher: String,
    pub content: String,
}
impl Blog {
    pub fn to_embedding_text(&self) -> String {
        // Prioritize semantic structure
        format!(
            "Title: {}\nAuthor: {}\nDate: {}\nContent: {}",
            self.title, self.author, self.date, self.content
        )
    }
}
