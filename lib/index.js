const diesel = require('../native');

let users_table = new Table("users");
let user_attr_records = users_table
  .filter(users_table.first_name.eq("Mike"))
  .limit(10)
  .load();

let users = user_attr_records.map(function (user_attrs) {
  return User.from_obj(user_attrs);
})

console.log(users);
