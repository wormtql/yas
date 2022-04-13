<div align="center">

# Yas
Yet Another Genshin Impact Scanner  
又一个原神圣遗物导出器

</div>

## 介绍
基于CRNN（MobileNetV3_Small + LSTM）字符识别模型，使用原神字体对原神中会出现的字符串进行训练，达到更高的速度和更精确的结果。  
导出结果可以导入分析工具（例如 [莫娜占卜铺](https://mona-uranai.com/) ）进行配装或者其他计算  
由于使用了 [Rust](https://www.rust-lang.org/) 进行编写，运行效率和文件体积都得到了很大的提升  
### 相关资料
- [MobileNetV3](https://arxiv.org/pdf/1905.02244.pdf)
- [CRNN](https://arxiv.org/pdf/1507.05717.pdf)

## 使用
- 打开原神，并切换到背包页面，将背包拉到最上面
- 下载单exe可执行文件，右键管理员运行
- 扫描过程中，鼠标右键终止
### 注意
- 默认4星以下圣遗物不扫描
- 不是所有窗口比例都支持，推荐16:9的分辨率（如1600x900, 1920x1080, 3840x2160)
- 扫描过程中不要对鼠标做任何操作

### 命令行使用
假设你知道如何使用命令行工具  
查看选项
```shell
yas --help
```
只扫描五星圣遗物
```shell
yas --min-star=5
```
只扫描一行
```shell
yas --max-row=1
```

## 编译

```shell
# Linux下需要首先安装rustup以及mingw-w64，然后再安装对应的rust target，
rustup default stable
rustup target add x86_64-pc-windows-gnu
cargo build --release --locked --target=x86_64-pc-windows-gnu
```

## 训练
[yas-train](https://github.com/wormtql/yas-train)

## 反馈
- Issue
- QQ群：801106595
