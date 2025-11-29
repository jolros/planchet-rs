use assert_cmd::prelude::*;
use mockito::Server;
use predicates::prelude::*;
use serde_json::json;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

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
        .create_async()
        .await;
    server
        .mock("GET", "/users/1/collected_items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Authorization", "Bearer test_token")
        .with_body(collection_response.to_string())
        .create_async()
        .await;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("dump")
        .arg("--user-id")
        .arg("1")
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
        .create_async()
        .await;
    server
        .mock("GET", "/users/1/collected_items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_header("Authorization", "Bearer test_token")
        .with_body(collection_response.to_string())
        .create_async()
        .await;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("summarize")
        .arg("--user-id")
        .arg("1")
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
async fn types_command_all_test() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let search_response_p1 = json!({
        "count": 3,
        "types": [
            { "id": 1, "title": "Type 1", "category": "coin", "issuer": {"code": "a", "name": "A"}, "min_year": 1, "max_year": 2 },
            { "id": 2, "title": "Type 2", "category": "coin", "issuer": {"code": "b", "name": "B"}, "min_year": 3, "max_year": 4 }
        ]
    });
    let search_response_p2 = json!({
        "count": 3,
        "types": [
            { "id": 3, "title": "Type 3", "category": "coin", "issuer": {"code": "c", "name": "C"}, "min_year": 5, "max_year": 6 }
        ]
    });
    let search_response_p3 = json!({ "count": 3, "types": [] });

    server
        .mock("GET", "/types?q=test&page=1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(search_response_p1.to_string())
        .create_async()
        .await;
    server
        .mock("GET", "/types?q=test&page=2")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(search_response_p2.to_string())
        .create_async()
        .await;
    server
        .mock("GET", "/types?q=test&page=3")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(search_response_p3.to_string())
        .create_async()
        .await;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("types")
        .arg("--query")
        .arg("test")
        .arg("--all")
        .env("NUMISTA_API_URL", url);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Found 3 results for query: 'test'"))
        .stdout(predicate::str::contains("Type 1"))
        .stdout(predicate::str::contains("Type 2"))
        .stdout(predicate::str::contains("Type 3"));
}

#[tokio::test]
async fn types_command_pagination_test() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let search_response_p1 = json!({
        "count": 26,
        "types": (1..=25).map(|i| json!({
            "id": i,
            "title": format!("Type {}", i),
            "category": "coin",
            "issuer": {"code": "a", "name": "A"},
            "min_year": 1,
            "max_year": 2
        })).collect::<Vec<_>>()
    });
    let search_response_p2 = json!({
        "count": 26,
        "types": [
            { "id": 26, "title": "Type 26", "category": "coin", "issuer": {"code": "b", "name": "B"}, "min_year": 3, "max_year": 4 }
        ]
    });

    server
        .mock("GET", "/types?q=test&page=1&count=25")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(search_response_p1.to_string())
        .create_async()
        .await;
    server
        .mock("GET", "/types?q=test&page=2&count=25")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(search_response_p2.to_string())
        .create_async()
        .await;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("types")
        .arg("--query")
        .arg("test")
        .env("NUMISTA_API_URL", url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();

    // First page, then "n" for next page
    stdin.write_all(b"n\n").unwrap();
    // Second page, then "q" to quit
    stdin.write_all(b"q\n").unwrap();

    let output = child.wait_with_output().unwrap();
    let output_str = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(output_str.contains("Found 26 results for query: 'test'"));
    assert!(output_str.contains("Type 1"));
    assert!(output_str.contains("Type 25"));
    assert!(output_str.contains("Type 26"));
}

#[tokio::test]
async fn types_command_year_test() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let search_response = json!({ "count": 0, "types": [] });

    server
        .mock("GET", "/types?q=test&date=2024&page=1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(search_response.to_string())
        .create_async()
        .await;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("types")
        .arg("--query")
        .arg("test")
        .arg("--year")
        .arg("2024")
        .arg("--all")
        .env("NUMISTA_API_URL", url);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Found 0 results for query: 'test', year: 2024"));
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
        .with_header("Numista-API-Key", "test_key")
        .with_status(500)
        .create_async()
        .await;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("dump")
        .arg("--user-id")
        .arg("1")
        .env("NUMISTA_API_URL", url);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Request error"));
}

#[tokio::test]
async fn test_no_api_key() {
    env::remove_var("NUMISTA_API_KEY");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("dump")
        .arg("--user-id")
        .arg("123")
        .assert()
        .failure()
        .stderr(predicates::str::contains("the following required arguments were not provided"));
}

#[tokio::test]
async fn test_api_key_from_arg() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock(
            "GET",
            "/oauth_token?grant_type=client_credentials&scope=view_collection",
        )
        .with_header("Numista-API-Key", "arg_key")
        .with_status(200)
        .create_async()
        .await;

    env::remove_var("NUMISTA_API_KEY");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("arg_key")
        .arg("dump")
        .arg("--user-id")
        .arg("123")
        .env("NUMISTA_API_URL", url)
        .assert()
        .failure();

    mock.assert_async().await;
}

#[tokio::test]
async fn debug_flag_test() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let token_response = json!({
        "access_token": "test_token",
        "token_type": "bearer",
        "expires_in": 3600,
        "user_id": 1
    });

    let collection_response = json!({
        "item_count": 0,
        "item_for_swap_count": 0,
        "item_type_count": 0,
        "item_type_for_swap_count": 0,
        "items": []
    });

    server
        .mock(
            "GET",
            "/oauth_token?grant_type=client_credentials&scope=view_collection",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(token_response.to_string())
        .create_async()
        .await;
    server
        .mock("GET", "/users/1/collected_items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(collection_response.to_string())
        .create_async()
        .await;

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("test_key")
        .arg("--debug")
        .arg("dump")
        .arg("--user-id")
        .arg("1")
        .env("NUMISTA_API_URL", url);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Request headers"));
}

#[tokio::test]
async fn test_api_key_from_env() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock(
            "GET",
            "/oauth_token?grant_type=client_credentials&scope=view_collection",
        )
        .with_header("Numista-API-Key", "env_key")
        .with_status(200)
        .create_async()
        .await;

    env::set_var("NUMISTA_API_KEY", "env_key");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("dump")
        .arg("--user-id")
        .arg("123")
        .env("NUMISTA_API_URL", url)
        .assert()
        .failure();
    env::remove_var("NUMISTA_API_KEY");

    mock.assert_async().await;
}

#[tokio::test]
async fn test_api_key_precedence() {
    let mut server = Server::new_async().await;
    let url = server.url();

    let mock = server
        .mock(
            "GET",
            "/oauth_token?grant_type=client_credentials&scope=view_collection",
        )
        .with_header("Numista-API-Key", "arg_key")
        .with_status(200)
        .create_async()
        .await;

    env::set_var("NUMISTA_API_KEY", "env_key");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("planchet-cli"));
    cmd.arg("--api-key")
        .arg("arg_key")
        .arg("dump")
        .arg("--user-id")
        .arg("123")
        .env("NUMISTA_API_URL", url)
        .assert()
        .failure();
    env::remove_var("NUMISTA_API_KEY");

    mock.assert_async().await;
}
