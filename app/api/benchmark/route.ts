import Anthropic from "@anthropic-ai/sdk";
import { NextRequest, NextResponse } from "next/server";
const client = new Anthropic({ apiKey: process.env.ANTHROPIC_API_KEY });
export async function POST(req: NextRequest) {
  const { playerModel, os, fps, memory, startupTime, appLoadTime, videoSmoothness, thermalThrottling, previousBenchmark } = await req.json();
  if (!playerModel?.trim()) return NextResponse.json({ error: "Player model required" }, { status: 400 });
  const prompt = `You are a digital signage hardware performance expert. Analyze benchmark results and explain performance regressions.
Player: ${playerModel} | OS: ${os || "Unknown"}
FPS: ${fps} | Memory usage: ${memory}% | Startup time: ${startupTime}s | App load time: ${appLoadTime}s | Video smoothness: ${videoSmoothness}/10 | Thermal throttling: ${thermalThrottling ? "Yes" : "No"}
Previous benchmark: ${previousBenchmark || "None"}
Return JSON: { "overallScore": 0-100, "grade": "A"|"B"|"C"|"D"|"F", "metrics": [{ "name": "string", "value": "string", "score": 0-100, "status": "good"|"warning"|"critical", "vsBaseline": "string", "insight": "string" }], "regressions": [{ "metric": "string", "delta": "string", "cause": "string", "severity": "critical"|"high"|"medium"|"low" }], "thermalAnalysis": "string", "bottleneck": "string", "optimizations": ["string"], "deviceRating": "string", "suitableFor": ["use case"], "notSuitableFor": ["use case"], "summary": "string" }
Return ONLY valid JSON.`;
  try {
    const msg = await client.messages.create({ model: "claude-sonnet-4-6", max_tokens: 1200, messages: [{ role: "user", content: prompt }] });
    return NextResponse.json(JSON.parse((msg.content[0] as { type: string; text: string }).text));
  } catch (e) { return NextResponse.json({ error: String(e) }, { status: 500 }); }
}
