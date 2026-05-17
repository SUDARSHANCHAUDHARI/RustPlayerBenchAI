"use client";
import { useState } from "react";
const OS_OPTIONS = ["SCOS (ScreenCloud OS)", "Android 11", "Android 13", "webOS 6", "Tizen 7", "Windows 10 IoT", "Chrome OS", "FireOS", "Raspbian"];
const GRADE_COLOR: Record<string, string> = { A: "text-emerald-400", B: "text-blue-400", C: "text-amber-400", D: "text-orange-400", F: "text-red-400" };
const SEV_COLORS: Record<string, string> = { critical: "text-red-400", high: "text-orange-400", medium: "text-amber-400", low: "text-zinc-400" };
interface Metric { name: string; value: string; score: number; status: string; vsBaseline: string; insight: string; }
interface Regression { metric: string; delta: string; cause: string; severity: string; }
interface Result { overallScore: number; grade: string; metrics: Metric[]; regressions: Regression[]; thermalAnalysis: string; bottleneck: string; optimizations: string[]; deviceRating: string; suitableFor: string[]; notSuitableFor: string[]; summary: string; }
const METRIC_BAR: Record<string, string> = { good: "bg-emerald-500", warning: "bg-amber-500", critical: "bg-red-500" };

function NumInput({ label, value, onChange, unit, min = 0, max = 100, step = 1 }: { label: string; value: number; onChange: (v: number) => void; unit?: string; min?: number; max?: number; step?: number }) {
  return (
    <div>
      <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2 block">{label}{unit && <span className="text-zinc-600 ml-1">({unit})</span>}</label>
      <input type="number" min={min} max={max} step={step} value={value} onChange={e => onChange(Number(e.target.value))} className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-3 py-2.5 text-white text-sm focus:outline-none focus:border-orange-500" />
    </div>
  );
}

export default function PlayerBenchAI() {
  const [playerModel, setPlayerModel] = useState("");
  const [os, setOs] = useState(OS_OPTIONS[0]);
  const [fps, setFps] = useState(60);
  const [memory, setMemory] = useState(65);
  const [startupTime, setStartupTime] = useState(8);
  const [appLoadTime, setAppLoadTime] = useState(3);
  const [videoSmoothness, setVideoSmoothness] = useState(8);
  const [thermalThrottling, setThermalThrottling] = useState(false);
  const [previousBenchmark, setPreviousBenchmark] = useState("");
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<Result | null>(null);
  const [error, setError] = useState("");

  async function run() {
    if (!playerModel.trim()) return;
    setLoading(true); setResult(null); setError("");
    try {
      const res = await fetch("/api/benchmark", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ playerModel, os, fps, memory, startupTime, appLoadTime, videoSmoothness, thermalThrottling, previousBenchmark }) });
      const data = await res.json();
      if (data.error) setError(data.error); else setResult(data);
    } catch { setError("Network error"); }
    setLoading(false);
  }

  return (
    <div className="min-h-screen bg-[#0a0a0f] text-white font-sans">
      <div className="max-w-5xl mx-auto px-6 py-10">
        <div className="mb-8">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-9 h-9 rounded-lg bg-orange-500/20 border border-orange-500/30 flex items-center justify-center text-lg">⚡</div>
            <h1 className="text-2xl font-bold">PlayerBenchAI</h1>
          </div>
          <p className="text-zinc-400 text-sm">Benchmark any signage player — FPS, memory, startup, thermal throttling. AI explains performance regressions.</p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
          <div className="bg-zinc-900/60 border border-zinc-800 rounded-xl p-6">
            <h2 className="text-sm font-semibold text-white mb-4">Device</h2>
            <div className="flex flex-col gap-4">
              <div>
                <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2 block">Player model *</label>
                <input value={playerModel} onChange={e => setPlayerModel(e.target.value)} placeholder="e.g. Raspberry Pi 4B, BrightSign XD235" className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-3 text-white placeholder-zinc-500 text-sm focus:outline-none focus:border-orange-500" />
              </div>
              <div>
                <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2 block">Operating system</label>
                <select value={os} onChange={e => setOs(e.target.value)} className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-3 py-2.5 text-white text-sm focus:outline-none focus:border-orange-500">
                  {OS_OPTIONS.map(o => <option key={o}>{o}</option>)}
                </select>
              </div>
              <label className="flex items-center gap-3 cursor-pointer">
                <div onClick={() => setThermalThrottling(v => !v)} className={`w-11 h-6 rounded-full transition-colors border ${thermalThrottling ? "bg-red-600 border-red-500" : "bg-zinc-700 border-zinc-600"} relative`}>
                  <div className={`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white transition-transform ${thermalThrottling ? "translate-x-5" : ""}`} />
                </div>
                <div><div className="text-sm text-zinc-300">Thermal throttling detected</div><div className="text-xs text-zinc-500">Device running hot</div></div>
              </label>
              <div>
                <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2 block">Previous benchmark (optional)</label>
                <input value={previousBenchmark} onChange={e => setPreviousBenchmark(e.target.value)} placeholder="e.g. FPS: 60, Memory: 55%, Startup: 6s" className="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-3 py-2.5 text-white placeholder-zinc-500 text-sm focus:outline-none focus:border-orange-500" />
              </div>
            </div>
          </div>

          <div className="bg-zinc-900/60 border border-zinc-800 rounded-xl p-6">
            <h2 className="text-sm font-semibold text-white mb-4">Metrics</h2>
            <div className="grid grid-cols-2 gap-4">
              <NumInput label="FPS" value={fps} onChange={setFps} min={1} max={120} />
              <NumInput label="Memory used" value={memory} onChange={setMemory} unit="%" />
              <NumInput label="Startup time" value={startupTime} onChange={setStartupTime} unit="s" min={1} max={300} step={0.5} />
              <NumInput label="App load time" value={appLoadTime} onChange={setAppLoadTime} unit="s" min={0.1} max={60} step={0.1} />
              <div className="col-span-2">
                <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider mb-2 block">Video smoothness <span className="text-zinc-600">(1-10)</span></label>
                <input type="range" min="1" max="10" value={videoSmoothness} onChange={e => setVideoSmoothness(Number(e.target.value))} className="w-full accent-orange-500" />
                <div className="flex justify-between text-xs text-zinc-500 mt-1"><span>Choppy</span><span className="text-orange-400 font-bold">{videoSmoothness}/10</span><span>Smooth</span></div>
              </div>
            </div>
            <button onClick={run} disabled={loading || !playerModel.trim()} className="mt-4 w-full py-3 bg-orange-600 hover:bg-orange-500 disabled:opacity-40 disabled:cursor-not-allowed text-white font-semibold rounded-lg text-sm transition-colors">
              {loading ? "Benchmarking…" : "Run Benchmark"}
            </button>
          </div>
        </div>

        {error && <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-4 text-red-400 text-sm mb-6">{error}</div>}
        {loading && <div className="flex items-center justify-center py-16"><div className="flex flex-col items-center gap-3"><div className="w-10 h-10 border-2 border-orange-500 border-t-transparent rounded-full animate-spin" /><p className="text-zinc-400 text-sm">Analyzing performance…</p></div></div>}

        {result && (
          <div className="flex flex-col gap-5">
            <div className="bg-zinc-900/60 border border-zinc-800 rounded-xl p-5 flex items-center gap-5">
              <div className="text-6xl font-black text-center w-24">
                <span className={GRADE_COLOR[result.grade]}>{result.grade}</span>
                <div className="text-lg text-zinc-400">{result.overallScore}/100</div>
              </div>
              <div className="flex-1"><p className="text-zinc-200 font-medium mb-1">{result.deviceRating}</p><p className="text-zinc-400 text-sm">{result.summary}</p><p className="text-xs text-orange-400 mt-1">Bottleneck: {result.bottleneck}</p></div>
            </div>

            <div className="bg-zinc-900/60 border border-zinc-800 rounded-xl p-5">
              <h2 className="text-sm font-semibold text-white mb-4">Metric Breakdown</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {result.metrics.map((m, i) => (
                  <div key={i}>
                    <div className="flex justify-between items-center mb-1">
                      <span className="text-sm text-zinc-300">{m.name}</span>
                      <div className="flex items-center gap-2">
                        <span className="text-xs text-zinc-500">{m.vsBaseline}</span>
                        <span className={`text-sm font-bold ${m.status === "good" ? "text-emerald-400" : m.status === "warning" ? "text-amber-400" : "text-red-400"}`}>{m.value}</span>
                      </div>
                    </div>
                    <div className="h-2 bg-zinc-800 rounded-full overflow-hidden mb-1"><div className={`h-full rounded-full ${METRIC_BAR[m.status]}`} style={{ width: `${m.score}%` }} /></div>
                    <p className="text-xs text-zinc-500">{m.insight}</p>
                  </div>
                ))}
              </div>
            </div>

            {result.regressions.length > 0 && (
              <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-5">
                <h2 className="text-sm font-semibold text-red-400 mb-4">Regressions Detected</h2>
                <div className="flex flex-col gap-3">{result.regressions.map((r, i) => <div key={i} className="bg-zinc-900/60 rounded-lg p-3"><div className="flex justify-between mb-1"><span className="text-sm font-semibold text-zinc-200">{r.metric}</span><span className={`text-xs font-bold ${SEV_COLORS[r.severity]}`}>{r.delta}</span></div><p className="text-xs text-zinc-400">{r.cause}</p></div>)}</div>
              </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div className="bg-emerald-500/10 border border-emerald-500/30 rounded-xl p-4"><h3 className="text-xs font-semibold text-emerald-400 uppercase tracking-wider mb-3">Suitable for</h3><ul className="flex flex-col gap-1.5">{result.suitableFor.map((s, i) => <li key={i} className="text-sm text-zinc-300 flex gap-2"><span className="text-emerald-400">✓</span>{s}</li>)}</ul></div>
              <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-4"><h3 className="text-xs font-semibold text-red-400 uppercase tracking-wider mb-3">Not suitable for</h3><ul className="flex flex-col gap-1.5">{result.notSuitableFor.map((s, i) => <li key={i} className="text-sm text-zinc-300 flex gap-2"><span className="text-red-400">✗</span>{s}</li>)}</ul></div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
