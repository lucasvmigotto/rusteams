import shotsData from "@/content/tui-shots.json";

interface Run {
  t: string;
  fg: string;
  bg: string;
}

interface Shot {
  width: number;
  height: number;
  rows: Run[][];
}

const NAMED: Record<string, string> = {
  black: "#000000",
  red: "#ef4444",
  green: "#22c55e",
  yellow: "#eab308",
  blue: "#3b82f6",
  magenta: "#d946ef",
  cyan: "#06b6d6",
  gray: "#9ca3af",
  darkgray: "#4b5563",
  lightred: "#f87171",
  lightgreen: "#4ade80",
  lightyellow: "#facc15",
  lightblue: "#60a5fa",
  lightmagenta: "#f0abfc",
  lightcyan: "#67e8f9",
  white: "#ffffff",
};

const INDEXED = [
  "#000000",
  "#ef4444",
  "#22c55e",
  "#eab308",
  "#3b82f6",
  "#d946ef",
  "#06b6d6",
  "#9ca3af",
  "#4b5563",
  "#f87171",
  "#4ade80",
  "#facc15",
  "#60a5fa",
  "#f0abfc",
  "#67e8f9",
  "#ffffff",
];

function css(color: string): string | undefined {
  if (color === "reset") {
    return undefined;
  }
  const rgb = /^rgb\((\d+),(\d+),(\d+)\)$/.exec(color);
  if (rgb) {
    return `rgb(${rgb[1]},${rgb[2]},${rgb[3]})`;
  }
  const indexed = /^indexed\((\d+)\)$/.exec(color);
  if (indexed) {
    return INDEXED[Number(indexed[1])] ?? "#9ca3af";
  }
  return NAMED[color];
}

/// Static print of a real TUI render, generated from TestBackend cells by
/// `cargo run --example tui_snapshot` — never hand-drawn. Exposed as an
/// image with a text transcript so screen readers get the same content.
export function TuiShot({
  id,
  alt,
  caption,
  transcript,
}: {
  id: string;
  alt: string;
  caption: string;
  transcript: string;
}) {
  const shots = (shotsData as { shots: Record<string, Shot> }).shots;
  const shot = shots[id];
  if (!shot) {
    return null;
  }
  return (
    <figure className="space-y-2">
      <div className="overflow-hidden rounded-lg border border-slate-800 bg-black/80">
        <div className="border-b border-slate-800 px-4 py-2 font-mono text-xs text-slate-400">
          ▚ rusteams
        </div>
        <pre
          role="img"
          aria-label={alt}
          className="overflow-x-auto p-4 font-mono text-[13px] leading-snug text-slate-200"
        >
          {shot.rows.map((row, y) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: fixed render order, never reordered
            <span key={y} className="block whitespace-pre">
              {row.map((run, i) => (
                <span
                  // biome-ignore lint/suspicious/noArrayIndexKey: fixed render order, never reordered
                  key={i}
                  style={{ color: css(run.fg), backgroundColor: css(run.bg) }}
                >
                  {run.t}
                </span>
              ))}
            </span>
          ))}
        </pre>
      </div>
      <p className="sr-only">{transcript}</p>
      <figcaption className="text-xs text-slate-400">{caption}</figcaption>
    </figure>
  );
}
