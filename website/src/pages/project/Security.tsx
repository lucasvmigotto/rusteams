import { Breadcrumbs, Section, StatusBadge } from "../../components/ui";
import { useLocale } from "../../i18n/useLocale";

export function Security() {
  const { docs, t } = useLocale();
  const page = docs.security;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.project") }, { label: t("nav.projectSecurity") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.projectSecurity")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="credentials" title={page.credentialsTitle}>
        <p>
          <StatusBadge status="implemented" label={t("status.implemented")} />
        </p>
        <p>{page.credentialsBody}</p>
      </Section>

      <Section id="terminal" title={page.terminalTitle}>
        <p>
          <StatusBadge status="implemented" label={t("status.implemented")} />
        </p>
        <p>{page.terminalBody}</p>
      </Section>

      <Section id="supply-chain" title={page.supplyChainTitle}>
        <p>
          <StatusBadge status="implemented" label={t("status.implemented")} />
        </p>
        <p>{page.supplyChainBody}</p>
      </Section>
    </div>
  );
}
