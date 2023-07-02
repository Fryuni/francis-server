use actix_utils::future::{err, ok, Ready};
use actix_web::{dev::Payload, error::InternalError, http::StatusCode, FromRequest, HttpRequest};
use color_eyre::{eyre::eyre, Report};
use firestore::{FirestoreDb, FirestoreDocument, FirestoreListingSupport};

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
    pub async fn list_docs(&self) -> color_eyre::Result<Vec<FirestoreDocument>> {
        let res = self.firestore
            .list_doc(firestore::FirestoreListDocParams::new("user".to_string()))
            .await?;

        Ok(res.documents)
    }
}
