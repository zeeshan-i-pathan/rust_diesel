use crate::{
    models::{NewPost, Post},
    schema::posts::dsl::*,
};
use diesel::prelude::*;
use diesel::{associations::HasTable, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub struct PostRepository {
    connection: PgConnection,
}

impl PostRepository {
    pub fn new(connection: PgConnection) -> Self {
        Self { connection }
    }

    pub fn find_all(&mut self) -> Result<Vec<Post>, diesel::result::Error> {
        posts.load::<Post>(&mut self.connection)
    }

    pub fn find_by_id(&mut self, _id: i32) -> Result<Post, diesel::result::Error> {
        posts.find(_id).first(&mut self.connection)
    }

    pub fn create(&mut self, post: NewPost) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts)
            .values(&post)
            .get_result(&mut self.connection)
    }

    pub fn update(&mut self, _id: i32, post: Post) -> Result<Post, diesel::result::Error> {
        diesel::update(posts.filter(id.eq(_id)))
            .set(post)
            .get_result(&mut self.connection)
    }

    pub fn delete(&mut self, _id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(posts.filter(id.eq(_id))).execute(&mut self.connection)
    }
}
