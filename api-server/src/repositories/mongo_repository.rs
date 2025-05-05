use mongodb::{bson::doc, Client, Collection};
use bson::Uuid;
use async_trait::async_trait;

use super::{StatusRepository, PictureRepository};
use crate::models::{Status, Picture};
use crate::errors::Error;

const STATUS_COLL: &str = "statuses";
const PICTURE_COLL: &str = "pictures";

pub struct MongoRepository {
    client: Client,
    db_name: String
}

impl MongoRepository {
    pub fn new(client: Client, db_name: String) -> Self {
        Self {
            client,
            db_name
        }
    }

    fn status_collection(&self) -> Collection<Status> {
        self.client.database(&self.db_name).collection(STATUS_COLL)
    }

    fn picture_collection(&self) -> Collection<Picture> {
        self.client.database(&self.db_name).collection(PICTURE_COLL)
    }
}

#[async_trait]
impl StatusRepository for MongoRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Status>, Error> {
        self.status_collection()
        .find_one(doc! {"_id": id})
        .await.map_err(|e| Error::DatabaseError(e.to_string()))
    }

    async fn insert(&self, status: &Status) -> Result<(), Error> {
        self.status_collection()
        .insert_one(status)
        .await
        .map(|_| ())
        .map_err(|e| Error::DatabaseError(e.to_string()))
    }

    async fn find_and_update_authorised(&self, id: Uuid, authorised: bool) -> Result<Option<Status>, Error> {
        self.status_collection()
        .find_one_and_update(doc! {"_id": id}, doc! {"$set": {"authorised": authorised}})
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))
    }
}

#[async_trait]
impl PictureRepository for MongoRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Picture>, Error> {
        self.picture_collection()
        .find_one(doc! {"_id": id})
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))
    }

    async fn insert(&self, picture: &Picture) -> Result<(), Error> {
        self.picture_collection()
        .insert_one(picture)
        .await
        .map(|_| ())
        .map_err(|e| Error::DatabaseError(e.to_string()))
    }
}