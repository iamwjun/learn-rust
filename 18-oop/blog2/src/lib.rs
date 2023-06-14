pub struct Post {
    content: String
}

pub struct DraftPost {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_text() {
        let mut post = Post::new();

        post.add_text("test add text");

        assert_eq!(post.content, "test add text");
    }

    #[test]
    fn test_request_review() {
        let mut post = Post::new();

        post.add_text("test request review");

        let post = post.request_review();

        assert_eq!(post.content, "test request review");
    }

    #[test]
    fn test_approve() {
        let mut post = Post::new();

        post.add_text("test approve");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!(post.content, "test approve");
    }
}
