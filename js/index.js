
const env = require('./env');
const greet = require('./utils/greet');

console.log(`Note: use '${env.NAME_VAR}' to set the name and '${env.GREETING_VAR}' to set the greeting.`);
console.log(greet());
