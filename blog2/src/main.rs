use blog2::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("This is my first post!");
    let post = post.request_review();
    let mut post = post.reject();
    post.add_text(" add more text");
    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
    assert_eq!("This is my first post! add more text", post.content());
}
