
# buntoy

toy javascript bundler

# what does it do?

```shell
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
