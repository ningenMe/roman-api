use std::marker::PhantomData;
use swagger::{Has, XSpanIdString};
use async_trait::async_trait;

use generated::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}
impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}
use generated::{
    Api,
    BookmarksGetResponse,
};
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn bookmarks_get(&self, _: &C) -> Result<BookmarksGetResponse, ApiError>
    {
        return Ok(BookmarksGetResponse::Get(
            models::GetBookmarkResponse{ 
            bookmark_directory_list: vec![]
        }));
    }

}
