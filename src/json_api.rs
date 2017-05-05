extern crate libc;
extern crate jsonapi;
use std::collections::HashMap;
use self::jsonapi::api::*;
use std::{iter, str};
use std::ffi::{CString, CStr};
use self::libc::{c_char, uint8_t, uint32_t};
use cbu::{Cbu,CbuError};

pub struct ResultToJsonApi<a, b>(pub Result<a, b>);

impl<a,b> From<ResultToJsonApi<a,b>> for JsonApiDocument
	where PrimaryData: From<a>, Vec<JsonApiError>: From<b>
{
	fn from(result: ResultToJsonApi<a,b>) -> Self {
		let (data, errors) = match result.0 {
			Ok(c) => (Some(c.into()), None),
			Err(e) => (None, Some(e.into())),
		};
		Self{
			data: data,
			errors: errors,
			meta: None,
			included: None,
			links: None,
			jsonapi: None,
		}
	}
}

impl<'a> From<Cbu<'a>> for PrimaryData {
    fn from(cbu: Cbu<'a>) -> Self {
		let attributes = [
			("bank_name", cbu.bank_name),
	     	("bank", cbu.bank),
		 	("branch", cbu.branch),
			("account", cbu.account)
		].into_iter().fold(HashMap::new(), |mut hash, &(k, v)|{
			hash.insert(k.into(), v.into());
			hash
		});

		PrimaryData::Single(Resource {
			_type: "cbu".into(),
			id: cbu.id.into(),
			attributes: attributes,
			relationships: None,
			links: None,
			meta: None,
		})
    }
}


impl From<CbuError> for JsonApiErrors {
    fn from(error: CbuError) -> Self {
		vec![JsonApiError {
			id: None,
			links: None,
			status: None,
			code: Some(format!("{:?}", error)),
			title: None,
			detail: None,
			source: None,
			meta: None,
		}]
    }
}



/*pub extern fn bureaucrat_cbus_create(request: const c_char) -> *mut c_char {
	
}*/

/*#[no_mangle]
pub extern fn theme_song_generate(length: uint8_t) -> *mut c_char {
    let mut song = String::from("💣 ");
    song.extend(iter::repeat("na ").take(length as usize));
    song.push_str("Batman! 💣");

    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern fn theme_song_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

pub struct ZipCodeDatabase {
    population: HashMap<String, u32>,
}

impl ZipCodeDatabase {
    fn new() -> ZipCodeDatabase {
        ZipCodeDatabase {
            population: HashMap::new(),
        }
    }

    fn populate(&mut self) {
        for i in 0..100000 {
            let zip = format!("{:05}", i);
            self.population.insert(zip, i);
        }
    }

    fn population_of(&self, zip: &str) -> u32 {
        self.population.get(zip).cloned().unwrap_or(0)
    }
}

#[no_mangle]
pub extern fn zip_code_database_new() -> *mut ZipCodeDatabase {
    Box::into_raw(Box::new(ZipCodeDatabase::new()))
}

#[no_mangle]
pub extern fn zip_code_database_free(ptr: *mut ZipCodeDatabase) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn zip_code_database_populate(ptr: *mut ZipCodeDatabase) {
    let database = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    database.populate();
}

#[no_mangle]
pub extern fn zip_code_database_population_of(ptr: *const ZipCodeDatabase, zip: *const c_char) -> uint32_t {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let zip = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip)
    };
    let zip_str = zip.to_str().unwrap();
    database.population_of(zip_str)
}
*/
