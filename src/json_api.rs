extern crate libc;
extern crate jsonapi;
extern crate serde_json;

use errors::*;
use self::jsonapi::api::*;
use std::str;
use std::error::Error;
use std::ffi::{CString, CStr};
use self::libc::c_char;
use cbu::Cbu;
use cuit::Cuit;

impl From<Cbu> for PrimaryData {
    fn from(cbu: Cbu) -> Self {
        let hash = [
         ("bank_name", cbu.bank_name),
         ("bank", cbu.bank),
         ("branch", cbu.branch),
         ("account", cbu.account)
        ].iter().cloned().map(|(k,v)| (k.into(), v.into())).collect();

		PrimaryData::Single(Resource {
			_type: "cbu".into(),
			id: cbu.id.into(),
			attributes: hash,
			relationships: None,
			links: None,
			meta: None,
		})
    }
}

impl From<Cuit> for PrimaryData {
    fn from(cuit: Cuit) -> Self {
        let hash = [
            ("kind", cuit.kind),
            ("person_id", cuit.person_id)
        ].iter().cloned().map(|(k,v)| (k.into(), v.into())).collect();

		PrimaryData::Single(Resource {
			_type: "cuit".into(),
			id: cuit.id.into(),
			attributes: hash,
			relationships: None,
			links: None,
			meta: None,
		})
    }
}

pub extern fn bureaucrat_cbus_create(request: *const c_char) -> *mut c_char {
    json_ffi_handler(request, |resource| Cbu::new(resource.id) )
}

pub extern fn bureaucrat_cuits_create(request: *const c_char) -> *mut c_char {
    json_ffi_handler(request, |resource| Cuit::new(resource.id) )
}

fn result_to_json_api_document(result: Result<PrimaryData>) -> JsonApiDocument {
	let (data, errors) = match result {
		Ok(a) => (Some(a), None),
		Err(b) => (None, Some(
			vec![ JsonApiError {
				id: None,
				links: None,
				status: None,
				code: None,
				title: None,
				detail: Some(b.description().to_string()),
				source: None,
				meta: None,
			}]
		)),
	};

	JsonApiDocument{
		data: data,
		errors: errors,
		meta: None,
		included: None,
		links: None,
		jsonapi: None,
	}
}

fn handle_helper<F,P>(raw_request: *const c_char, f: F) -> Result<PrimaryData>
    where F: Fn(Resource) -> Result<P>, PrimaryData: From<P>
{
    let raw_request = unsafe {
        assert!(!raw_request.is_null());
        CStr::from_ptr(raw_request).to_str()?
    };
    let primary_data = JsonApiDocument::from_str(raw_request)
        .chain_err(||ErrorKind::InvalidRequestFormat)?
        .data.ok_or(ErrorKind::InvalidRequestEmpty)?;

    if let PrimaryData::Single(resource) = primary_data {
        f(resource).map(|c| c.into())
    } else {
        bail!(ErrorKind::InvalidRequestResource)
    }
}

fn json_ffi_handler<F, P>(raw_request: *const c_char, f: F) -> *mut c_char
    where F: Fn(Resource) -> Result<P>, PrimaryData: From<P>
{
    let doc = result_to_json_api_document(handle_helper(raw_request, f));
    CString::new(serde_json::to_string(&doc).unwrap()).unwrap().into_raw()
}

pub extern fn bureaucrat_free(s: *mut c_char){
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
