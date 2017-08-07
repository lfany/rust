from ctypes import cdll

import platform, os

os_family = platform.system()

print(os_family)
print(os.getcwd())
if os.getcwd().endswith('src'):
    os.chdir('..')
    print(os.getcwd())

if os_family == 'Windows':
    lib = cdll.LoadLibrary('target\\release\\ruby_process.dll')
elif os_family == 'Linux':
    lib = cdll.LoadLibrary('target/release/libruby_process.so')
elif os_family == 'Darwin':
    lib = cdll.LoadLibrary('target/release/libruby_process.dylib')
else:
    lib = cdll.LoadLibrary('target/release/libruby_process.so')


lib.process()

print('done!')
