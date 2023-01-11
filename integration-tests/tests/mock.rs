use crate::db::*;
use crate::utils::*;

#[tokio::test]
async fn returns() -> TestResult {
    let (client, mock) = PrismaClient::_mock();

    user::select!(basic_user { id name });

    let expected = basic_user::Data {
        id: "123".to_string(),
        name: "Brendan".to_string(),
    };

    let query = || {
        client
            .user()
            .find_unique(user::id::equals("123".to_string()))
            .select(basic_user::select())
    };

    mock.expect(query(), Some(expected.clone())).await;

    let result = query().exec().await?.unwrap();

    assert_eq!(expected.id, result.id);
    assert_eq!(expected.name, result.name);

    Ok(())
}

#[tokio::test]
async fn returns_many() -> TestResult {
    let (client, mock) = PrismaClient::_mock();

    user::select!(basic_user { id name });

    let expected = basic_user::Data {
        id: "123".to_string(),
        name: "Brendan".to_string(),
    };

    let query = || {
        client
            .user()
            .find_many(vec![user::name::equals("Brendan".to_string())])
            .select(basic_user::select())
    };

    mock.expect(query(), vec![expected.clone()]).await;

    let result = &query().exec().await?[0];

    assert_eq!(expected.id, result.id);
    assert_eq!(expected.name, result.name);

    Ok(())
}

#[tokio::test]
async fn delete_many() -> TestResult {
    let (client, mock) = PrismaClient::_mock();

    user::select!(basic_user { id name });

    let query = || {
        client
            .user()
            .delete_many(vec![user::name::equals("Brendan".to_string())])
    };

    mock.expect(query(), 4).await;

    let result = query().exec().await?;

    assert_eq!(result, 4);

    Ok(())
}

// TODO: Errors
