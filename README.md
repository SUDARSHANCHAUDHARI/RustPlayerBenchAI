# PlayerBenchAI ⚡📊

**Benchmark any digital signage player and get AI-explained performance analysis — FPS, memory, startup time, thermal throttling, and regression detection.**

Enter your player model, OS, and benchmark metrics (FPS, memory %, startup time, app load time, video smoothness, thermal throttling), optionally provide a previous benchmark for comparison, and get a full performance report: overall score, grade (A–F), per-metric analysis, regression causes, bottleneck identification, optimization steps, and suitability ratings.

---

## 🌟 Features

### ⚡ Core Features
- ✅ **Player Model Input** — any signage player (e.g. Raspberry Pi 4B, BrightSign XD235)
- ✅ **OS Selector** — SCOS, Android 11/13, webOS 6, Tizen 7, Windows 10 IoT, Chrome OS, FireOS, Raspbian
- ✅ **FPS Input** — frames per second (numeric)
- ✅ **Memory Usage** — % used (0–100)
- ✅ **Startup Time** — seconds to boot
- ✅ **App Load Time** — seconds to load signage app
- ✅ **Video Smoothness** — 1–10 subjective rating
- ✅ **Thermal Throttling Toggle** — on/off switch with visual indicator
- ✅ **Previous Benchmark** — optional free-text for regression comparison
- ✅ **Overall Score** — 0–100 composite performance score
- ✅ **Grade** — A / B / C / D / F
- ✅ **Per-Metric Cards** — value, score bar, vs-baseline delta, insight per metric
- ✅ **Regression List** — metric, delta, root cause, severity
- ✅ **Thermal Analysis** — text explanation of thermal behavior
- ✅ **Bottleneck** — single identified primary bottleneck
- ✅ **Optimizations** — ordered list of actionable fixes
- ✅ **Suitability** — suitable-for and not-suitable-for use case lists

### 🤖 AI Features
- ✅ **Claude Sonnet 4.6** — interprets benchmark data with signage hardware expertise
- ✅ **OS-aware analysis** — webOS 6 baseline differs from Raspbian baseline
- ✅ **Regression detection** — compares current vs previous benchmark and names the cause
- ✅ **Thermal reasoning** — explains throttling impact on FPS and memory behavior
- ✅ **Use-case matching** — rates suitability for 4K video, web apps, live data, etc.

### ⚙️ Technical Features
- ✅ **Next.js 15 App Router** — server + client components
- ✅ **TypeScript strict mode** — fully typed metric and regression interfaces
- ✅ **Tailwind CSS** — dark orange theme with score bars and grade coloring

---

## 🏗️ Architecture

```
PlayerBenchAI/
├── 📁 app/
│   ├── 📄 page.tsx           # Main UI — benchmark form + performance report
│   ├── 📄 layout.tsx         # Root layout with dark background
│   ├── 📄 globals.css        # Global styles
│   └── 📁 api/
│       └── 📁 benchmark/
│           └── 📄 route.ts   # POST /api/benchmark — Claude performance analyzer
├── 📁 public/                # Static assets
├── 📄 .env.example           # Environment variable template
├── 📄 package.json
└── 📄 README.md
```

---

## 🖥️ UI Overview

| Section | Description |
|---|---|
| **Device Panel** | Player model, OS dropdown, thermal throttling toggle, previous benchmark |
| **Metrics Panel** | FPS, memory %, startup time, app load time, video smoothness inputs |
| **Run Benchmark** | Triggers Claude performance analysis |
| **Score + Grade** | Large numeric score and letter grade (A–F) |
| **Metric Cards** | Per-metric: value, colored score bar, vs-baseline, insight |
| **Regressions** | Metric, delta, cause, severity badge |
| **Thermal Analysis** | Text paragraph on thermal behavior |
| **Bottleneck** | Single primary bottleneck identified |
| **Optimizations** | Ordered fix list |
| **Suitability** | Suitable-for and not-suitable-for use case chips |

---

## 🚀 Getting Started

### Prerequisites
- Node.js 18+
- pnpm
- Anthropic API key ([console.anthropic.com](https://console.anthropic.com))

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/SUDARSHANCHAUDHARI/PlayerBenchAI.git
   cd PlayerBenchAI
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Set up environment**
   ```bash
   cp .env.example .env.local
   # Edit .env.local and add your ANTHROPIC_API_KEY
   ```

4. **Run dev server**
   ```bash
   pnpm dev
   ```
   Open [http://localhost:3000](http://localhost:3000)

---

## 📜 Scripts

```bash
pnpm dev      # Start development server (Turbopack)
pnpm build    # Production build
pnpm start    # Start production server
pnpm lint     # ESLint check
```

---

## 🔑 Environment Variables

| Variable | Description | Required |
|---|---|---|
| `ANTHROPIC_API_KEY` | Your Anthropic API key | ✅ Yes |

Get your key at [console.anthropic.com](https://console.anthropic.com). Add it to `.env.local` — this file is gitignored and never committed.

---

## 📊 Current Status

| Property | Value |
|---|---|
| **Version** | 1.0.0 |
| **Status** | ✅ MVP Complete |
| **Model** | claude-sonnet-4-6 |
| **OS Options** | 9 (SCOS, Android 11/13, webOS 6, Tizen 7, Windows 10 IoT, Chrome OS, FireOS, Raspbian) |
| **Grades** | 5 (A, B, C, D, F) |
| **Metrics** | 6 (FPS, memory, startup, app load, video smoothness, thermal throttling) |

---

## 🛠️ Tech Stack

| Component | Technology |
|---|---|
| **Framework** | Next.js 15 (App Router) |
| **Language** | TypeScript (strict mode) |
| **Styling** | Tailwind CSS |
| **AI** | Claude API — claude-sonnet-4-6 |
| **Package Manager** | pnpm |

---

## 🔒 Security

- `ANTHROPIC_API_KEY` lives in `.env.local` — gitignored, never committed
- `.env.example` contains placeholder values only
- API key sent directly to Anthropic — no intermediate server
- Benchmark data not stored or logged server-side

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for details.

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/your-feature`)
3. Commit your changes (`git commit -m 'feat: add your feature'`)
4. Push to the branch (`git push origin feat/your-feature`)
5. Open a Pull Request

---

## 📞 Support

- 🐛 Issues: [GitHub Issues](https://github.com/SUDARSHANCHAUDHARI/PlayerBenchAI/issues)

---

<div align="center">

**Made with ❤️ by [SUDARSHANCHAUDHARI](https://github.com/SUDARSHANCHAUDHARI)**

[⭐ Star this repo](https://github.com/SUDARSHANCHAUDHARI/PlayerBenchAI) · [🐛 Report Issue](https://github.com/SUDARSHANCHAUDHARI/PlayerBenchAI/issues)

</div>
