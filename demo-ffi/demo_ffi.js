var ffi = require('ffi');

var lib = ffi.Library('./target/debug/libdemo_ffi', {
	'double_input': ['int', ['int']]
});

var input = 4;
var output = lib.double_input(input);
console.log(output);