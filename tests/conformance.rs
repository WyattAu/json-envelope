//! Conformance: pass the shared envelope fixture.
//!
//! Canonical fixture: tests/conformance_cases.json (vendored by
//! web-estate/conformance/envelope/cases.json). The decode rules here
//! mirror @est/envelope's decodeEnvelope; both implementations must
//! agree case for case.

use serde_json::Value;

const CASES: &str = include_str!("conformance_cases.json");

fn is_record(v: &Value) -> bool {
    v.is_object()
}

/// The decode rules shared with @est/envelope. Returns Ok(envelope) or
/// Err(reason); fixtures assert only the ok/reject outcome plus the exact
/// accepted shape.
fn decode(input: &Value) -> Result<Value, String> {
    let obj = input.as_object().ok_or("envelope: not an object")?;
    let success = obj
        .get("success")
        .and_then(|v| v.as_bool())
        .ok_or("success: not a boolean")?;

    let mut out = serde_json::Map::new();
    out.insert("success".into(), Value::Bool(success));

    if let Some(err) = obj.get("error") {
        if success {
            return Err("success:true with error object".into());
        }
        let e = err.as_object().ok_or("error: not an object")?;
        if !e.get("code").is_some_and(|v| v.is_string()) {
            return Err("error.code: not a string".into());
        }
        if !e.get("message").is_some_and(|v| v.is_string()) {
            return Err("error.message: not a string".into());
        }
    } else if !success {
        return Err("success:false without error".into());
    }
    if let Some(err) = obj.get("error") {
        out.insert("error".into(), err.clone());
    }

    if let Some(data) = obj.get("data") {
        out.insert("data".into(), data.clone());
    }

    if let Some(pag) = obj.get("pagination") {
        let p = pag.as_object().ok_or("pagination: not an object")?;
        for key in ["page", "per_page", "total_pages"] {
            if !p.get(key).is_some_and(Value::is_number) {
                return Err(format!("pagination.{key}: not an integer"));
            }
        }
        if !p.get("total").is_some_and(Value::is_number) {
            return Err("pagination.total: not a number".into());
        }
        out.insert("pagination".into(), pag.clone());
    }

    Ok(Value::Object(out))
}

#[test]
fn conformance_fixtures_pass() {
    let fixture: Value = serde_json::from_str(CASES).expect("fixture parses");
    assert_eq!(fixture["suite"], "envelope");

    let cases = fixture["cases"].as_array().expect("cases array");
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");
        match case["kind"].as_str() {
            Some("decode") => {
                let result = decode(&case["input"]);
                match case["expect"].as_str() {
                    Some("ok") => {
                        let env = result.unwrap_or_else(|e| {
                            panic!("case {name}: expected ok, rejected with {e}")
                        });
                        let expected = &case["expected"];
                        if !expected.is_null() {
                            assert_eq!(&env, expected, "case {name}: decoded shape diverged");
                        }
                    }
                    Some("reject") => {
                        assert!(result.is_err(), "case {name}: expected reject, got ok");
                    }
                    other => panic!("case {name}: unknown expect {other:?}"),
                }
            }
            Some("encode") => {
                let construct = &case["construct"];
                let env = if let Some(code) = construct.get("code") {
                    // Normalize variant-style names to the wire code via the
                    // error-codes crate (mirrors @est/envelope's table).
                    let wire = serde_json::from_value::<error_codes::ErrorCode>(serde_json::Value::String(
                        code.as_str().unwrap_or_default().to_string(),
                    ))
                    .map(|e| e.as_str().to_string())
                    .unwrap_or_else(|_| code.as_str().unwrap_or_default().to_string());
                    let mut err = serde_json::Map::new();
                    err.insert("code".into(), Value::String(wire));
                    err.insert("message".into(), construct["message"].clone());
                    serde_json::json!({ "success": false, "error": Value::Object(err) })
                } else {
                    serde_json::json!({ "success": true, "data": construct["data"] })
                };
                assert_eq!(&env, &case["expected"], "case {name}: encode diverged");
            }
            other => panic!("unknown case kind {other:?}"),
        }
    }
}

#[test]
fn api_response_wire_shape_is_snake_case() {
    // The paginated constructor must emit per_page/total_pages verbatim.
    use json_envelope::{ApiResponse, PaginationMeta};
    let env = ApiResponse::paginated(
        serde_json::json!([]),
        PaginationMeta {
            page: 2,
            per_page: 50,
            total: 506,
            total_pages: 11,
        },
    );
    let json = serde_json::to_string(&env).unwrap();
    assert!(
        json.contains("\"per_page\":50"),
        "wire key per_page missing: {json}"
    );
    assert!(
        json.contains("\"total_pages\":11"),
        "wire key total_pages missing: {json}"
    );
    assert!(
        !json.contains("perPage"),
        "camelCase leaked onto the wire: {json}"
    );
}

#[test]
fn is_record_smoke() {
    assert!(is_record(&serde_json::json!({})));
    assert!(!is_record(&serde_json::json!([])));
}
