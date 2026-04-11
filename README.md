# GLM-Token-Monitor

<div align="center">

![GLM-Token-Monitor Logo](design/icons/方案1-环形进度.svg)

**🔍 一个可爱的跨平台桌面 API 配额监控工具**

[![GitHub stars](https://img.shields.io/github/stars/huangbh2020/glm-token-monitor?style=social)](https://github.com/huangbh2020/glm-token-monitor/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/huangbh2020/glm-token-monitor?style=social)](https://github.com/huangbh2020/glm-token-monitor/network/members)
[![GitHub issues](https://img.shields.io/github/issues/huangbh2020/glm-token-monitor)](https://github.com/huangbh2020/glm-token-monitor/issues)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](./README_EN.md) | 简体中文

</div>

---

## ✨ 简介

**GLM-Token-Monitor** 是一个跨平台的桌面 API 配额监控应用，通过可爱的电子宠物和多种炫酷的显示模式，实时展示您的 API 使用情况。

### 🎯 核心特性

- **🐾 可爱宠物** - 多种电子宠物角色，根据使用量展示不同动画状态
- **📊 实时监控** - 自动轮询 API 配额，支持多提供商（智谱、Z.AI 等）
- **🎨 多种显示模式** - 全息气泡、赛博圆环、光场、能量核心等 6 种显示效果
- **⚡ 轻量高效** - 基于 Tauri 2.0 + Vue 3，资源占用极低
- **🪟 跨平台支持** - Windows、macOS、Linux
- **🎨 高度可定制** - 支持自定义宠物、显示模式、光晕效果等

## 📖 目录

- [功能展示](#-功能展示)
- [技术栈](#-技术栈)
- [安装和使用](#-安装和使用)
- [项目结构](#-项目结构)
- [配置说明](#-配置说明)
- [开发指南](#-开发指南)
- [常见问题](#-常见问题)
- [贡献指南](#-贡献指南)

---

## 🎬 功能展示

### 宠物状态系统

应用根据 API 使用量自动切换宠物状态，每种状态都有独特的动画效果：

| 状态 | 使用量 | 颜色 | 动画效果 |
|:----:|:------:|:----:|:--------|
| 🟢 Fresh | 0-30% | 绿色 | 呼吸动画，悠闲自在 |
| 🔵 Flow | 31-60% | 蓝色 | 敲打动画，努力工作中 |
| 🟡 Warning | 61-80% | 黄色 | 抖动动画，需要注意用量 |
| 🟠 Panic | 81-95% | 橙色 | 冒汗动画，快要用完了 |
| ⚫ Dead | 96-100% | 灰色 | 灵魂出窍，配额耗尽 |

### 显示模式

提供 6 种炫酷的 Token 剩余量显示模式：

- **None** - 极简模式，不显示数值
- **Holo Bubble** - 全息气泡，科幻感十足
- **Cyber Ring** - 赛博圆环，经典进度环设计
- **Aura Field** - 光场扩散，视觉效果柔和
- **Energy Core** - 能量核心，像素风格网格
- **Status Floater** - 状态浮动条，侧边显示

### 双指标监控

- **🗓 月度 MCP 额度** - 监控每月 API 调用次数限制
- **⏱ 5h Token 额度** - 监控 5 小时窗口内的 Token 使用量（驱动宠物状态）

## 🛠 技术栈

### 前端
- **Vue 3.4** - 渐进式 JavaScript 框架
- **TypeScript 5.3** - 类型安全
- **TailwindCSS 3.4** - 实用优先的 CSS 框架
- **Vite 5.0** - 快速的前端构建工具

### 后端
- **Rust** - 系统级性能
- **Tauri 2.0** - 跨平台桌面应用框架
- **Tokio** - 异步运行时
- **reqwest** - HTTP 客户端

### 主要依赖
- `tauri-plugin-shell` - 系统 shell 操作
- `serde/serde_json` - 序列化/反序列化
- `chrono` - 时间处理
- `dirs` - 系统目录获取

## 📦 安装和使用

### 前置要求

- **Node.js** 18+
- **Rust** 1.70+
- **npm** 或 **yarn**

### 从源码构建

```bash
# 1. 克隆仓库
git clone https://github.com/huangbh2020/glm-token-monitor.git
cd glm-token-monitor

# 2. 安装依赖
npm install

# 3. 启动开发模式（热重载）
npm run tauri:dev

# 4. 构建生产版本
npm run tauri:build
```

构建完成后，安装包位于 `src-tauri/target/release/bundle/` 目录。

### 配置 API

首次运行后，需要配置您的 API 信息：

1. 右键点击系统托盘图标，选择"设置"
2. 填写 API Key
3. 选择提供商（智谱 BigModel 或 Z.AI）
4. 点击"测试连接"验证配置
5. 保存设置

### Windows 用户

- 下载 `.exe` 安装包，双击安装
- 应用会自动启动并显示在桌面
- 点击宠物窗口查看详细信息
- 拖动窗口可移动位置

### macOS 用户

- 下载 `.dmg` 文件，拖拽到 Applications
- 在系统托盘中找到应用图标
- 点击托盘图标查看用量信息

## 📁 项目结构

```
glm-token-monitor/
├── src/                          # Vue 前端代码
│   ├── components/               # Vue 组件
│   │   ├── PetWidget.vue        # Windows 桌面宠物组件
│   │   ├── PopoverPanel.vue     # macOS 气泡面板组件
│   │   └── pets/                # 宠物组件目录
│   │       ├── JellySpirit.vue  # 果冻精灵
│   │       ├── PixelGhost.vue   # 像素幽灵
│   │       ├── DogSit.vue       # 狗狗-坐
│   │       └── ...
│   ├── composables/             # Vue 组合式函数
│   │   ├── useUsageState.ts    # 使用量状态管理
│   │   ├── useTauriEvents.ts   # Tauri 事件监听
│   │   ├── usePlatform.ts       # 平台检测
│   │   ├── useSettings.ts       # 配置管理
│   │   ├── useDisplayMode.ts    # 显示模式管理
│   │   └── usePetAction.ts      # 宠物动作管理
│   ├── App.vue                  # 主应用组件
│   ├── main.ts                  # 应用入口
│   └── style.css                # 全局样式
├── src-tauri/                   # Rust 后端代码
│   ├── src/
│   │   ├── main.rs              # 应用入口
│   │   ├── events.rs            # 事件定义和数据结构
│   │   ├── polling.rs           # 轮询逻辑
│   │   ├── commands.rs          # Tauri 命令
│   │   ├── config.rs            # 配置管理
│   │   ├── settings_commands.rs # 设置相关命令
│   │   ├── windows.rs           # 窗口管理
│   │   └── tray.rs              # 系统托盘支持
│   ├── Cargo.toml               # Rust 依赖配置
│   └── tauri.conf.json          # Tauri 配置
├── design/                      # 设计资源
│   └── icons/                   # 图标设计方案
├── CLAUDE.md                    # Claude Code 开发指南
├── README.md                    # 项目说明
└── package.json                 # Node.js 依赖配置
```

## ⚙️ 配置说明

### 轮询间隔

默认每 **1 分钟**自动刷新 API 使用量，可在设置中调整。

### 配置文件位置

- **Windows**: `%APPDATA%\glm-token-monitor\config.json`
- **macOS**: `~/Library/Application Support/glm-token-monitor/config.json`
- **Linux**: `~/.config/glm-token-monitor/config.json`

### 可配置项

- **API Key**: 您的 API 访问密钥
- **提供商**: 智谱 BigModel / Z.AI
- **轮询间隔**: 1-60 分钟
- **显示模式**: 6 种显示效果
- **宠物类型**: 多种可爱宠物
- **光晕效果**: 开启/关闭状态光晕
- **开机自启**: 系统启动时自动运行

## 🚀 开发指南

### 环境配置

```bash
# 克隆仓库
git clone https://github.com/huangbh2020/glm-token-monitor.git
cd glm-token-monitor

# 安装依赖
npm install

# 启动开发模式
npm run tauri:dev
```

### 添加新的 API 提供商

1. 编辑 `src-tauri/src/config.rs`：

```rust
pub fn api_domain(&self) -> &'static str {
    match self.provider.as_str() {
        "bigmodel" => "https://open.bigmodel.cn/",
        "zai" => "https://api.z.ai/",
        "your-provider" => "https://your-api.com/", // 新增
        _ => "https://open.bigmodel.cn/",
    }
}
```

2. 更新 API 响应解析逻辑（如果需要）

### 添加新的宠物类型

1. 在 `src/components/pets/` 创建新的宠物组件
2. 在 `src/components/PetWidget.vue` 中注册组件
3. 在 `src/composables/usePetAction.ts` 添加动作逻辑
4. 更新配置默认值

### 添加新的显示模式

1. 在 `src/components/PetWidget.vue` 添加新的显示分支
2. 在 `<style scoped>` 添加对应样式
3. 在 `src/composables/useDisplayMode.ts` 更新模式列表

### 代码规范

- **Rust**: 遵循 `rustfmt` 格式化
- **TypeScript/Vue**: 遵循 ESLint 配置
- **Commit**: 使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范

```
feat: 新功能
fix: 修复 bug
docs: 文档更新
style: 代码格式调整
refactor: 重构
test: 测试相关
chore: 构建/工具链更新
```

## 🧪 测试

### Rust 端测试

```bash
cd src-tauri
cargo test
```

### 前端构建测试

```bash
npm run build
```

## ❓ 常见问题

### 编译错误

**问题**: 构建失败，提示依赖错误

**解决方案**:
```bash
# 更新 Rust 工具链
rustup update

# 清理并重新安装依赖
rm -rf node_modules
npm install

# 清理 Rust 构建缓存
cd src-tauri
cargo clean
```

### Tauri 开发服务器无法启动

**问题**: 端口被占用或启动失败

**解决方案**:
```bash
# 检查端口 1420 是否被占用
# Windows: netstat -ano | findstr :1420
# macOS/Linux: lsof -i :1420

# 清理缓存
npm run clean

# 重新启动
npm run tauri:dev
```

### API 连接失败

**问题**: 无法获取 API 使用量

**解决方案**:
1. 检查 API Key 是否正确
2. 检查网络连接
3. 在设置中点击"测试连接"验证配置
4. 查看日志文件获取详细错误信息

## 🤝 贡献指南

我们欢迎所有形式的贡献！

### 如何贡献

1. **Fork** 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'feat: add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 **Pull Request**

### 报告问题

- 使用 [GitHub Issues](https://github.com/huangbh2020/glm-token-monitor/issues) 报告 Bug
- 提供详细的复现步骤和环境信息
- 如果可能，附上截图或日志

### 功能建议

- 在 Issues 中使用 `Feature Request` 标签
- 详细描述您的使用场景和期望功能
- 说明为什么这个功能对项目有价值

## 📄 许可证

本项目采用 [MIT](LICENSE) 许可证 - 查看 LICENSE 文件了解详情

## 🌟 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [TailwindCSS](https://tailwindcss.com/) - 实用优先的 CSS 框架
- 所有贡献者的付出

## 📮 联系方式

- **GitHub**: [huangbh2020/glm-token-monitor](https://github.com/huangbh2020/glm-token-monitor)
- **Issues**: [提交问题](https://github.com/huangbh2020/glm-token-monitor/issues)

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐️ Star 支持一下！**

Made with ❤️ by [huangbh2020](https://github.com/huangbh2020)

</div>
