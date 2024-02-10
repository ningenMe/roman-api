use async_trait::async_trait;
use axum::{http::Method, extract::Host};

use axum_extra::extract::CookieJar;
use generated::{BookmarksGetResponse, BookmarksPostResponse, models::{BookmarkGetOkResponse, BookmarkPostRequestBody, BookmarkPostOkResponse}};

#[derive(Clone)]
pub struct Api {}
impl AsRef<Api> for Api {
    #[inline]
    fn as_ref(&self) -> &Api {
        self
    }
}

#[async_trait]
impl generated::Api for Api where 
{
    async fn bookmarks_get(
        &self,
        _method:Method,
        _host:Host,
        _cookies:CookieJar,
    ) ->  Result<BookmarksGetResponse,String> {
        return Ok(BookmarksGetResponse::Status200_OkResponse(
            BookmarkGetOkResponse{ 
            bookmark_directory_list: infra::get()
        }));
    }

    async fn bookmarks_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: BookmarkPostRequestBody,
    ) -> Result<BookmarksPostResponse, String> {
        infra::set(body.bookmark_directory_list);
        return Ok(BookmarksPostResponse::Status200_OkResponse(
            BookmarkPostOkResponse { 
                message: "ok".to_string() 
            }
        ));
    }    
}
