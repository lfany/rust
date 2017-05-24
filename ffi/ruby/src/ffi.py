from ctypes import cdll

import platform, os

os_family = platform.system()

print(os_family)
print(os.getcwd())
if os.getcwd().endswith('src'):
    os.chdir('..')
    print(os.getcwd())

if os_family == 'Windows':
    lib = cdll.LoadLibrary('target\\release\\ruby.dll')
elif os_family == 'Linux':
    lib = cdll.LoadLibrary('target/release/libruby.so')
elif os_family == 'Darwin':
    lib = cdll.LoadLibrary('target/release/libruby.dylib')
else:
    lib = cdll.LoadLibrary('target/release/libruby.so')


lib.process()

print('done!')
