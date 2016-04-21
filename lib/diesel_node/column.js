export default class Column {
  constructor(name) {
    this.name = name;
    this.query_fragment = [name];
  }

  eq(value) {
    this.query_fragment.push(['eq', value]);
    return this.query_fragment;
  }
}
