use crate::{
    api::{assert_payload_eq, Payload},
    types::{tests::assert_json_eq, AnswerWebAppQuery, InlineQueryResultArticle, SentWebAppMessage},
};

#[test]
fn sent_web_app_message() {
    assert_json_eq(
        SentWebAppMessage {
            inline_message_id: Some(String::from("id")),
        },
        serde_json::json!({
            "inline_message_id": "id"
        }),
    );
    assert_json_eq(
        SentWebAppMessage {
            inline_message_id: None,
        },
        serde_json::json!({}),
    )
}

#[test]
fn answer_web_app_query() {
    assert_payload_eq(
        Payload::json(
            "answerWebAppQuery",
            serde_json::json!({
                "web_app_query_id": "query-id",
                "result": {
                    "type": "article",
                    "id": "article-id",
                    "title": "article-title",
                    "input_message_content": {
                        "message_text": "article-text"
                    }
                }
            }),
        ),
        AnswerWebAppQuery::new(
            "query-id",
            InlineQueryResultArticle::new("article-id", "article-title", "article-text"),
        ),
    );
}
