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
use std::fmt::Debug;

fn load(call: Call) -> JsResult<JsArray> {
  use users::dsl::*;
  let scope = call.scope;

  let key: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
  let value: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
  let string = &value.value()[..];
  let connection = establish_connection();

  let records: Vec<User> = users.filter(first_name.eq(string))
    .limit(10)
    .load(&connection).unwrap();

  (scope, records).to_js_array()
}

register_module!(m, {
    m.export("load", load)
});

trait ToJsArray<'a, T> {
  fn to_js_array(self) -> JsResult<'a, JsArray>;
}

macro_rules! configure_model {
  (
    $model:ty,
    $js_int:ident => [ $( ( $int_key:expr, $int_name:ident ), )* ],
    $js_string:ident => [ $( ( $string_key:expr, $string_name:ident ), )* ],
  ) => {
    impl<'a> ToJsArray<'a, JsArray> for (&'a mut RootScope<'a>, Vec<$model>) {
      fn to_js_array(self) -> JsResult<'a, JsArray> {
        let scope = self.0;
        let records = self.1;

        let js_array: Handle<JsArray> = JsArray::new(scope, records.len() as u32);

        for (i, record) in records.iter().enumerate() {
          let js_object: Handle<JsObject> = JsObject::new(scope);

          $(js_object.set($int_key, $js_int::new(scope, record.$int_name));)*
          $(js_object.set(
              $string_key,
              $js_string::new(scope, &record.$string_name[..]).unwrap()
            );
          )*

          try!(js_array.set(i as u32, js_object));
        }

        Ok(js_array)
      }
    }
  }
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

// // TODO: remove once con macro is in place
// impl<'a> ToJsArray<'a, JsArray> for (&'a mut RootScope<'a>, Vec<User>) {
//   fn to_js_array(self) -> JsResult<'a, JsArray> {
//     let scope = self.0;
//     let records = self.1;

//     let js_array: Handle<JsArray> = JsArray::new(scope, records.len() as u32);

//     for (i, record) in records.iter().enumerate() {
//       let js_object: Handle<JsObject> = JsObject::new(scope);
//       js_object.set("id", JsInteger::new(scope, record.id));
//       js_object.set("first_name", JsString::new(scope, &record.first_name[..]).unwrap());
//       js_object.set("last_name", JsString::new(scope, &record.last_name[..]).unwrap());
//       js_object.set("email", JsString::new(scope, &record.email[..]).unwrap());

//       try!(js_array.set(i as u32, js_object));
//     }

//     Ok(js_array)
//   }
// }

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(Queryable, Debug)]
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

