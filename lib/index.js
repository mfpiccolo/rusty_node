const diesel = require('../native');

let users_table = new UsersTable("users");
let users = users_table
  .filter(users_table.first_name.eq("Mike"))
  .limit(10)
  .load();

console.log(users);
