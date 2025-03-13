
const env = require('../env');
const casing = require('./casing');

function greet() {
  const greeting = casing.englishCaseName(env.getGreeting());
  const name = casing.englishCaseName(env.getName());
  return `${greeting}, ${name}`;
}

module.export = greet;
