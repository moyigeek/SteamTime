# Steam Time

Steam Time 是一个工具，用于获取 Steam 游戏信息并将其保存为 CSV 文件。

## 功能

- 获取拥有的游戏列表
- 获取最近玩的游戏列表
- 将游戏信息保存为 CSV 文件

## 安装

1. 克隆此仓库：
    ```sh
    git clone https://github.com/yourusername/steamTime.git
    cd steamTime
    ```

2. 确保你已经安装了 [Rust](https://www.rust-lang.org/tools/install) 和 [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)。

3. 安装依赖项：
    ```sh
    cargo build
    ```

## 使用

运行以下命令来获取 Steam 游戏信息：

```sh
cargo run -- -i <SteamID> [-c <config.yaml>]
```
参数说明：

-i, --id <SteamID>: 必需，指定 Steam ID。
-c, --config <config.yaml>: 可选，指定配置文件路径，默认为 config.yaml。
示例：
```sh
cargo run -- -i 76561198006409530 -c custom_config.yaml
```
配置文件
配置文件是一个 YAML 文件，包含以下内容：
    
```yaml
domain: "api.steampowered.com"
api_key: "your_steam_api_key"
```
输出
程序将生成两个 CSV 文件：

owned_games.csv: 包含拥有的游戏信息。
recently_played_games.csv: 包含最近玩的游戏信息。
贡献
欢迎贡献代码！请 fork 此仓库并提交 pull request。

## TODO
- [ ] 添加更多导出选项
- [ ] 添加更多 Steam API 方法
- [ ] 添加游戏图标解析