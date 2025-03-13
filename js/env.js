
function getName() {
  return process.env.NAME || 'unknown';
}

function getGreeting() {
  return process.env.GREETING || 'hello';
}

module.exports = {
  getName,
  getGreeting,
  NAME_VAR: 'NAME',
  GREETING_VAR: 'GREETING',
}
