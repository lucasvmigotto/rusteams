import { clsx } from "clsx";
import type { ReactNode } from "react";
import { useId, useState } from "react";
import { twMerge } from "tailwind-merge";
import { useLocale } from "../i18n/useLocale";

export function cx(...inputs: (string | false | null | undefined)[]) {
  return twMerge(clsx(...inputs));
}

export function SkipLink() {
  const { t } = useLocale();
  return (
    <a
      href="#main-content"
      className="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50 focus:rounded-md focus:bg-sky-600 focus:px-4 focus:py-2 focus:text-white"
    >
      {t("skip.link")}
    </a>
  );
}

export function Section({
  id,
  title,
  children,
}: {
  id: string;
  title: string;
  children: ReactNode;
}) {
  return (
    <section id={id} aria-labelledby={`${id}-heading`} className="scroll-mt-20 space-y-3">
      <h2 id={`${id}-heading`} className="text-xl font-semibold text-sky-300">
        {title}
      </h2>
      <div className="space-y-3 text-sm leading-relaxed text-slate-300">{children}</div>
    </section>
  );
}

export function Card({
  title,
  children,
  className,
}: {
  title?: string;
  children: ReactNode;
  className?: string;
}) {
  return (
    <div className={cx("rounded-xl border border-slate-800 bg-slate-900/60 p-4", className)}>
      {title ? <h3 className="mb-2 text-sm font-semibold text-slate-100">{title}</h3> : null}
      <div className="text-sm text-slate-300">{children}</div>
    </div>
  );
}

export function FeatureCard({
  title,
  status,
  children,
}: {
  title: string;
  status?: ReactNode;
  children: ReactNode;
}) {
  return (
    <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-4">
      <div className="mb-2 flex flex-wrap items-center gap-2">
        <h3 className="text-sm font-semibold text-slate-100">{title}</h3>
        {status}
      </div>
      <div className="text-sm text-slate-300">{children}</div>
    </div>
  );
}

export type Maturity = "implemented" | "partial" | "planned" | "graphLimited" | "unavailable";

const STATUS_STYLES: Record<Maturity, string> = {
  implemented: "border-emerald-500/40 bg-emerald-950/40 text-emerald-200",
  partial: "border-amber-400/40 bg-amber-950/40 text-amber-100",
  planned: "border-slate-600 bg-slate-800/60 text-slate-300",
  graphLimited: "border-purple-400/40 bg-purple-950/40 text-purple-100",
  unavailable: "border-red-400/40 bg-red-950/40 text-red-100",
};

export function StatusBadge({ status, label }: { status: Maturity; label: string }) {
  return (
    <span
      className={cx(
        "inline-flex items-center gap-1.5 rounded-full border px-2.5 py-0.5 text-xs font-medium",
        STATUS_STYLES[status],
      )}
    >
      <span aria-hidden="true">
        {status === "implemented" ? "●" : status === "planned" ? "○" : "◐"}
      </span>
      {label}
    </span>
  );
}

export function Callout({
  kind = "note",
  title,
  children,
}: {
  kind?: "note" | "warn";
  title?: string;
  children: ReactNode;
}) {
  // Warnings interrupt: role="alert". Plain notes are static text and
  // deliberately carry no landmark role.
  if (kind === "warn") {
    return (
      <div
        role="alert"
        className="rounded-lg border-l-4 border-amber-400 bg-amber-950/40 p-3 text-sm text-amber-100"
      >
        {title ? <p className="mb-1 font-semibold">{title}</p> : null}
        {children}
      </div>
    );
  }
  return (
    <div className="rounded-lg border-l-4 border-sky-400 bg-sky-950/40 p-3 text-sm text-sky-100">
      {title ? <p className="mb-1 font-semibold">{title}</p> : null}
      {children}
    </div>
  );
}

export function CodeCopyButton({ code }: { code: string }) {
  const { t } = useLocale();
  const [copied, setCopied] = useState(false);

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 2000);
    } catch {
      setCopied(false);
    }
  }

  return (
    <button
      type="button"
      onClick={handleCopy}
      aria-live="polite"
      className="rounded-md border border-slate-700 bg-slate-800 px-2 py-1 text-xs text-slate-200 hover:bg-slate-700 focus-visible:outline-2 focus-visible:outline-sky-400"
    >
      {copied ? t("copy.copied") : t("copy.copy")}
    </button>
  );
}

export function CodeBlock({ code, lang = "bash" }: { code: string; lang?: string }) {
  return (
    <div className="overflow-hidden rounded-lg border border-slate-800 bg-black/60">
      <div className="flex items-center justify-between border-b border-slate-800 px-4 py-2">
        <span className="font-mono text-xs text-slate-400">{lang}</span>
        <CodeCopyButton code={code} />
      </div>
      <pre className="overflow-x-auto p-4 text-[13px] leading-relaxed">
        <code data-lang={lang} className="text-slate-200">
          {code}
        </code>
      </pre>
    </div>
  );
}

export function Table({
  caption,
  headers,
  rows,
}: {
  caption: string;
  headers: string[];
  rows: string[][];
}) {
  return (
    <div className="overflow-x-auto rounded-lg border border-slate-800">
      <table className="w-full text-left text-sm">
        <caption className="sr-only">{caption}</caption>
        <thead>
          <tr className="bg-slate-900 text-xs uppercase tracking-wide text-slate-400">
            {headers.map((h) => (
              <th key={h} scope="col" className="px-4 py-2 font-medium">
                {h}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => (
            <tr
              key={row.join("|")}
              className="border-t border-slate-800 odd:bg-slate-950/60 even:bg-slate-900/40"
            >
              {row.map((cell) => (
                <td key={cell} className="px-4 py-2 align-top text-slate-300">
                  {cell}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function Accordion({ title, children }: { title: string; children: ReactNode }) {
  const [open, setOpen] = useState(false);
  const panelId = useId();
  const buttonId = useId();

  return (
    <div className="rounded-lg border border-slate-800">
      <h3>
        <button
          type="button"
          id={buttonId}
          aria-expanded={open}
          aria-controls={panelId}
          onClick={() => setOpen((value) => !value)}
          className="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium text-slate-100 hover:bg-slate-900/60"
        >
          {title}
          <span aria-hidden="true" className="ml-2 text-sky-300">
            {open ? "−" : "+"}
          </span>
        </button>
      </h3>
      <section
        id={panelId}
        aria-labelledby={buttonId}
        hidden={!open}
        className="px-4 pb-3 text-sm text-slate-300"
      >
        {children}
      </section>
    </div>
  );
}

export function Breadcrumbs({ trail }: { trail: { label: string; to?: string }[] }) {
  const { t } = useLocale();
  return (
    <nav aria-label="Breadcrumb">
      <ol className="flex flex-wrap items-center gap-2 text-xs text-slate-400">
        <li>
          <a href="#/" className="underline hover:text-slate-200">
            {t("breadcrumb.home")}
          </a>
        </li>
        {trail.map((item) => (
          <li key={item.label} className="flex items-center gap-2">
            <span aria-hidden="true">/</span>
            {item.to ? (
              <a href={item.to} className="underline hover:text-slate-200">
                {item.label}
              </a>
            ) : (
              <span aria-current="page" className="text-slate-200">
                {item.label}
              </span>
            )}
          </li>
        ))}
      </ol>
    </nav>
  );
}

export function VersionBadge({ version }: { version: string }) {
  return (
    <span className="rounded bg-slate-800 px-1.5 py-0.5 font-mono text-xs text-sky-200">
      v{version}
    </span>
  );
}
