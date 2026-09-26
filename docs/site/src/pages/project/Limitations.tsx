import { Breadcrumbs, Section, StatusBadge } from "../../components/ui";
import { useLocale } from "../../i18n/useLocale";

export function Limitations() {
  const { docs, t } = useLocale();
  const page = docs.limitations;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.project") }, { label: t("nav.projectLimitations") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.projectLimitations")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="graph" title={page.graphTitle}>
        <p>
          <StatusBadge status="graphLimited" label={t("status.graphLimited")} />
        </p>
        <p>{page.graphBody}</p>
      </Section>

      <Section id="tui" title={page.tuiTitle}>
        <p>
          <StatusBadge status="unavailable" label={t("status.unavailable")} />
        </p>
        <p>{page.tuiBody}</p>
      </Section>
    </div>
  );
}
