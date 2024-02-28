fn main() {
    {
        println!("*** *** *** ***");
        let mut post = oop::post1::Post::new();
        println!("1 post.content = {:?}", post.content());
        post.add_text("Hello, World!");
        println!("2 post.content = {:?}", post.content());
        post.approve();
        println!("3 post.content = {:?}", post.content());
        post.request_review();
        println!("4 post.content = {:?}", post.content());
        post.approve();
        println!("5 post.content = {:?}", post.content());
        post.request_review();
        println!("6 post.content = {:?}", post.content());
    }

    {
        println!("*** *** *** ***");
        let mut draft = oop::post2::Post::new();
        println!("draft: {:?}", draft);
        draft.add_text("Hello, World!");
        println!("draft: {:?}", draft);
        let pending_review_post = draft.request_review();
        // println!("draft: {:?}", draft); // borrow of moved value: `draft`
        println!("pending_review_post: {:?}", pending_review_post);
        let post = pending_review_post.approve();
        // println!("pending_review_post: {:?}", pending_review_post); // borrow of moved value: `pending_review_post`
        println!("post: {:?}", post);
        println!("post content: {:?}", post.content());
        println!("post: {:?}", post);
    }
}
