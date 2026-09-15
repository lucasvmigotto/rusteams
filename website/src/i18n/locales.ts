export type Locale = "en" | "pt-BR";

export const LOCALES: Locale[] = ["en", "pt-BR"];

const en = {
  "skip.link": "Skip to main content",
  "nav.main": "Main navigation",
  "nav.overview": "Overview",
  "nav.gettingStarted": "Getting Started",
  "nav.usage": "Usage",
  "nav.architecture": "Architecture",
  "nav.development": "Development",
  "nav.reference": "Reference",
  "nav.referenceCli": "CLI",
  "nav.referenceConfiguration": "Configuration",
  "nav.referenceKeybindings": "Keybindings",
  "nav.project": "Project",
  "nav.projectSecurity": "Security",
  "nav.projectRoadmap": "Roadmap",
  "nav.projectFaq": "FAQ",
  "nav.projectTroubleshooting": "Troubleshooting",
  "nav.projectLimitations": "Limitations",
  "nav.openMenu": "Open navigation menu",
  "nav.closeMenu": "Close navigation menu",
  "search.label": "Search documentation",
  "search.placeholder": "Search…",
  "search.results": "results",
  "search.noResults": "No results. Try different keywords.",
  "search.close": "Close search",
  "lang.label": "Language",
  "lang.en": "English",
  "lang.ptBR": "Português (BR)",
  "footer.docsFor": "Documentation for",
  "footer.versionMismatch": "version unknown",
  "footer.github": "GitHub",
  "footer.license": "License",
  "copy.copy": "Copy code",
  "copy.copied": "Copied",
  "status.implemented": "Implemented",
  "status.partial": "Partially implemented",
  "status.planned": "Planned",
  "status.graphLimited": "Microsoft Graph limitation",
  "status.unavailable": "Unavailable",
  "breadcrumb.home": "Home",
  "toc.title": "On this page",
} as const;

export type UIKey = keyof typeof en;

const ptBR: Record<UIKey, string> = {
  "skip.link": "Pular para o conteúdo principal",
  "nav.main": "Navegação principal",
  "nav.overview": "Visão geral",
  "nav.gettingStarted": "Primeiros passos",
  "nav.usage": "Uso",
  "nav.architecture": "Arquitetura",
  "nav.development": "Desenvolvimento",
  "nav.reference": "Referência",
  "nav.referenceCli": "CLI",
  "nav.referenceConfiguration": "Configuração",
  "nav.referenceKeybindings": "Atalhos de teclado",
  "nav.project": "Projeto",
  "nav.projectSecurity": "Segurança",
  "nav.projectRoadmap": "Roteiro",
  "nav.projectFaq": "FAQ",
  "nav.projectTroubleshooting": "Solução de problemas",
  "nav.projectLimitations": "Limitações",
  "nav.openMenu": "Abrir menu de navegação",
  "nav.closeMenu": "Fechar menu de navegação",
  "search.label": "Pesquisar documentação",
  "search.placeholder": "Pesquisar…",
  "search.results": "resultados",
  "search.noResults": "Nenhum resultado. Tente outras palavras-chave.",
  "search.close": "Fechar pesquisa",
  "lang.label": "Idioma",
  "lang.en": "English",
  "lang.ptBR": "Português (BR)",
  "footer.docsFor": "Documentação do",
  "footer.versionMismatch": "versão desconhecida",
  "footer.github": "GitHub",
  "footer.license": "Licença",
  "copy.copy": "Copiar código",
  "copy.copied": "Copiado",
  "status.implemented": "Implementado",
  "status.partial": "Parcialmente implementado",
  "status.planned": "Planejado",
  "status.graphLimited": "Limitação do Microsoft Graph",
  "status.unavailable": "Indisponível",
  "breadcrumb.home": "Início",
  "toc.title": "Nesta página",
};

export const STRINGS: Record<Locale, Record<UIKey, string>> = { en, "pt-BR": ptBR };

export function isLocale(value: string | null): value is Locale {
  return value === "en" || value === "pt-BR";
}

export function detectLocale(): Locale {
  const stored = localStorage.getItem("rusteams-locale");
  if (isLocale(stored)) {
    return stored;
  }
  const browser = navigator.language;
  if (browser === "pt-BR" || browser.startsWith("pt")) {
    return "pt-BR";
  }
  return "en";
}
