extern crate bureaucrat;
extern crate jsonapi;
extern crate serde_json;

use bureaucrat::cbu::Cbu;
use bureaucrat::json_api::ResultToJsonApi;
use self::jsonapi::model::JsonApiModel;
use self::jsonapi::api::*;

#[test]
fn encodes_cbu_as_json_api() {
	let cbu = Cbu::new("0170035040000002373188");
	let c: JsonApiDocument = ResultToJsonApi(cbu).into();
    println!("Doc {:?}", serde_json::to_string(&c).unwrap());
}
