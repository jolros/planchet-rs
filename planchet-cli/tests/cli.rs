use assert_cmd::Command;
use mockito::Server;
use predicates::prelude::*;
use serde_json::json;

#[tokio::test]
async fn dump_command_test() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let token_response = json!({
        "access_token": "test_token",
        "token_type": "bearer",
        "expires_in": 3600,
        "user_id": 1
    });

    let collection_response = json!({
        "item_count": 2,
        "item_for_swap_count": 0,
        "item_type_count": 2,
        "item_type_for_swap_count": 0,
        "items": [
            {
                "id": 1,
                "quantity": 1,
                "for_swap": false,
                "type": {
                    "id": 420,
                    "title": "5 Cents - Victoria",
                    "category": "coin",
                    "issuer": { "code": "canada", "name": "Canada" }
                },
                "issue": { "id": 1, "is_dated": true, "year": 1858, "gregorian_year": 1858 },
                "grade": null,
                "private_comment": null,
                "public_comment": null,
                "price": null,
                "collection": null,
                "pictures": null,
                "storage_location": null,
                "acquisition_place": null,
                "acquisition_date": null,
                "serial_number": null,
                "internal_id": null,
                "weight": null,
                "size": null,
                "axis": null,
                "grading_details": null
            },
            {
                "id": 2,
                "quantity": 1,
                "for_swap": false,
                "type": {
                    "id": 1,
                    "title": "1 Cent - George V",
                    "category": "coin",
                    "issuer": { "code": "canada", "name": "Canada" }
                },
                "issue": { "id": 2, "is_dated": true, "year": 1920, "gregorian_year": 1920 },
                "grade": null,
                "private_comment": null,
                "public_comment": null,
                "price": null,
                "collection": null,
                "pictures": null,
                "storage_location": null,
                "acquisition_place": null,
                "acquisition_date": null,
                "serial_number": null,
                "internal_id": null,
                "weight": null,
                "size": null,
                "axis": null,
                "grading_details": null
            },
            {
                "id": 3,
                "quantity": 1,
                "for_swap": false,
                "type": {
                    "id": 2,
                    "title": "1 Cent - Elizabeth II",
                    "category": "coin",
                    "issuer": null
                },
                "issue": { "id": 3, "is_dated": true, "year": null, "gregorian_year": null },
                "grade": null,
                "private_comment": null,
                "public_comment": null,
                "price": null,
                "collection": null,
                "pictures": null,
                "storage_location": null,
                "acquisition_place": null,
                "acquisition_date": null,
                "serial_number": null,
                "internal_id": null,
                "weight": null,
                "size": null,
                "axis": null,
                "grading_details": null
            }
        ]
    });

    server
        .mock(
            "GET",
            "/oauth_token?grant_type=client_credentials&scope=view_collection",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Numista-API-Key", "test_key")
        .with_body(token_response.to_string())
        .create();
    server
        .mock("GET", "/users/1/collected_items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Numista-API-Key", "test_key")
        .with_header("Authorization", "Bearer test_token")
        .with_body(collection_response.to_string())
        .create();

    let mut cmd = Command::cargo_bin("planchet-cli").unwrap();
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("--user-id")
        .arg("1")
        .arg("dump")
        .env("NUMISTA_API_URL", url);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Canada - 5 Cents - Victoria (1858)",
        ))
        .stdout(predicate::str::contains(
            "Canada - 1 Cent - George V (1920)",
        ))
        .stdout(predicate::str::contains(
            "<Unknown> - 1 Cent - Elizabeth II (<Unknown>)",
        ));
}

#[tokio::test]
async fn summarize_command_test() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let token_response = json!({
        "access_token": "test_token",
        "token_type": "bearer",
        "expires_in": 3600,
        "user_id": 1
    });

    let collection_response = json!({
        "item_count": 2,
        "item_for_swap_count": 0,
        "item_type_count": 2,
        "item_type_for_swap_count": 0,
        "items": [
            {
                "id": 1,
                "quantity": 1,
                "for_swap": false,
                "type": {
                    "id": 420,
                    "title": "5 Cents - Victoria",
                    "category": "coin",
                    "issuer": { "code": "canada", "name": "Canada" }
                },
                "issue": { "id": 1, "is_dated": true, "year": 1858, "gregorian_year": 1858 },
                "grade": null,
                "private_comment": null,
                "public_comment": null,
                "price": null,
                "collection": null,
                "pictures": null,
                "storage_location": null,
                "acquisition_place": null,
                "acquisition_date": null,
                "serial_number": null,
                "internal_id": null,
                "weight": null,
                "size": null,
                "axis": null,
                "grading_details": null
            },
            {
                "id": 2,
                "quantity": 1,
                "for_swap": false,
                "type": {
                    "id": 1,
                    "title": "1 Cent - George V",
                    "category": "coin",
                    "issuer": { "code": "canada", "name": "Canada" }
                },
                "issue": { "id": 2, "is_dated": true, "year": 1920, "gregorian_year": 1920 },
                "grade": null,
                "private_comment": null,
                "public_comment": null,
                "price": null,
                "collection": null,
                "pictures": null,
                "storage_location": null,
                "acquisition_place": null,
                "acquisition_date": null,
                "serial_number": null,
                "internal_id": null,
                "weight": null,
                "size": null,
                "axis": null,
                "grading_details": null
            },
            {
                "id": 3,
                "quantity": 1,
                "for_swap": false,
                "type": {
                    "id": 2,
                    "title": "1 Cent - Elizabeth II",
                    "category": "coin",
                    "issuer": null
                },
                "issue": { "id": 3, "is_dated": true, "year": null, "gregorian_year": null },
                "grade": null,
                "private_comment": null,
                "public_comment": null,
                "price": null,
                "collection": null,
                "pictures": null,
                "storage_location": null,
                "acquisition_place": null,
                "acquisition_date": null,
                "serial_number": null,
                "internal_id": null,
                "weight": null,
                "size": null,
                "axis": null,
                "grading_details": null
            }
        ]
    });

    server
        .mock(
            "GET",
            "/oauth_token?grant_type=client_credentials&scope=view_collection",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Numista-API-Key", "test_key")
        .with_body(token_response.to_string())
        .create();
    server
        .mock("GET", "/users/1/collected_items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Numista-API-Key", "test_key")
        .with_header("Authorization", "Bearer test_token")
        .with_body(collection_response.to_string())
        .create();

    let mut cmd = Command::cargo_bin("planchet-cli").unwrap();
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("--user-id")
        .arg("1")
        .arg("summarize")
        .env("NUMISTA_API_URL", url);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Canada"))
        .stdout(predicate::str::contains("2"))
        .stdout(predicate::str::contains("1858"))
        .stdout(predicate::str::contains("1920"))
        .stdout(predicate::str::contains("<Unknown>"))
        .stdout(predicate::str::contains("1"));
}

#[tokio::test]
async fn api_error_test() {
    let mut server = Server::new_async().await;
    let url = server.url();

    server
        .mock(
            "GET",
            "/oauth_token?grant_type=client_credentials&scope=view_collection",
        )
        .with_status(500)
        .with_header("Numista-API-Key", "test_key")
        .create();

    let mut cmd = Command::cargo_bin("planchet-cli").unwrap();
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("--user-id")
        .arg("1")
        .arg("dump")
        .env("NUMISTA_API_URL", url);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("HTTP error"));
}
