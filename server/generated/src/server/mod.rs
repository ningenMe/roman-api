use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::models;

use crate::{Api,
     BookmarksGetResponse,
     BookmarksPostResponse
};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: Api + 'static,
{
    // build our application with a route
    Router::new()
        .route("/bookmarks",
            get(bookmarks_get::<I, A>).post(bookmarks_post::<I, A>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn bookmarks_get_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}

/// BookmarksGet - GET /bookmarks
#[tracing::instrument(skip_all)]
async fn bookmarks_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    bookmarks_get_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().bookmarks_get(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                BookmarksGetResponse::Status200_OkResponse
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct BookmarksPostBodyValidator<'a> {
            #[validate]
          body: &'a models::BookmarkPostRequestBody,
    }


#[tracing::instrument(skip_all)]
fn bookmarks_post_validation(
        body: models::BookmarkPostRequestBody,
) -> std::result::Result<(
        models::BookmarkPostRequestBody,
), ValidationErrors>
{
              let b = BookmarksPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}

/// BookmarksPost - POST /bookmarks
#[tracing::instrument(skip_all)]
async fn bookmarks_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::BookmarkPostRequestBody>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    bookmarks_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().bookmarks_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                BookmarksPostResponse::Status200_OkResponse
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

