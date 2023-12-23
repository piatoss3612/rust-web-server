use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;
use common::{create_test_rustacean, delete_test_rustacean, APP_HOST};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();

    let rustacean1: Value = create_test_rustacean(&client);
    let rustacean2: Value = create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustaceans: Value = response.json().unwrap();
    assert!(rustaceans.as_array().unwrap().contains(&rustacean1));
    assert!(rustaceans.as_array().unwrap().contains(&rustacean2));

    delete_test_rustacean(&client, &rustacean1);
    delete_test_rustacean(&client, &rustacean2);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo Bar",
            "email": "foo@bar.com",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Foo Bar",
            "email":  "foo@bar.com",
            "created_at": rustacean["created_at"],
        })
    );

    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();

    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Foo Bar",
            "email":  "foo@bar.com",
            "created_at": rustacean["created_at"],
        })
    );

    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();

    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .put(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .json(&json!({
            "name": "Fooz Bar",
            "email": "fooz@bar.com",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Fooz Bar",
            "email":  "fooz@bar.com",
            "created_at": rustacean["created_at"],
        })
    );

    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();

    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
