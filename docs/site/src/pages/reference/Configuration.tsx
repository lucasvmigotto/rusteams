import { Breadcrumbs, Callout, CodeBlock, Section, Table } from "../../components/ui";
import { ENV_VARS } from "../../content/shared";
import { useLocale } from "../../i18n/useLocale";

export function Configuration() {
  const { docs, t } = useLocale();
  const page = docs.configuration;

  return (
    <div className="space-y-8">
      <Breadcrumbs
        trail={[{ label: t("nav.reference") }, { label: t("nav.referenceConfiguration") }]}
      />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.referenceConfiguration")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="shape" title={page.shapeTitle}>
        <CodeBlock
          lang="toml"
          code={[
            "# $XDG_CONFIG_HOME/rusteams/config.toml (parsed but not read by run() yet)",
            "# [rusteams] table or bare keys both parse",
            'tenant_id = "organizations"',
            'graph_base_url = "https://graph.microsoft.com/v1.0"',
            "poll_interval_secs = 15  # max(1)",
          ].join("\n")}
        />
      </Section>

      <Section id="env" title={page.envTitle}>
        <Table
          caption="Environment variables"
          headers={["Variable", "Effect"]}
          rows={ENV_VARS.map(([name, effect]) => [name, effect])}
        />
        <Callout kind="warn" title={page.realPrecedence.title}>
          {page.realPrecedence.body}
        </Callout>
      </Section>
    </div>
  );
}
