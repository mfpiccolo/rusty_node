var diesel = require('../native');

class User {
  constructor(id, first_name, last_name, email) {
    this.id = id;
    this.first_name = first_name;
    this.last_name = last_name;
    this.email = email;
  }

  static where(column, value) {
    return diesel.where(column, value);
  }

  toString() {
    return `User {id: ${this.id}, first_name: "${this.first_name}", last_name: "${this.last_name}" email: "${this.email}"}`;
  }
}

let user = new User('Mike', 'Piccolo', 'mfpiccolo@gmail.com');

console.log(user.toString());

console.log(User.where("name", "Mike"));
