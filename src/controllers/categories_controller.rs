extern crate jsonwebtoken as jwt;
use super::schema::{Category, CategoryDeserializeModel, CategoryResponseModel, File, SerlizedId};
use crate::middleware::error::ContentBuilderCustomResponseError;
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse,
};
use awmp::Parts;
use bson::Document;
use futures::stream::StreamExt;
use std::path::PathBuf;

#[get("category/all")]
async fn get_all_categories(
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state.container.category.find_all().await {
        Ok(cursor) => {
            let categories: Vec<CategoryResponseModel> = cursor
                .map(|document| {
                    let category: CategoryDeserializeModel =
                        bson::from_document::<CategoryDeserializeModel>(match document {
                            Ok(category_document) => match category_document {
                                category_document => category_document,
                            },
                            Err(_mongodb_error) => bson::Document::new(),
                        })
                        .unwrap();
                    CategoryResponseModel::build_category(category)
                })
                .collect::<Vec<CategoryResponseModel>>()
                .await;
            Ok(HttpResponse::Ok().json(categories))
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[post("category/add")]
async fn add_category(
    app_state: web::Data<crate::AppState>,
    mut parts: Parts,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    let form_data = parts.texts.as_hash_map();
    let file_name = parts
        .files
        .take("image")
        .pop()
        .and_then(|file| {
            file.persist_in(PathBuf::from("./static/uploads/categories"))
                .ok()
        })
        .and_then(|file_path| {
            file_path.file_name().and_then(|os_path| {
                os_path
                    .to_str()
                    .and_then(|sentaized_path| Some(sentaized_path.to_string()))
            })
        })
        .unwrap();

    match app_state
        .container
        .category
        .insert_one(Category {
            name: form_data["name"].to_string(),
            description: form_data["description"].to_string(),
            image: File {
                name: file_name.clone(),
                src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/categories/{}", file_name.clone()),
            },
        })
        .await
    {
        Ok(category_id) => match category_id.inserted_id.as_object_id() {
            Some(object_id) => {
                match app_state
                    .container
                    .category
                    .find_one_by_id(&object_id.to_string())
                    .await.and_then(|document|{Ok(match document {
                        Some(document)=>document,
                        None => Document::new()
                    })})
                {
                    Ok(document) =>match  document {
                            document=>match bson::from_document::<CategoryDeserializeModel>(document){
                                Ok(category) => Ok(HttpResponse::Ok()
                                    .json(CategoryResponseModel::build_category(category))),
                                Err(_bson_de_error) => {
                                    Err(ContentBuilderCustomResponseError::InternalError)
                                }
                            }
                    }
                    Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
                }
            }
            None => Err(ContentBuilderCustomResponseError::InternalError),
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[put("category/update")]
async fn update_category(
    app_state: web::Data<crate::AppState>,
    mut parts: Parts,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    let form_data = parts.texts.as_hash_map();

    let file_name = parts
        .files
        .take("image")
        .pop()
        .and_then(|file| {
            file.persist_in(PathBuf::from("./static/uploads/categories"))
                .ok()
        })
        .and_then(|file_path| {
            file_path.file_name().and_then(|os_path| {
                os_path
                    .to_str()
                    .and_then(|sentaized_path| Some(sentaized_path.to_string()))
            })
        })
        .unwrap();

    match app_state
        .container
        .category
        .update_one(
            &form_data["id"].to_string(),
            Category {
                name: form_data["name"].to_string(),
                description: form_data["description"].to_string(),
                image: File {
                    name: file_name.clone(),
                    src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/categories/{}", file_name.clone()),
                },
            },
        )
        .await.and_then(|document|{Ok(match document {
            Some(document)=>document,
            None => Document::new()
        })})
    {
        Ok(document) =>match  document {
            document=>match bson::from_document::<CategoryDeserializeModel>(document){
                Ok(category) => Ok(HttpResponse::Ok()
                    .json(CategoryResponseModel::build_category(category))),
                Err(_bson_de_error) => {
                    Err(ContentBuilderCustomResponseError::NotFound)
                }
            }
    }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[delete("category/delete")]
async fn delete_category(
    app_state: web::Data<crate::AppState>,
    category_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .category
        .delete_one(&category_data.id)
        .await
        .and_then(|document| {
            Ok(match document {
                Some(document) => document,
                None => bson::Document::new(),
            })
        }) {
        Ok(document) => match document {
            document => match bson::from_document::<CategoryDeserializeModel>(document) {
                Ok(category) => {
                    Ok(HttpResponse::Ok().json(CategoryResponseModel::build_category(category)))
                }
                Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::NotFound),
            },
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[post("category/get")]
async fn get_category_by_id(
    app_state: web::Data<crate::AppState>,
    category_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .category
        .find_one_by_id(&category_data.id)
        .await
        .and_then(|document| {
            Ok(match document {
                Some(doc) => doc,
                None => bson::Document::new(),
            })
        }) {
        Ok(document) => match document {
            document => match bson::from_document::<CategoryDeserializeModel>(document) {
                Ok(category) => {
                    Ok(HttpResponse::Ok().json(CategoryResponseModel::build_category(category)))
                }
                Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::NotFound),
            },
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}
