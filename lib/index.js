const diesel = require('../native'),
      User = require('./native').User;

let users_table = new Table("users");
let user_attr_records = users_table
  .filter(users_table.first_name.eq("Mike"))
  .limit(10)
  .load();

console.log(users);
