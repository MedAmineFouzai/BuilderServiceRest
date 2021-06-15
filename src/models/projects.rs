use bson::{doc, oid::ObjectId, Document};
use mongodb::{
    error::Error,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    results::InsertOneResult,
    Collection, Cursor,
};

#[derive(Debug, Clone)]
pub struct ProjectsCollection {
    collection: Collection,
}

impl ProjectsCollection {
    pub fn new(collection: Collection) -> ProjectsCollection {
        ProjectsCollection { collection }
    }

    pub async fn find_one<T>(&self, document: T) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one(
                bson::to_bson(&document)
                    .unwrap()
                    .as_document()
                    .unwrap()
                    .clone(),
                None,
            )
            .await?)
    }

    pub async fn find_all(&self) -> Result<Cursor, Error> {
        Ok(self
            .collection
            .aggregate(
                vec![
                    doc! {
                        "$lookup": {
                               "from": "Templates",
                               "localField": "template",
                               "foreignField": "_id",
                               "as": "template"
                            }
                    },
                    doc! {
                       "$unwind":
                        {
                            "path": "$template",
                            "preserveNullAndEmptyArrays": true
                        }
                    },
                    doc! {
                         "$lookup": {
                                "from": "Features",
                                "localField": "template.features",
                                "foreignField": "_id",
                                "as": "template.features"
                             }

                    },
                    doc! {
                        "$lookup": {
                               "from": "Features",
                               "localField": "features",
                               "foreignField": "_id",
                               "as": "features"
                            }
                    },
                ],
                None,
            )
            .await?)
    }

    pub async fn insert_one<T>(&self, document: T) -> Result<InsertOneResult, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .insert_one(
                bson::to_bson(&document)
                    .unwrap()
                    .as_document()
                    .unwrap()
                    .clone(),
                None,
            )
            .await?)
    }

    pub async fn delete_one(&self, user_id: &str) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_delete(
                doc! {
                "_id":ObjectId::with_string(user_id).unwrap()
                    },
                None,
            )
            .await?)
    }

    pub async fn update_one<T>(
        &self,
        user_id: &str,
        name: &str,
        image: T,
    ) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(user_id).unwrap()
                },
                doc! {
                      "$set":{
                        "name":name,
                        "image":bson::to_bson(&image)
                        .unwrap()
                        .as_document()
                        .unwrap()
                        .clone()
                      }


                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn find_one_by_id(&self, id: &str) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one(
                doc! {
                    "_id":ObjectId::with_string(id).unwrap()
                },
                None,
            )
            .await?)
    }

    pub async fn refactor_one_by_id(&self, id: &str) -> Result<Cursor, Error> {
        Ok(self
            .collection
            .aggregate(
                vec![
                    doc! {
                         "$match": {
                                "_id":ObjectId::with_string(id).unwrap()
                            }
                    },
                    doc! {
                        "$lookup": {
                               "from": "Templates",
                               "localField": "template",
                               "foreignField": "_id",
                               "as": "template"
                            }
                    },
                    doc! {
                       "$unwind":
                        {
                            "path": "$template",
                            "preserveNullAndEmptyArrays": true
                        }
                    },
                    doc! {
                         "$lookup": {
                                "from": "Features",
                                "localField": "template.features",
                                "foreignField": "_id",
                                "as": "template.features"
                             }

                    },
                    doc! {
                        "$lookup": {
                               "from": "Features",
                               "localField": "features",
                               "foreignField": "_id",
                               "as": "features"
                            }
                    },
                ],
                None,
            )
            .await?)
    }

    pub async fn refactor_one_by_client_id(&self, client_id: &str) -> Result<Cursor, Error> {
        Ok(self
            .collection
            .aggregate(
                vec![
                    doc! {
                         "$match": {
                                "client_id":ObjectId::with_string(client_id).unwrap()
                            }
                    },
                    doc! {
                        "$lookup": {
                               "from": "Templates",
                               "localField": "template",
                               "foreignField": "_id",
                               "as": "template"
                            }
                    },
                    doc! {
                       "$unwind":
                        {
                            "path": "$template",
                            "preserveNullAndEmptyArrays": true
                        }
                    },
                    doc! {
                         "$lookup": {
                                "from": "Features",
                                "localField": "template.features",
                                "foreignField": "_id",
                                "as": "template.features"
                             }

                    },
                    doc! {
                        "$lookup": {
                               "from": "Features",
                               "localField": "features",
                               "foreignField": "_id",
                               "as": "features"
                            }
                    },
                ],
                None,
            )
            .await?)
    }

    pub async fn update_state(
        &self,
        user_id: &str,
        state: &str,
    ) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(user_id).unwrap()
                },
                doc! {
                      "$set":{
                          "state":state
                      }

                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn update_full_build(
        &self,
        project_id: &str,
        full_build: &str,
    ) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(project_id).unwrap()
                },
                doc! {
                      "$set":{
                          "delivrable.full_build":full_build
                      }

                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn update_propsal<T>(
        &self,
        project_id: &str,
        propsal: T,
    ) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(project_id).unwrap()
                },
                doc! {
                      "$set":{
                          "proposal":bson::to_bson(&propsal)
                          .unwrap()
                          .as_document()
                          .unwrap()
                          .clone()
                      }

                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn update_mvp<T>(&self, project_id: &str, mvp: T) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(project_id).unwrap()
                },
                doc! {
                      "$set":{
                          "delivrable.mvp":bson::to_bson(&mvp)
                          .unwrap()
                          .as_document()
                          .unwrap()
                          .clone()
                      }

                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn update_design<T>(
        &self,
        project_id: &str,
        design: T,
    ) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(project_id).unwrap()
                },
                doc! {
                      "$set":{

                          "delivrable.design":bson::to_bson(&design)
                          .unwrap()
                          .as_document()
                          .unwrap()
                          .clone()
                      }

                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn add_feature(
        &self,
        project_id: &str,
        features_id: Vec<ObjectId>,
    ) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(project_id).unwrap()
                },
                doc! {
                  "$push":{
                      "features":{
                        "$each":features_id
                        }
                     }
                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn delete_feature(
        &self,
        project_id: &str,
        feature_id: &str,
    ) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(project_id).unwrap()
                },
                doc! {
                  "$pull":{
                    "features":ObjectId::with_string(feature_id).unwrap()
                  }
                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }
    
}
