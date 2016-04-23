const diesel = require('../native'),
      User = require('../native').User;

let users_table = new Table("users");
let users = users_table
  .filter(users_table.first_name.eq("Mike"))
  .limit(10)
  .load();

console.log(users);
