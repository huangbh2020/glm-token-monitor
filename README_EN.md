# GLM-Token-Monitor

<div align="center">

**🔍 A Cute Cross-Platform Desktop API Quota Monitoring Tool**

[![GitHub stars](https://img.shields.io/github/stars/huangbh2020/glm-token-monitor?style=social)](https://github.com/huangbh2020/glm-token-monitor/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/huangbh2020/glm-token-monitor?style=social)](https://github.com/huangbh2020/glm-token-monitor/network/members)
[![GitHub issues](https://img.shields.io/github/issues/huangbh2020/glm-token-monitor)](https://github.com/huangbh2020/glm-token-monitor/issues)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

English | [简体中文](./README.md)

</div>

---

## ✨ Introduction

**GLM-Token-Monitor** is a cross-platform desktop API quota monitoring application that displays your API usage in real-time through cute digital pets and various cool display modes.

### 🎯 Key Features

- **🐾 Cute Pets** - Multiple digital pet characters with different animations based on usage
- **📊 Real-time Monitoring** - Automatic API quota polling, supports multiple providers (Zhipu, Z.AI, etc.)
- **🎨 Multiple Display Modes** - 6 display effects including holographic bubble, cyber ring, aura field, energy core, etc.
- **⚡ Lightweight & Efficient** - Built with Tauri 2.0 + Vue 3, minimal resource usage
- **🪟 Cross-platform** - Windows, macOS, Linux
- **🎨 Highly Customizable** - Custom pets, display modes, glow effects, and more

## 🎬 Feature Showcase

### Pet Status System

The app automatically switches pet states based on API usage, each with unique animations:

| State | Usage | Color | Animation |
|:----:|:------:|:----:|:---------|
| 🟢 Fresh | 0-30% | Green | Breathing, relaxed |
| 🔵 Flow | 31-60% | Blue | Typing, working hard |
| 🟡 Warning | 61-80% | Yellow | Shaking, watch usage |
| 🟠 Panic | 81-95% | Orange | Sweating, almost depleted |
| ⚫ Dead | 96-100% | Gray | Soul out of body, depleted |

### Display Modes

6 cool Token remaining display modes:

- **None** - Minimalist mode, no numerical display
- **Holo Bubble** - Holographic bubble, sci-fi feel
- **Cyber Ring** - Cyber ring, classic progress design
- **Aura Field** - Aura diffusion, gentle visual effect
- **Energy Core** - Energy core, pixel style grid
- **Status Floater** - Status floating bar, side display

### Dual Metrics Monitoring

- **🗓 Monthly MCP Quota** - Monitor monthly API call limits
- **⏱ 5h Token Quota** - Monitor Token usage within 5-hour window (drives pet status)

## 🛠 Tech Stack

### Frontend
- **Vue 3.4** - Progressive JavaScript Framework
- **TypeScript 5.3** - Type safety
- **TailwindCSS 3.4** - Utility-first CSS framework
- **Vite 5.0** - Fast frontend build tool

### Backend
- **Rust** - System-level performance
- **Tauri 2.0** - Cross-platform desktop app framework
- **Tokio** - Async runtime
- **reqwest** - HTTP client

### Key Dependencies
- `tauri-plugin-shell` - System shell operations
- `serde/serde_json` - Serialization/deserialization
- `chrono` - Time handling
- `dirs` - System directory access

## 📦 Installation & Usage

### Prerequisites

- **Node.js** 18+
- **Rust** 1.70+
- **npm** or **yarn**

### Build from Source

```bash
# 1. Clone the repository
git clone https://github.com/huangbh2020/glm-token-monitor.git
cd glm-token-monitor

# 2. Install dependencies
npm install

# 3. Start development mode (hot reload)
npm run tauri:dev

# 4. Build production version
npm run tauri:build
```

After building, installers are located in `src-tauri/target/release/bundle/`.

### Configure API

After first launch, configure your API information:

1. Right-click the system tray icon and select "Settings"
2. Fill in your API Key
3. Select provider (Zhipu BigModel or Z.AI)
4. Click "Test Connection" to verify
5. Save settings

### Windows Users

- Download `.exe` installer and double-click to install
- App launches automatically and appears on desktop
- Click the pet window to view details
- Drag the window to reposition

### macOS Users

- Download `.dmg` file and drag to Applications
- Find the app icon in system tray
- Click the tray icon to view usage information

## 📁 Project Structure

```
glm-token-monitor/
├── src/                          # Vue frontend code
│   ├── components/               # Vue components
│   │   ├── PetWidget.vue        # Windows desktop pet component
│   │   ├── PopoverPanel.vue     # macOS popover panel component
│   │   └── pets/                # Pet component directory
│   ├── composables/             # Vue composables
│   ├── App.vue                  # Main app component
│   ├── main.ts                  # App entry point
│   └── style.css                # Global styles
├── src-tauri/                   # Rust backend code
│   ├── src/
│   │   ├── main.rs              # App entry point
│   │   ├── events.rs            # Event definitions
│   │   ├── polling.rs           # Polling logic
│   │   ├── config.rs            # Configuration management
│   │   └── ...
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── design/                      # Design resources
├── CLAUDE.md                    # Claude Code development guide
├── README.md                    # Project documentation
└── package.json                 # Node.js dependencies
```

## ⚙️ Configuration

### Polling Interval

Default refresh interval is **1 minute**, adjustable in settings.

### Config File Location

- **Windows**: `%APPDATA%\glm-token-monitor\config.json`
- **macOS**: `~/Library/Application Support/glm-token-monitor/config.json`
- **Linux**: `~/.config/glm-token-monitor/config.json`

### Configurable Options

- **API Key**: Your API access key
- **Provider**: Zhipu BigModel / Z.AI
- **Polling Interval**: 1-60 minutes
- **Display Mode**: 6 display effects
- **Pet Type**: Multiple cute pets
- **Glow Effect**: Toggle status glow
- **Auto-start**: Launch on system startup

## 🚀 Development Guide

### Environment Setup

```bash
# Clone repository
git clone https://github.com/huangbh2020/glm-token-monitor.git
cd glm-token-monitor

# Install dependencies
npm install

# Start development mode
npm run tauri:dev
```

### Adding New API Provider

1. Edit `src-tauri/src/config.rs`:

```rust
pub fn api_domain(&self) -> &'static str {
    match self.provider.as_str() {
        "bigmodel" => "https://open.bigmodel.cn/",
        "zai" => "https://api.z.ai/",
        "your-provider" => "https://your-api.com/",
        _ => "https://open.bigmodel.cn/",
    }
}
```

### Adding New Pet Type

1. Create new pet component in `src/components/pets/`
2. Register component in `src/components/PetWidget.vue`
3. Add action logic in `src/composables/usePetAction.ts`
4. Update config defaults

### Code Standards

- **Rust**: Follow `rustfmt` formatting
- **TypeScript/Vue**: Follow ESLint configuration
- **Commit**: Use [Conventional Commits](https://www.conventionalcommits.org/) specification

```
feat: new feature
fix: bug fix
docs: documentation update
style: code formatting
refactor: refactoring
test: testing related
chore: build/toolchain update
```

## 🧪 Testing

### Rust Tests

```bash
cd src-tauri
cargo test
```

### Frontend Build Test

```bash
npm run build
```

## ❓ FAQ

### Build Errors

**Problem**: Build fails with dependency errors

**Solution**:
```bash
# Update Rust toolchain
rustup update

# Clean and reinstall dependencies
rm -rf node_modules
npm install

# Clean Rust build cache
cd src-tauri
cargo clean
```

### Tauri Dev Server Won't Start

**Problem**: Port occupied or startup failed

**Solution**:
```bash
# Check if port 1420 is occupied
# Windows: netstat -ano | findstr :1420
# macOS/Linux: lsof -i :1420

# Clear cache
npm run clean

# Restart
npm run tauri:dev
```

## 🤝 Contributing

We welcome all forms of contributions!

### How to Contribute

1. **Fork** this repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'feat: add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a **Pull Request**

### Report Issues

- Use [GitHub Issues](https://github.com/huangbh2020/glm-token-monitor/issues) to report bugs
- Provide detailed reproduction steps and environment info
- Attach screenshots or logs if possible

## 📄 License

This project is licensed under the [MIT](LICENSE) License - see the LICENSE file for details

## 🌟 Acknowledgments

- [Tauri](https://tauri.app/) - Cross-platform desktop app framework
- [Vue.js](https://vuejs.org/) - Progressive JavaScript framework
- [TailwindCSS](https://tailwindcss.com/) - Utility-first CSS framework
- All contributors

## 📮 Contact

- **GitHub**: [huangbh2020/glm-token-monitor](https://github.com/huangbh2020/glm-token-monitor)
- **Issues**: [Submit Issue](https://github.com/huangbh2020/glm-token-monitor/issues)

---

<div align="center">

**If this project helps you, please give it a ⭐️ Star!**

Made with ❤️ by [huangbh2020](https://github.com/huangbh2020)

</div>
