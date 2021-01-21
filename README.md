# rust-xs-test（初步完成）
Auto Test Routine Under the Hood for XiangShan Processor  

## Build
环境配置请看[这里](https://github.com/RISCVERS/rust-xs-evaluation/blob/main/doc/build.md)  
```bash
git clone https://github.com/RISCVERS/rust-xs-test
```
## Run
```bash
cargo run
```

## Features
+ 一直运行在后台
+ 隔一定时间另外开一个线程去执行仿真任务：
    - 根据当前时间创建工作目录 `workload`
    - 配置文件中如果没有指定本地香山目录，则从 `github` 上 `clone` 到工作目录下
    - 在香山目录执行 `make init`
    - 在香山目录执行 `git pull`
    - 编译香山
    - 在香山目录下执行 `git log`，输出重定向到工作目录下的 `git_log.txt` 文件
    - 复制香山 `build` 目录下的 `XSSimTop.v` 和 `emu` 文件到工作目录
    - 寻找当前机器中数量和配置文件指定的 `emu` 线程数相等的连续的空闲处理核，如果找不到，则等待一定时间，等待三次后线程退出
    - 从配置文件中指定的测试用例目录中随机找一个来进行仿真，并重定向输出到工作目录下的 `emu_res/stdout.txt` 和 `emu_res/stderr.txt`
    - 如果仿真结果 `hit good trap` 或者跑完配置文件中指定的最大指令数，则删除工作目录，线程退出
    - TODO: 如果仿真结果不是以上两种情况，则应该回溯一万周期重跑并打印波形
+ 完整的 `log` 信息
+ 极小的 CPU 开销
+ 基于 `Rust` 语言实现，极少或者基本不存在内存泄漏等运行时 BUG
+ TODO: 完善的自动错误处理
+ **TODO: 通过 `FFI` 提供 `C` 或 `C++` 错误处理函数接口，这样一来不熟悉 `Rust` 的使用者可以写 `C/C++` 代码来对错误进行自定义处理**

## Config
配置文件为 `toml` 格式，默认为本项目目录下的 `config.toml` 文件。（使用者可以修改，不过这样需要同时修改源码）  
example:  
```toml
# 多线程测试后台程序
[hook]
# 测试线程数，目前建议设为 1
workers_num = 1
# 工作根目录
work_root = "/home/ccc/rust_xs_test_workload"
# 内部每次循环线程休眠时间，单位为秒
sleep_time = 120

[emu]
# emu 编译线程数
thread_num = 8
# 仿真最大指令数
max_instr = 1000000
# 香山目录，可选。如果不指定的话会自动从 github 上 clone 最新的香山源码到工作目录
noop_home = "/home/ccc/XiangShan"
# NEMU 目录，可选。默认为 /home/ccc/NEMU
nemu_home = "/home/ccc/NEMU"
# AM 目录，可选。默认为 /home/ccc/nexus-am
am_home = "/home/ccc/nexus-am"
# 测试列表目录，可选。默认为 /bigdata/zyy/checkpoints_profiles/betapoint_profile_06
img_list = "/bigdata/zyy/checkpoints_profiles/betapoint_profile_06"
```

## TODO
+ 完善错误处理
+ 通过 `FFI` 提供 `C/C++` 错误处理函数接口
+ 预期是想支持多线程测试的，但目前在同一个机器上有多个香山编译进程的话会出问题，因此暂时只支持单线程。考虑通过对编译环节添加锁机制使之支持多线程
+ 目前香山编译有时候会出现卡住的情况，这样将会导致线程一直退不出来，还在思考解决办法
+ 修缮代码，使之更加优雅


