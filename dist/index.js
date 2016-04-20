'use strict';

var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

var diesel = require('../native');

var User = function () {
  function User(id, first_name, last_name, email) {
    _classCallCheck(this, User);

    this.id = id;
    this.first_name = first_name;
    this.last_name = last_name;
    this.email = email;
  }

  _createClass(User, [{
    key: 'toString',
    value: function toString() {
      return 'User {id: ' + this.id + ', first_name: "' + this.first_name + '", last_name: "' + this.last_name + '" email: "' + this.email + '"}';
    }
  }], [{
    key: 'where',
    value: function where(column, value) {
      return diesel.where(column, value);
    }
  }]);

  return User;
}();

var user = new User('Mike', 'Piccolo', 'mfpiccolo@gmail.com');

console.log(user.toString());

console.log(User.where("name", "Mike"));