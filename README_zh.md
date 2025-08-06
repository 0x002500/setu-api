# Setu API (简体中文)

一个使用Rust编写的简单纯色图片生成API服务。

## 功能特点

- 支持通过十六进制颜色代码生成纯色图片
- 可自定义图片尺寸
- 支持RGBA颜色通道（包括透明度）
- 输出PNG格式图片
- RESTful API接口
- 轻量且快速

## 技术栈

- Rust 2024 Edition
- actix-web (Web框架)
- image (图像处理库)
- serde (序列化/反序列化)

## 系统要求

- Rust 2024 Edition
- Cargo包管理器

## 安装

1. 克隆仓库：
```bash
git clone https://github.com/0x002500/setu-api.git
cd setu-api
```

2. 编译项目：
```bash
cargo build --release
```

3. 运行服务：
```bash
cargo run --release
```

服务将在 `http://127.0.0.1:8080` 启动

## API使用说明

### 接口列表

#### 1. 健康检查
```
GET /
```
返回： "Thanks for using setu-api!"

#### 2. 生成图片
```
GET /pic?color={hex_color}&width={width}&height={height}
```

参数：
- `color`: 十六进制颜色代码（6位或8位RGBA）
- `width`: 图片宽度（像素）
- `height`: 图片高度（像素）

返回：PNG格式图片文件

### 示例

1. 生成一个红色的200x200像素图片：
```
GET http://127.0.0.1:8080/pic?color=FF0000&width=200&height=200
```

2. 生成一个半透明的蓝色100x100像素图片：
```
GET http://127.0.0.1:8080/pic?color=0000FF80&width=100&height=100
```

## 开发

要参与开发，请确保已安装Rust开发环境。

1. 安装依赖：
```bash
cargo build
```

2. 运行测试：
```bash
cargo test
```

## 依赖项

- actix-web: ^4.11.0
- image: ^0.25.6
- serde: ^1.0.219
- serde_json: ^1.0.142

## 许可证

MIT许可证

## 贡献

欢迎提交Issue和Pull Request！

## 作者

- 0x002500
