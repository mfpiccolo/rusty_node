export default class Table {
  constructor(name) {
    this.name       = name;
    this.query      = [name];
    this.id         = new Column("id");
    this.first_name = new Column("first_name");
    this.last_name  = new Column("last_name");
    this.email      = new Column("email");
  }

  filter(query_fragment) {
    this.query.push(query_fragment);
    return this;
  }

  limit(number) {
    this.query.push(['limit', number]);
    return this;
  }

  load() {
    console.log(this.query);
    return diesel.load("first_name", "Mike");
  }
}
