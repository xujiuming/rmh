# rmh

```shell
cargo run  
# 默认使用当前操作系统的 glibc 打包 如果比较新 发布到服务器上可能找不到对应的glibc版本   例如arch上打包 在ubuntu server 2004上就缺少高版本的glibc 
cargo build --release 
# 出现  error occurred: Failed to find tool. Is `musl-gcc` installed?
yay -S musl 
# 安装交叉编译musl支持  
rustup target add x86_64-unknown-linux-musl
# 交叉编译 musl版本  
cargo build --release --target=x86_64-unknown-linux-musl
# 查看静态链接   
ldd target/x86_64-unknown-linux-musl/release/ming-api
# 登录crate.io 
cargo login  [secret]
#  打包一个 crate 包
cargo package --list
cargo package  
cargo package --allow-dirty 
# 上传 
cargo publish --allow-dirty 
cargo publish --dry-run --allow-dirty 
```
