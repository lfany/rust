require 'ffi'

def os_family
  case RUBY_PLATFORM
    when /linux/
      :linux
    when /ix/i, /ux/i, /gnu/i,
        /sysv/i, /solaris/i,
        /sunos/i, /bsd/i
      :unix
     when /darwin|mac os/
      :macosx
    when /win/i, /ming/i
      :windows
    else
      :other
  end
end

module Hello
  extend FFI::Library
  ffi_lib os_family == :windows ? 'target/release/ruby.dll' : os_family == :linux || os_family == :unix ?
      'target/release/libruby.so' : os_family == :macosx ?
      'target/release/libruby.dylib' : 'target/release/libruby.so'
  attach_function :process, [], :void
end

Hello.process

puts 'done!'

puts os_family