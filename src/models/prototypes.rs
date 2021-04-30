use bson::{doc, oid::ObjectId, Document};
use mongodb::{
    error::Error,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    results::InsertOneResult,
    Collection, Cursor,
};

#[derive(Debug, Clone)]
pub struct PrototypesCollection {
    collection: Collection,
}

impl PrototypesCollection {
    pub fn new(collection: Collection) -> PrototypesCollection {
        PrototypesCollection { collection }
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
        Ok(self.collection.find(None, None).await?)
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

    pub async fn update_one<T>(&self, user_id: &str, document: T) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "template_id":ObjectId::with_string(user_id).unwrap()
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
                                "template_id":ObjectId::with_string(id).unwrap()
                            }


                    },
                    doc! {

                        "$unwind":
                        {
                            "path": "$prototype",
                            "preserveNullAndEmptyArrays": true
                        }
                    },
                    doc! {
                        "$lookup": {
                               "from": "Features",
                               "localField": "prototype.feature_id",
                               "foreignField": "_id",
                               "as": "prototype.feature"
                            }
                    },
                    doc! {
                        "$unset": "prototype.feature_id"
                    },
                    doc! {

                        "$unwind":{

                            "path": "$prototype.feature",
                            "preserveNullAndEmptyArrays": true
                        }
                    },
                    doc! {
                      "$group":
                        {
                          "_id": "$_id",
                          "template": {"$first": "$template_id"},
                          "prototype": { "$push":  "$prototype" }
                        }
                    },
                ],
                None,
            )
            .await?)
    }
}
