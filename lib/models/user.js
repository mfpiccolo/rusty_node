export default class User {
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
