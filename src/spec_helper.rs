#[macro_export]
macro_rules! assert_error {
    ( $error_type:pat, $result:expr ) => {{
		let result = $result;
		match result {
			Err(Error($error_type, _)) => (),
			_ => panic!("Expected an error of type {:?}, got {:?}",
					stringify!($error_type), result),
		}
    }};
}

#[macro_export]
macro_rules! assert_request_response {
    ( $method:ident, $request:expr, $response:expr ) => {{
        let request : CString = CString::new($request).unwrap();
        let response = unsafe {
            let r = $method(request.into_raw());
            CStr::from_ptr(r).to_str().unwrap()
        };
        let a : serde_json::Value = serde_json::from_str(response).unwrap();
        let b : serde_json::Value = serde_json::from_str($response).unwrap();
        assert_eq!(a, b);
    }}
}

