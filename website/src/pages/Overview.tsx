import {
  Breadcrumbs,
  Callout,
  Card,
  CodeBlock,
  FeatureCard,
  Section,
  StatusBadge,
} from "../components/ui";
import { useLocale } from "../i18n/useLocale";

export function Overview() {
  const { docs, t } = useLocale();
  const page = docs.overview;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.overview") }]} />
      <div className="space-y-3">
        <h1 className="font-mono text-3xl font-bold text-slate-50">
          ▚ rusteams <span className="text-lg font-normal text-slate-400">{page.tagline}</span>
        </h1>
        <p className="max-w-3xl text-sm leading-relaxed text-slate-300">{page.intro}</p>
        <p>
          <StatusBadge status="partial" label={t("status.partial")} />{" "}
          <span className="text-sm text-slate-400">
            v{__RUSTEAMS_VERSION__} · {page.mvpNote}
          </span>
        </p>
      </div>

      <Callout kind="warn" title={page.maturity.title}>
        {page.maturity.body}
      </Callout>

      <Section id="what-works" title={page.whatWorksTitle}>
        <div className="grid gap-3 md:grid-cols-2">
          <FeatureCard
            title="CLI"
            status={<StatusBadge status="implemented" label={t("status.implemented")} />}
          >
            <code className="font-mono text-xs">{page.cardCliBody}</code>
          </FeatureCard>
          <FeatureCard
            title={page.cardAuthTitle}
            status={<StatusBadge status="implemented" label={t("status.implemented")} />}
          >
            {page.cardAuthBody}
          </FeatureCard>
          <FeatureCard
            title={page.cardGraphTitle}
            status={<StatusBadge status="implemented" label={t("status.implemented")} />}
          >
            {page.cardGraphBody}
          </FeatureCard>
          <FeatureCard
            title="TUI"
            status={<StatusBadge status="partial" label={t("status.partial")} />}
          >
            {page.cardTuiBody}
          </FeatureCard>
        </div>
      </Section>

      <Section id="quickstart" title={page.quickstartTitle}>
        <CodeBlock
          lang="bash"
          code={[
            "# 1. Configure your own Entra app, then export its ID",
            "export RUSTEAMS_CLIENT_ID=<your-client-id>",
            "export RUSTEAMS_TENANT_ID=organizations",
            "",
            "# 2. Log in (device code) and check status",
            "rusteams login",
            "rusteams status",
            "",
            "# 3. Launch (configure first)",
            "rusteams",
          ].join("\n")}
        />
        <Card title={page.prereqsTitle}>{page.prereqsBody}</Card>
      </Section>
    </div>
  );
}
