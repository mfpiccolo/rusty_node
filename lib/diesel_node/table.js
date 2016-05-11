export default class Table {
  constructor(name) {
    this.name       = name;
    this.query      = ["select", name];
  }

  filter(query_fragment) {
    this.query.push("filter", query_fragment);
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
