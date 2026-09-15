import type { DocsContent } from "./types";

export const ptBrDocs: DocsContent = {
  overview: {
    tagline: "Teams no terminal",
    intro:
      "rusteams é um cliente de interface de terminal (TUI) para o Microsoft Teams via Microsoft Graph. Esta documentação descreve o estado real do projeto na versão atual — o que funciona, o que é parcial e o que ainda é planejado.",
    mvpNote: "MVP core — núcleo pronto, loop ao vivo pendente",
    maturity: {
      title: "Aviso de maturidade",
      body: "Listagem de conversas ao vivo, envio/recebimento de mensagens, TUI completa e polling em tempo real ainda não estão prontos para uso diário. Qualquer afirmação contrária é um bug da documentação.",
    },
    whatWorksTitle: "O que funciona hoje",
    cardCliBody: "login · logout · status · config · doctor · version",
    cardAuthTitle: "Autenticação",
    cardAuthBody: "Device-code OAuth2 com app Entra próprio, refresh silencioso, keyring do SO.",
    cardGraphTitle: "Cliente Graph",
    cardGraphBody:
      "Chats, mensagens, presença, busca, edição, reações, citações, menções e referências de arquivo — com paginação, retry e sanitização.",
    cardTuiBody:
      "Painéis de leitura, composer, fila de notificações visuais, keymap testado. Loop ao vivo e verificação em TTY real pendentes.",
    quickstartTitle: "Início rápido",
    prereqsTitle: "Pré-requisitos",
    prereqsBody:
      "Rust 1.89.0, Linux x86_64/ARM64, conta corporativa/escolar (contas pessoais não suportadas) e um app Entra próprio. Segredos nunca vão para arquivos de configuração.",
  },
  gettingStarted: {
    intro:
      "Instalação, autenticação, configuração e primeira execução — apenas com o que existe de verdade no código.",
    installationTitle: "Instalação",
    installationBody:
      "Plataformas suportadas: Linux x86_64/ARM64 com Rust 1.89.0. Imagens publicadas apenas como X.Y.Z, sem latest.",
    authenticationTitle: "Autenticação",
    authenticationBody:
      "Modelo BYO: você registra seu próprio app público no Entra (fluxo device-code, RFC 8628). Apenas contas corporativas/escolares; contas pessoais falham de forma fechada.",
    noSecrets: {
      title: "Sem segredos",
      body: "Nunca coloque segredos em exemplos, logs ou arquivos. Access tokens vivem só em memória; apenas o refresh token vai para o keyring.",
    },
    configurationTitle: "Configuração",
    realBehavior: {
      title: "Comportamento real hoje",
      body: "A precedência documentada no código (CLI > env > arquivo > padrões) ainda não está ligada no run(): apenas padrões + env têm efeito. O arquivo TOML existe mas nada o lê; --client-id aparece numa mensagem de erro mas não existe como flag. Use variáveis de ambiente.",
    },
    firstRunTitle: "Primeira execução",
    firstRunBody:
      "Sem sessão, o app orienta: no session; run rusteams login. Teclas: j/k navegar, i compor, Enter enviar, Esc descartar, Ctrl+Q sair.",
  },
  usage: {
    intro:
      "Visão geral da TUI, atalhos reais e notificações — com o que é no-op claramente marcado.",
    tuiBodyA:
      "Cabeçalho (título + conexão), barra lateral (30%), conversa (70%) e barra de status com dicas + versão. Estados vazios mostram hints, nunca panic. Conexão: Connected, Degraded, Disconnected, Reconnecting ou Syncing.",
    tuiBodyB:
      "Mensagens renderizam segmentos estilizados: links sublinhados com URL visível, blocos de código em negrito indentados, fallback em texto puro. Todo texto remoto passa pelo sanitizador (C0/CSI/OSC/controles removidos).",
    shortcutsTitle: "Atalhos de teclado",
    boundNote: {
      title: "Mapeado ≠ funcional",
      body: "Enter (abrir) e / ou Ctrl+K (paleta) estão mapeados mas são no-op no fold atual — documentados como planejados. O modo compose (i, Enter, Esc, Backspace) funciona no loop ao vivo.",
    },
    notificationsBody:
      "Fila visual limitada com TTL de 30s, texto sanitizado e flags de não-lidas por chat. Política: apenas visual — sem campainha (bell) por padrão; bell opt-in é trabalho futuro explícito, não-MVP.",
  },
  architecture: {
    intro:
      "Camadas, providers, Graph, sincronização e persistência — como o sistema realmente é montado.",
    layersTitle: "Camadas",
    layersBody:
      "A TUI nunca chama o Graph diretamente. Redutores puros (transition/apply) com relógios/RNG do chamador; cancelamento cooperativo via Shutdown. Cache apenas em memória — sem persistência até novo ADR.",
    graphTitle: "Microsoft Graph",
    graphBody:
      "Disciplina de retry compartilhada: 429/5xx com Retry-After limitado, 401 falha rápido. HTML vira RichSegments (links, código, listas) com fallback em texto.",
    syncTitle: "Sincronização",
    syncBody:
      "Polling-first, sem webhooks no MVP: poll + diff + máquina de reconexão com watermarks, backoff e cicatrização de eventos perdidos. Latência honesta de polling — nunca afirme push/tempo real.",
    persistenceTitle: "Persistência",
    persistenceBody:
      "Apenas memória. Refresh token no keyring do SO (rusteams/default); access tokens nunca saem da memória; logout limpa. Logs com Bearer redigido.",
  },
  development: {
    intro: "Setup, testes, CI/CD e contribuição — o fluxo real do repositório.",
    setupBody:
      "Desenvolvimento em DevContainer com Docker; copie .env.example para .env (RUST_LOG — o código lê RUST_LOG, não LOG_LEVEL).",
    testingBody:
      "Modelo TDD por fases: teste falhando → implementação mínima → estilo → docs. Suítes offline (mock + wiremock + proptest) mais runbooks manuais para TTY ao vivo, tenant real e leitor de tela — verificação ao vivo nunca roda no CI.",
    cicdBody:
      "ci.yml: fmt, check/clippy/test/build no x86_64 + ARM64. security.yml: cargo audit + cargo deny check (semanal). release.yml: quality → build multi-arch → imagem dual-registry (X.Y.Z, sem latest) → tag anotada, nunca movida.",
    contributingBody:
      "Veja CONTRIBUTING.md: branches feat/*, commits pequenos e frequentes, nunca invente comportamento — derive tudo do código e marque implementado vs. planejado.",
  },
  cli: {
    intro: "Todos os comandos, como implementados em src/cli.",
    commandsTitle: "Comandos",
    flagsTitle: "Flags",
    missingFlags: {
      title: "Flags que não existem",
      body: "--client-id, --graph-base-url, --poll-interval e --no-keyring não existem. A mensagem de login sugere --client-id, mas é um texto — use RUSTEAMS_CLIENT_ID.",
    },
    examplesTitle: "Exemplos",
  },
  configuration: {
    intro: "Estrutura do AppConfig, todas as variáveis e a precedência real.",
    shapeTitle: "Estrutura",
    envTitle: "Variáveis de ambiente",
    realPrecedence: {
      title: "Precedência real",
      body: "Efetivo hoje: padrões + env. Arquivo TOML e overrides de CLI existem no código mas o run() não os aplica.",
    },
  },
  keybindings: {
    intro: "Mapeamento completo de src/tui/keys.rs — chaves não listadas são ignoradas, sem panic.",
    allTitle: "Todas as teclas",
  },
  security: {
    intro:
      "Como o rusteams protege credenciais, renderiza texto não confiável e audita dependências.",
    credentialsTitle: "Credenciais",
    credentialsBody:
      "Traga seu próprio app Entra: o client ID é configuração pública, não segredo. Access tokens vivem só em memória; o refresh token fica no keyring do SO (serviço rusteams, conta default) e o logout o apaga. Escopos delegados mínimos; contas pessoais falham de forma fechada.",
    terminalTitle: "Renderização no terminal",
    terminalBody:
      "Toda string remota passa por sanitize() antes de chegar ao ratatui: controles C0, sequências CSI/OSC, DEL, códigos Fe/charset e não-caracteres são removidos. body.content bruto nunca é renderizado; não há hyperlinks remotos clicáveis — URLs aparecem como texto visível.",
    supplyChainTitle: "Cadeia de suprimentos",
    supplyChainBody:
      "cargo audit + cargo deny check rodam no CI e semanalmente (apenas licenças compatíveis com GPL-3.0, registry fixado). Imagens de release são distroless nonroot, versionadas X.Y.Z sem tag latest.",
  },
  roadmap: {
    intro:
      "Onde o projeto está e o que vem a seguir. Os rótulos de status seguem as fases do repositório.",
    doneTitle: "Pronto (núcleo MVP)",
    doneBody:
      "CLI, auth device-code com refresh silencioso, cliente Graph de chat/mensagem/presença/busca/operações, redutores puros, motor de polling, painéis de leitura, composer, notificações visuais.",
    nextTitle: "Próximo (precisa verificação ao vivo)",
    nextBody:
      "Login em tenant real e chamadas Graph, loop de eventos em TTY ao vivo, binding do sync por timer, passes de leitor de tela e teclado-only, smoke tests em tenant.",
    futureTitle: "Futuro (estratégico)",
    futureBody:
      "Times/canais, reuniões e chamadas, upload de bytes no OneDrive, transcrições, bots, superfícies de admin, temas. Cada um precisa de revisão de API, permissão e viabilidade — sem implementação antes das fases 0–6.",
  },
  faq: {
    intro: "Respostas curtas para perguntas comuns.",
    items: [
      {
        question: "Preciso do meu próprio app Entra?",
        answer:
          "Sim. O rusteams não traz client ID; registre um app público com redirect https://login.microsoftonline.com/common/oauth2/nativeclient e exporte RUSTEAMS_CLIENT_ID.",
      },
      {
        question: "Posso usar conta pessoal Microsoft?",
        answer:
          "Não. Apenas contas corporativas/escolares — contas pessoais falham de forma fechada no login. É limitação da política do Entra, não bug.",
      },
      {
        question: "Onde coloco meu client ID?",
        answer:
          "Na variável de ambiente RUSTEAMS_CLIENT_ID. O arquivo TOML parseia mas o binário ainda não o lê, e --client-id não é flag real.",
      },
      {
        question: "Há som de notificação?",
        answer:
          "Não, por design. Notificações são apenas visuais; bell opt-in atrás de flag é trabalho futuro documentado.",
      },
    ],
  },
  troubleshooting: {
    intro: "Corrija as falhas mais comuns com as strings exatas que a CLI imprime.",
    items: [
      {
        title: "client-id is not set",
        body: "Defina RUSTEAMS_CLIENT_ID (ou RUSTEAMS_TENANT_ID=organizations). Não existe flag --client-id, apesar do que o erro sugere.",
      },
      {
        title: "no session; run rusteams login",
        body: "Sem refresh token no keyring. Rode rusteams login e depois rusteams status para confirmar logged_in: true.",
      },
      {
        title: "credential store unavailable",
        body: "Keyring do SO inalcançável (comum em Docker headless). Tokens ficam só em memória naquela execução; autentique-se a cada início.",
      },
      {
        title: "authentication failed / graph rejected credentials",
        body: "Refresh ou access token rejeitado. Refaça o login; para contas pessoais veja o FAQ — não são suportadas.",
      },
    ],
  },
  limitations: {
    intro: "O que não pode existir num cliente de terminal, e por quê.",
    graphTitle: "Limitações do Microsoft Graph",
    graphBody:
      "Respostas de listagem não trazem estado de leitura, então badges de não-lidas vêm só da visão local. Fetch de hosted-content limita a 5 MiB. Upload de bytes é território da API do OneDrive — só referências de arquivo pré-enviadas são suportadas.",
    tuiTitle: "Limitações do terminal",
    tuiBody:
      "Sem chamadas de voz/vídeo, sem canvas de reunião, sem cards ricos ou temas — um terminal renderiza texto sanitizado. Teclas open e palette estão mapeadas mas ainda sem função; compose, navegação e quit totalmente ligados.",
  },
};
