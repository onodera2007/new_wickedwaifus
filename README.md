# Wicked Waifus

![截图](screenshot.png)

## 关于
**Wicked Waifus 是一个用 Rust 编写的开源呼啸山庄服务器模拟器。
该项目的目标是确保一个简洁、易懂的代码环境。
Wicked Waifus 使用**tokio**进行异步网络操作，**axum**作为 http 框架，**ZeroMQ**用于服务器之间的通信。
它还实现了**性能良好、可扩展的 ECS**，用于模拟游戏世界。

## 入门
#### 要求
- Rust](https://www.rust-lang.org/tools/install)
- PostgreSQL](https://www.postgresql.org/download/)
- Protoc](https://github.com/protocolbuffers/protobuf/releases) （用于 protobuf 代码源

#### 设置
##### a) 从源代码构建

``sh
git clone --recursive https://git.xeondev.com/wickedwaifus/wicked-waifus-rs.git
cd wicked-waifus-rs
cargo run --bin wicked-waifus-config-server
cargo run --bin wicked-waifus-hotpatch-server
cargo run --bin wicked-waifus-login-server
cargo run --bin wicked-waifus-gateway-server
cargo run --bin wicked-waifus-game-server
````

##### b) 从源代码构建（docker 版本）
如果你的轮椅不适合选项 A，你可以退而求其次选择选项 B。
在这种情况下，你需要 [Docker Desktop](https://www.docker.com/products/docker-desktop/)。

安装完成后，运行以下命令即可构建镜像：
``sh
# 或 builder.bat（如果在 Windows 上运行）
./builder.sh
````

然后运行：
```sh
docker compose up -d
```

##### c) 使用预编译的二进制文件
浏览 [Releases](https://git.xeondev.com/wickedwaifus/wicked-waifus-rs/releases)
页面，为你的平台下载最新版本。<br>
启动所有服务器： wicked-waifus-config-server`、`wicked-waifus-hotpatch-server`、`wicked-waifus-login-server`、`wicked-waifus-gateway-server`、`wicked-waifus-game-server`。

##### 注意：如果你要使用预编译的二进制文件，你不必安装 Rust 和 Protoc，尽管首选的方式是从源代码编译。<br>我们不为预编译的二进制文件提供任何支持。

#### 配置
你应该使用自己的配置文件配置每台服务器。首次启动时，它们会在当前工作目录中创建。

##### 数据库部分
您必须指定**PostgreSQL**的凭据<br>。
###### 数据库配置示例：
```
[database]
host = "localhost:5432"
user_name = "postgres"
password = "password"
db_name = "ww"
```
##### 注意：不要忘记用指定的 `db_name`（默认： `wicked_waifus_db`）创建数据库。例如，您可以使用 PgAdmin 进行创建。

#### 数据
数据文件： 逻辑 JSON 集合（`data/assets/game-data/BinData`）和配置/热补索引（`data/assets/config-server`, `data/assets/hotpatch-server`）包含在此资源库中。请注意，当前工作目录中必须有 `data` 子目录。

### 疑难解答
[如果您有任何疑问/问题，请访问我们的 discord](https://discord.gg/reversedrooms)

### 支持
如果您想支持本项目，请随时 [通过 boosty 发送提示](https://boosty.to/xeondev/donate)
