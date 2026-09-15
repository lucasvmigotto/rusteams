import { useEffect, useId, useMemo, useRef, useState } from "react";
import { useLocale } from "../i18n/useLocale";

export interface SearchEntry {
  title: string;
  keywords: string;
  to: string;
}

export function Search({ index }: { index: SearchEntry[] }) {
  const { t } = useLocale();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const inputId = useId();
  const statusId = useId();

  useEffect(() => {
    if (open) {
      inputRef.current?.focus();
    }
  }, [open]);

  const results = useMemo(() => {
    const needle = query.trim().toLowerCase();
    if (needle.length < 2) {
      return [];
    }
    return index
      .filter((entry) => `${entry.title} ${entry.keywords}`.toLowerCase().includes(needle))
      .slice(0, 8);
  }, [index, query]);

  function close() {
    setOpen(false);
    setQuery("");
  }

  return (
    <div className="relative">
      <button
        type="button"
        onClick={() => setOpen((value) => !value)}
        aria-expanded={open}
        aria-controls="site-search"
        className="rounded-md border border-slate-700 bg-slate-900 px-3 py-1.5 text-sm text-slate-300 hover:bg-slate-800"
      >
        {t("search.label")}
      </button>
      {open ? (
        <div
          id="site-search"
          className="absolute right-0 z-40 mt-2 w-72 rounded-lg border border-slate-700 bg-slate-950 p-3 shadow-xl"
        >
          <label htmlFor={inputId} className="sr-only">
            {t("search.label")}
          </label>
          <input
            ref={inputRef}
            id={inputId}
            type="search"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Escape") {
                close();
              }
            }}
            placeholder={t("search.placeholder")}
            className="w-full rounded-md border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100"
          />
          <p id={statusId} role="status" className="sr-only">
            {query.trim().length >= 2 ? `${results.length} ${t("search.results")}` : ""}
          </p>
          {query.trim().length >= 2 && results.length === 0 ? (
            <p className="mt-2 text-sm text-slate-400">{t("search.noResults")}</p>
          ) : null}
          <ul aria-describedby={statusId} className="mt-2 space-y-1">
            {results.map((entry) => (
              <li key={entry.to}>
                <a
                  href={entry.to}
                  onClick={close}
                  className="block rounded-md px-2 py-1.5 text-sm text-sky-200 hover:bg-slate-800"
                >
                  {entry.title}
                </a>
              </li>
            ))}
          </ul>
          <button
            type="button"
            onClick={close}
            className="mt-2 text-xs text-slate-400 underline hover:text-slate-200"
          >
            {t("search.close")}
          </button>
        </div>
      ) : null}
    </div>
  );
}
