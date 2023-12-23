use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

use common::{
    create_test_crate, create_test_rustacean, delete_test_crate, delete_test_rustacean, APP_HOST,
};

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "1.0",
            "description": "Foo create description",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "1.0",
            "description": "Foo create description",
            "created_at": a_crate["created_at"],
        })
    );

    delete_test_crate(&client, &a_crate);
    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "1.0",
            "description": "Foo create description",
            "created_at": a_crate["created_at"],
        })
    );

    delete_test_crate(&client, &a_crate);
    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);
    let a_crate1 = create_test_crate(&client, &rustacean1);
    let a_crate2 = create_test_crate(&client, &rustacean2);

    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let crates: Value = response.json().unwrap();

    assert!(crates.as_array().unwrap().contains(&a_crate1));
    assert!(crates.as_array().unwrap().contains(&a_crate2));

    delete_test_crate(&client, &a_crate1);
    delete_test_crate(&client, &a_crate2);
    delete_test_rustacean(&client, &rustacean1);
    delete_test_rustacean(&client, &rustacean2);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "code": "bar",
            "name": "Bar crate",
            "version": "2.0",
            "description": "Bar create description",
            "rustacean_id": rustacean["id"],
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "bar",
            "name": "Bar crate",
            "version": "2.0",
            "description": "Bar create description",
            "created_at": a_crate["created_at"],
        })
    );

    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "code": "bar",
            "name": "Bar crate",
            "version": "2.0",
            "description": "Bar create description",
            "rustacean_id": 99999,
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    delete_test_crate(&client, &a_crate);
    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    delete_test_rustacean(&client, &rustacean);
}
