export default class User extends ModelBase {
  constructor(id, first_name, last_name, email) {
    super();
    this.id = id;
    this.first_name = first_name;
    this.last_name = last_name;
    this.email = email;
  }
}
