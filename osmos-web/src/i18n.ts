export type Locale = "en" | "zh";

const en = {
  documentTitle: "Osmos: mass field",
  brand: "OSMOS FIELD",
  tagline: "mass, momentum, absorption",
  bestMass: "BEST MASS",
  seed: "SEED",
  scenarioGroup: "Scenario",
  speedGroup: "Simulation speed",
  canvasLabel:
    "A simulation of motes absorbing smaller motes and pellets. Each circle is a mote, and larger circles have more mass.",
  pause: "Pause",
  resume: "Resume",
  reset: "Reset",
  languageChinese: "中文",
  languageEnglish: "EN",
  switchToChinese: "Switch to Chinese",
  switchToEnglish: "Switch to English",
  selectedMote: "Selected mote",
  closeInspector: "Close inspector",
  lineage: "LINEAGE",
  mote: "Mote",
  mass: "MASS",
  massLong: "mass",
  absorbed: "absorbed",
  pellets: "pellets",
  motes: "motes",
  ejected: "EJECTED",
  sensoryRing: "SENSORY RING · 16 SECTORS",
  massShort: "MASS",
  proximity: "PROX",
  radial: "RADIAL",
  threat: "THREAT",
  food: "FOOD",
  controlOutput: "CONTROL OUTPUT",
  thrustX: "THRUST X",
  thrustY: "THRUST Y",
  captionMass: "MASS IS THE ONLY RESOURCE. LARGER CIRCLES ABSORB SMALLER ONES. EDGES WRAP.",
  captionOrbital: "A CENTRAL ATTRACTOR BENDS EVERY ORBIT. EDGES WRAP.",
  captionRepulsor: "A CENTRAL REPULSOR PUSHES EVERY ORBIT APART. EDGES WRAP.",
  errorTitle: "Simulation failed to load",
  errorMessage: "The simulation engine failed to load.",
  retry: "Retry",
  gen: "GEN",
  tick: "TICK",
  alive: "ALIVE",
  ejecta: "EJECTA"
} as const;

export type MessageKey = keyof typeof en;

const zh: Record<MessageKey, string> = {
  documentTitle: "Osmos：质量场",
  brand: "OSMOS FIELD",
  tagline: "质量、动量、吞噬",
  bestMass: "最佳质量",
  seed: "种子",
  scenarioGroup: "场景",
  speedGroup: "模拟速度",
  canvasLabel: "质体吸收更小质体和颗粒的模拟。每个圆是一个质体，较大的圆质量更大。",
  pause: "暂停",
  resume: "继续",
  reset: "重置",
  languageChinese: "中文",
  languageEnglish: "EN",
  switchToChinese: "切换到中文",
  switchToEnglish: "切换到英文",
  selectedMote: "选中的质体",
  closeInspector: "关闭检查面板",
  lineage: "谱系",
  mote: "质体",
  mass: "质量",
  massLong: "质量",
  absorbed: "已吸收",
  pellets: "颗粒",
  motes: "质体",
  ejected: "已喷出",
  sensoryRing: "感知环 · 16 扇区",
  massShort: "质量",
  proximity: "邻近",
  radial: "径向",
  threat: "威胁",
  food: "食物",
  controlOutput: "控制输出",
  thrustX: "推力 X",
  thrustY: "推力 Y",
  captionMass: "质量是唯一资源。较大的圆吸收较小的圆。边界环绕。",
  captionOrbital: "中心吸引子弯曲每一条轨道。边界环绕。",
  captionRepulsor: "中心斥力推开每一条轨道。边界环绕。",
  errorTitle: "模拟加载失败",
  errorMessage: "模拟引擎加载失败。",
  retry: "重试",
  gen: "第",
  tick: "刻度",
  alive: "存活",
  ejecta: "喷出物"
};

export const messages: Record<Locale, Record<MessageKey, string>> = { en, zh };

export const scenarioNames: Record<Locale, readonly string[]> = {
  en: ["Ambient", "Sentient", "Orbital", "Repulsor"],
  zh: ["环境", "感知", "轨道", "斥力"]
};

export function readLocale(): Locale {
  const value = new URLSearchParams(window.location.search).get("lang");
  if (value?.toLowerCase().startsWith("zh")) return "zh";
  if (value?.toLowerCase().startsWith("en")) return "en";

  try {
    const stored = window.localStorage.getItem("osmos.locale");
    if (stored === "zh" || stored === "en") return stored;
  } catch {
    // 隐私模式或沙箱环境可能禁用存储。
  }

  return window.navigator.language?.toLowerCase().startsWith("zh") ? "zh" : "en";
}

export function persistLocale(locale: Locale) {
  try {
    window.localStorage.setItem("osmos.locale", locale);
  } catch {
    // 存储不可用时保留内存中的语言设置。
  }
}

export function localeTag(locale: Locale) {
  return locale === "zh" ? "zh-Hans" : "en";
}
