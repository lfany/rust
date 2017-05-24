require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib 'target/release/ruby.dll'
  attach_function :process, [], :void
end

Hello.process

puts 'done!'