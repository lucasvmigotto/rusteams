import { HashRouter, Route, Routes } from "react-router-dom";
import { Layout } from "./components/Layout";
import type { SearchEntry } from "./components/Search";
import { LocaleProvider, useLocale } from "./i18n/useLocale";
import { Architecture } from "./pages/Architecture";
import { Development } from "./pages/Development";
import { GettingStarted } from "./pages/GettingStarted";
import { Overview } from "./pages/Overview";
import { Faq } from "./pages/project/Faq";
import { Limitations } from "./pages/project/Limitations";
import { Roadmap } from "./pages/project/Roadmap";
import { Security } from "./pages/project/Security";
import { Troubleshooting } from "./pages/project/Troubleshooting";
import { Cli } from "./pages/reference/Cli";
import { Configuration } from "./pages/reference/Configuration";
import { Keybindings } from "./pages/reference/Keybindings";
import { Usage } from "./pages/Usage";

// HashRouter: R2 static hosting has no SPA fallback rewrites,
// so hash-based routing keeps deep links working with zero server config.
function Shell() {
  const { t } = useLocale();
  const searchIndex: SearchEntry[] = [
    { title: t("nav.overview"), keywords: "rusteams teams terminal quickstart", to: "#/" },
    {
      title: t("nav.gettingStarted"),
      keywords: "install authentication configuration entra login client id env first run",
      to: "#/getting-started",
    },
    {
      title: t("nav.usage"),
      keywords: "tui shortcuts keybindings notifications compose",
      to: "#/usage",
    },
    {
      title: t("nav.architecture"),
      keywords: "layers provider graph sync persistence polling reducer",
      to: "#/architecture",
    },
    {
      title: t("nav.development"),
      keywords: "setup testing ci cd contributing tdd",
      to: "#/development",
    },
    {
      title: t("nav.referenceCli"),
      keywords: "cli login logout status config doctor version flags",
      to: "#/reference/cli",
    },
    {
      title: t("nav.referenceConfiguration"),
      keywords: "configuration env vars toml precedence overrides",
      to: "#/reference/configuration",
    },
    {
      title: t("nav.referenceKeybindings"),
      keywords: "keys shortcuts vim navigation",
      to: "#/reference/keybindings",
    },
    {
      title: t("nav.projectSecurity"),
      keywords: "security keyring credentials sanitizer supply chain",
      to: "#/project/security",
    },
    {
      title: t("nav.projectRoadmap"),
      keywords: "roadmap planned future phases mvp",
      to: "#/project/roadmap",
    },
    { title: t("nav.projectFaq"), keywords: "faq questions entra account", to: "#/project/faq" },
    {
      title: t("nav.projectTroubleshooting"),
      keywords: "troubleshooting errors session keyring login failed",
      to: "#/project/troubleshooting",
    },
    {
      title: t("nav.projectLimitations"),
      keywords: "limitations graph unavailable terminal calls",
      to: "#/project/limitations",
    },
  ];

  return (
    <HashRouter>
      <Routes>
        <Route element={<Layout searchIndex={searchIndex} />}>
          <Route index element={<Overview />} />
          <Route path="getting-started" element={<GettingStarted />} />
          <Route path="usage" element={<Usage />} />
          <Route path="architecture" element={<Architecture />} />
          <Route path="development" element={<Development />} />
          <Route path="reference/cli" element={<Cli />} />
          <Route path="reference/configuration" element={<Configuration />} />
          <Route path="reference/keybindings" element={<Keybindings />} />
          <Route path="project/security" element={<Security />} />
          <Route path="project/roadmap" element={<Roadmap />} />
          <Route path="project/faq" element={<Faq />} />
          <Route path="project/troubleshooting" element={<Troubleshooting />} />
          <Route path="project/limitations" element={<Limitations />} />
          <Route path="*" element={<Overview />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}

export function App() {
  return (
    <LocaleProvider>
      <Shell />
    </LocaleProvider>
  );
}
