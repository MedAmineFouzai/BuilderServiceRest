extern crate jsonwebtoken as jwt;
use super::schema::{
    CategoiresIds, FeatureToAnyModel, File, Introduction, NonFunctionalRequirements,
    OverallDescription, SerlizedId, Specification, Template, TemplateDeserializeModel,
    TemplateObject, TemplateObjectWithId, TemplateReafactorDeserializeModel, TemplateResponseModel,
    TemplateResponseRefactorModel,
};
use crate::middleware::error::ContentBuilderCustomResponseError;
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse,
};
use awmp::Parts;
use bson::oid::ObjectId;
use futures::stream::StreamExt;
use std::path::PathBuf;

#[get("template/all")] // no need
async fn get_all_templates(
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state.container.template.find_all().await {
        Ok(cursor) => {
            let templates: Vec<TemplateResponseRefactorModel> = cursor
                .map(|doc| {
                    let template =
                        bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
                            Ok(template) => match template {
                                template => template,
                            },
                            Err(_mongodb_error) => bson::Document::new(),
                        })
                        .ok();

                    TemplateResponseRefactorModel::build_template(template.unwrap())
                })
                .collect()
                .await;
            Ok(HttpResponse::Ok().json(templates))
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::BadHeaderData),
    }
}

#[post("template/categories/all")] //  no need
async fn get_templates_by_categories_id(
    app_state: web::Data<crate::AppState>,
    categoires: Json<CategoiresIds>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    let categories_id = categoires
        .categories_id
        .clone()
        .into_iter()
        .map(|feature_id| ObjectId::with_string(&feature_id.clone()).unwrap())
        .collect::<Vec<ObjectId>>();

    match app_state
        .container
        .template
        .find_templates_by_categories_id(categories_id)
        .await
    {
        Ok(cursor) => {
            let templates: Vec<TemplateResponseRefactorModel> = cursor
                .map(|doc| {
                    let template =
                        bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
                            Ok(template) => match template {
                                template => template,
                            },
                            Err(_mongodb_error) => bson::Document::new(),
                        })
                        .ok();

                    TemplateResponseRefactorModel::build_template(template.unwrap())
                })
                .collect()
                .await;
            Ok(HttpResponse::Ok().json(templates))
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::BadHeaderData),
    }
}

// #[post("template/add")]//todo
// async fn add_template(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();

//     let image = parts
//         .files
//         .take("image")
//         .pop()
//         .and_then(|file| {
//             file.persist_in(PathBuf::from("./static/uploads/template_imgs"))
//                 .ok()
//         })
//         .unwrap()
//         .file_name()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string();

//     match app_state
//         .container
//         .template
//         .insert_one(Template {
//             name: form_data["name"].to_string(),
//             description: form_data["description"].to_string(),
//             // catagorys: Some(vec![]), // form_data["catagorys"].to_string(),
//             image: File {
//                 name: image.clone(),
//                 src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/features_imgs/{}", image.clone()),
//             },
//             category: ObjectId::with_string(&form_data["category"].to_string()).unwrap(),
//             features: Some(vec![]),
//             specification: Some(Specification::new()),
//             // prototype_id: Some(
//             //     // ObjectId::with_string(&form_data["prototype_id"].to_string()).unwrap(),
//             //     ObjectId::new()
//             // ),
//         })
//         .await
//     {
//         Ok(id) => match id.inserted_id.as_object_id() {
//             Some(id) => {
//                 match app_state
//                     .container
//                     .template
//                     .find_one_by_id(&id.to_string())
//                     .await
//                 {
//                     Ok(result) => {
//                         if result != None {
//                             match bson::from_document::<TemplateDeserializeModel>(result.unwrap()) {
//                                 Ok(template) => Ok(HttpResponse::Ok()
//                                     .json(TemplateResponseModel::build_template(template))),
//                                 Err(_bson_de_error) => {
//                                     Err(ContentBuilderCustomResponseError::InternalError)
//                                 }
//                             }
//                         } else {
//                             Err(ContentBuilderCustomResponseError::NotFound)
//                         }
//                     }
//                     Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                 }
//             }
//             None => Err(ContentBuilderCustomResponseError::InternalError),
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
//     // Ok(HttpResponse::Ok().body("ok"))
// }

#[post("template/create")] // no need
async fn create_template(
    app_state: web::Data<crate::AppState>,
    template: Json<TemplateObject>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match serde_json::to_string(&template.into_inner()).and_then(|template| {
        match serde_json::from_str::<TemplateObject>(&template) {
            Ok(template) => Ok(template),
            Err(serde_error) => Err(serde_error.into()),
        }
    }) {
        Ok(template) => {
            match app_state
                .container
                .template
                .insert_one(Template {
                    name: template.name,
                    description: template.description,
                    category: ObjectId::with_string(&template.category).unwrap(),
                    features: Some(match template.features {
                        Some(features) => features
                            .into_iter()
                            .map(|object_id| ObjectId::with_string(&object_id).unwrap())
                            .collect::<Vec<ObjectId>>(),
                        None => vec![],
                    }),
                    image: template.image,
                    specification: Some(match template.specification {
                        Some(specification) => specification,
                        None => Specification::new(),
                    }),
                })
                .await
            {
                Ok(id) => match id.inserted_id.as_object_id() {
                    Some(id) => {
                        match app_state
                            .container
                            .template
                            .refactor_template(&id.to_string())
                            .await
                        {
                            Ok(cursor) => {
                                let templates: Vec<TemplateResponseRefactorModel> = cursor
                                    .map(|doc| {
                                        let template = bson::from_document::<
                                            TemplateReafactorDeserializeModel,
                                        >(
                                            match doc {
                                                Ok(template) => match template {
                                                    template => template,
                                                },
                                                Err(_mongodb_error) => bson::Document::new(),
                                            },
                                        )
                                        .ok();
                                        println!("Tempalte Dezrlized: {:?}", template);
                                        TemplateResponseRefactorModel::build_template(
                                            template.unwrap(),
                                        )
                                    })
                                    .collect()
                                    .await;
                                Ok(HttpResponse::Ok().json(templates.last()))
                            }
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
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }

    // Ok(HttpResponse::Ok().body("ok"))
}

#[put("template/update")] //no need
async fn update_template(
    app_state: web::Data<crate::AppState>,
    template: Json<TemplateObjectWithId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match serde_json::to_string(&template.into_inner()).and_then(|template| {
        match serde_json::from_str::<TemplateObjectWithId>(&template) {
            Ok(template) => Ok(template),
            Err(serde_error) => Err(serde_error.into()),
        }
    }) {
        Ok(template) => {
            match app_state
                .container
                .template
                .update_one(
                    &template.id,
                    Template {
                        name: template.name,
                        description: template.description,
                        category: ObjectId::with_string(&template.category).unwrap(),
                        features: Some(match template.features {
                            Some(features) => features
                                .into_iter()
                                .map(|object_id| ObjectId::with_string(&object_id).unwrap())
                                .collect::<Vec<ObjectId>>(),
                            None => vec![],
                        }),
                        image: template.image,
                        specification: Some(match template.specification {
                            Some(specification) => specification,
                            None => Specification::new(),
                        }),
                    },
                )
                .await
            {
                Ok(document) => match document {
                    Some(document) => {
                        match app_state
                            .container
                            .template
                            .refactor_template(&document.get_object_id("_id").unwrap().to_string())
                            .await
                        {
                            Ok(cursor) => {
                                let templates: Vec<TemplateResponseRefactorModel> = cursor
                                    .map(|doc| {
                                        let template = bson::from_document::<
                                            TemplateReafactorDeserializeModel,
                                        >(
                                            match doc {
                                                Ok(template) => match template {
                                                    template => template,
                                                },
                                                Err(_mongodb_error) => bson::Document::new(),
                                            },
                                        )
                                        .ok();
                                        TemplateResponseRefactorModel::build_template(
                                            template.unwrap(),
                                        )
                                    })
                                    .collect()
                                    .await;
                                Ok(HttpResponse::Ok().json(templates.last()))
                            }
                            Err(_mongodb_error) => {
                                Err(ContentBuilderCustomResponseError::InternalError)
                            }
                        }
                    }
                    None => Err(ContentBuilderCustomResponseError::NotFound),
                },
                Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
            }
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
    // Ok(HttpResponse::Ok().body("ok"))
}

#[delete("template/delete")]
async fn delete_template(
    app_state: web::Data<crate::AppState>,
    feature_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .template
        .delete_one(&feature_data.id)
        .await
        .and_then(|document| {
            let feature = match document {
                Some(doc) => doc,
                None => bson::Document::new(),
            };
            Ok(feature)
        }) {
        Ok(result) => match result {
            result => {
                if !result.is_empty() {
                    match bson::from_document::<TemplateDeserializeModel>(result) {
                        Ok(template) => Ok(HttpResponse::Ok()
                            .json(TemplateResponseModel::build_template(template))),
                        Err(_bson_de_error) => {
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

#[put("template/feature/update")] // no need
async fn update_template_feature(
    app_state: web::Data<crate::AppState>,
    data: Json<FeatureToAnyModel>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    let features_id = data
        .features_id
        .clone()
        .into_iter()
        .map(|feature_id| ObjectId::with_string(&feature_id.clone()).unwrap())
        .collect::<Vec<ObjectId>>();

    match app_state
        .container
        .template
        .update_features(&data.id, features_id)
        .await
        .and_then(|document| {
            Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
            //    Ok(HttpResponse::Ok().body("ok"))
        }) {
        Ok(id) => match app_state.container.template.refactor_template(&id).await {
            Ok(cursor) => {
                let templates: Vec<TemplateResponseRefactorModel> = cursor
                    .map(|doc| {
                        let template =
                            bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
                                Ok(template) => match template {
                                    template => template,
                                },
                                Err(_mongodb_error) => bson::Document::new(),
                            })
                            .ok();
                        println!("Tempalte Dezrlized: {:?}", template);
                        TemplateResponseRefactorModel::build_template(template.unwrap())
                    })
                    .collect()
                    .await;
                Ok(HttpResponse::Ok().json(templates.last()))
            }
            Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
        },
        Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }

    // Ok(HttpResponse::Ok().body("ok"))
}

#[post("template/get")] //no need
async fn get_template_by_id(
    app_state: web::Data<crate::AppState>,
    template_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .template
        .refactor_template(&template_data.id)
        .await
    {
        Ok(cursor) => {
            let templates: Vec<TemplateResponseRefactorModel> = cursor
                .map(|doc| {
                    let doc = match doc {
                        Ok(document) => {
                            println!("document :{:?}", document);
                            document
                        }
                        Err(e) => {
                            println!("error :{:?}", e);

                            bson::Document::new()
                        }
                    };

                    let template =
                        bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
                            doc => doc,
                        })
                        .ok();

                    TemplateResponseRefactorModel::build_template(template.unwrap())
                })
                .collect()
                .await;
            if !templates.last().is_none() {
                Ok(HttpResponse::Ok().json(templates.last()))
            } else {
                Err(ContentBuilderCustomResponseError::NotFound)
            }
        }
        Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[put("template/specification/add")] // no need
async fn add_template_specification(
    app_state: web::Data<crate::AppState>,
    mut parts: Parts,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    let form_data = parts.texts.as_hash_map();

    let specs = Specification {
        introduction: Introduction {
            purpose: form_data["purpose"].to_string(),
            document_conventions: form_data["document_conventions"].to_string(),
            intended_audience: form_data["intended_audience"].to_string(),
            project_scope: form_data["project_scope"].to_string(),
        },
        overall_description: OverallDescription {
            perspective: form_data["perspective"].to_string(),
            user_characteristics: form_data["user_characteristics"].to_string(),
            operating_environment: form_data["operating_environment"].to_string(),
            design_implementation_constraints: form_data["design_implementation_constraints"]
                .to_string(),
            user_documentation: form_data["user_documentation"].to_string(),
            assemptions_dependencies: form_data["assemptions_dependencies"].to_string(),
        },
        non_functional_requirements: NonFunctionalRequirements {
            performance_requirements: form_data["performance_requirements"].to_string(),
            safety_requirements: form_data["safety_requirements"].to_string(),
            security_requirements: form_data["security_requirements"].to_string(),
            software_quality_attributes: form_data["software_quality_attributes"].to_string(),
        },
        other_requirements: form_data["other_requirements"].to_string(),
        glossary: form_data["glossary"].to_string(),
        analysis_models: form_data["analysis_models"].to_string(),
        issues_list: form_data["issues_list"].to_string(),
    };
    println!("{:?}", specs);
    match app_state
        .container
        .template
        .update_specification(&form_data["id"].to_string(), specs)
        .await
        .and_then(|document| {
            Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
            //    Ok(HttpResponse::Ok().body("ok"))
        }) {
        Ok(id) => match app_state.container.template.refactor_template(&id).await {
            Ok(cursor) => {
                let templates: Vec<TemplateResponseRefactorModel> = cursor
                    .map(|doc| {
                        let template =
                            bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
                                Ok(template) => match template {
                                    template => template,
                                },
                                Err(_mongodb_error) => bson::Document::new(),
                            })
                            .ok();

                        TemplateResponseRefactorModel::build_template(template.unwrap())
                    })
                    .collect()
                    .await;
                Ok(HttpResponse::Ok().json(templates.last()))
            }
            Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
        },
        Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}
