var addon = require('../native/index.node');

console.log(addon.hello());
module.exports = addon;