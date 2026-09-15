// Typed shape for documentation content (the `docs` namespace).
// Every locale file must satisfy this interface — a missing key is a
// type error, never a silent English fallback inside a page.
// Technical identifiers (commands, flags, env vars, URLs) live in
// content/shared.ts and stay untranslated by design.

export interface CalloutText {
  title: string;
  body: string;
}

export interface OverviewContent {
  tagline: string;
  intro: string;
  mvpNote: string;
  maturity: CalloutText;
  whatWorksTitle: string;
  cardCliBody: string;
  cardAuthTitle: string;
  cardAuthBody: string;
  cardGraphTitle: string;
  cardGraphBody: string;
  cardTuiBody: string;
  quickstartTitle: string;
  prereqsTitle: string;
  prereqsBody: string;
}

export interface GettingStartedContent {
  intro: string;
  installationTitle: string;
  installationBody: string;
  authenticationTitle: string;
  authenticationBody: string;
  noSecrets: CalloutText;
  configurationTitle: string;
  realBehavior: CalloutText;
  firstRunTitle: string;
  firstRunBody: string;
}

export interface UsageContent {
  intro: string;
  tuiBodyA: string;
  tuiBodyB: string;
  shortcutsTitle: string;
  boundNote: CalloutText;
  notificationsBody: string;
}

export interface ArchitectureContent {
  intro: string;
  layersTitle: string;
  layersBody: string;
  graphTitle: string;
  graphBody: string;
  syncTitle: string;
  syncBody: string;
  persistenceTitle: string;
  persistenceBody: string;
}

export interface DevelopmentContent {
  intro: string;
  setupBody: string;
  testingBody: string;
  cicdBody: string;
  contributingBody: string;
}

export interface CliContent {
  intro: string;
  commandsTitle: string;
  flagsTitle: string;
  missingFlags: CalloutText;
  examplesTitle: string;
}

export interface ConfigurationContent {
  intro: string;
  shapeTitle: string;
  envTitle: string;
  realPrecedence: CalloutText;
}

export interface KeybindingsContent {
  intro: string;
  allTitle: string;
}

export interface SecurityContent {
  intro: string;
  credentialsTitle: string;
  credentialsBody: string;
  terminalTitle: string;
  terminalBody: string;
  supplyChainTitle: string;
  supplyChainBody: string;
}

export interface RoadmapContent {
  intro: string;
  doneTitle: string;
  doneBody: string;
  nextTitle: string;
  nextBody: string;
  futureTitle: string;
  futureBody: string;
}

export interface FaqContent {
  intro: string;
  items: { question: string; answer: string }[];
}

export interface TroubleshootingContent {
  intro: string;
  items: { title: string; body: string }[];
}

export interface LimitationsContent {
  intro: string;
  graphTitle: string;
  graphBody: string;
  tuiTitle: string;
  tuiBody: string;
}

export interface DocsContent {
  overview: OverviewContent;
  gettingStarted: GettingStartedContent;
  usage: UsageContent;
  architecture: ArchitectureContent;
  development: DevelopmentContent;
  cli: CliContent;
  configuration: ConfigurationContent;
  keybindings: KeybindingsContent;
  security: SecurityContent;
  roadmap: RoadmapContent;
  faq: FaqContent;
  troubleshooting: TroubleshootingContent;
  limitations: LimitationsContent;
}
