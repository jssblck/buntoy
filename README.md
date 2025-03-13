
# buntoy

toy javascript bundler.

i basically was like "what if i wanted to write a bundler from scratch, with only what i know, super quick and dirty".
it's super unoptimized, has barely any features, and generates some truly horrific bundled code.

but it was fun!

# what does it do?

```shell
# input:
# ❯ lt js
# js
# ├── env.js
# ├── index.js
# ├── package.json
# └── utils
#     ├── casing.js
#     ├── greet.js
#     └── orphan.js

buntoy on  main is 📦 v0.1.0 via  v23.7.0 via 🦀 v1.85.0
❯ cargo run -- js/index.js
warning: unused manifest key: package.about
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/buntoy js/index.js`
Bundling "js/index.js" to "index.bundle.js"
reading "js/index.js"
resolve import ./env -> "/Users/jess/projects/buntoy/js/env.js" as affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555
resolve import ./utils/greet -> "/Users/jess/projects/buntoy/js/utils/greet.js" as 81415136e5711a425bf66e43b141ce8e62703d86955745322f5838f31fe05d65
reading "/Users/jess/projects/buntoy/js/env.js"
reading "/Users/jess/projects/buntoy/js/utils/greet.js"
resolve import ../env -> "/Users/jess/projects/buntoy/js/env.js" as affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555
resolve import ./casing -> "/Users/jess/projects/buntoy/js/utils/casing.js" as c9345f6b7311a6d30e24402fc6ecd16ce1c4fc756b31790e8a32caf179a80356
reading "/Users/jess/projects/buntoy/js/env.js"
reading "/Users/jess/projects/buntoy/js/utils/casing.js"
writing c9345f6b7311a6d30e24402fc6ecd16ce1c4fc756b31790e8a32caf179a80356, imported by ["81415136e5711a425bf66e43b141ce8e62703d86955745322f5838f31fe05d65"]
writing affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555, imported by ["81415136e5711a425bf66e43b141ce8e62703d86955745322f5838f31fe05d65"]
writing affb629211c4a3293fcc9ff17b1fe30249874fa4394a4212e468d829625e1555, imported by ["81415136e5711a425bf66e43b141ce8e62703d86955745322f5838f31fe05d65"]
writing 81415136e5711a425bf66e43b141ce8e62703d86955745322f5838f31fe05d65, imported by []

buntoy on  main is 📦 v0.1.0 via  v23.7.0 via 🦀 v1.85.0
❯ node index.bundle.js
Note: use 'NAME' to set the name and 'GREETING' to set the greeting.
Hello, Unknown

buntoy on  main is 📦 v0.1.0 via  v23.7.0 via 🦀 v1.85.0
❯ NAME=jess node index.bundle.js
Note: use 'NAME' to set the name and 'GREETING' to set the greeting.
Hello, Jess

buntoy on  main is 📦 v0.1.0 via  v23.7.0 via 🦀 v1.85.0
❯ NAME=jess GREETING=heya node index.bundle.js
Note: use 'NAME' to set the name and 'GREETING' to set the greeting.
Heya, Jess
```

<details>
  <summary>Input files</summary>

  Contrived project 🥲

  ```js
  // --- index.js
  const env = require('./env');
  const greet = require('./utils/greet');

  console.log(`Note: use '${env.NAME_VAR}' to set the name and '${env.GREETING_VAR}' to set the greeting.`);
  console.log(greet());

  // --- env.js
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

  // --- utils/casing.js
  function englishCaseName(name) {
    const parts = name.split(' ')
    return parts.map(part => part.charAt(0).toUpperCase() + part.slice(1)).join(' ')
  }

  module.exports = {
    englishCaseName,
  }

  // --- utils/greet.js
  const env = require('../env');
  const casing = require('./casing');

  function greet() {
    const greeting = casing.englishCaseName(env.getGreeting());
    const name = casing.englishCaseName(env.getName());
    return `${greeting}, ${name}`;
  }

  module.export = greet;

  // --- utils/orphan.js
  function orphaned() {
    return 'this file is an orphan and should not be bundled'
  }

  module.exports = {
    orphaned,
  }
  ```

</details>

<details>
  <summary>Output bundle</summary>

  Warning: it's quite ugly 😅

  ```js
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
  ```
</details>
