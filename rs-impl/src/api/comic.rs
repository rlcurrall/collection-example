use chrono::NaiveDate;
use diesel::prelude::*;
use poem::{web::Data, Result};
use poem_openapi::param::{Path, Query};
use poem_openapi::{payload::Json, OpenApi};
use poem_openapi::{ApiResponse, Object};

use crate::api::{ApiTags, OrderDirection};
use crate::auth::JwtAuthorization;
use crate::database::DbPool;
use crate::models::{Comic, NewComic, ReplaceComic, UpdateComic};

const JS_SOURCE: &str = r#"
fetch("/api/comics")
    .then(res => res.json())
    .then(data => console.log(data));
"#;

pub struct ComicsApi;

#[OpenApi(tag = "ApiTags::Comics")]
impl ComicsApi {
    /// Get all comics
    ///
    /// The request must be authorized by a JWT token, but all records are freely
    /// available to view.
    #[oai(
        path = "/comics",
        method = "get",
        code_sample(lang = "js", source = "JS_SOURCE")
    )]
    async fn get_comics(
        &self,
        pool: Data<&DbPool>,
        page: Query<Option<i64>>,
        #[oai(name = "order[price]")] order_price: Query<Option<OrderDirection>>,
    ) -> Result<PaginateResponse> {
        use crate::schema::comics::dsl::*;

        let limit = 10;
        let page = page.unwrap_or(1) - 1;
        let offset = limit * page;

        let query = comics.limit(limit).offset(offset).into_boxed();
        let query = match order_price.0 {
            Some(OrderDirection::Desc) => query.order(price.desc()),
            Some(OrderDirection::Asc) => query.order(price.asc()),
            None => query,
        };

        let conn = pool.get().map_err(|err| {
            PaginateResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        let res = query.load::<Comic>(&conn).map_err(|err| {
            PaginateResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        Ok(PaginateResponse::Success(Json(res)))
    }

    /// Create a comic
    ///
    /// Add a comic to the currently authenticated user's collection
    #[oai(path = "/comics", method = "post")]
    async fn create_comic(
        &self,
        pool: Data<&DbPool>,
        comic_data: Json<NewComicRequest>,
        jwt: JwtAuthorization,
    ) -> Result<CreateResponse> {
        use crate::schema::comics::table;

        let conn = pool.get().map_err(|err| {
            CreateResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        let res: Comic = diesel::insert_into(table)
            .values(&comic_data.0.for_user(jwt.0.username))
            .get_result(&conn)
            .map_err(|err| {
                CreateResponse::ServerError(Json(ErrorResponse {
                    message: err.to_string(),
                }))
            })?;

        Ok(CreateResponse::Created(Json(res)))
    }

    /// Get a comic by ID
    ///
    /// The request must be authorized by a JWT token, but all records are freely
    /// available to view.
    #[oai(path = "/comics/:id", method = "get")]
    async fn get_comic_by_id(&self, pool: Data<&DbPool>, id: Path<i32>) -> Result<FindResponse> {
        use crate::schema::comics::dsl::comics;

        let conn = pool.get().map_err(|err| {
            FindResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        let res = comics.find(id.0).first(&conn).map_err(|err| {
            FindResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        Ok(FindResponse::Success(Json(res)))
    }

    /// Replace a comic
    ///
    /// All fields are required. Must be the owner of the comic to be able to successfully update.
    #[oai(path = "/comics/:id", method = "put")]
    async fn replace_comic(
        &self,
        pool: Data<&DbPool>,
        id: Path<i32>,
        comic_data: Json<ReplaceComic>,
        jwt: JwtAuthorization,
    ) -> Result<ReplaceResponse> {
        use crate::schema::comics::dsl::comics;

        let conn = pool.get().map_err(|err| {
            ReplaceResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        let comic: Comic = comics.find(id.0).first(&conn).map_err(|err| {
            ReplaceResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        if comic.username != jwt.0.username {
            return Err(ReplaceResponse::Forbidden(Json(ErrorResponse {
                message: "You do not own this item".into(),
            })))?;
        }

        let res: Comic = diesel::update(comics.find(comic.id))
            .set(&comic_data.0)
            .get_result(&conn)
            .map_err(|err| {
                ReplaceResponse::ServerError(Json(ErrorResponse {
                    message: err.to_string(),
                }))
            })?;

        Ok(ReplaceResponse::Success(Json(res)))
    }

    /// Update a comic
    ///
    /// All fields are optional. Must be the owner of the comic to be able to successfully update.
    #[oai(path = "/comics/:id", method = "patch")]
    async fn update_comic(
        &self,
        pool: Data<&DbPool>,
        id: Path<i32>,
        comic_data: Json<UpdateComic>,
        jwt: JwtAuthorization,
    ) -> Result<UpdateResponse> {
        use crate::schema::comics::dsl::comics;

        let conn = pool.get().map_err(|err| {
            UpdateResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        let comic: Comic = comics.find(id.0).first(&conn).map_err(|err| {
            UpdateResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        if comic.username != jwt.0.username {
            return Err(UpdateResponse::Forbidden(Json(ErrorResponse {
                message: "You do not own this item".into(),
            })))?;
        }

        let res: Comic = diesel::update(comics.find(comic.id))
            .set(&comic_data.0)
            .get_result(&conn)
            .map_err(|err| {
                UpdateResponse::ServerError(Json(ErrorResponse {
                    message: err.to_string(),
                }))
            })?;

        Ok(UpdateResponse::Success(Json(res)))
    }

    /// Delete a comic
    ///
    /// Must be the owner of the comic to be able to successfully delete.
    #[oai(path = "/comics/:id", method = "delete")]
    async fn delete_comic(
        &self,
        pool: Data<&DbPool>,
        id: Path<i32>,
        jwt: JwtAuthorization,
    ) -> Result<DeleteResponse> {
        use crate::schema::comics::dsl::comics;

        let conn = pool.get().map_err(|err| {
            DeleteResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        let comic: Comic = comics.find(id.0).first(&conn).map_err(|err| {
            DeleteResponse::ServerError(Json(ErrorResponse {
                message: err.to_string(),
            }))
        })?;

        if comic.username != jwt.0.username {
            return Err(DeleteResponse::Forbidden(Json(ErrorResponse {
                message: "You do not own this item".into(),
            })))?;
        }

        diesel::delete(comics.find(comic.id))
            .execute(&conn)
            .map_err(|err| {
                DeleteResponse::ServerError(Json(ErrorResponse {
                    message: err.to_string(),
                }))
            })?;

        Ok(DeleteResponse::Success)
    }
}

#[derive(Object)]
struct NewComicRequest {
    pub username: String,
    pub title: String,
    pub issue_number: String,
    pub main_character: String,
    pub genre: String,
    pub cover_year: NaiveDate,
    pub publisher: String,
    pub grade: f64,
    pub price: f64,
    pub image_url: String,
}

impl NewComicRequest {
    fn for_user(self, username: String) -> NewComic {
        NewComic {
            username,
            title: self.title,
            issue_number: self.issue_number,
            main_character: self.main_character,
            genre: self.genre,
            cover_year: self.cover_year,
            publisher: self.publisher,
            grade: self.grade,
            price: self.price,
            image_url: self.image_url,
        }
    }
}

#[derive(Debug, Object)]
struct ErrorResponse {
    message: String,
}

#[derive(ApiResponse)]
enum PaginateResponse {
    #[oai(status = 200)]
    Success(Json<Vec<Comic>>),
    #[oai(status = 401)]
    _Unauthorized(Json<ErrorResponse>),
    #[oai(status = 500)]
    ServerError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
enum CreateResponse {
    #[oai(status = 201)]
    Created(Json<Comic>),
    #[oai(status = 401)]
    _Unauthorized(Json<ErrorResponse>),
    #[oai(status = 500)]
    ServerError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
enum FindResponse {
    #[oai(status = 200)]
    Success(Json<Comic>),
    #[oai(status = 401)]
    _Unauthorized(Json<ErrorResponse>),
    #[oai(status = 500)]
    ServerError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
enum ReplaceResponse {
    #[oai(status = 200)]
    Success(Json<Comic>),
    #[oai(status = 401)]
    _Unauthorized(Json<ErrorResponse>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),
    #[oai(status = 500)]
    ServerError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
enum UpdateResponse {
    #[oai(status = 200)]
    Success(Json<Comic>),
    #[oai(status = 401)]
    _Unauthorized(Json<ErrorResponse>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),
    #[oai(status = 500)]
    ServerError(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
enum DeleteResponse {
    #[oai(status = 204)]
    Success,
    #[oai(status = 401)]
    _Unauthorized(Json<ErrorResponse>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),
    #[oai(status = 500)]
    ServerError(Json<ErrorResponse>),
}
