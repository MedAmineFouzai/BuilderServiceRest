extern crate jsonwebtoken as jwt;
use crate::middleware::error::ContentBuilderCustomResponseError;
use actix_web::{
    post, put,
    web::{self, Json},
    HttpResponse,
};
use bson::oid::ObjectId;
use futures::stream::StreamExt;

use super::schema::{
    Connections, ProtoType, ProtoTypeObject, ProtoTypeRefactorDeserializeModel, ProtoTypeRequest,
    ProtoTypeResponseModel, SerlizedId,
};

#[post("prototype/add")]
async fn add_prototype(
    app_state: web::Data<crate::AppState>,
    prototype_data: Json<ProtoTypeRequest>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    println!("{:?}", prototype_data);
    match app_state
        .container
        .prototype
        .insert_one(ProtoType {
            template_id: ObjectId::with_string(&prototype_data.template_id).unwrap(),
            prototype: prototype_data
                .prototype
                .clone()
                .into_iter()
                .map(|prototype_object| ProtoTypeObject {
                    feature_id: ObjectId::with_string(&prototype_object.feature_id).unwrap(),
                    connections: prototype_object
                        .connections
                        .into_iter()
                        .map(|connections| Connections {
                            to: ObjectId::with_string(&connections.to).unwrap(),
                            releations: connections.releations,
                        })
                        .collect::<Vec<Connections>>(),
                })
                .collect::<Vec<ProtoTypeObject>>(),
        })
        .await
    {
        Ok(id) => match id.inserted_id.as_object_id() {
            Some(_id) => {
                match app_state
                    .container
                    .prototype
                    .refactor_one_by_id(&prototype_data.template_id)
                    .await
                {
                    Ok(cursor) => {
                        let prototypes: Vec<ProtoTypeResponseModel> = cursor
                            .map(|doc| {
                                let prototype = bson::from_document::<
                                    ProtoTypeRefactorDeserializeModel,
                                >(match doc {
                                    Ok(prototype) => match prototype {
                                        prototype => prototype,
                                    },
                                    Err(_mongodb_error) => bson::Document::new(),
                                })
                                .ok();
                                ProtoTypeResponseModel::build_prototype(prototype.unwrap())
                            })
                            .collect()
                            .await;
                        if !prototypes.last().is_none() {
                            Ok(HttpResponse::Ok().json(prototypes.last()))
                        } else {
                            Err(ContentBuilderCustomResponseError::NotFound)
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

#[post("prototype/get")]
async fn get_prototype_by_template_id(
    app_state: web::Data<crate::AppState>,
    template_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .prototype
        .refactor_one_by_id(&template_data.id)
        .await
    {
        Ok(cursor) => {
            let prototypes: Vec<ProtoTypeResponseModel> = cursor
                .map(|doc| {
                    let prototype =
                        bson::from_document::<ProtoTypeRefactorDeserializeModel>(match doc {
                            Ok(prototype) => match prototype {
                                prototype => prototype,
                            },
                            Err(_mongodb_error) => bson::Document::new(),
                        })
                        .ok();
                    // println!("Prototype Dezrlized: {:?}", prototype);
                    ProtoTypeResponseModel::build_prototype(prototype.unwrap())
                })
                .collect()
                .await;
            if !prototypes.last().is_none() {
                Ok(HttpResponse::Ok().json(prototypes.last()))
            } else {
                Err(ContentBuilderCustomResponseError::NotFound)
            }
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[put("prototype/update")]
async fn update_prototype(
    app_state: web::Data<crate::AppState>,
    prototype_data: Json<ProtoTypeRequest>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    // println!("{:?}", prototype_data);
    match app_state
        .container
        .prototype
        .update_one(
            &prototype_data.template_id,
            ProtoType {
                template_id: ObjectId::with_string(&prototype_data.template_id).unwrap(),
                prototype: prototype_data
                    .prototype
                    .clone()
                    .into_iter()
                    .map(|prototype_object| ProtoTypeObject {
                        feature_id: ObjectId::with_string(&prototype_object.feature_id).unwrap(),
                        connections: prototype_object
                            .connections
                            .into_iter()
                            .map(|connections| Connections {
                                to: ObjectId::with_string(&connections.to).unwrap(),
                                releations: connections.releations,
                            })
                            .collect::<Vec<Connections>>(),
                    })
                    .collect::<Vec<ProtoTypeObject>>(),
            },
        )
        .await
    {
        Ok(document) => match document {
            Some(_doc) => {
                match app_state
                    .container
                    .prototype
                    .refactor_one_by_id(&prototype_data.template_id)
                    .await
                {
                    Ok(cursor) => {
                        let prototypes: Vec<ProtoTypeResponseModel> = cursor
                            .map(|doc| {
                                let prototype = bson::from_document::<
                                    ProtoTypeRefactorDeserializeModel,
                                >(match doc {
                                    Ok(prototype) => match prototype {
                                        prototype => prototype,
                                    },
                                    Err(_mongodb_error) => bson::Document::new(),
                                })
                                .ok();
                                // println!("Prototype Dezrlized: {:?}", prototype);
                                ProtoTypeResponseModel::build_prototype(prototype.unwrap())
                            })
                            .collect()
                            .await;
                        if !prototypes.last().is_none() {
                            Ok(HttpResponse::Ok().json(prototypes.last()))
                        } else {
                            Err(ContentBuilderCustomResponseError::NotFound)
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
