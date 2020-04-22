#[cfg(test)]
mod tests {
    use actix_web::{http, test::{call_service, TestRequest}};
    use diesel::{PgConnection,RunQueryDsl};

    use crate::tests::test_helpers::tests::{create_test_app};
    use crate::app::models::word_entry::{WordEntry};
    use crate::app::database::{get_database_pool};

    #[actix_rt::test]
    async fn test_list_word_entries() {
        // setup test app
        let mut app = create_test_app().await;

        // make request
        let req = TestRequest::get()
            .uri("/word_entries")
            .to_request();
        let resp = call_service(&mut app, req).await;

        // expect success
        assert_eq!(resp.status(), http::StatusCode::OK);

        // parse json from response
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let parsed_json: Vec<WordEntry> = serde_json::from_slice(response_body)
            .expect("Failed to parse Vec<WordEntry> from response");

        // expect returned details
        assert_eq!(parsed_json.len(), 1);
        assert!(parsed_json[0].id > 0, "Expected result to have valid id");
        assert_eq!(parsed_json[0].orth, "test_orth");
        assert_eq!(parsed_json[0].quote, "test quote");
    }
}