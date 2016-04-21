const diesel = require('../native');

let users_table = new Table("users");
let user_records = users_table
  .filter(users_table.first_name.eq("Mike"))
  .limit(10)
  .load();

console.log(user_records);

class Table {
  constructor(name) {
    this.name       = name;
    this.id         = new Column("id");
    this.first_name = new Column("first_name");
    this.last_name  = new Column("last_name");
    this.email      = new Column("email");
  }

  filter(query_fragment) {
    return this;
  }

  limit(number) {
    return this;
  }

  load() {
    return diesel.load("first_name", "Mike");
  }
}

class Column {
  constructor(name) {
    this.name = name;
  }

  eq(value) {
    return "eq query fragment";
  }
}

class User {
  constructor(id, first_name, last_name, email) {
    this.id = id;
    this.first_name = first_name;
    this.last_name = last_name;
    this.email = email;
  }

  toString() {
    return `User {id: ${this.id}, first_name: "${this.first_name}", last_name: "${this.last_name}" email: "${this.email}"}`;
  }
}

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
