const ffi = require('ffi');
const libPath = 'native/target/release/libplutonium.dylib';

const libWeb = ffi.Library(libPath, {
  miner: ['Object', []],
  test: ['int', []],
});

const { miner, test } = libWeb;

console.log('Test', test());
console.log('Miner', miner());
