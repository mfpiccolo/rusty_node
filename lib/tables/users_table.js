export default class UsersTable extends Table {
  constructor(id, first_name, last_name, email) {
    super();
    this.id         = new Column("id");
    this.first_name = new Column("first_name");
    this.last_name  = new Column("last_name");
    this.email      = new Column("email");
  }
}
