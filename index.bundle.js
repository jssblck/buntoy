const __modules = {};
function require(path) {
    if (__modules[path]) {
        return __modules[path];
    }
    throw new Error('Module not found: ' + path);
}
;(function(require, modules) {
  const module = {};
  const __digest = 'c9345f6b7311a6d30e24402fc6ecd16ce1c4fc756b31790e8a32caf179a80356';
  (function() {
    
function englishCaseName(name) {
  const parts = name.split(' ')
  return parts.map(part => part.charAt(0).toUpperCase() + part.slice(1)).join(' ')
}

module.exports = {
  englishCaseName,
}

  })();
  if (!!module.exports) {
    modules[__digest] = module.exports;
  } else if (!!module.export) {
    modules[__digest] = module.export;
  } else if (!!module.default) {
    modules[__digest] = module.default;
  } else if (!!module) {
    modules[__digest] = module;
  }
})(require, __modules);;(function(require, modules) {
  const module = {};
  const __digest = 'affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555';
  (function() {
    
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

  })();
  if (!!module.exports) {
    modules[__digest] = module.exports;
  } else if (!!module.export) {
    modules[__digest] = module.export;
  } else if (!!module.default) {
    modules[__digest] = module.default;
  } else if (!!module) {
    modules[__digest] = module;
  }
})(require, __modules);;(function(require, modules) {
  const module = {};
  const __digest = 'affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555';
  (function() {
    
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

  })();
  if (!!module.exports) {
    modules[__digest] = module.exports;
  } else if (!!module.export) {
    modules[__digest] = module.export;
  } else if (!!module.default) {
    modules[__digest] = module.default;
  } else if (!!module) {
    modules[__digest] = module;
  }
})(require, __modules);;(function(require, modules) {
  const module = {};
  const __digest = '81415136e5711a425bf66e43b141ce8e62703d86955745322f5838f31fe05d65';
  (function() {
    
const env = require('affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555');
const casing = require('c9345f6b7311a6d30e24402fc6ecd16ce1c4fc756b31790e8a32caf179a80356');

function greet() {
  const greeting = casing.englishCaseName(env.getGreeting());
  const name = casing.englishCaseName(env.getName());
  return `${greeting}, ${name}`;
}

module.export = greet;

  })();
  if (!!module.exports) {
    modules[__digest] = module.exports;
  } else if (!!module.export) {
    modules[__digest] = module.export;
  } else if (!!module.default) {
    modules[__digest] = module.default;
  } else if (!!module) {
    modules[__digest] = module;
  }
})(require, __modules);;(function(require) {
  
const env = require('affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555');
const greet = require('81415136e5711a425bf66e43b141ce8e62703d86955745322f5838f31fe05d65');

console.log(`Note: use '${env.NAME_VAR}' to set the name and '${env.GREETING_VAR}' to set the greeting.`);
console.log(greet());

})(require);