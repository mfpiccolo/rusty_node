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

fn _where(call: Call) -> JsResult<JsArray> {
  use users::dsl::*;
  let scope = call.scope;

  let key: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
  let value: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
  let string = &value.value()[..];
  let connection = establish_connection();

  let _users: Vec<User> = users.filter(first_name.eq(string))
    .limit(10)
    .load(&connection).unwrap();

  let js_array: Handle<JsArray> = JsArray::new(scope, 3);

  let js_object: Handle<JsObject> = JsObject::new(scope);
  let _id = JsInteger::new(scope, 1);
  let _first_name = JsString::new(scope, "Mike");
  let _last_name = JsString::new(scope, "Piccolo");
  let _email = JsString::new(scope, "mfpiccolo@gmail.com");

  try!(js_object.set("id", _id));
  try!(js_object.set("first_name", _first_name.unwrap()));
  try!(js_object.set("last_name", _last_name.unwrap()));
  try!(js_object.set("email", _email.unwrap()));

  try!(js_array.set(0, js_object));

  Ok(js_array)
}

// fn to_js_array<T: Into<JsObject>>(items: Vec<T>) -> JsArray {

// }

// impl Into<JsObject> for User {
//   fn into(self) -> JsObject {
//     let js_object = JsObject::new(scope);

//     ret.set("id", JsInteger::new(scope, 9000));
//     ret.set("first_name", JsString::new(scope, "Mike"));
//     ret.set("last_name", JsStrin::new(scope, "Piccolo"));
//     ret.set("email", JsStrin::new(scope, "mfpiccolo@gmail.com"));

//     js_object
//   }
// }

register_module!(m, {
    m.export("where", _where)
});

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(Queryable)]
struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

infer_schema!(dotenv!("DATABASE_URL"));

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


