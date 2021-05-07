use bson::{doc, oid::ObjectId, Document};
use mongodb::{
    error::Error,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    results::InsertOneResult,
    Collection, Cursor,
};

#[derive(Debug, Clone)]
pub struct TemplatesCollection {
    collection: Collection,
}

impl TemplatesCollection {
    pub fn new(collection: Collection) -> TemplatesCollection {
        TemplatesCollection { collection }
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
                vec![doc! {
                    "$lookup":{
                        "from": "Features",
                        "localField": "features",
                        "foreignField": "_id",
                        "as": "features"
                    }
                }],
                None,
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

    pub async fn update_one<T>(&self, id: &str, document: T) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(id).unwrap()
                },
                doc! {
                      "$set":bson::to_bson(&document)
                        .unwrap()
                        .as_document()
                        .unwrap()
                        .clone()

                },
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?)
    }

    pub async fn find_templates_by_categories_id(
        &self,
        categories_id: Vec<ObjectId>,
    ) -> Result<Cursor, Error> {
        Ok(self
            .collection
            .aggregate(
                vec![
                    doc! {"$match": {

                    "category": {
                            "$in":categories_id
                            },
                        }
                    },
                    doc! {
                       "$lookup":{
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

    pub async fn refactor_template(&self, id: &str) -> Result<Cursor<Document>, Error> {
        Ok(self
            .collection
            .aggregate(
                vec![
                    doc! {
                        "$match":{
                            "_id":ObjectId::with_string(id).unwrap()
                        }
                    },
                    doc! {
                        "$lookup":{
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

    pub async fn add_feature(
        &self,
        template_id: &str,
        features_id: Vec<ObjectId>,
    ) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(template_id).unwrap()
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

    pub async fn update_features(
        &self,
        template_id: &str,
        features_id: Vec<ObjectId>,
    ) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(template_id).unwrap()
                },
                doc! {
                  "$set":{
                      "features":features_id
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
        template_id: &str,
        feature_id: &str,
    ) -> Result<Option<Document>, Error> {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(template_id).unwrap()
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

    pub async fn update_specification<T>(
        &self,
        template_id: &str,
        document: T,
    ) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(template_id).unwrap()
                },
                doc! {
                      "$set":{

                         "specification" :bson::to_bson(&document)
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
}
