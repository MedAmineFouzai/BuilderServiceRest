pub mod categories_controller;
pub mod features_controller;
pub mod projects_controller;
pub mod prototypes_controller;
pub mod schema;
pub mod templates_controller;

pub use categories_controller::{
    create_category, delete_category, get_all_categories, get_category_by_id, update_category,
};

pub use features_controller::{
    add_feature_wireframe, create_feature, delete_feature, delete_feature_wireframe,
    get_all_features, get_feature_by_id, update_feature,
};

pub use projects_controller::{
    add_design_project, add_full_build_project, add_mvp_project, add_project, add_proposal_project,
    change_project_state, generate_project_specification, get_all_project_by_client_id,
    get_all_projects, get_project_by_id, update_project,
};

pub use prototypes_controller::{add_prototype, get_prototype_by_template_id, update_prototype};

pub use templates_controller::{
    add_template_specification, create_template, delete_template, get_all_templates,
    get_template_by_id, get_templates_by_categories_id, update_template, update_template_feature,
};
