use bson::oid::ObjectId;
use futures::stream::{Empty, StreamFuture};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWithId {
    pub _id: ObjectId,
    pub name: String,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWithOutOId {
    pub id: String,
    pub name: String,
    pub src: String,
}

//////////////////Category schema
#[derive(Debug, Serialize, Deserialize)]
pub struct SerlizedId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub description: String,
    pub image: File,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: File,
}

impl CategoryResponseModel {
    pub fn build_category(category: CategoryDeserializeModel) -> CategoryResponseModel {
        CategoryResponseModel {
            id: category._id.to_string(),
            name: category.name,
            description: category.description,
            image: category.image,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDeserializeModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub image: File,
}

//////////////////Feature schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDeserializeModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub feature_type: String,
    pub image: File,
    pub wireframes: Option<Vec<FileWithId>>,
    pub price: f64,
    pub repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub description: String,
    // pub catagorys: Option<Vec<Category>>,
    pub feature_type: String,
    pub image: File,
    pub wireframes: Option<Vec<FileWithId>>,
    pub price: f64,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub feature_type: String,
    pub image: File,
    pub wireframes: Option<Vec<FileWithOutOId>>,
    pub price: f64,
    pub repo: String,
}

impl FeatureResponseModel {
    pub fn build_feature(feature: FeatureDeserializeModel) -> FeatureResponseModel {
        FeatureResponseModel {
            id: feature._id.to_string(),
            name: feature.name,
            description: feature.description,
            // catagorys: feature.catagorys,
            feature_type: feature.feature_type,
            image: feature.image,

            wireframes: feature.wireframes.map(|file| {
                file.into_iter()
                    .map(|file| FileWithOutOId {
                        id: file._id.to_string(),
                        name: file.name,
                        src: file.src,
                    })
                    .collect()
            }),
            price: feature.price,
            repo: feature.repo,
        }
    }
}
////////////////////////template schema

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub category: ObjectId,
    pub features: Option<Vec<ObjectId>>,
    pub image: File,
    pub specification: Option<Specification>,
    // pub prototype_id: Option<ObjectId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateReafactorDeserializeModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub category: ObjectId,
    pub features: Option<Vec<FeatureDeserializeModel>>,
    pub image: File,
    pub specification: Option<Specification>,
    // pub prototype_id: Option<ObjectId>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TemplateResponseRefactorModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<FeatureResponseModel>>,
    pub image: File,
    pub specification: Option<Specification>,
    // pub prototype_id: String,
}

impl TemplateResponseRefactorModel {
    pub fn build_template(
        template: TemplateReafactorDeserializeModel,
    ) -> TemplateResponseRefactorModel {
        TemplateResponseRefactorModel {
            id: template._id.to_string(),
            name: template.name,
            description: template.description,
            category: template.category.to_string(),
            features: template.features.into_iter().next().and_then(|features| {
                Some(
                    features
                        .into_iter()
                        .map(|feature| FeatureResponseModel::build_feature(feature))
                        .collect::<Vec<FeatureResponseModel>>(),
                )
            }),
            image: template.image,
            specification: template.specification,
            // prototype_id: template.prototype_id.unwrap().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<String>>,
    pub image: File,
    pub specification: Option<Specification>,
    // pub prototype_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureToAnyModel {
    pub id: String,
    pub features_id: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoiresIds {
    pub categories_id: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateDeserializeModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub category: ObjectId,
    pub features: Option<Vec<ObjectId>>,
    pub image: File,
    pub specification: Option<Specification>,
    // pub prototype_id: Option<ObjectId>,
}

impl TemplateResponseModel {
    pub fn build_template(template: TemplateDeserializeModel) -> TemplateResponseModel {
        TemplateResponseModel {
            id: template._id.to_string(),
            name: template.name,
            description: template.description,
            category: template.category.to_string(),
            features: template.features.into_iter().next().and_then(|features| {
                Some(
                    features
                        .into_iter()
                        .map(|feature_id| feature_id.to_string())
                        .collect::<Vec<String>>(),
                )
            }),
            image: template.image,
            specification: template.specification,
            // prototype_id: template.prototype_id.unwrap().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub introduction: Introduction,
    pub overall_description: OverallDescription,
    pub non_functional_requirements: NonFunctionalRequirements,
    pub other_requirements: String,
    pub glossary: String,
    pub analysis_models: String,
    pub issues_list: String,
}

impl Specification {
    pub fn new() -> Specification {
        Specification {
            introduction: Introduction {
                purpose: "".to_string(),
                document_conventions: "".to_string(),
                intended_audience: "".to_string(),
                project_scope: "".to_string(),
            },
            overall_description: OverallDescription {
                perspective: "".to_string(),
                user_characteristics: "".to_string(),
                operating_environment: "".to_string(),
                design_implementation_constraints: "".to_string(),
                user_documentation: "".to_string(),
                assemptions_dependencies: "".to_string(),
            },
            non_functional_requirements: NonFunctionalRequirements {
                performance_requirements: "".to_string(),
                safety_requirements: "".to_string(),
                security_requirements: "".to_string(),
                software_quality_attributes: "".to_string(),
            },
            other_requirements: "".to_string(),
            glossary: "".to_string(),
            analysis_models: "".to_string(),
            issues_list: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Introduction {
    pub purpose: String,
    pub document_conventions: String,
    pub intended_audience: String,
    pub project_scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonFunctionalRequirements {
    pub performance_requirements: String,
    pub safety_requirements: String,
    pub security_requirements: String,
    pub software_quality_attributes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallDescription {
    pub perspective: String,
    pub user_characteristics: String,
    pub operating_environment: String,
    pub design_implementation_constraints: String,
    pub user_documentation: String,
    pub assemptions_dependencies: String,
}

////////////////////////prototype schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoType {
    pub template_id: ObjectId,
    pub prototype: Vec<ProtoTypeObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoTypeRequest {
    pub template_id: String,
    pub prototype: Vec<ProtoTypeRequestObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoTypeRequestObject {
    pub feature_id: String,
    pub connections: Vec<ConnectionsResponseModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoTypeObject {
    pub feature_id: ObjectId,
    pub connections: Vec<Connections>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoTypeRefactorObject {
    pub feature: FeatureDeserializeModel,
    pub connections: Vec<Connections>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProtoTypeObjectResponseModel {
    pub feature: FeatureResponseModel,
    pub connections: Vec<ConnectionsResponseModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connections {
    pub to: ObjectId,
    pub releations: Relations,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ConnectionsResponseModel {
    pub to: String,
    pub releations: Relations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relations {
    pub back: bool,
    pub forword: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoTypeDeserializeModel {
    pub _id: ObjectId,
    pub template_id: ObjectId,
    pub prototype: Vec<ProtoTypeObject>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProtoTypeResponseModel {
    pub id: String,
    pub template: String,
    pub prototype: Vec<ProtoTypeObjectResponseModel>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProtoTypeRefactorDeserializeModel {
    pub _id: ObjectId,
    pub template: ObjectId,
    pub prototype: Vec<ProtoTypeRefactorObject>,
}

impl ProtoTypeResponseModel {
    pub fn build_prototype(prototype: ProtoTypeRefactorDeserializeModel) -> ProtoTypeResponseModel {
        ProtoTypeResponseModel {
            id: prototype._id.to_string(),
            template: prototype.template.to_string(),
            prototype: prototype
                .prototype
                .into_iter()
                .map(|prototype_object| ProtoTypeObjectResponseModel {
                    feature: FeatureResponseModel::build_feature(prototype_object.feature),
                    connections: prototype_object
                        .connections
                        .into_iter()
                        .map(|connections| ConnectionsResponseModel {
                            to: connections.to.to_string(),
                            releations: connections.releations,
                        })
                        .collect::<Vec<ConnectionsResponseModel>>(),
                })
                .collect::<Vec<ProtoTypeObjectResponseModel>>(),
        }
    }
}

////////////////////////prototype schema

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Project {
    pub client_id: ObjectId,
    pub name: String,
    pub platforms: Vec<String>,
    pub template: ObjectId,
    pub features: Vec<ObjectId>,
    pub state: String,
    pub proposal: Option<Proposal>,
    pub delivrable: Option<Delivrable>,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectDeserializeModel {
    pub _id: ObjectId,
    pub client_id: ObjectId,
    pub name: String,
    pub platforms: Vec<String>,
    pub template: TemplateReafactorDeserializeModel,
    pub features: Vec<FeatureDeserializeModel>,
    pub state: String,
    pub proposal: Option<Proposal>,
    pub delivrable: Option<Delivrable>,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectRequestModel {
    pub client_id: String,
    pub name: String,
    pub platforms: Vec<String>,
    pub template: String,
    pub features: Vec<String>,
    pub state: String,
    pub proposal: Option<Proposal>,
    pub delivrable: Option<Delivrable>,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectState {
    pub id: String,
    pub state: String,
}
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectFullBuild {
    pub id: String,
    pub full_build: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectProposal {
    pub id: String,
    pub proposal: Proposal,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectResponseModel {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub platforms: Vec<String>,
    pub template: TemplateResponseRefactorModel,
    pub features: Vec<FeatureResponseModel>,
    pub state: String,
    pub proposal: Option<Proposal>,
    pub delivrable: Option<Delivrable>,
    pub total_price: f64,
}

impl ProjectResponseModel {
    pub fn build_project(project: ProjectDeserializeModel) -> ProjectResponseModel {
        ProjectResponseModel {
            id: project._id.to_string(),
            client_id: project.client_id.to_string(),
            name: project.name,
            platforms: project.platforms,
            template: TemplateResponseRefactorModel::build_template(project.template),
            features: project
                .features
                .into_iter()
                .map(|feature| FeatureResponseModel::build_feature(feature))
                .collect::<Vec<FeatureResponseModel>>(),
            state: project.state,
            proposal: project.proposal,
            delivrable: project.delivrable,
            total_price: project.total_price,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Proposal {
    pub devtime: Devtime,
    pub summary: String,
    pub purpose: String,
    pub resources: Vec<Resource>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Devtime {
    pub months: i32,
    pub days: i32,
    pub hours: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Resource {
    pub resource_type: String,
    pub developers: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Delivrable {
    pub specification: File,
    pub full_build: String,
    pub mvp: File,
    pub design_and_prototype: File,
}
