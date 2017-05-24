/**
 * Created by dp on 2017/5/24.
 */

var ffi = require('ffi');

// process.platform
// os.platform()

var lib
if(process.platform == 'win32'){
    lib = ffi.Library('target\\release\\ruby.dll', {
        'process': ['void', []]
    })
}else if(process.platform == 'linux'){
    lib = ffi.Library('target/release/libruby.so', {
        'process': ['void', []]
    })
}else{
    lib = ffi.Library('target/release/libruby.so', {
        'process': ['void', []]
    })
}

lib.process()

console.log('done!')