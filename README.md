<div align="center">

# Yas

Yet Another Genshin Impact Scanner
又一个原神圣遗物导出器

</div>

## 介绍

基于 SVTR（基本上是 MobileNetV3_Small + Transformer）字符识别模型，使用原神字体对原神中会出现的字符串进行训练，达到更高的速度和更精确的结果。相比 CRNN，SVTR 可以达到更小的体积及更好的识别率
导出结果可以导入分析工具（例如 [莫娜占卜铺](https://mona-uranai.com/) ）进行配装或者其他计算
由于使用了 [Rust](https://www.rust-lang.org/) 进行编写，运行效率和文件体积都得到了很大的提升

### 相关资料

- [MobileNetV3](https://arxiv.org/pdf/1905.02244.pdf)
- [CRNN](https://arxiv.org/pdf/1507.05717.pdf)
- [SVTR](https://arxiv.org/pdf/2205.00159.pdf)
- [Transformer](https://proceedings.neurips.cc/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf)

### 识别模型

SVTR 原文使用了多个 Local/Global Mixing，其中 Global Mixing 就是 Transformer 层，而根据*PaddleOCR*的代码，其 SVTR 识别模型也并未完全遵照 SVTR 原模型，而是骨干网络 + Transformer 的结构
*Yas*同样采用 PaddleOCR 的做法，即 MobileNetV3_Small + Global Mixing，相当于将原 RNN 替换为 Transformer。

## 使用
`yas.exe`把不同游戏的功能都集成到了一个exe中，因此需要使用命令行指定游戏，例如：
```shell
yas.exe genshin
```
运行`yas.exe --help`查看所有指令，运行`yas.exe help genshin`查看游戏特定的指令。

也可以下载特定游戏的版本，例如`yas_artifact.exe`只能用于扫描原神的圣遗物。

### Windows

- 打开原神/星铁，并切换到背包页面，将背包拉到最上面
- 如果是`yas.exe`，需要用命令行运行`yas.exe genshin`，如果是`yas_artifact.exe`，直接运行即可
- 扫描过程中，鼠标右键终止

### Linux
- 还没有经过详细测试
- 首先请确保自己在 x11 下或者 GNOME/Wayland 下（其他 wayland de 下[会有很坏的性能](https://github.com/poly000/screenshots-rs/blob/d96dff76c5f5cbd849d80451f0df8f415f8e5f4b/src/linux/wayland_screenshot.rs#L109)）
- 用 wine 窗口化运行原神（或者全屏+虚拟桌面），打开圣遗物界面，拉到最顶
- 启动 yas
- Alt+Tab 切换到原神窗口，并且在鼠标变为十字后点击一下（还没做窗口聚焦），注意保证原神窗口整体在屏幕内
- 等待扫描结束。

### 注意

- 默认 4 星以下圣遗物不扫描
- 不是所有窗口比例都支持，推荐 16:9 的分辨率（如 1600x900, 1920x1080, 3840x2160）
- 扫描过程中不要对鼠标做任何操作
- 当前仅支持中文环境，若默认系统为非中文，请前往游戏设置界面修改 Language 为“简体中文”，否则无法读取原神窗口
- 当前仅支持键鼠作为控制设备，暂不支持手柄。

### 命令行使用

假设你知道如何使用命令行工具。


查看选项：
```shell
yas --help
yas help genshin
yas help starrail
```

只扫描五星圣遗物：
```shell
yas genshin --min-star=5
```

只扫描一行：
```shell
yas genshin --max-row=1
```

## 编译

在构建前，请确保安装`Git LFS`，并运行`git lfs pull`。否则[yas 在运行时会使用错误的模型](https://github.com/wormtql/yas/pull/102#issuecomment-1375503803)。

```shell
# Linux 下需要首先安装 rustup 以及 mingw-w64 ，然后再安装对应的 rust target，
# 构建到Linux需要 `libxdo` 和 `libxcb`
rustup default stable
rustup target add x86_64-pc-windows-gnu
cargo build --release --locked --target=x86_64-pc-windows-gnu
```

如果使用 macOS，为了保证正常捕捉窗口，需要在编译后运行 `codesign.sh` 对二进制文件进行签名

## 训练

[yas-train](https://github.com/wormtql/yas-train)

## 反馈

- Issue
- QQ 群：801106595
