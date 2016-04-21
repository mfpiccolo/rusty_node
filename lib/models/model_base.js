export default class ModelBase {
  static from_obj(obj) {
    let values = Object.keys(obj).map(function(key){return obj[key]});
    return new this.prototype.constructor(...values);
  }
}
