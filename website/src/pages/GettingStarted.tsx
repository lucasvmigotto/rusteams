import { Breadcrumbs, Callout, CodeBlock, Section, Table } from "../components/ui";
import { SCOPES } from "../content/shared";
import { useLocale } from "../i18n/useLocale";

export function GettingStarted() {
  const { docs, t } = useLocale();
  const page = docs.gettingStarted;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.gettingStarted") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.gettingStarted")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="installation" title={page.installationTitle}>
        <CodeBlock
          lang="bash"
          code={[
            "# From source (pinned toolchain)",
            "cargo build --release",
            "./target/release/rusteams version",
            "",
            "# Or via Docker (linux/amd64 + linux/arm64, no :latest tag)",
            "docker run --rm -e RUSTEAMS_CLIENT_ID=... \\",
            "  -e RUSTEAMS_TENANT_ID=organizations \\",
            `  lucasvmigotto/rusteams:${__RUSTEAMS_VERSION__} --help`,
          ].join("\n")}
        />
        <p>{page.installationBody}</p>
      </Section>

      <Section id="authentication" title={page.authenticationTitle}>
        <p>{page.authenticationBody}</p>
        <CodeBlock
          lang="bash"
          code={[
            "# Redirect URI + public-client flows required in the app registration:",
            "# https://login.microsoftonline.com/common/oauth2/nativeclient",
            "",
            "export RUSTEAMS_CLIENT_ID=<your-client-id>",
            "export RUSTEAMS_TENANT_ID=organizations",
            "rusteams login    # prints a sanitized device message, polls up to 60x",
            "rusteams status   # logged_in + tenant_id, never secrets",
            "rusteams logout   # clears the stored refresh token",
          ].join("\n")}
        />
        <Table
          caption="Delegated scopes"
          headers={["Scope", "Used for"]}
          rows={SCOPES.map((scope) => [scope, "Delegated permission"])}
        />
        <Callout kind="warn" title={page.noSecrets.title}>
          {page.noSecrets.body}
        </Callout>
      </Section>

      <Section id="configuration" title={page.configurationTitle}>
        <Table
          caption="Environment variables"
          headers={["Variable", "Effect"]}
          rows={[
            ["RUSTEAMS_CLIENT_ID", "Entra app ID (required for login)."],
            ["RUSTEAMS_TENANT_ID", "Default: organizations."],
            ["RUSTEAMS_GRAPH_BASE_URL", "Default: https://graph.microsoft.com/v1.0."],
            ["RUSTEAMS_POLL_INTERVAL_SECS", "Default 15, minimum 1."],
            ["RUST_LOG", "Default: rusteams=info."],
          ]}
        />
        <Callout kind="warn" title={page.realBehavior.title}>
          {page.realBehavior.body}
        </Callout>
        <CodeBlock
          lang="bash"
          code={[
            "rusteams config   # tenant_id, graph_base_url, poll_interval_secs, client_id set/unset",
            "rusteams doctor   # config file path + keyring reachability",
          ].join("\n")}
        />
      </Section>

      <Section id="first-run" title={page.firstRunTitle}>
        <CodeBlock
          lang="bash"
          code={[
            "rusteams status   # confirm logged_in: true",
            "rusteams        # launch the TUI (read panes + compose)",
          ].join("\n")}
        />
        <p>{page.firstRunBody}</p>
      </Section>
    </div>
  );
}
