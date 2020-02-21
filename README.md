# ip_sniffer
* Rust 小项目系列，主要用于练习使用 Rust。
* 本项目是 ip 端口嗅探器工具。

## build
* `cargo build`

## usage
* `./target/debug/ip_sniffer -j 81 192.168.1.1` 开启 81 个线程，进行嗅探对应 ip 的网络设备端口是否开放。
* 运行后，等待一段时间运行完成后输出形如下方：

```
this port 64022 may not open
this port 65022 may not open
80 is open
139 is open
445 is open
21 is open
53 is open
8080 is open
17998 is open
37443 is open
37444 is open
49652 is open
49653 is open
```

## 参考资料
* https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb&index=1
