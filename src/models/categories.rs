use bson::{doc, oid::ObjectId, Document};
use mongodb::{
    error::Error,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    results::InsertOneResult,
    Collection, Cursor,
};

#[derive(Debug, Clone)]
pub struct CategoriesCollection {
    collection: Collection,
}

impl CategoriesCollection {
    pub fn new(collection: Collection) -> CategoriesCollection {
        CategoriesCollection { collection }
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

    pub async fn update_one<T>(
        &self,
        catagory_id: &str,
        document: T,
    ) -> Result<Option<Document>, Error>
    where
        T: serde::Serialize,
    {
        Ok(self
            .collection
            .find_one_and_update(
                doc! {
                    "_id":ObjectId::with_string(catagory_id).unwrap()
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
}
