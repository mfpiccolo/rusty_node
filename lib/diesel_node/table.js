export default class Table {
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
