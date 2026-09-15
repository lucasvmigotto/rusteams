import { useEffect, useRef, useState } from "react";
import { NavLink, Outlet } from "react-router-dom";
import type { Locale } from "../i18n/locales";
import { LOCALES } from "../i18n/locales";
import { useLocale } from "../i18n/useLocale";
import type { SearchEntry } from "./Search";
import { Search } from "./Search";
import { cx, SkipLink, VersionBadge } from "./ui";

const DESKTOP_LINKS = [
  { to: "/", labelKey: "nav.overview", end: true },
  { to: "/getting-started", labelKey: "nav.gettingStarted" },
  { to: "/usage", labelKey: "nav.usage" },
  { to: "/architecture", labelKey: "nav.architecture" },
  { to: "/development", labelKey: "nav.development" },
] as const;

const REFERENCE_LINKS = [
  { to: "/reference/cli", labelKey: "nav.referenceCli" },
  { to: "/reference/configuration", labelKey: "nav.referenceConfiguration" },
  { to: "/reference/keybindings", labelKey: "nav.referenceKeybindings" },
] as const;

const PROJECT_LINKS = [
  { to: "/project/security", labelKey: "nav.projectSecurity" },
  { to: "/project/roadmap", labelKey: "nav.projectRoadmap" },
  { to: "/project/faq", labelKey: "nav.projectFaq" },
  { to: "/project/troubleshooting", labelKey: "nav.projectTroubleshooting" },
  { to: "/project/limitations", labelKey: "nav.projectLimitations" },
] as const;

function LanguageSwitcher() {
  const { locale, setLocale, t } = useLocale();
  return (
    <label className="flex items-center gap-2 text-xs text-slate-400">
      <span>{t("lang.label")}</span>
      <select
        value={locale}
        onChange={(event) => setLocale(event.target.value as Locale)}
        aria-label={t("lang.label")}
        className="rounded-md border border-slate-700 bg-slate-900 px-2 py-1 text-sm text-slate-200"
      >
        {LOCALES.map((code) => (
          <option key={code} value={code}>
            {code === "en" ? t("lang.en") : t("lang.ptBR")}
          </option>
        ))}
      </select>
    </label>
  );
}

function MobileNav({ searchIndex }: { searchIndex: SearchEntry[] }) {
  const { t } = useLocale();
  const [open, setOpen] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);
  const panelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (open) {
      const firstFocusable = panelRef.current?.querySelector("a, button, select, input");
      if (firstFocusable instanceof HTMLElement) {
        firstFocusable.focus();
      }
    } else {
      buttonRef.current?.focus();
    }
  }, [open]);

  useEffect(() => {
    if (!open) {
      return;
    }
    function onKey(event: KeyboardEvent) {
      if (event.key === "Escape") {
        setOpen(false);
      }
    }
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open]);

  return (
    <div className="md:hidden">
      <button
        ref={buttonRef}
        type="button"
        aria-expanded={open}
        aria-controls="mobile-nav"
        onClick={() => setOpen((value) => !value)}
        className="rounded-md border border-slate-700 px-3 py-1.5 text-sm text-slate-200"
      >
        {open ? t("nav.closeMenu") : t("nav.openMenu")}
      </button>
      {open ? (
        <div
          ref={panelRef}
          id="mobile-nav"
          className="absolute inset-x-0 top-full z-40 space-y-4 border-b border-slate-800 bg-[#0A0F1C] px-4 py-4"
        >
          <nav aria-label={t("nav.main")}>
            <ul className="space-y-1 text-sm">
              {[...DESKTOP_LINKS, ...REFERENCE_LINKS, ...PROJECT_LINKS].map((link) => (
                <li key={link.to}>
                  <NavLink
                    to={link.to}
                    end={"end" in link && link.end}
                    onClick={() => setOpen(false)}
                    className={({ isActive }) =>
                      cx(
                        "block rounded-md px-3 py-2",
                        isActive ? "bg-slate-800 text-white" : "text-slate-300 hover:bg-slate-900",
                      )
                    }
                  >
                    {t(link.labelKey)}
                  </NavLink>
                </li>
              ))}
            </ul>
          </nav>
          <div className="flex flex-wrap items-center gap-3">
            <Search index={searchIndex} />
            <LanguageSwitcher />
          </div>
        </div>
      ) : null}
    </div>
  );
}

function Footer() {
  const { t } = useLocale();
  return (
    <footer className="border-t border-slate-800 py-6 text-center text-xs text-slate-500">
      <p>
        {t("footer.docsFor")} rusteams <VersionBadge version={__RUSTEAMS_VERSION__} />
      </p>
      <p className="mt-2 flex items-center justify-center gap-4">
        <a
          className="underline hover:text-slate-300"
          href="https://github.com/lucasvmigotto/rusteams"
        >
          {t("footer.github")}
        </a>
        <a
          className="underline hover:text-slate-300"
          href="https://github.com/lucasvmigotto/rusteams/blob/main/LICENSE"
        >
          {t("footer.license")}: GPL-3.0-or-later
        </a>
      </p>
    </footer>
  );
}

export function Layout({ searchIndex }: { searchIndex: SearchEntry[] }) {
  const { t } = useLocale();
  return (
    <div className="flex min-h-screen flex-col">
      <SkipLink />
      <header className="sticky top-0 z-30 border-b border-slate-800 bg-[#0A0F1C]/95 backdrop-blur">
        <div className="relative mx-auto flex max-w-6xl items-center gap-4 px-4 py-3">
          <NavLink
            to="/"
            className="font-mono text-lg font-bold text-sky-300"
            aria-label="rusteams home"
          >
            ▚ rusteams
          </NavLink>
          <nav
            aria-label={t("nav.main")}
            className="hidden flex-wrap items-center gap-1 text-sm md:flex"
          >
            {DESKTOP_LINKS.map((link) => (
              <NavLink
                key={link.to}
                to={link.to}
                end={"end" in link && link.end}
                className={({ isActive }) =>
                  cx(
                    "rounded-md px-3 py-1.5",
                    isActive
                      ? "bg-slate-800 text-white"
                      : "text-slate-400 hover:bg-slate-900 hover:text-slate-200",
                  )
                }
              >
                {t(link.labelKey)}
              </NavLink>
            ))}
            <span aria-hidden="true" className="text-slate-700">
              |
            </span>
            {REFERENCE_LINKS.map((link) => (
              <NavLink
                key={link.to}
                to={link.to}
                className={({ isActive }) =>
                  cx(
                    "rounded-md px-3 py-1.5",
                    isActive
                      ? "bg-slate-800 text-white"
                      : "text-slate-400 hover:bg-slate-900 hover:text-slate-200",
                  )
                }
              >
                {t(link.labelKey)}
              </NavLink>
            ))}
            <span aria-hidden="true" className="text-slate-700">
              |
            </span>
            {PROJECT_LINKS.map((link) => (
              <NavLink
                key={link.to}
                to={link.to}
                className={({ isActive }) =>
                  cx(
                    "rounded-md px-3 py-1.5",
                    isActive
                      ? "bg-slate-800 text-white"
                      : "text-slate-400 hover:bg-slate-900 hover:text-slate-200",
                  )
                }
              >
                {t(link.labelKey)}
              </NavLink>
            ))}
          </nav>
          <div className="ml-auto hidden items-center gap-3 md:flex">
            <Search index={searchIndex} />
            <LanguageSwitcher />
          </div>
          <div className="ml-auto md:hidden">
            <MobileNav searchIndex={searchIndex} />
          </div>
        </div>
      </header>
      <main id="main-content" tabIndex={-1} className="mx-auto w-full max-w-6xl flex-1 px-4 py-8">
        <Outlet />
      </main>
      <Footer />
    </div>
  );
}
