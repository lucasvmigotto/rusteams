import type { ReactNode } from "react";
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";
import { enDocs } from "./content/en";
import { ptBrDocs } from "./content/pt-BR";
import type { DocsContent } from "./content/types";
import type { Locale, UIKey } from "./locales";
import { detectLocale, STRINGS } from "./locales";

const DOCS: Record<Locale, DocsContent> = { en: enDocs, "pt-BR": ptBrDocs };

interface LocaleContextValue {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: (key: UIKey) => string;
  docs: DocsContent;
}

const LocaleContext = createContext<LocaleContextValue | null>(null);

export function LocaleProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>("en");

  useEffect(() => {
    setLocaleState(detectLocale());
  }, []);

  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);

  const setLocale = useCallback((next: Locale) => {
    localStorage.setItem("rusteams-locale", next);
    setLocaleState(next);
  }, []);

  const t = useCallback((key: UIKey): string => STRINGS[locale][key] ?? STRINGS.en[key], [locale]);

  const docs = useMemo(() => DOCS[locale], [locale]);

  return (
    <LocaleContext.Provider value={{ locale, setLocale, t, docs }}>
      {children}
    </LocaleContext.Provider>
  );
}

export function useLocale(): LocaleContextValue {
  const value = useContext(LocaleContext);
  if (!value) {
    throw new Error("useLocale must be used inside <LocaleProvider>");
  }
  return value;
}
