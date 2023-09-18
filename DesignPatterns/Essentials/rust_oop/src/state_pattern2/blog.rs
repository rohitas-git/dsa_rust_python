/**------------------------------------------------------------------------
 *                  Encoding States and Behavior as Types 
 *------------------------------------------------------------------------**/

pub fn call_main(){
    let mut post = Post::new();

    post.add_text("I woke up at 5am");

    let post = post.request_review();
    
    let post = post.approve();
    assert_eq!("I woke up at 5am", post.content());
}

pub struct Post{
    content: String
}
impl Post{
    pub fn new() -> DraftPost{
        DraftPost{
            content: String::new()
        }
    }

    pub fn content(&self) -> &str{
        &self.content
    }
}

pub struct DraftPost{
    content: String
}
impl DraftPost{
    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost{
        PendingReviewPost{
            content: self.content
        }
    }
}

pub struct PendingReviewPost{
    content: String
}
impl PendingReviewPost{
    pub fn approve(self) -> Post{
        Post{
            content: self.content,
        }
    }
}
