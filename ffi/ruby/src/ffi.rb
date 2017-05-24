require 'ffi'

def os_family
  case RUBY_PLATFORM
    when /ix/i, /ux/i, /gnu/i,
        /sysv/i, /solaris/i,
        /sunos/i, /bsd/i
      :unix
    when /linux/
      :linux
    when /win/i, /ming/i
      :windows
    when /darwin|mac os/
      :macosx
    else
      :other
  end
end

module Hello
  extend FFI::Library
  ffi_lib os_family == :windows ? 'target/release/ruby.dll' : os_family == :linux|| :unix ?
      'target/release/libruby.so' : 'target/release/xxx'
  attach_function :process, [], :void
end

Hello.process

puts 'done!'

puts os_family