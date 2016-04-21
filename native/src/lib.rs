#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsInteger, JsObject, JsArray, Object};
use neon::mem::Handle;
use neon::scope::RootScope;

fn load(call: Call) -> JsResult<JsArray> {
  use users::dsl::*;
  let scope = call.scope;

  let key: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
  let value: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
  let string = &value.value()[..];
  let connection = establish_connection();

  let _users: Vec<User> = users.filter(first_name.eq(string))
    .limit(10)
    .load(&connection).unwrap();

  (scope, _users).to_js_array()
}

register_module!(m, {
    m.export("load", load)
});

trait ToJsArray<'a,T> {
  fn to_js_array(self) -> JsResult<'a, JsArray>;
}

impl<'a> ToJsArray<'a, JsArray> for (&'a mut RootScope<'a>, Vec<User>) {
  fn to_js_array(self) -> JsResult<'a, JsArray> {
    let scope = self.0;
    let _users = self.1;

    let js_array: Handle<JsArray> = JsArray::new(scope, _users.len() as u32);
    let _id = JsInteger::new(scope, 1);
    let _first_name = JsString::new(scope, "Mike");
    let _last_name = JsString::new(scope, "Piccolo");
    let _email = JsString::new(scope, "mfpiccolo@gmail.com");

    for (i, user) in _users.iter().enumerate() {
      let js_object: Handle<JsObject> = JsObject::new(scope);
      js_object.set("id", _id);
      js_object.set("first_name", _first_name.unwrap());
      js_object.set("last_name", _last_name.unwrap());
      js_object.set("email", _email.unwrap());

      try!(js_array.set(i as u32, js_object));
    }

    Ok(js_array)
  }
}

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(Queryable)]
struct User {
  id: i32,
  first_name: String,
  last_name: String,
  email: String,
}

infer_schema!(dotenv!("DATABASE_URL"));

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


