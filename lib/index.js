const diesel = require('../native');

let users_table = new Table("users");
let user_records = users_table
  .filter(users_table.first_name.eq("Mike"))
  .limit(10)
  .load();

console.log(user_records);

// let users = diesel.users;
// users.filter(users.first_name.eq("Mike")).limit(10).load()
//
// // A few ideas here:
// // 1. Use a compile step where this will cache a perpared statment and this
// //    combination will create a uniqe query TypeId. Then when this is run, the
// //    chained query will build up a unique signature that can be looked up
// //    when send over to rust
// //
// // 2.
