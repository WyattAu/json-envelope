// Property tests assert invariants directly; unwraps keep failures loud.
#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Shim validation: everything historically importable from `json-envelope`
//! still resolves through the `api-types` re-export, using the migrated
//! constructors (`error_with` replaces `error(code, message)`).

use json_envelope::{ApiError, ApiResponse, JSend, PaginationMeta};
use proptest::prelude::*;

proptest! {
    #[test]
    fn api_response_success_json_roundtrip(data in "[a-z]{1,100}") {
        let resp = ApiResponse::success(data.clone());
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["success"], serde_json::json!(true));
        assert_eq!(parsed["data"].as_str(), Some(data.as_str()));
    }

    #[test]
    fn api_response_error_json_roundtrip(
        code in "[a-z]{1,20}",
        message in "[a-z ]{1,100}",
    ) {
        let resp = ApiResponse::<()>::error_with(&code, &message);
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["success"], serde_json::json!(false));
        assert_eq!(parsed["error"]["code"].as_str(), Some(code.as_str()));
        assert_eq!(parsed["error"]["message"].as_str(), Some(message.as_str()));
    }

    #[test]
    fn api_response_paginated_json_roundtrip(
        data in "[a-z]{1,50}",
        page in 1u32..1000u32,
        per_page in 1u32..100u32,
        total in 0u64..1_000_000u64,
    ) {
        let total_pages = if per_page > 0 { ((total as f64 / per_page as f64).ceil() as u32).max(1) } else { 1 };
        let meta = PaginationMeta { page, per_page, total, total_pages };
        let resp = ApiResponse::paginated(data.clone(), meta);
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["success"], serde_json::json!(true));
        assert_eq!(parsed["data"].as_str(), Some(data.as_str()));
        assert_eq!(parsed["pagination"]["page"].as_u64(), Some(page as u64));
        assert_eq!(parsed["pagination"]["total"].as_u64(), Some(total));
    }

    #[test]
    fn api_response_from_trait(data in "[a-z]{1,100}") {
        let resp: ApiResponse<String> = data.clone().into();
        assert!(resp.success);
        assert_eq!(resp.data.as_deref(), Some(data.as_str()));
    }

    #[test]
    fn api_response_success_always_has_data(data in "[a-z]{1,100}") {
        let resp = ApiResponse::success(data);
        assert!(resp.success);
        assert!(resp.data.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn api_response_error_always_has_error(
        code in "[a-z]{1,20}",
        msg in "[a-z ]{1,100}",
    ) {
        let resp = ApiResponse::<()>::error_with(&code, &msg);
        assert!(!resp.success);
        assert!(resp.data.is_none());
        assert!(resp.error.is_some());
        let err = resp.error.unwrap();
        assert_eq!(err.code, code);
        assert_eq!(err.message, msg);
    }

    #[test]
    fn api_error_struct_literal_still_works(
        code in "[a-z]{1,20}",
        message in "[a-z ]{1,100}",
    ) {
        // The struct shape is unchanged, so literal construction keeps compiling.
        let err = ApiError {
            code: code.clone(),
            message: message.clone(),
            details: None,
        };
        assert_eq!(err.code, code);
        assert_eq!(err.message, message);
    }

    #[test]
    fn jsend_reexported_through_shim(data in "[a-z]{1,100}") {
        let resp = JSend::success(data.clone());
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["status"], serde_json::json!("success"));
        assert_eq!(parsed["data"].as_str(), Some(data.as_str()));
    }
}
