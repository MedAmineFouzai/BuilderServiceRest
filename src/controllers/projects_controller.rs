extern crate jsonwebtoken as jwt;
use super::schema::{
    FeatureToAnyModel, File, Project, ProjectDeserializeModel, ProjectFullBuild, ProjectProposal,
    ProjectRequestModel, ProjectResponseModel, ProjectState, SerlizedId,
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

#[post("project/add")]
async fn add_project(
    app_state: web::Data<crate::AppState>,
    project_data: Json<ProjectRequestModel>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    println!("{:?}", project_data);
    match app_state
        .container
        .project
        .insert_one(Project {
            client_id: ObjectId::with_string(&project_data.client_id).unwrap(),
            name: project_data.name.clone(),
            platforms: project_data.platforms.clone(),
            template: ObjectId::with_string(&project_data.template).unwrap(),
            features: project_data
                .features
                .clone()
                .into_iter()
                .map(|feature_id| ObjectId::with_string(&feature_id).unwrap())
                .collect::<Vec<ObjectId>>(),
            state: project_data.state.clone(),
            proposal: project_data.proposal.clone(),
            delivrable: project_data.delivrable.clone(),
            total_price: project_data.total_price,
        })
        .await
    {
        Ok(id) => match id.inserted_id.as_object_id() {
            Some(_id) => {
                match app_state
                    .container
                    .project
                    .refactor_one_by_id(&_id.to_string())
                    .await
                {
                    Ok(cursor) => {
                        let projects: Vec<ProjectResponseModel> = cursor
                            .map(|doc| {
                                let project =
                                    bson::from_document::<ProjectDeserializeModel>(match doc {
                                        Ok(project) => match project {
                                            project => project,
                                        },
                                        Err(_mongodb_error) => bson::Document::new(),
                                    })
                                    .ok();
                                println!("Prototype Dezrlized: {:?}", project);
                                ProjectResponseModel::build_project(project.unwrap())
                            })
                            .collect()
                            .await;
                        if !projects.last().is_none() {
                            Ok(HttpResponse::Ok().json(projects.last()))
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

#[post("project/get")]
async fn get_project_by_id(
    app_state: web::Data<crate::AppState>,
    project_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .project
        .refactor_one_by_id(&project_data.id)
        .await
    {
        Ok(cursor) => {
            let projects: Vec<ProjectResponseModel> = cursor
                .map(|doc| {
                    let project = bson::from_document::<ProjectDeserializeModel>(match doc {
                        Ok(project) => match project {
                            project => project,
                        },
                        Err(_mongodb_error) => bson::Document::new(),
                    })
                    .ok();
                    println!("Project Dezrlized: {:?}", project);
                    ProjectResponseModel::build_project(project.unwrap())
                })
                .collect()
                .await;
            if !projects.last().is_none() {
                Ok(HttpResponse::Ok().json(projects.last()))
            } else {
                Err(ContentBuilderCustomResponseError::NotFound)
            }
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[post("project/user/get")]
async fn get_all_project_by_client_id(
    app_state: web::Data<crate::AppState>,
    client_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .project
        .refactor_one_by_client_id(&client_data.id)
        .await
    {
        Ok(cursor) => {
            let projects: Vec<ProjectResponseModel> = cursor
                .map(|doc| {
                    let project = bson::from_document::<ProjectDeserializeModel>(match doc {
                        Ok(project) => match project {
                            project => project,
                        },
                        Err(_mongodb_error) => bson::Document::new(),
                    })
                    .ok();
                    println!("Project Dezrlized: {:?}", project);
                    ProjectResponseModel::build_project(project.unwrap())
                })
                .collect()
                .await;

            Ok(HttpResponse::Ok().json(projects))
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[delete("project/state")]
async fn archive_project(
    app_state: web::Data<crate::AppState>,
    project_data: Json<ProjectState>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .project
        .update_state(&project_data.id, &project_data.state)
        .await
        .and_then(|document| {
            let project = match document {
                Some(doc) => doc,
                None => bson::Document::new(),
            };
            Ok(project)
        }) {
        Ok(result) => match result {
            result => {
                if !result.is_empty() {
                    match app_state
                        .container
                        .project
                        .refactor_one_by_id(&result.get_object_id("_id").unwrap().to_string())
                        .await
                    {
                        Ok(cursor) => {
                            let projects: Vec<ProjectResponseModel> = cursor
                                .map(|doc| {
                                    let project =
                                        bson::from_document::<ProjectDeserializeModel>(match doc {
                                            Ok(project) => match project {
                                                project => project,
                                            },
                                            Err(_mongodb_error) => bson::Document::new(),
                                        })
                                        .ok();
                                    println!("Project Dezrlized: {:?}", project);
                                    ProjectResponseModel::build_project(project.unwrap())
                                })
                                .collect()
                                .await;
                            if !projects.is_empty() {
                                Ok(HttpResponse::Ok().json(projects))
                            } else {
                                Err(ContentBuilderCustomResponseError::NotFound)
                            }
                        }
                        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
                    }
                } else {
                    Err(ContentBuilderCustomResponseError::NotFound)
                }
            }
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[get("project/all")]
async fn get_all_projects(
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state.container.project.find_all().await {
        Ok(cursor) => {
            let projects: Vec<ProjectResponseModel> = cursor
                .map(|doc| {
                    let project = bson::from_document::<ProjectDeserializeModel>(match doc {
                        Ok(project) => match project {
                            project => project,
                        },
                        Err(_mongodb_error) => bson::Document::new(),
                    })
                    .ok();
                    println!("Project Dezrlized: {:?}", project);
                    ProjectResponseModel::build_project(project.unwrap())
                })
                .collect()
                .await;

            Ok(HttpResponse::Ok().json(projects))
        }
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[post("project/feature/add")]
async fn add_project_feature(
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
        .project
        .add_feature(&data.id, features_id)
        .await
        .and_then(|document| {
            Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
            //    Ok(HttpResponse::Ok().body("ok"))
        }) {
        Ok(id) => match app_state.container.project.refactor_one_by_id(&id).await {
            Ok(cursor) => {
                let projects: Vec<ProjectResponseModel> = cursor
                    .map(|doc| {
                        let project = bson::from_document::<ProjectDeserializeModel>(match doc {
                            Ok(project) => match project {
                                project => project,
                            },
                            Err(_mongodb_error) => bson::Document::new(),
                        })
                        .ok();
                        println!("Tempalte Dezrlized: {:?}", project);
                        ProjectResponseModel::build_project(project.unwrap())
                    })
                    .collect()
                    .await;

                Ok(HttpResponse::Ok().json(projects.last()))
            }
            Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
        },
        Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[delete("project/feature/delete")]
async fn delete_project_feature(
    app_state: web::Data<crate::AppState>,
    data: Json<FeatureToAnyModel>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .project
        .delete_feature(&data.id, &data.features_id[0])
        .await
        .and_then(|document| Ok(document.unwrap().get_object_id("_id").unwrap().to_string()))
    {
        Ok(id) => match app_state.container.project.refactor_one_by_id(&id).await {
            Ok(cursor) => {
                let projects: Vec<ProjectResponseModel> = cursor
                    .map(|doc| {
                        let project = bson::from_document::<ProjectDeserializeModel>(match doc {
                            Ok(project) => match project {
                                project => project,
                            },
                            Err(_mongodb_error) => bson::Document::new(),
                        })
                        .ok();
                        println!("Tempalte Dezrlized: {:?}", project);
                        ProjectResponseModel::build_project(project.unwrap())
                    })
                    .collect()
                    .await;

                Ok(HttpResponse::Ok().json(projects.last()))
            }
            Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
        },
        Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[put("project/update")]
async fn update_project(
    app_state: web::Data<crate::AppState>,
    project_data: Json<ProjectRequestModel>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    println!("{:?}", project_data);
    match app_state
        .container
        .project
        .update_one(
            &project_data.client_id,
            Project {
                client_id: ObjectId::with_string(&project_data.client_id).unwrap(),
                name: project_data.name.clone(),
                platforms: project_data.platforms.clone(),
                template: ObjectId::with_string(&project_data.template).unwrap(),
                features: project_data
                    .features
                    .clone()
                    .into_iter()
                    .map(|feature_id| ObjectId::with_string(&feature_id).unwrap())
                    .collect::<Vec<ObjectId>>(),
                state: project_data.state.clone(),
                proposal: project_data.proposal.clone(),
                delivrable: project_data.delivrable.clone(),
                total_price: project_data.total_price,
            },
        )
        .await
    {
        Ok(doc) => match doc {
            Some(doc) => {
                match app_state
                    .container
                    .project
                    .refactor_one_by_id(&doc.get_object_id("_id").unwrap().to_string())
                    .await
                {
                    Ok(cursor) => {
                        let projects: Vec<ProjectResponseModel> = cursor
                            .map(|doc| {
                                let project =
                                    bson::from_document::<ProjectDeserializeModel>(match doc {
                                        Ok(project) => match project {
                                            project => project,
                                        },
                                        Err(_mongodb_error) => bson::Document::new(),
                                    })
                                    .ok();
                                println!("Prototype Dezrlized: {:?}", project);
                                ProjectResponseModel::build_project(project.unwrap())
                            })
                            .collect()
                            .await;
                        if !projects.last().is_none() {
                            Ok(HttpResponse::Ok().json(projects.last()))
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

#[post("project/specification/generate")]
async fn generate_project_specification(
    app_state: web::Data<crate::AppState>,
    project_data: Json<SerlizedId>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    Ok(HttpResponse::Ok().body("ok"))
}

#[put("project/full_build/add")]
async fn add_full_build_project(
    app_state: web::Data<crate::AppState>,
    project_data: Json<ProjectFullBuild>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .project
        .update_full_build(&project_data.id, &project_data.full_build)
        .await
        .and_then(|document| {
            let project = match document {
                Some(doc) => doc,
                None => bson::Document::new(),
            };
            Ok(project)
        }) {
        Ok(result) => match result {
            result => {
                if !result.is_empty() {
                    match app_state
                        .container
                        .project
                        .refactor_one_by_id(&result.get_object_id("_id").unwrap().to_string())
                        .await
                    {
                        Ok(cursor) => {
                            let projects: Vec<ProjectResponseModel> = cursor
                                .map(|doc| {
                                    let project =
                                        bson::from_document::<ProjectDeserializeModel>(match doc {
                                            Ok(project) => match project {
                                                project => project,
                                            },
                                            Err(_mongodb_error) => bson::Document::new(),
                                        })
                                        .ok();
                                    println!("Project Dezrlized: {:?}", project);
                                    ProjectResponseModel::build_project(project.unwrap())
                                })
                                .collect()
                                .await;
                            if !projects.is_empty() {
                                Ok(HttpResponse::Ok().json(projects))
                            } else {
                                Err(ContentBuilderCustomResponseError::NotFound)
                            }
                        }
                        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
                    }
                } else {
                    Err(ContentBuilderCustomResponseError::NotFound)
                }
            }
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[put("project/proposal/add")]
async fn add_proposal_project(
    app_state: web::Data<crate::AppState>,
    project_data: Json<ProjectProposal>,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    match app_state
        .container
        .project
        .update_propsal(&project_data.id, project_data.proposal.clone())
        .await
        .and_then(|document| {
            let project = match document {
                Some(doc) => doc,
                None => bson::Document::new(),
            };
            Ok(project)
        }) {
        Ok(result) => match result {
            result => {
                if !result.is_empty() {
                    match app_state
                        .container
                        .project
                        .refactor_one_by_id(&result.get_object_id("_id").unwrap().to_string())
                        .await
                    {
                        Ok(cursor) => {
                            let projects: Vec<ProjectResponseModel> = cursor
                                .map(|doc| {
                                    let project =
                                        bson::from_document::<ProjectDeserializeModel>(match doc {
                                            Ok(project) => match project {
                                                project => project,
                                            },
                                            Err(_mongodb_error) => bson::Document::new(),
                                        })
                                        .ok();
                                    println!("Project Dezrlized: {:?}", project);
                                    ProjectResponseModel::build_project(project.unwrap())
                                })
                                .collect()
                                .await;
                            if !projects.is_empty() {
                                Ok(HttpResponse::Ok().json(projects))
                            } else {
                                Err(ContentBuilderCustomResponseError::NotFound)
                            }
                        }
                        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
                    }
                } else {
                    Err(ContentBuilderCustomResponseError::NotFound)
                }
            }
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[put("project/mvp/add")]
async fn add_mvp_project(
    app_state: web::Data<crate::AppState>,
    mut parts: Parts,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    let form_data = parts.texts.as_hash_map();

    let file_name = parts
        .files
        .take("file")
        .pop()
        .and_then(|file| {
            file.persist_in(PathBuf::from("./static/uploads/projects_files"))
                .ok()
        })
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match app_state
        .container
        .project
        .update_mvp(
            &form_data["id"].to_string(),
            File {
                name: file_name.clone(),
                src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/projects_files/{}", file_name.clone()),
            },
        )
        .await
    {
        Ok(doc) => match doc.unwrap() {
            doc => {
                if !doc.is_empty() {
                    match app_state
                        .container
                        .project
                        .refactor_one_by_id(&doc.get_object_id("_id").unwrap().to_string())
                        .await
                    {
                        Ok(cursor) => {
                            let projects: Vec<ProjectResponseModel> = cursor
                                .map(|doc| {
                                    let project =
                                        bson::from_document::<ProjectDeserializeModel>(match doc {
                                            Ok(project) => match project {
                                                project => project,
                                            },
                                            Err(_mongodb_error) => bson::Document::new(),
                                        })
                                        .ok();
                                    println!("Project Dezrlized: {:?}", project);
                                    ProjectResponseModel::build_project(project.unwrap())
                                })
                                .collect()
                                .await;
                            if !projects.is_empty() {
                                Ok(HttpResponse::Ok().json(projects))
                            } else {
                                Err(ContentBuilderCustomResponseError::NotFound)
                            }
                        }
                        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
                    }
                } else {
                    Err(ContentBuilderCustomResponseError::NotFound)
                }
            }
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}

#[put("project/design/add")]
async fn add_design_project(
    app_state: web::Data<crate::AppState>,
    mut parts: Parts,
) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
    let form_data = parts.texts.as_hash_map();

    let file_name = parts
        .files
        .take("file")
        .pop()
        .and_then(|file| {
            file.persist_in(PathBuf::from("./static/uploads/projects_files"))
                .ok()
        })
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match app_state
        .container
        .project
        .update_design(
            &form_data["id"].to_string(),
            File {
                name: file_name.clone(),
                src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/projects_files/{}", file_name.clone()),
            },
        )
        .await
    {
        Ok(doc) => match doc.unwrap() {
            doc => {
                if !doc.is_empty() {
                    match app_state
                        .container
                        .project
                        .refactor_one_by_id(&doc.get_object_id("_id").unwrap().to_string())
                        .await
                    {
                        Ok(cursor) => {
                            let projects: Vec<ProjectResponseModel> = cursor
                                .map(|doc| {
                                    let project =
                                        bson::from_document::<ProjectDeserializeModel>(match doc {
                                            Ok(project) => match project {
                                                project => project,
                                            },
                                            Err(_mongodb_error) => bson::Document::new(),
                                        })
                                        .ok();
                                    println!("Project Dezrlized: {:?}", project);
                                    ProjectResponseModel::build_project(project.unwrap())
                                })
                                .collect()
                                .await;
                            if !projects.is_empty() {
                                Ok(HttpResponse::Ok().json(projects))
                            } else {
                                Err(ContentBuilderCustomResponseError::NotFound)
                            }
                        }
                        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
                    }
                } else {
                    Err(ContentBuilderCustomResponseError::NotFound)
                }
            }
        },
        Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
    }
}
