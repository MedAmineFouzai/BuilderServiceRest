pub mod categories_controller;
pub mod features_controller;
pub mod projects_controller;
pub mod prototypes_controller;
pub mod schema;
pub mod templates_controller;
pub use categories_controller::{
    delete_category, get_all_categories, get_category_by_id, update_category,
    create_category
};
pub use features_controller::{
    add_feature_wireframe, delete_feature, delete_feature_wireframe, get_all_features,
    get_feature_by_id, update_feature,create_feature
};
pub use projects_controller::{
    add_design_project, add_full_build_project, add_mvp_project, add_project, 
    add_proposal_project, change_project_state,  generate_project_specification,
    get_all_project_by_client_id, get_all_projects, get_project_by_id, update_project,
};
pub use prototypes_controller::{
    add_prototype, get_prototype_by_template_id, update_prototype,
};
pub use templates_controller::{
    create_template, add_template_specification, delete_template, get_all_templates,
    get_template_by_id, get_templates_by_categories_id, update_template, update_template_feature,
};

///////////Category CRUD
// #[get("category/all")]
// async fn get_all_categories(
//     app_state: web::Data<crate::AppState>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state.container.category.find_all().await {
//         Ok(cursor) => {
//             let categories: Vec<CategoryResponseModel> = cursor
//                 .map(|document| {
//                     let category:CategoryDeserializeModel = bson::from_document::<CategoryDeserializeModel>(match document {
//                         Ok(category_document) => match category_document {
//                             category_document => category_document,
//                         },
//                         Err(_mongodb_error) => bson::Document::new(),
//                     })
//                     .unwrap();
//                     CategoryResponseModel::build_category(category)
//                 })
//                 .collect::<Vec<CategoryResponseModel>>()
//                 .await;
//             Ok(HttpResponse::Ok().json(categories))
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[post("category/add")]
// async fn add_category(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();
//     let file_name = parts
//         .files
//         .take("image")
//         .pop()
//         .and_then(|file| {
//             file.persist_in(PathBuf::from("./static/uploads/catagorys_imgs"))
//                 .ok()
//         }).and_then(|file_path|{
//             file_path.file_name().and_then(|os_path|{
//                 os_path.to_str().and_then(|sentaized_path|{
//                     Some(sentaized_path.to_string())
//                 })
//             })
//         }).unwrap();

//     match app_state
//         .container
//         .category
//         .insert_one(Category {
//             name: form_data["name"].to_string(),
//             description: form_data["description"].to_string(),
//             image: File {
//                 name: file_name.clone(),
//                 src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/catagorys_imgs/{}", file_name.clone()),
//             },
//         })
//         .await
//     {
//         Ok(category_id) => match category_id.inserted_id.as_object_id() {
//             Some(object_id) => {
//                 match app_state
//                     .container
//                     .category
//                     .find_one_by_id(&object_id.to_string())
//                     .await.and_then(|document|{Ok(match document {
//                         Some(document)=>document,
//                         None => Document::new()
//                     })})
//                 {
//                     Ok(document) =>match  document {

//                             document=>match bson::from_document::<CategoryDeserializeModel>(document){
//                                 Ok(category) => Ok(HttpResponse::Ok()
//                                     .json(CategoryResponseModel::build_category(category))),
//                                 Err(_bson_de_error) => {
//                                     Err(ContentBuilderCustomResponseError::InternalError)
//                                 }
//                             }

//                     }
//                     Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                 }
//             }
//             None => Err(ContentBuilderCustomResponseError::InternalError),
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[put("category/update")]
// async fn update_category(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();

//     let file_name = parts
//     .files
//     .take("image")
//     .pop()
//     .and_then(|file| {
//         file.persist_in(PathBuf::from("./static/uploads/catagorys_imgs"))
//             .ok()
//     }).and_then(|file_path|{
//         file_path.file_name().and_then(|os_path|{
//             os_path.to_str().and_then(|sentaized_path|{
//                 Some(sentaized_path.to_string())
//             })
//         })
//     }).unwrap();

//     match app_state
//         .container
//         .category
//         .update_one(
//             &form_data["id"].to_string(),
//             Category {
//                 name: form_data["name"].to_string(),
//                 description: form_data["description"].to_string(),
//                 image: File {
//                     name: file_name.clone(),
//                     src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/catagorys_imgs/{}", file_name.clone()),
//                 },
//             },
//         )
//         .await.and_then(|document|{Ok(match document {
//             Some(document)=>document,
//             None => Document::new()
//         })})
//     {
//         Ok(document) =>match  document {
//             document=>match bson::from_document::<CategoryDeserializeModel>(document){
//                 Ok(category) => Ok(HttpResponse::Ok()
//                     .json(CategoryResponseModel::build_category(category))),
//                 Err(_bson_de_error) => {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//     }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[delete("category/delete")]
// async fn delete_category(
//     app_state: web::Data<crate::AppState>,
//     category_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .category
//         .delete_one(&category_data.id)
//         .await
//         .and_then(|document| {
//             Ok(match document {
//                 Some(document) => document,
//                 None => bson::Document::new(),
//             })
//         }) {
//         Ok(document) => match  document {
//             document=>match bson::from_document::<CategoryDeserializeModel>(document){
//                 Ok(category) => Ok(HttpResponse::Ok()
//                     .json(CategoryResponseModel::build_category(category))),
//                 Err(_bson_de_error) => {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[post("category/get")]
// async fn get_category_by_id(
//     app_state: web::Data<crate::AppState>,
//     category_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .category
//         .find_one_by_id(&category_data.id)
//         .await
//         .and_then(|document| {
//             Ok(match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//                 }
//             )
//         }) {
//         Ok(document) =>match  document {
//             document=>match bson::from_document::<CategoryDeserializeModel>(document){
//                 Ok(category) => Ok(HttpResponse::Ok()
//                     .json(CategoryResponseModel::build_category(category))),
//                 Err(_bson_de_error) => {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

///////////features CRUD

// #[get("feature/all")]
// async fn get_all_features(
//     app_state: web::Data<crate::AppState>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state.container.feature.find_all().await {
//         Ok(cursor) => {
//             let features: Vec<FeatureResponseModel> = cursor
//                 .map(|doc| {
//                     let feature = bson::from_document::<FeatureDeserializeModel>(match doc {
//                         Ok(feature) => match feature {
//                             feature => feature,
//                         },
//                         Err(_mongodb_error) => bson::Document::new(),
//                     })
//                     .ok();
//                     println!("Feature Dezrlized: {:?}", feature);
//                     FeatureResponseModel::build_feature(feature.unwrap())
//                 })
//                 .collect()
//                 .await;
//             Ok(HttpResponse::Ok().json(features))
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::BadHeaderData),
//     }
// }

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
//         .unwrap()
//         .file_name()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string();

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
//             let file_name = path
//                 .unwrap()
//                 .file_name()
//                 .unwrap()
//                 .to_str()
//                 .unwrap()
//                 .to_string();
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
//             // catagorys: Some(vec![]), // form_data["catagorys"].to_string(),
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
//         Ok(id) => match id.inserted_id.as_object_id() {
//             Some(id) => {
//                 match app_state
//                     .container
//                     .feature
//                     .find_one_by_id(&id.to_string())
//                     .await
//                 {
//                     Ok(result) => {
//                         if result != None {
//                             match bson::from_document::<FeatureDeserializeModel>(result.unwrap()) {
//                                 Ok(feature) => Ok(HttpResponse::Ok()
//                                     .json(FeatureResponseModel::build_feature(feature))),
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
//         .unwrap()
//         .file_name()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string();

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
//             let file_name = path
//                 .unwrap()
//                 .file_name()
//                 .unwrap()
//                 .to_str()
//                 .unwrap()
//                 .to_string();
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
//                 // catagorys: Some(vec![]), // form_data["catagorys"].to_string(),
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
//     // Ok(HttpResponse::Ok().body("ok"))
// }

// #[post("feature/get")]
// async fn get_feature_by_id(
//     app_state: web::Data<crate::AppState>,
//     feature_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .feature
//         .find_one_by_id(&feature_data.id)
//         .await
//         .and_then(|document| {
//             let feature = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(feature)
//         }) {
//         Ok(result) => {
//             match result {
//                 result => {
//                     if !result.is_empty() {
//                         match bson::from_document::<FeatureDeserializeModel>(result) {
//                             Ok(feature) => Ok(HttpResponse::Ok()
//                                 .json(FeatureResponseModel::build_feature(feature))),
//                             Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                         }
//                     } else {
//                         Err(ContentBuilderCustomResponseError::NotFound)
//                     }
//                 }
//             }
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[delete("feature/delete")]
// async fn delete_feature(
//     app_state: web::Data<crate::AppState>,
//     feature_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .feature
//         .delete_one(&feature_data.id)
//         .await
//         .and_then(|document| {
//             let feature = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(feature)
//         }) {
//         Ok(result) => {
//             match result {
//                 result => {
//                     if !result.is_empty() {
//                         match bson::from_document::<FeatureDeserializeModel>(result) {
//                             Ok(feature) => Ok(HttpResponse::Ok()
//                                 .json(FeatureResponseModel::build_feature(feature))),
//                             Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                         }
//                     } else {
//                         Err(ContentBuilderCustomResponseError::NotFound)
//                     }
//                 }
//             }
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[post("feature/wireframe/add")]
// async fn add_feature_wireframe(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let feature_id = parts.texts.as_hash_map()["id"];
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
//             let file_name = path
//                 .unwrap()
//                 .file_name()
//                 .unwrap()
//                 .to_str()
//                 .unwrap()
//                 .to_string();
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
//         .find_one_by_id(feature_id)
//         .await
//         .and_then(|document| {
//             let feature = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(feature)
//         }) {
//         Ok(feature) => match feature {
//             feature => {
//                 println!("Feature Dezrlized: {:?}", feature);
//                 if !feature.is_empty() {
//                     match bson::from_document::<FeatureDeserializeModel>(feature) {
//                         Ok(feature) => {
//                             let mut files: Vec<bson::Document> = Vec::new();
//                             let feature = FeatureResponseModel::build_feature(feature);
//                             for file in images {
//                                 let doc = match app_state
//                                     .container
//                                     .feature
//                                     .add_wireframe(&feature.id, file)
//                                     .await
//                                 {
//                                     Ok(document) => Ok(match document {
//                                         Some(doc) => doc,
//                                         None => bson::Document::new(),
//                                     }),
//                                     Err(_mongodb_error) => {
//                                         Err(ContentBuilderCustomResponseError::InternalError)
//                                     }
//                                 };
//                                 files.push(doc?);
//                             }

//                             match bson::from_document::<FeatureDeserializeModel>(
//                                 files.last().unwrap().clone(),
//                             ) {
//                                 Ok(feature) => Ok(HttpResponse::Ok()
//                                     .json(FeatureResponseModel::build_feature(feature))),
//                                 Err(_bson_de_error) => {
//                                     Err(ContentBuilderCustomResponseError::InternalError)
//                                 }
//                             }
//                         }
//                         Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[delete("feature/wireframe/delete")]
// async fn delete_feature_wireframe(
//     app_state: web::Data<crate::AppState>,
//     wireframe_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .feature
//         .find_wireframe_by_id(&wireframe_data.id)
//         .await
//         .and_then(|cursor| {
//             Ok(async {
//                 let feature = cursor
//                     .map(|doc| {
//                         let feature = bson::from_document::<FeatureDeserializeModel>(match doc {
//                             Ok(feature) => match feature {
//                                 feature => feature,
//                             },
//                             Err(_mongodb_error) => bson::Document::new(),
//                         })
//                         .ok();
//                         // println!("Feature Dezrlized: {:?}", feature);
//                         FeatureResponseModel::build_feature(feature.unwrap())
//                     })
//                     .collect::<Vec<FeatureResponseModel>>()
//                     .await;

//                 Ok(feature)
//             })
//         }) {
//         Ok(result) => match result.await? {
//             result => {
//                 if !result.is_empty() {
//                     let feature = result.last().unwrap();
//                     match app_state
//                         .container
//                         .feature
//                         .delete_wireframe(
//                             &feature.id,
//                             feature
//                                 .wireframes
//                                 .as_ref()
//                                 .and_then(|frame| {
//                                     let file = frame
//                                         .iter()
//                                         .find(|&file| file.id == wireframe_data.id)?
//                                         .clone();
//                                     Some(FileWithId {
//                                         _id: ObjectId::with_string(&file.id).unwrap(),
//                                         name: file.name.clone(),
//                                         src: file.src.clone(),
//                                     })
//                                 })
//                                 .unwrap(),
//                         )
//                         .await
//                     {
//                         Ok(document) => {
//                             match bson::from_document::<FeatureDeserializeModel>(document.unwrap())
//                             {
//                                 Ok(feature) => Ok(HttpResponse::Ok()
//                                     .json(FeatureResponseModel::build_feature(feature))),
//                                 Err(_bson_de_error) => {
//                                     Err(ContentBuilderCustomResponseError::InternalError)
//                                 }
//                             }
//                         }
//                         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
//     // Ok(HttpResponse::Ok().body("ok"))
// }

///////////Template CRUD

// #[get("template/all")]
// async fn get_all_templates(
//     app_state: web::Data<crate::AppState>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state.container.template.find_all().await {
//         Ok(cursor) => {
//             let templates: Vec<TemplateResponseRefactorModel> = cursor
//                 .map(|doc| {
//                     let template =
//                         bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
//                             Ok(template) => match template {
//                                 template => template,
//                             },
//                             Err(_mongodb_error) => bson::Document::new(),
//                         })
//                         .ok();

//                     TemplateResponseRefactorModel::build_template(template.unwrap())
//                 })
//                 .collect()
//                 .await;
//             Ok(HttpResponse::Ok().json(templates))
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::BadHeaderData),
//     }
// }

// #[post("template/add")]
// async fn add_template(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();

//     // let name: String = String::from_utf8(data[0].1.to_vec()).unwrap();
//     // let description: String = String::from_utf8(data[1].1.to_vec()).unwrap();
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

// #[put("template/update")]
// async fn update_template(
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
//         .update_one(
//             &form_data["id"].to_string(),
//             Template {
//                 name: form_data["name"].to_string(),
//                 description: form_data["description"].to_string(),
//                 // catagorys: Some(vec![]), // form_data["catagorys"].to_string(),
//                 image: File {
//                     name: image.clone(),
//                     src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/features_imgs/{}", image.clone()),
//                 },
//                 category: ObjectId::with_string(&form_data["category"].to_string()).unwrap(),
//                 features: Some(vec![]),
//                 specification: Some(Specification::new()),
//                 // prototype_id: Some(
//                 //     // ObjectId::with_string(&form_data["prototype_id"].to_string()).unwrap(),
//                 //     ObjectId::new()
//                 // ),
//             },
//         )
//         .await
//     {
//         Ok(result) => {
//             if result != None {
//                 match bson::from_document::<TemplateDeserializeModel>(result.unwrap()) {
//                     Ok(template) => Ok(
//                         HttpResponse::Ok().json(TemplateResponseModel::build_template(template))
//                     ),
//                     Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                 }
//             } else {
//                 Err(ContentBuilderCustomResponseError::NotFound)
//             }
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
//     // Ok(HttpResponse::Ok().body("ok"))
// }

// #[delete("template/delete")]
// async fn delete_template(
//     app_state: web::Data<crate::AppState>,
//     feature_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .template
//         .delete_one(&feature_data.id)
//         .await
//         .and_then(|document| {
//             let feature = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(feature)
//         }) {
//         Ok(result) => match result {
//             result => {
//                 if !result.is_empty() {
//                     match bson::from_document::<TemplateDeserializeModel>(result) {
//                         Ok(template) => Ok(HttpResponse::Ok()
//                             .json(TemplateResponseModel::build_template(template))),
//                         Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// // #[post("template/feature/add")]
// // async fn add_template_feature(
// //     app_state: web::Data<crate::AppState>,
// //     data: Json<FeatureToAnyModel>,
// // ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
// //     let features_id = data
// //         .features_id
// //         .clone()
// //         .into_iter()
// //         .map(|feature_id| ObjectId::with_string(&feature_id.clone()).unwrap())
// //         .collect::<Vec<ObjectId>>();

// //     match app_state
// //         .container
// //         .template
// //         .add_feature(&data.id, features_id)
// //         .await
// //         .and_then(|document| {
// //             Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
// //             //    Ok(HttpResponse::Ok().body("ok"))
// //         }) {
// //         Ok(id) => match app_state.container.template.refactor_template(&id).await {
// //             Ok(cursor) => {
// //                 let templates: Vec<TemplateResponseRefactorModel> = cursor
// //                     .map(|doc| {
// //                         let template =
// //                             bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
// //                                 Ok(template) => match template {
// //                                     template => template,
// //                                 },
// //                                 Err(_mongodb_error) => bson::Document::new(),
// //                             })
// //                             .ok();
// //                         println!("Tempalte Dezrlized: {:?}", template);
// //                         TemplateResponseRefactorModel::build_template(template.unwrap())
// //                     })
// //                     .collect()
// //                     .await;
// //                 Ok(HttpResponse::Ok().json(templates.last()))
// //             }
// //             Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
// //         },
// //         Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
// //     }

// //     // Ok(HttpResponse::Ok().body("ok"))
// // }

// #[put("template/feature/update")]
// async fn update_template_feature(
//     app_state: web::Data<crate::AppState>,
//     data: Json<FeatureToAnyModel>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let features_id = data
//         .features_id
//         .clone()
//         .into_iter()
//         .map(|feature_id| ObjectId::with_string(&feature_id.clone()).unwrap())
//         .collect::<Vec<ObjectId>>();

//     match app_state
//         .container
//         .template
//         .update_features(&data.id, features_id)
//         .await
//         .and_then(|document| {
//             Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
//             //    Ok(HttpResponse::Ok().body("ok"))
//         }) {
//         Ok(id) => match app_state.container.template.refactor_template(&id).await {
//             Ok(cursor) => {
//                 let templates: Vec<TemplateResponseRefactorModel> = cursor
//                     .map(|doc| {
//                         let template =
//                             bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
//                                 Ok(template) => match template {
//                                     template => template,
//                                 },
//                                 Err(_mongodb_error) => bson::Document::new(),
//                             })
//                             .ok();
//                         println!("Tempalte Dezrlized: {:?}", template);
//                         TemplateResponseRefactorModel::build_template(template.unwrap())
//                     })
//                     .collect()
//                     .await;
//                 Ok(HttpResponse::Ok().json(templates.last()))
//             }
//             Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
//         },
//         Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }

//     // Ok(HttpResponse::Ok().body("ok"))
// }

// // #[delete("template/feature/delete")]
// // async fn delete_template_feature(
// //     app_state: web::Data<crate::AppState>,
// //     data: Json<FeatureToAnyModel>,
// // ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
// //     match app_state
// //         .container
// //         .template
// //         .delete_feature(&data.id, &data.features_id[0])
// //         .await
// //         .and_then(|document| {
// //             Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
// //             //    Ok(HttpResponse::Ok().body("ok"))
// //         }) {
// //         Ok(id) => match app_state.container.template.refactor_template(&id).await {
// //             Ok(cursor) => {
// //                 let templates: Vec<TemplateResponseRefactorModel> = cursor
// //                     .map(|doc| {
// //                         let template =
// //                             bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
// //                                 Ok(template) => match template {
// //                                     template => template,
// //                                 },
// //                                 Err(_mongodb_error) => bson::Document::new(),
// //                             })
// //                             .ok();
// //                         println!("Tempalte Dezrlized: {:?}", template);
// //                         TemplateResponseRefactorModel::build_template(template.unwrap())
// //                     })
// //                     .collect()
// //                     .await;
// //                 Ok(HttpResponse::Ok().json(templates.last()))
// //             }
// //             Err(e) => Err(ContentBuilderCustomResponseError::InternalError),
// //         },
// //         Err(some_error) => Err(ContentBuilderCustomResponseError::InternalError),
// //     }

// //     // Ok(HttpResponse::Ok().body("ok"))
// // }

// #[post("template/get")]
// async fn get_template_by_id(
//     app_state: web::Data<crate::AppState>,
//     template_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .template
//         .refactor_template(&template_data.id)
//         .await
//     {
//         Ok(cursor) => {
//             let templates: Vec<TemplateResponseRefactorModel> = cursor
//                 .map(|doc| {
//                     let doc= match doc {
//                         Ok(document)=>{
//                              println!("document :{:?}",document);
//                              document
//                         },
//                              Err(e)=>{
//                                 println!("error :{:?}",e);

//                                 bson::Document::new()

//                              }
//                     };

//                     let template =
//                         bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
//                             doc=>doc
//                             // Ok(template) => match template {
//                             //     template => template,
//                             // },
//                             // Err(_mongodb_error) => bson::Document::new(),
//                         })
//                         .ok();
//                     println!("Tempalte Dezrlized: {:?}", template);
//                     TemplateResponseRefactorModel::build_template(template.unwrap())
//                 })
//                 .collect()
//                 .await;
//             if !templates.last().is_none() {
//                 Ok(HttpResponse::Ok().json(templates.last()))
//             } else {
//                 println!("Tempalte Dezrlized: {:?}", templates);
//                 Err(ContentBuilderCustomResponseError::NotFound)
//             }
//         }
//         Err(e) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[put("template/specification/add")]
// async fn add_template_specification(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();

//     let specs = Specification {
//         introduction: Introduction {
//             purpose: form_data["purpose"].to_string(),
//             document_conventions: form_data["document_conventions"].to_string(),
//             intended_audience: form_data["intended_audience"].to_string(),
//             project_scope: form_data["project_scope"].to_string(),
//         },
//         overall_description: OverallDescription {
//             perspective: form_data["perspective"].to_string(),
//             user_characteristics: form_data["user_characteristics"].to_string(),
//             operating_environment: form_data["operating_environment"].to_string(),
//             design_implementation_constraints: form_data["design_implementation_constraints"]
//                 .to_string(),
//             user_documentation: form_data["user_documentation"].to_string(),
//             assemptions_dependencies: form_data["assemptions_dependencies"].to_string(),
//         },
//         non_functional_requirements: NonFunctionalRequirements {
//             performance_requirements: form_data["performance_requirements"].to_string(),
//             safety_requirements: form_data["safety_requirements"].to_string(),
//             security_requirements: form_data["security_requirements"].to_string(),
//             software_quality_attributes: form_data["software_quality_attributes"].to_string(),
//         },
//         other_requirements: form_data["other_requirements"].to_string(),
//         glossary: form_data["glossary"].to_string(),
//         analysis_models: form_data["analysis_models"].to_string(),
//         issues_list: form_data["issues_list"].to_string(),
//     };
//     println!("{:?}", specs);
//     match app_state
//         .container
//         .template
//         .update_specification(&form_data["id"].to_string(), specs)
//         .await
//         .and_then(|document| {
//             Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
//             //    Ok(HttpResponse::Ok().body("ok"))
//         }) {
//         Ok(id) => match app_state.container.template.refactor_template(&id).await {
//             Ok(cursor) => {
//                 let templates: Vec<TemplateResponseRefactorModel> = cursor
//                     .map(|doc| {
//                         let template =
//                             bson::from_document::<TemplateReafactorDeserializeModel>(match doc {
//                                 Ok(template) => match template {
//                                     template => template,
//                                 },
//                                 Err(_mongodb_error) => bson::Document::new(),
//                             })
//                             .ok();

//                         TemplateResponseRefactorModel::build_template(template.unwrap())
//                     })
//                     .collect()
//                     .await;
//                 Ok(HttpResponse::Ok().json(templates.last()))
//             }
//             Err(e) => Err(ContentBuilderCustomResponseError::InternalError),
//         },
//         Err(some_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

//////////////////////////////////// ProtoTypeCrud

// #[delete("prototype/delete")]
// async fn delete_prototype(
//     app_state: web::Data<crate::AppState>,
//     prototype_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .prototype
//         .delete_one(&prototype_data.id)
//         .await
//         .and_then(|document| {
//             let feature = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(feature)
//         }) {
//         Ok(result) => match result {
//             result => {
//                 if !result.is_empty() {
//                     match bson::from_document::<ProtoTypeDeserializeModel>(result) {
//                         Ok(prototype) => Ok(HttpResponse::Ok().json(json!({
//                             "id": prototype._id.to_string(),
//                             "template_id": prototype.template_id.to_string(),
//                         }))),
//                         Err(_bson_de_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[post("prototype/add")]
// async fn add_prototype(
//     app_state: web::Data<crate::AppState>,
//     prototype_data: Json<ProtoTypeRequest>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     println!("{:?}", prototype_data);
//     match app_state
//         .container
//         .prototype
//         .insert_one(ProtoType {
//             template_id: ObjectId::with_string(&prototype_data.template_id).unwrap(),
//             prototype: prototype_data
//                 .prototype
//                 .clone()
//                 .into_iter()
//                 .map(|prototype_object| ProtoTypeObject {
//                     feature_id: ObjectId::with_string(&prototype_object.feature_id).unwrap(),
//                     connections: prototype_object
//                         .connections
//                         .into_iter()
//                         .map(|connections| Connections {
//                             to: ObjectId::with_string(&connections.to).unwrap(),
//                             releations: connections.releations,
//                         })
//                         .collect::<Vec<Connections>>(),
//                 })
//                 .collect::<Vec<ProtoTypeObject>>(),
//         })
//         .await
//     {
//         Ok(id) => match id.inserted_id.as_object_id() {
//             Some(_id) => {
//                 match app_state
//                     .container
//                     .prototype
//                     .refactor_one_by_id(&prototype_data.template_id)
//                     .await
//                 {
//                     Ok(cursor) => {
//                         let prototypes: Vec<ProtoTypeResponseModel> = cursor
//                             .map(|doc| {
//                                 let prototype = bson::from_document::<
//                                     ProtoTypeRefactorDeserializeModel,
//                                 >(match doc {
//                                     Ok(prototype) => match prototype {
//                                         prototype => prototype,
//                                     },
//                                     Err(_mongodb_error) => bson::Document::new(),
//                                 })
//                                 .ok();
//                                 println!("Prototype Dezrlized: {:?}", prototype);
//                                 ProtoTypeResponseModel::build_prototype(prototype.unwrap())
//                             })
//                             .collect()
//                             .await;
//                         if !prototypes.last().is_none() {
//                             Ok(HttpResponse::Ok().json(prototypes.last()))
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
// }

// #[post("prototype/get")]
// async fn get_prototype_by_template_id(
//     app_state: web::Data<crate::AppState>,
//     template_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .prototype
//         .refactor_one_by_id(&template_data.id)
//         .await
//     {
//         Ok(cursor) => {
//             let prototypes: Vec<ProtoTypeResponseModel> = cursor
//                 .map(|doc| {
//                     let prototype =
//                         bson::from_document::<ProtoTypeRefactorDeserializeModel>(match doc {
//                             Ok(prototype) => match prototype {
//                                 prototype => prototype,
//                             },
//                             Err(_mongodb_error) => bson::Document::new(),
//                         })
//                         .ok();
//                     println!("Prototype Dezrlized: {:?}", prototype);
//                     ProtoTypeResponseModel::build_prototype(prototype.unwrap())
//                 })
//                 .collect()
//                 .await;
//             if !prototypes.last().is_none() {
//                 Ok(HttpResponse::Ok().json(prototypes.last()))
//             } else {
//                 Err(ContentBuilderCustomResponseError::NotFound)
//             }
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[post("prototype/update")]
// async fn update_prototype(
//     app_state: web::Data<crate::AppState>,
//     prototype_data: Json<ProtoTypeRequest>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     println!("{:?}", prototype_data);
//     match app_state
//         .container
//         .prototype
//         .update_one(
//             &prototype_data.template_id,
//             ProtoType {
//                 template_id: ObjectId::with_string(&prototype_data.template_id).unwrap(),
//                 prototype: prototype_data
//                     .prototype
//                     .clone()
//                     .into_iter()
//                     .map(|prototype_object| ProtoTypeObject {
//                         feature_id: ObjectId::with_string(&prototype_object.feature_id).unwrap(),
//                         connections: prototype_object
//                             .connections
//                             .into_iter()
//                             .map(|connections| Connections {
//                                 to: ObjectId::with_string(&connections.to).unwrap(),
//                                 releations: connections.releations,
//                             })
//                             .collect::<Vec<Connections>>(),
//                     })
//                     .collect::<Vec<ProtoTypeObject>>(),
//             },
//         )
//         .await
//     {
//         Ok(document) => match document {
//             Some(_doc) => {
//                 match app_state
//                     .container
//                     .prototype
//                     .refactor_one_by_id(&prototype_data.template_id)
//                     .await
//                 {
//                     Ok(cursor) => {
//                         let prototypes: Vec<ProtoTypeResponseModel> = cursor
//                             .map(|doc| {
//                                 let prototype = bson::from_document::<
//                                     ProtoTypeRefactorDeserializeModel,
//                                 >(match doc {
//                                     Ok(prototype) => match prototype {
//                                         prototype => prototype,
//                                     },
//                                     Err(_mongodb_error) => bson::Document::new(),
//                                 })
//                                 .ok();
//                                 println!("Prototype Dezrlized: {:?}", prototype);
//                                 ProtoTypeResponseModel::build_prototype(prototype.unwrap())
//                             })
//                             .collect()
//                             .await;
//                         if !prototypes.last().is_none() {
//                             Ok(HttpResponse::Ok().json(prototypes.last()))
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
// }

////////////////////Project Crud

// #[post("project/add")]
// async fn add_project(
//     app_state: web::Data<crate::AppState>,
//     project_data: Json<ProjectRequestModel>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     println!("{:?}", project_data);
//     match app_state
//         .container
//         .project
//         .insert_one(Project {
//             client_id: ObjectId::with_string(&project_data.client_id).unwrap(),
//             name: project_data.name.clone(),
//             platforms: project_data.platforms.clone(),
//             template: ObjectId::with_string(&project_data.template).unwrap(),
//             features: project_data
//                 .features
//                 .clone()
//                 .into_iter()
//                 .map(|feature_id| ObjectId::with_string(&feature_id).unwrap())
//                 .collect::<Vec<ObjectId>>(),
//             state: project_data.state.clone(),
//             proposal: project_data.proposal.clone(),
//             delivrable: project_data.delivrable.clone(),
//             total_price: project_data.total_price,
//         })
//         .await
//     {
//         Ok(id) => match id.inserted_id.as_object_id() {
//             Some(_id) => {
//                 match app_state
//                     .container
//                     .project
//                     .refactor_one_by_id(&_id.to_string())
//                     .await
//                 {
//                     Ok(cursor) => {
//                         let projects: Vec<ProjectResponseModel> = cursor
//                             .map(|doc| {
//                                 let project =
//                                     bson::from_document::<ProjectDeserializeModel>(match doc {
//                                         Ok(project) => match project {
//                                             project => project,
//                                         },
//                                         Err(_mongodb_error) => bson::Document::new(),
//                                     })
//                                     .ok();
//                                 println!("Prototype Dezrlized: {:?}", project);
//                                 ProjectResponseModel::build_project(project.unwrap())
//                             })
//                             .collect()
//                             .await;
//                         if !projects.last().is_none() {
//                             Ok(HttpResponse::Ok().json(projects.last()))
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
// }

// #[post("project/get")]
// async fn get_project_by_id(
//     app_state: web::Data<crate::AppState>,
//     project_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .project
//         .refactor_one_by_id(&project_data.id)
//         .await
//     {
//         Ok(cursor) => {
//             let projects: Vec<ProjectResponseModel> = cursor
//                 .map(|doc| {
//                     let project = bson::from_document::<ProjectDeserializeModel>(match doc {
//                         Ok(project) => match project {
//                             project => project,
//                         },
//                         Err(_mongodb_error) => bson::Document::new(),
//                     })
//                     .ok();
//                     println!("Project Dezrlized: {:?}", project);
//                     ProjectResponseModel::build_project(project.unwrap())
//                 })
//                 .collect()
//                 .await;
//             if !projects.last().is_none() {
//                 Ok(HttpResponse::Ok().json(projects.last()))
//             } else {
//                 Err(ContentBuilderCustomResponseError::NotFound)
//             }
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[post("project/user/get")]
// async fn get_all_project_by_client_id(
//     app_state: web::Data<crate::AppState>,
//     client_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .project
//         .refactor_one_by_client_id(&client_data.id)
//         .await
//     {
//         Ok(cursor) => {
//             let projects: Vec<ProjectResponseModel> = cursor
//                 .map(|doc| {
//                     let project = bson::from_document::<ProjectDeserializeModel>(match doc {
//                         Ok(project) => match project {
//                             project => project,
//                         },
//                         Err(_mongodb_error) => bson::Document::new(),
//                     })
//                     .ok();
//                     println!("Project Dezrlized: {:?}", project);
//                     ProjectResponseModel::build_project(project.unwrap())
//                 })
//                 .collect()
//                 .await;

//             Ok(HttpResponse::Ok().json(projects))
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[delete("project/state")]
// async fn archive_project(
//     app_state: web::Data<crate::AppState>,
//     project_data: Json<ProjectState>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .project
//         .update_state(&project_data.id, &project_data.state)
//         .await
//         .and_then(|document| {
//             let project = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(project)
//         }) {
//         Ok(result) => match result {
//             result => {
//                 if !result.is_empty() {
//                     match app_state
//                         .container
//                         .project
//                         .refactor_one_by_id(&result.get_object_id("_id").unwrap().to_string())
//                         .await
//                     {
//                         Ok(cursor) => {
//                             let projects: Vec<ProjectResponseModel> = cursor
//                                 .map(|doc| {
//                                     let project =
//                                         bson::from_document::<ProjectDeserializeModel>(match doc {
//                                             Ok(project) => match project {
//                                                 project => project,
//                                             },
//                                             Err(_mongodb_error) => bson::Document::new(),
//                                         })
//                                         .ok();
//                                     println!("Project Dezrlized: {:?}", project);
//                                     ProjectResponseModel::build_project(project.unwrap())
//                                 })
//                                 .collect()
//                                 .await;
//                             if !projects.is_empty() {
//                                 Ok(HttpResponse::Ok().json(projects))
//                             } else {
//                                 Err(ContentBuilderCustomResponseError::NotFound)
//                             }
//                         }
//                         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[get("project/all")]
// async fn get_all_projects(
//     app_state: web::Data<crate::AppState>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state.container.project.find_all().await {
//         Ok(cursor) => {
//             let projects: Vec<ProjectResponseModel> = cursor
//                 .map(|doc| {
//                     let project = bson::from_document::<ProjectDeserializeModel>(match doc {
//                         Ok(project) => match project {
//                             project => project,
//                         },
//                         Err(_mongodb_error) => bson::Document::new(),
//                     })
//                     .ok();
//                     println!("Project Dezrlized: {:?}", project);
//                     ProjectResponseModel::build_project(project.unwrap())
//                 })
//                 .collect()
//                 .await;

//             Ok(HttpResponse::Ok().json(projects))
//         }
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[post("project/feature/add")]
// async fn add_project_feature(
//     app_state: web::Data<crate::AppState>,
//     data: Json<FeatureToAnyModel>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let features_id = data
//         .features_id
//         .clone()
//         .into_iter()
//         .map(|feature_id| ObjectId::with_string(&feature_id.clone()).unwrap())
//         .collect::<Vec<ObjectId>>();

//     match app_state
//         .container
//         .project
//         .add_feature(&data.id, features_id)
//         .await
//         .and_then(|document| {
//             Ok(document.unwrap().get_object_id("_id").unwrap().to_string())
//             //    Ok(HttpResponse::Ok().body("ok"))
//         }) {
//         Ok(id) => match app_state.container.project.refactor_one_by_id(&id).await {
//             Ok(cursor) => {
//                 let projects: Vec<ProjectResponseModel> = cursor
//                     .map(|doc| {
//                         let project = bson::from_document::<ProjectDeserializeModel>(match doc {
//                             Ok(project) => match project {
//                                 project => project,
//                             },
//                             Err(_mongodb_error) => bson::Document::new(),
//                         })
//                         .ok();
//                         println!("Tempalte Dezrlized: {:?}", project);
//                         ProjectResponseModel::build_project(project.unwrap())
//                     })
//                     .collect()
//                     .await;

//                 Ok(HttpResponse::Ok().json(projects.last()))
//             }
//             Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
//         },
//         Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[delete("project/feature/delete")]
// async fn delete_project_feature(
//     app_state: web::Data<crate::AppState>,
//     data: Json<FeatureToAnyModel>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .project
//         .delete_feature(&data.id, &data.features_id[0])
//         .await
//         .and_then(|document| Ok(document.unwrap().get_object_id("_id").unwrap().to_string()))
//     {
//         Ok(id) => match app_state.container.project.refactor_one_by_id(&id).await {
//             Ok(cursor) => {
//                 let projects: Vec<ProjectResponseModel> = cursor
//                     .map(|doc| {
//                         let project = bson::from_document::<ProjectDeserializeModel>(match doc {
//                             Ok(project) => match project {
//                                 project => project,
//                             },
//                             Err(_mongodb_error) => bson::Document::new(),
//                         })
//                         .ok();
//                         println!("Tempalte Dezrlized: {:?}", project);
//                         ProjectResponseModel::build_project(project.unwrap())
//                     })
//                     .collect()
//                     .await;

//                 Ok(HttpResponse::Ok().json(projects.last()))
//             }
//             Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
//         },
//         Err(_some_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[put("project/update")]
// async fn update_project(
//     app_state: web::Data<crate::AppState>,
//     project_data: Json<ProjectRequestModel>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     println!("{:?}", project_data);
//     match app_state
//         .container
//         .project
//         .update_one(
//             &project_data.client_id,
//             Project {
//                 client_id: ObjectId::with_string(&project_data.client_id).unwrap(),
//                 name: project_data.name.clone(),
//                 platforms: project_data.platforms.clone(),
//                 template: ObjectId::with_string(&project_data.template).unwrap(),
//                 features: project_data
//                     .features
//                     .clone()
//                     .into_iter()
//                     .map(|feature_id| ObjectId::with_string(&feature_id).unwrap())
//                     .collect::<Vec<ObjectId>>(),
//                 state: project_data.state.clone(),
//                 proposal: project_data.proposal.clone(),
//                 delivrable: project_data.delivrable.clone(),
//                 total_price: project_data.total_price,
//             },
//         )
//         .await
//     {
//         Ok(doc) => match doc {
//             Some(doc) => {
//                 match app_state
//                     .container
//                     .project
//                     .refactor_one_by_id(&doc.get_object_id("_id").unwrap().to_string())
//                     .await
//                 {
//                     Ok(cursor) => {
//                         let projects: Vec<ProjectResponseModel> = cursor
//                             .map(|doc| {
//                                 let project =
//                                     bson::from_document::<ProjectDeserializeModel>(match doc {
//                                         Ok(project) => match project {
//                                             project => project,
//                                         },
//                                         Err(_mongodb_error) => bson::Document::new(),
//                                     })
//                                     .ok();
//                                 println!("Prototype Dezrlized: {:?}", project);
//                                 ProjectResponseModel::build_project(project.unwrap())
//                             })
//                             .collect()
//                             .await;
//                         if !projects.last().is_none() {
//                             Ok(HttpResponse::Ok().json(projects.last()))
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
// }

// #[post("project/specification/generate")]
// async fn generate_project_specification(
//     app_state: web::Data<crate::AppState>,
//     project_data: Json<SerlizedId>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     Ok(HttpResponse::Ok().body("ok"))
// }

// #[put("project/full_build/add")]
// async fn add_full_build_project(
//     app_state: web::Data<crate::AppState>,
//     project_data: Json<ProjectFullBuild>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .project
//         .update_full_build(&project_data.id, &project_data.full_build)
//         .await
//         .and_then(|document| {
//             let project = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(project)
//         }) {
//         Ok(result) => match result {
//             result => {
//                 if !result.is_empty() {
//                     match app_state
//                         .container
//                         .project
//                         .refactor_one_by_id(&result.get_object_id("_id").unwrap().to_string())
//                         .await
//                     {
//                         Ok(cursor) => {
//                             let projects: Vec<ProjectResponseModel> = cursor
//                                 .map(|doc| {
//                                     let project =
//                                         bson::from_document::<ProjectDeserializeModel>(match doc {
//                                             Ok(project) => match project {
//                                                 project => project,
//                                             },
//                                             Err(_mongodb_error) => bson::Document::new(),
//                                         })
//                                         .ok();
//                                     println!("Project Dezrlized: {:?}", project);
//                                     ProjectResponseModel::build_project(project.unwrap())
//                                 })
//                                 .collect()
//                                 .await;
//                             if !projects.is_empty() {
//                                 Ok(HttpResponse::Ok().json(projects))
//                             } else {
//                                 Err(ContentBuilderCustomResponseError::NotFound)
//                             }
//                         }
//                         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[put("project/proposal/add")]
// async fn add_proposal_project(
//     app_state: web::Data<crate::AppState>,
//     project_data: Json<ProjectProposal>,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     match app_state
//         .container
//         .project
//         .update_propsal(&project_data.id, project_data.proposal.clone())
//         .await
//         .and_then(|document| {
//             let project = match document {
//                 Some(doc) => doc,
//                 None => bson::Document::new(),
//             };
//             Ok(project)
//         }) {
//         Ok(result) => match result {
//             result => {
//                 if !result.is_empty() {
//                     match app_state
//                         .container
//                         .project
//                         .refactor_one_by_id(&result.get_object_id("_id").unwrap().to_string())
//                         .await
//                     {
//                         Ok(cursor) => {
//                             let projects: Vec<ProjectResponseModel> = cursor
//                                 .map(|doc| {
//                                     let project =
//                                         bson::from_document::<ProjectDeserializeModel>(match doc {
//                                             Ok(project) => match project {
//                                                 project => project,
//                                             },
//                                             Err(_mongodb_error) => bson::Document::new(),
//                                         })
//                                         .ok();
//                                     println!("Project Dezrlized: {:?}", project);
//                                     ProjectResponseModel::build_project(project.unwrap())
//                                 })
//                                 .collect()
//                                 .await;
//                             if !projects.is_empty() {
//                                 Ok(HttpResponse::Ok().json(projects))
//                             } else {
//                                 Err(ContentBuilderCustomResponseError::NotFound)
//                             }
//                         }
//                         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[put("project/mvp/add")]
// async fn add_mvp_project(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();

//     let file_name = parts
//         .files
//         .take("file")
//         .pop()
//         .and_then(|file| {
//             file.persist_in(PathBuf::from("./static/uploads/projects_files"))
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
//         .project
//         .update_mvp(
//             &form_data["id"].to_string(),
//             File {
//                 name: file_name.clone(),
//                 src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/projects_files/{}", file_name.clone()),
//             },
//         )
//         .await
//     {
//         Ok(doc) => match doc.unwrap() {
//             doc => {
//                 if !doc.is_empty() {
//                     match app_state
//                         .container
//                         .project
//                         .refactor_one_by_id(&doc.get_object_id("_id").unwrap().to_string())
//                         .await
//                     {
//                         Ok(cursor) => {
//                             let projects: Vec<ProjectResponseModel> = cursor
//                                 .map(|doc| {
//                                     let project =
//                                         bson::from_document::<ProjectDeserializeModel>(match doc {
//                                             Ok(project) => match project {
//                                                 project => project,
//                                             },
//                                             Err(_mongodb_error) => bson::Document::new(),
//                                         })
//                                         .ok();
//                                     println!("Project Dezrlized: {:?}", project);
//                                     ProjectResponseModel::build_project(project.unwrap())
//                                 })
//                                 .collect()
//                                 .await;
//                             if !projects.is_empty() {
//                                 Ok(HttpResponse::Ok().json(projects))
//                             } else {
//                                 Err(ContentBuilderCustomResponseError::NotFound)
//                             }
//                         }
//                         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }

// #[put("project/design/add")]
// async fn add_design_project(
//     app_state: web::Data<crate::AppState>,
//     mut parts: Parts,
// ) -> Result<HttpResponse, ContentBuilderCustomResponseError> {
//     let form_data = parts.texts.as_hash_map();

//     let file_name = parts
//         .files
//         .take("file")
//         .pop()
//         .and_then(|file| {
//             file.persist_in(PathBuf::from("./static/uploads/projects_files"))
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
//         .project
//         .update_design(
//             &form_data["id"].to_string(),
//             File {
//                 name: file_name.clone(),
//                 src: format!("https://astrobuild-builder-service-v1.herokuapp.com/media/static/uploads/projects_files/{}", file_name.clone()),
//             },
//         )
//         .await
//     {
//         Ok(doc) => match doc.unwrap() {
//             doc => {
//                 if !doc.is_empty() {
//                     match app_state
//                         .container
//                         .project
//                         .refactor_one_by_id(&doc.get_object_id("_id").unwrap().to_string())
//                         .await
//                     {
//                         Ok(cursor) => {
//                             let projects: Vec<ProjectResponseModel> = cursor
//                                 .map(|doc| {
//                                     let project =
//                                         bson::from_document::<ProjectDeserializeModel>(match doc {
//                                             Ok(project) => match project {
//                                                 project => project,
//                                             },
//                                             Err(_mongodb_error) => bson::Document::new(),
//                                         })
//                                         .ok();
//                                     println!("Project Dezrlized: {:?}", project);
//                                     ProjectResponseModel::build_project(project.unwrap())
//                                 })
//                                 .collect()
//                                 .await;
//                             if !projects.is_empty() {
//                                 Ok(HttpResponse::Ok().json(projects))
//                             } else {
//                                 Err(ContentBuilderCustomResponseError::NotFound)
//                             }
//                         }
//                         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//                     }
//                 } else {
//                     Err(ContentBuilderCustomResponseError::NotFound)
//                 }
//             }
//         },
//         Err(_mongodb_error) => Err(ContentBuilderCustomResponseError::InternalError),
//     }
// }
