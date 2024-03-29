#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
      





#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Bookmark {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "url")]
    pub url: String,

}


impl Bookmark {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, url: String, ) -> Bookmark {
        Bookmark {
            title,
            url,
        }
    }
}

/// Converts the Bookmark value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Bookmark {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("url".to_string()),
            Some(self.url.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Bookmark value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Bookmark {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub title: Vec<String>,
            pub url: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Bookmark".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "url" => intermediate_rep.url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Bookmark".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Bookmark {
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in Bookmark".to_string())?,
            url: intermediate_rep.url.into_iter().next().ok_or_else(|| "url missing in Bookmark".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Bookmark> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Bookmark>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Bookmark>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Bookmark - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Bookmark> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Bookmark as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Bookmark - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BookmarkDirectory {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "bookmarkList")]
    pub bookmark_list: Vec<models::Bookmark>,

}


impl BookmarkDirectory {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, bookmark_list: Vec<models::Bookmark>, ) -> BookmarkDirectory {
        BookmarkDirectory {
            title,
            bookmark_list,
        }
    }
}

/// Converts the BookmarkDirectory value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BookmarkDirectory {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("title".to_string()),
            Some(self.title.to_string()),

            // Skipping bookmarkList in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BookmarkDirectory value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BookmarkDirectory {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub title: Vec<String>,
            pub bookmark_list: Vec<Vec<models::Bookmark>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BookmarkDirectory".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "bookmarkList" => return std::result::Result::Err("Parsing a container in this style is not supported in BookmarkDirectory".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing BookmarkDirectory".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BookmarkDirectory {
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in BookmarkDirectory".to_string())?,
            bookmark_list: intermediate_rep.bookmark_list.into_iter().next().ok_or_else(|| "bookmarkList missing in BookmarkDirectory".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BookmarkDirectory> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<BookmarkDirectory>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BookmarkDirectory>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BookmarkDirectory - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<BookmarkDirectory> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BookmarkDirectory as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BookmarkDirectory - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BookmarkGetOkResponse {
    #[serde(rename = "bookmarkDirectoryList")]
    pub bookmark_directory_list: Vec<models::BookmarkDirectory>,

}


impl BookmarkGetOkResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(bookmark_directory_list: Vec<models::BookmarkDirectory>, ) -> BookmarkGetOkResponse {
        BookmarkGetOkResponse {
            bookmark_directory_list,
        }
    }
}

/// Converts the BookmarkGetOkResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BookmarkGetOkResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping bookmarkDirectoryList in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BookmarkGetOkResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BookmarkGetOkResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bookmark_directory_list: Vec<Vec<models::BookmarkDirectory>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BookmarkGetOkResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "bookmarkDirectoryList" => return std::result::Result::Err("Parsing a container in this style is not supported in BookmarkGetOkResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing BookmarkGetOkResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BookmarkGetOkResponse {
            bookmark_directory_list: intermediate_rep.bookmark_directory_list.into_iter().next().ok_or_else(|| "bookmarkDirectoryList missing in BookmarkGetOkResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BookmarkGetOkResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<BookmarkGetOkResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BookmarkGetOkResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BookmarkGetOkResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<BookmarkGetOkResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BookmarkGetOkResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BookmarkGetOkResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BookmarkPostOkResponse {
    #[serde(rename = "message")]
    pub message: String,

}


impl BookmarkPostOkResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(message: String, ) -> BookmarkPostOkResponse {
        BookmarkPostOkResponse {
            message,
        }
    }
}

/// Converts the BookmarkPostOkResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BookmarkPostOkResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("message".to_string()),
            Some(self.message.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BookmarkPostOkResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BookmarkPostOkResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BookmarkPostOkResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BookmarkPostOkResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BookmarkPostOkResponse {
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in BookmarkPostOkResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BookmarkPostOkResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<BookmarkPostOkResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BookmarkPostOkResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BookmarkPostOkResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<BookmarkPostOkResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BookmarkPostOkResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BookmarkPostOkResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BookmarkPostRequestBody {
    #[serde(rename = "bookmarkDirectoryList")]
    pub bookmark_directory_list: Vec<models::BookmarkDirectory>,

}


impl BookmarkPostRequestBody {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(bookmark_directory_list: Vec<models::BookmarkDirectory>, ) -> BookmarkPostRequestBody {
        BookmarkPostRequestBody {
            bookmark_directory_list,
        }
    }
}

/// Converts the BookmarkPostRequestBody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BookmarkPostRequestBody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping bookmarkDirectoryList in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BookmarkPostRequestBody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BookmarkPostRequestBody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bookmark_directory_list: Vec<Vec<models::BookmarkDirectory>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BookmarkPostRequestBody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "bookmarkDirectoryList" => return std::result::Result::Err("Parsing a container in this style is not supported in BookmarkPostRequestBody".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing BookmarkPostRequestBody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BookmarkPostRequestBody {
            bookmark_directory_list: intermediate_rep.bookmark_directory_list.into_iter().next().ok_or_else(|| "bookmarkDirectoryList missing in BookmarkPostRequestBody".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BookmarkPostRequestBody> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<BookmarkPostRequestBody>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BookmarkPostRequestBody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BookmarkPostRequestBody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<BookmarkPostRequestBody> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BookmarkPostRequestBody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BookmarkPostRequestBody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



