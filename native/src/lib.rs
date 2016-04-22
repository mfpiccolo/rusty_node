#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate neon;

#[macro_use]
mod to_js;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsInteger, JsObject, JsArray, Object};
use neon::mem::Handle;
use neon::scope::RootScope;
use to_js::ToJsArray;

fn load(call: Call) -> JsResult<JsArray> {
  use users::dsl::*;
  let scope = call.scope;

  // let key: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
  let value: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
  let string = &value.value()[..];
  let connection = establish_connection();

  let query = users.filter(first_name.eq(string)).limit(10);
  print_sql!(query);
  let records = query.load(&connection).unwrap();
  (scope, records).to_js_array()
}

register_module!(m, {
    m.export("load", load)
});

#[derive(Queryable, Debug)]
struct User {
  id: i32,
  first_name: String,
  last_name: String,
  email: String,
}

configure_model!(
  User,
  JsInteger => [("id", id),],
  JsString => [
    ("first_name", first_name),
    ("last_name", last_name),
    ("email", email),
  ],
);

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

infer_schema!(dotenv!("DATABASE_URL"));

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

