use std::marker::PhantomData;
use swagger::{Has, XSpanIdString};
use async_trait::async_trait;

use generated::models::{self, BookmarkPostOkResponse};

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
    BookmarksGetResponse, BookmarksPostResponse,
};
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn bookmarks_get(&self, _: &C) -> Result<BookmarksGetResponse, ApiError>
    {
        return Ok(BookmarksGetResponse::OkResponse(
            models::BookmarkGetOkResponse{ 
            bookmark_directory_list: vec![]
        }));
    }

    async fn bookmarks_post(&self, 
        bookmark_post_request_body: models::BookmarkPostRequestBody,
        _: &C) -> Result<BookmarksPostResponse, ApiError>
    {
        return Ok(BookmarksPostResponse::OkResponse(
            models::BookmarkPostOkResponse { 
                message: "ok".to_string() 
            }
        ));
    }

}
