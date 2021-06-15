extern crate jsonwebtoken as jwt;
use super::schema::{
    Feature, FeatureDeserializeModel, FeatureObject, FeatureResponseModel, File, FileWithId,
    SerlizedId, UpdateFeatureWireframesModel,
};
use crate::middleware::error::ContentBuilderCustomResponseError;
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse,
};
use awmp::Parts;
use bson::{oid::ObjectId, Document};
use futures::stream::StreamExt;
use std::path::PathBuf;

#[get("feature/all")]
async fn get_all_features(
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state.container.feature.find_all().await {
        Ok(cursor) => {
            let features: Vec<FeatureResponseModel> = cursor
                .map(|document| {
                    let feature = bson::from_document::<FeatureDeserializeModel>(match document {
                        Ok(feature_document) => match feature_document {
                            feature => feature,
                        },
                        Err(_mongodb_error) => bson::Document::new(),
                    })
                    .ok();
                    FeatureResponseModel::build_feature(feature.unwrap())
                })
                .collect()
                .await;
            Ok(HttpResponse::Ok().json(features))
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

// #[post("feature/add")]
// async fn add_feature(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();
//     let image = parts
//         .files
//         .take("image")
//         .pop()
//         .and_then(|file| {
//             file.persist_in(PathBuf::from("./static/uploads/features_imgs"))
//                 .ok()
//         })
//         .and_then(|file_path| {
//             file_path.file_name().and_then(|os_path| {
//                 os_path
//                     .to_str()
//                     .and_then(|sentaized_path| Some(sentaized_path.to_string()))
//             })
//         })
//         .unwrap();

//     let images = parts
//         .files
//         .take("wireframes")
//         .into_iter()
//         .map(|file| {

//             file.persist_in(PathBuf::from("./static/uploads/wireframes"))
//                 .ok()
//         })
//         .into_iter()
//         .map(|path| {
//             let file_name = path.and_then(|file_path|{
//                 file_path.file_name().and_then(|os_path|{
//                     os_path.to_str().and_then(|sentaized_path|{
//                         Some(sentaized_path.to_string())
//                     })
//                 })
//             }).unwrap();

//             FileWithId {
//                 _id: ObjectId::new(),
//                 name: format!("{}", file_name.clone()),
//                 src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/wireframes/{}", file_name.clone()),
//             }
//         })
//         .collect::<Vec<FileWithId>>();

//     match app_state
//         .container
//         .feature
//         .insert_one(Feature {
//             name: form_data["name"].to_string(),
//             description: form_data["description"].to_string(),
//             feature_type: form_data["feature_type"].to_string(),
//             image: File {
//                 name: image.clone(),
//                 src: format!("/media/static/uploads/features_imgs/{}", image.clone()),
//             },
//             wireframes: Some(images),
//             price: form_data["price"].parse::<f64>().unwrap(),
//             repo: form_data["repo"].to_string(),
//         })
//         .await
//     {
//         Ok(feature_id) => match feature_id.inserted_id.as_object_id() {
//             Some(object_id) => {
//                 match app_state
//                     .container
//                     .feature
//                     .find_one_by_id(&object_id.to_string())
//                     .await
//                     .and_then(|document| {
//                         Ok(match document {
//                             Some(document) => document,
//                             None => Document::new(),
//                         })
//                     }) {
//                     Ok(feature_document) => match feature_document {
//                         document => {
//                             match bson::from_document::<FeatureDeserializeModel>(document) {
//                                 Ok(category) => Ok(HttpResponse::Ok()
//                                     .json(FeatureResponseModel::build_feature(category))),
//                                 Err(_bson_de_error) => {
//                                     Err(ContentBuilderCustomResponseError::InternalError)
//                                 }
//                             }
//                         }
//                     },

//                     Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                 }
//             }
//             None => Err(ContentBuilderCustomResponseError::InternalError),
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
//     // Ok(HttpResponse::Ok().body("ok"))
// }

// #[put("feature/update")]
// async fn update_feature(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();
//     let image = parts
//         .files
//         .take("image")
//         .pop()
//         .and_then(|file| {
//             file.persist_in(PathBuf::from("./static/uploads/features_imgs"))
//                 .ok()
//         })
//         .and_then(|file_path| {
//             file_path.file_name().and_then(|os_path| {
//                 os_path
//                     .to_str()
//                     .and_then(|sentaized_path| Some(sentaized_path.to_string()))
//             })
//         })
//         .unwrap();

//     let images = parts
//         .files
//         .take("wireframes")
//         .into_iter()
//         .map(|file| {
//             file.persist_in(PathBuf::from("./static/uploads/wireframes"))
//                 .ok()
//         })
//         .into_iter()
//         .map(|path| {
//             let file_name = path.and_then(|file_path|{
//                 file_path.file_name().and_then(|os_path|{
//                     os_path.to_str().and_then(|sentaized_path|{
//                         Some(sentaized_path.to_string())
//                     })
//                 })
//             }).unwrap();
//             FileWithId {
//                 _id: ObjectId::new(),
//                 name: format!("{}", file_name.clone()),
//                 src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/wireframes/{}", file_name.clone()),
//             }
//         })
//         .collect::<Vec<FileWithId>>();

//     match app_state
//         .container
//         .feature
//         .update_one(
//             &form_data["id"].to_string(),
//             Feature {
//                 name: form_data["name"].to_string(),
//                 description: form_data["description"].to_string(),
//                 feature_type: form_data["feature_type"].to_string(),
//                 image: File {
//                     name: image.clone(),
//                     src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/features_imgs/{}", image.clone()),
//                 },
//                 wireframes: Some(images),
//                 price: form_data["price"].parse::<f64>().unwrap(),
//                 repo: form_data["repo"].to_string(),
//             },
//         )
//         .await
//     {
//         Ok(result) => {
//             if result != None {
//                 match bson::from_document::<FeatureDeserializeModel>(result.unwrap()) {
//                     Ok(feature) => {
//                         Ok(HttpResponse::Ok().json(FeatureResponseModel::build_feature(feature)))
//                     }
//                     Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                 }
//             } else {
//                 Err(ContentBuilderCustomResponseError::NotFound)
//             }
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

#[post("feature/get")]
async fn get_feature_by_id(
    app_state: web::Data<crate::AppState>,
    feature_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .feature
        .find_one_by_id(&feature_data.id)
        .await
        .and_then(|document| {
            let feature = match document {
                Some(doc) => doc,
                None => bson::Document::new(),
            };
            Ok(feature)
        }) {
        Ok(feature_document) => {
            match feature_document {
                feature_document => {
                    if !feature_document.is_empty() {
                        match bson::from_document::<FeatureDeserializeModel>(feature_document) {
                            Ok(feature) => Ok(HttpResponse::Ok()
                                .json(FeatureResponseModel::build_feature(feature))),
                            Err(_bson_de_error) => {
                                Err(ContentBuilderCustomResponseError::InternalError)
                            }
                        }
                    } else {
                        Err(ContentBuilderCustomResponseError::NotFound)
                    }
                }
            }
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[delete("feature/delete")]
async fn delete_feature(
    app_state: web::Data<crate::AppState>,
    feature_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .feature
        .delete_one(&feature_data.id)
        .await
        .and_then(|document| {
            let feature = match document {
                Some(doc) => doc,
                None => bson::Document::new(),
            };
            Ok(feature)
        }) {
        Ok(feature_dcoument) => {
            match feature_dcoument {
                feature_dcoument => {
                    if !feature_dcoument.is_empty() {
                        match bson::from_document::<FeatureDeserializeModel>(feature_dcoument) {
                            Ok(feature) => Ok(HttpResponse::Ok()
                                .json(FeatureResponseModel::build_feature(feature))),
                            Err(_bson_de_error) => {
                                Err(ContentBuilderCustomResponseError::InternalError)
                            }
                        }
                    } else {
                        Err(ContentBuilderCustomResponseError::NotFound)
                    }
                }
            }
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[post("feature/wireframe/add")]
async fn add_feature_wireframe(
    app_state: web::Data<crate::AppState>,
    wireframes: Json<UpdateFeatureWireframesModel>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match serde_json::to_string(&wireframes.into_inner()).and_then(|wireframes| {
        match serde_json::from_str::<UpdateFeatureWireframesModel>(&wireframes) {
            Ok(feature) => Ok(feature),
            Err(serde_error) => Err(serde_error.into()),
        }
    }) {
        Ok(feature_wireframes) => {
            match app_state
                .container
                .feature
                .add_wireframe(
                    &feature_wireframes.id,
                    feature_wireframes
                        .wireframes
                        .into_iter()
                        .map(|wireframe| {
                            let document: Document = bson::to_bson(&FileWithId {
                                _id: ObjectId::with_string(&wireframe.id).unwrap(),
                                name: wireframe.name,
                                src: wireframe.src,
                            })
                            .unwrap()
                            .as_document()
                            .unwrap()
                            .clone();
                            document
                        })
                        .collect::<Vec<Document>>(),
                )
                .await
            {
                Ok(document) => {
                    match document {
                        Some(doc) => match bson::from_document::<FeatureDeserializeModel>(doc) {
                            Ok(feature) => Ok(HttpResponse::Ok()
                                .json(FeatureResponseModel::build_feature(feature))),
                            Err(_bson_de_error) => {
                                Err(ContentBuilderCustomResponseError::InternalError)
                            }
                        },
                        None => Err(ContentBuilderCustomResponseError::NotFound),
                    }
                }
                Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
            }
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
    // let feature_id = parts.texts.as_hash_map()["id"];
    // let images = parts
    //     .files
    //     .take("wireframes")
    //     .into_iter()
    //     .map(|file| {
    //         file.persist_in(PathBuf::from("./static/uploads/wireframes"))
    //             .ok()
    //     })
    //     .into_iter()
    //     .map(|path| {
    //         let file_name =  path.and_then(|file_path|{
    //             file_path.file_name().and_then(|os_path|{
    //                 os_path.to_str().and_then(|sentaized_path|{
    //                     Some(sentaized_path.to_string())
    //                 })
    //             })
    //         }).unwrap();
    //         FileWithId {
    //             _id: ObjectId::new(),
    //             name: format!("{}", file_name.clone()),
    //             src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/wireframes/{}", file_name.clone()),
    //         }
    //     })
    //     .collect::<Vec<FileWithId>>();

    // match app_state
    //     .container
    //     .feature
    //     .find_one_by_id(feature_id)
    //     .await
    //     .and_then(|document| {
    //         let feature = match document {
    //             Some(doc) => doc,
    //             None => bson::Document::new(),
    //         };
    //         Ok(feature)
    //     }) {
    //     Ok(feature) => match feature {
    //         feature => {
    //             println!("Feature Dezrlized: {:?}", feature);
    //             if !feature.is_empty() {
    //                 match bson::from_document::<FeatureDeserializeModel>(feature) {
    //                     Ok(feature) => {
    //                         let mut files: Vec<bson::Document> = Vec::new();
    //                         let feature = FeatureResponseModel::build_feature(feature);
    //                         for file in images {
    //                             let doc = match app_state
    //                                 .container
    //                                 .feature
    //                                 .add_wireframe(&feature.id, file)
    //                                 .await
    //                             {
    //                                 Ok(document) => Ok(match document {
    //                                     Some(doc) => doc,
    //                                     None => bson::Document::new(),
    //                                 }),
    //                                 Err(_mongodb_error) => {
    //                                     Err(ContentBuilderCustomResponseError::InternalError)
    //                                 }
    //                             };
    //                             files.push(doc?);
    //                         }

    //                         match bson::from_document::<FeatureDeserializeModel>(
    //                             files.last().unwrap().clone(),
    //                         ) {
    //                             Ok(feature) => Ok(HttpResponse::Ok()
    //                                 .json(FeatureResponseModel::build_feature(feature))),
    //                             Err(_bson_de_error) => {
    //                                 Err(ContentBuilderCustomResponseError::InternalError)
    //                             }
    //                         }
    //                     }
    //                     Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
    //                 }
    //             } else {
    //                 Err(ContentBuilderCustomResponseError::NotFound)
    //             }
    //         }
    //     },
    //     Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    // }
}

#[delete("feature/wireframe/delete")]
async fn delete_feature_wireframe(
    app_state: web::Data<crate::AppState>,
    wireframe_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .feature
        .find_wireframe_by_id(&wireframe_data.id)
        .await
        .and_then(|cursor| {
            Ok(async {
                let feature = cursor
                    .map(|doc| {
                        let feature = bson::from_document::<FeatureDeserializeModel>(match doc {
                            Ok(feature) => match feature {
                                feature => feature,
                            },
                            Err(_mongodb_error) => bson::Document::new(),
                        })
                        .ok();
                        FeatureResponseModel::build_feature(feature.unwrap())
                    })
                    .collect::<Vec<FeatureResponseModel>>()
                    .await;

                Ok(feature)
            })
        }) {
        Ok(result) => match result.await? {
            featuers => {
                if !featuers.is_empty() {
                    let feature = featuers.last().unwrap();
                    match app_state
                        .container
                        .feature
                        .delete_wireframe(
                            &feature.id,
                            feature
                                .wireframes
                                .as_ref()
                                .and_then(|frame| {
                                    let file = frame
                                        .iter()
                                        .find(|&file| file.id == wireframe_data.id)?
                                        .clone();
                                    Some(FileWithId {
                                        _id: ObjectId::with_string(&file.id).unwrap(),
                                        name: file.name.clone(),
                                        src: file.src.clone(),
                                    })
                                })
                                .unwrap(),
                        )
                        .await
                    {
                        Ok(document) => {
                            match bson::from_document::<FeatureDeserializeModel>(document.unwrap())
                            {
                                Ok(feature) => Ok(HttpResponse::Ok()
                                    .json(FeatureResponseModel::build_feature(feature))),
                                Err(_bson_de_error) => {
                                    Err(ContentBuilderCustomResponseError::InternalError)
                                }
                            }
                        }
                        Err(_mongodb_error) => {
                            Err(ContentBuilderCustomResponseError::InternalError)
                        }
                    }
                } else {
                    Err(ContentBuilderCustomResponseError::NotFound)
                }
            }
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[post("feature/create")]
async fn create_feature(
    app_state: web::Data<crate::AppState>,
    feature: Json<FeatureObject>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match serde_json::to_string(&feature.into_inner()).and_then(|feature_data| {
        match serde_json::from_str::<FeatureObject>(&feature_data) {
            Ok(feature) => Ok(feature),
            Err(serde_error) => Err(serde_error.into()),
        }
    }) {
        Ok(feature) => {
            match app_state
                .container
                .feature
                .insert_one(Feature {
                    name: feature.name,
                    description: feature.description,
                    feature_type: feature.feature_type,
                    image: feature.image,
                    wireframes: Some(match feature.wireframes {
                        Some(wireframes) => wireframes
                            .into_iter()
                            .map(|wireframe| FileWithId {
                                _id: ObjectId::with_string(&wireframe.id).unwrap(),
                                name: wireframe.name,
                                src: wireframe.src,
                            })
                            .collect::<Vec<FileWithId>>(),
                        None => vec![],
                    }),
                    price: feature.price,
                    repo: feature.repo,
                })
                .await
            {
                Ok(feature_id) => match feature_id.inserted_id.as_object_id() {
                    Some(object_id) => {
                        match app_state
                            .container
                            .feature
                            .find_one_by_id(&object_id.to_string())
                            .await
                            .and_then(|document| {
                                Ok(match document {
                                    Some(document) => document,
                                    None => Document::new(),
                                })
                            }) {
                            Ok(feature_document) => match feature_document {
                                document => {
                                    match bson::from_document::<FeatureDeserializeModel>(document) {
                                        Ok(category) => Ok(HttpResponse::Ok()
                                            .json(FeatureResponseModel::build_feature(category))),
                                        Err(_bson_de_error) => {
                                            Err(ContentBuilderCustomResponseError::InternalError)
                                        }
                                    }
                                }
                            },

                            Err(_mongodb_error) => {
                                Err(ContentBuilderCustomResponseError::InternalError)
                            }
                        }
                    }
                    None => Err(ContentBuilderCustomResponseError::InternalError),
                },
                Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
            }
        }
        Err(e) => Err(ContentBuilderCustomResponseError::InternalError),
    }
    // Ok(HttpResponse::Ok().body("ok"))
}

#[put("feature/update")]
async fn update_feature(
    app_state: web::Data<crate::AppState>,
    feature: Json<FeatureResponseModel>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match serde_json::to_string(&feature.into_inner()).and_then(|feature_data| {
        match serde_json::from_str::<FeatureResponseModel>(&feature_data) {
            Ok(feature) => Ok(feature),
            Err(serde_error) => Err(serde_error.into()),
        }
    }) {
        Ok(feature) => {
            match app_state
                .container
                .feature
                .update_one(
                    &feature.id,
                    Feature {
                        name: feature.name,
                        description: feature.description,
                        feature_type: feature.feature_type,
                        image: feature.image,
                        wireframes: Some(match feature.wireframes {
                            Some(wireframes) => wireframes
                                .into_iter()
                                .map(|wireframe| FileWithId {
                                    _id: ObjectId::with_string(&wireframe.id).unwrap(),
                                    name: wireframe.name,
                                    src: wireframe.src,
                                })
                                .collect::<Vec<FileWithId>>(),
                            None => vec![],
                        }),
                        price: feature.price,
                        repo: feature.repo,
                    },
                )
                .await
            {
                Ok(result) => {
                    if result != None {
                        match bson::from_document::<FeatureDeserializeModel>(result.unwrap()) {
                            Ok(feature) => Ok(HttpResponse::Ok()
                                .json(FeatureResponseModel::build_feature(feature))),
                            Err(_bson_de_error) => {
                                Err(ContentBuilderCustomResponseError::InternalError)
                            }
                        }
                    } else {
                        Err(ContentBuilderCustomResponseError::NotFound)
                    }
                }
                Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
            }
        }
        Err(e) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}
