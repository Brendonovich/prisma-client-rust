use prisma_client_rust::{ModelMutationCallbackData, ModelMutationType};
use std::sync::{Arc, Mutex};

use crate::db::*;
use crate::utils::*;

#[tokio::test]
async fn mutation() -> TestResult {
    let client = client().await;

    let callback_data = {
        let callback_data = Arc::new(Mutex::new(vec![]));

        let callback_callback_data = callback_data.clone();

        let client = PrismaClient::_builder()
            .with_model_mutation_callback(move |data| {
                callback_callback_data.lock().unwrap().push(data)
            })
            .build()
            .await
            .unwrap();

        let user = client
            .user()
            .create("Brendan".to_string(), vec![])
            .exec()
            .await
            .unwrap();

        client
            .user()
            .update(user::id::equals(user.id.clone()), vec![])
            .exec()
            .await
            .unwrap();

        client
            .user()
            .delete(user::id::equals(user.id))
            .exec()
            .await
            .unwrap();

        callback_data
    };

    assert_eq!(
        &*callback_data.lock().unwrap(),
        &vec![
            ModelMutationCallbackData {
                action: ModelMutationType::Create,
                model: "User"
            },
            ModelMutationCallbackData {
                action: ModelMutationType::Update,
                model: "User"
            },
            ModelMutationCallbackData {
                action: ModelMutationType::Delete,
                model: "User"
            },
        ]
    );

    cleanup(client).await
}
