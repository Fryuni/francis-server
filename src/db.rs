use actix_utils::future::{err, ok, Ready};
use actix_web::{dev::Payload, error::InternalError, http::StatusCode, FromRequest, HttpRequest};
use color_eyre::{eyre::eyre, Report};
use firestore::{FirestoreDb, FirestoreDocument, FirestoreListingSupport};

use crate::model::AppliedItem;

#[derive(Debug)]
pub struct DbHandle {
    firestore: FirestoreDb,
}

impl FromRequest for DbHandle {
    type Error = InternalError<Report>;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let fire = match req.app_data::<FirestoreDb>() {
            Some(fire) => fire,
            None => {
                return err(InternalError::new(
                    eyre!("Missing FirestoreDb on app_data"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        };

        ok(Self {
            firestore: fire.clone(),
        })
    }
}

impl DbHandle {
    pub async fn list_items(&self) -> color_eyre::Result<Vec<AppliedItem<'static>>> {
        let res = self
            .firestore
            .list_doc(firestore::FirestoreListDocParams::new("items".to_string()))
            .await?;

        Ok(res
            .documents
            .iter()
            .map(firestore::firestore_document_to_serializable::<AppliedItem<'static>>)
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub async fn set_items(&self, items: Vec<AppliedItem<'_>>) -> color_eyre::Result<()> {
        let batch_writer = self.firestore.create_simple_batch_writer().await?;
        let mut batch = batch_writer.new_batch();

        for item in items {
            batch.update_object(
                "items",
                &item.id,
                &item,
                None,
                None,
                vec![firestore::FirestoreFieldTransform::new(
                    "update_time".to_string(),
                    firestore::FirestoreFieldTransformType::SetToServerValue(
                        firestore::FirestoreTransformServerValue::RequestTime,
                    ),
                )],
            )?;
        }

        batch.write().await?;

        Ok(())
    }
}
