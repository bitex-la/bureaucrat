#[macro_use]
extern crate bureaucrat;
extern crate jsonapi;
extern crate serde_json;

use bureaucrat::json_api::*;
use std::ffi::{CString, CStr};

#[test]
fn encodes_valid_cbu_as_json_api() {
    assert_request_response!(
        bureaucrat_cbus_create,
        r#"{
            "data":{
                "type":"cbu",
                "id":"0170035040000002373188",
                "attributes":{}
            }
        }"#,
        r#"{"data":{
            "type":"cbu",
            "id":"0170035040000002373188",
            "attributes":{
                "bank_name":"BBVA Banco Francés S.A.",
                "account":"40000002373188",
                "bank":"017",
                "branch":"0035"
            },
            "relationships":null,
            "links":null,
            "meta":null
        },
        "included":null,
        "links":null,
        "meta":null,
        "jsonapi":null
        }"#
    );
}

#[test]
fn encodes_valid_cuit_as_json_api() {
    assert_request_response!(
        bureaucrat_cuits_create,
        r#"{
            "data":{
                "type":"cuit",
                "id":"20319274228",
                "attributes":{}
            }
        }"#,
        r#"{"data":{
            "type":"cuit",
            "id":"20319274228",
            "attributes":{
                "kind":"20",
                "person_id":"31927422"
            },
            "relationships":null,
            "links":null,
            "meta":null
        },
        "included":null,
        "links":null,
        "meta":null,
        "jsonapi":null
        }"#
    );
}

#[test]
fn encodes_empty_request_errors() {
    assert_request_response!(
        bureaucrat_cbus_create,
        "{}",
        r#"{"meta":null,
            "included":null,
            "links":null,
            "jsonapi":null,
            "errors": [ {
                "id":null,
                "links":null,
                "status":null,
                "code":null,
                "title":null,
                "detail":"InvalidRequestEmpty",
                "source":null,
                "meta":null
            }]
        }"#
    )
}

#[test]
fn encodes_malformed_errors() {
    assert_request_response!(
        bureaucrat_cbus_create,
        "{sanata",
        r#"{"meta":null,
            "included":null,
            "links":null,
            "jsonapi":null,
            "errors": [ {
                "id":null,
                "links":null,
                "status":null,
                "code":null,
                "title":null,
                "detail":"InvalidRequestFormat",
                "source":null,
                "meta":null
            }]
        }"#
    )
}
