pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: "".to_string(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().enable_add_text() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        self.state = Some(self.state.take().unwrap().request_review());
    }

    pub fn approve(&mut self) {
        self.state = Some(self.state.take().unwrap().approve());
    }

    pub fn reject(&mut self) {
        self.state = Some(self.state.take().unwrap().reject());
    }
}

pub trait State {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn enable_add_text(&self) -> bool {
        false
    }
}

struct Draft;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview)
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn enable_add_text(&self) -> bool {
        true
    }
}

struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(SingleApproved)
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft)
    }
}

struct SingleApproved;

impl State for SingleApproved {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published)
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft)
    }
}

struct Published;

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
