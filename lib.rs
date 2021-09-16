// use prisma_client::{PrismaClient, User}

pub enum PostFields {
    ID
}

impl PostFields {
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "id" => Some(PostFields::ID),
            _ => None
        }
    }
}

pub mod Post {
    use super::Comment;
    
    pub trait StringField {
        fn equals(value: &str) -> PostFilter;
    }

    pub enum Unique {
        ID(String)
    }
    
    pub enum PostFilter<'a> {
        ID(IDFilter),
        Comments(&'a [Comment::CommentFilter])
    }
    
    pub enum PostSelect {
        ID
    }
    
    pub enum IDFilter {
        Equals(String)
    }
    
    impl<'a> From<IDFilter> for PostFilter<'a> {
        fn from(value: IDFilter) -> Self {
            PostFilter::ID(value)
        }
    }
    
    pub struct ID {}
    
    impl ID {
        pub fn equals(value: &str) -> PostFilter {
            IDFilter::Equals(value.into()).into()
        }
        
        pub fn unique(value: &str) -> Unique {
            Unique::ID(value.into())
        }
        
        pub fn select() -> PostSelect {
            PostSelect::ID
        }
    }
    
    pub struct Comments {}
    
    impl Comments {
        pub fn some<'a>(filters: &'a [Comment::CommentFilter]) -> PostFilter<'a> {
            PostFilter::Comments(filters)
        }
    }

    pub fn select_field(name: &str) -> Option<PostSelect> {
        match name {
            "id" => Some(ID::select()),
            _ => None
        }
    }
    
    // pub fn field(name: &str) -> 
}

pub mod Comment {
    pub enum CommentFilter {
        ID(IDFilter)
    }
    
    pub enum IDFilter {
        Equals(String)
    }
    
    impl From<IDFilter> for CommentFilter {
        fn from(value: IDFilter) -> Self {
            CommentFilter::ID(value)
        }
    }
    
    pub struct ID {}
    
    impl ID {
        pub fn equals(value: &str) -> CommentFilter {
            IDFilter::Equals(value.into()).into()
        }
    }
    
}

pub struct PostModel {}

pub struct CommentsModel {}

struct PostStuff {}

struct CommentStuff {}

impl PostStuff {
    async fn find_unique(&self, fields: &[Post::Unique]) -> Option<PostModel> {
        None
    }
    
    async fn find_first(&self, args: &[Post::PostFilter<'_>]) -> Option<PostModel> {
        None
    }
}

struct PrismaClient {
    // Engine stuff
    
    // Connection stuff
    
    // Actual struct
    post: PostStuff,
    comment: CommentStuff
}

fn test(client: &PrismaClient){
    client.post.find_unique(&[
        Post::ID::unique("1234"),
    ]).with(&[
        Post::Comments::fetch()
        .with(&[
            Comments::Post::fetch()
        ])
        .take(5)
        .skip(1)
    ]).take(2)
    .skip(1)
    .cursor()
}