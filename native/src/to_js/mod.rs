use neon::vm::JsResult;
use neon::js::JsArray;

pub trait ToJsArray<'a, T> {
  fn to_js_array(self) -> JsResult<'a, JsArray>;
}

#[macro_export]
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
