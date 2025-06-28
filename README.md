# 改版RS
如果想要知道这个RS可以干什么可以点击[这个链接](https://git.xeondev.com/wickedwaifus/wicked-waifus-rs)<br>
# 这个改版和原来的版本有什么区别
和原来的最大的区别是在启动时会分别报错2个错误
'''
thread 'main' panicked at C:\Users\Administrator\OneDrive\Desktop\wicked-waifus-rs\wicked-waifus-http\src\lib.rs:57:14:

Path segments must not start with `:`. For capture groups, use `{capture}`. If you meant to literally match a segment starting with a colon, call `without_v07_checks` on the router.

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: process didn't exit successfully: `target\debug\wicked-waifus-hotpatch-server.exe` (exit code: 101)
'''
'''
error: process didn't exit successfully: `target\debug\wicked-waifus-login-server.exe` (exit code: 1)
'''
我把这个错误给修复了，就是这么简单。
