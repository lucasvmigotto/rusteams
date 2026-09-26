import { Breadcrumbs, Section, StatusBadge } from "../../components/ui";
import { useLocale } from "../../i18n/useLocale";

export function Roadmap() {
  const { docs, t } = useLocale();
  const page = docs.roadmap;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.project") }, { label: t("nav.projectRoadmap") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.projectRoadmap")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="done" title={page.doneTitle}>
        <p>
          <StatusBadge status="implemented" label={t("status.implemented")} />
        </p>
        <p>{page.doneBody}</p>
      </Section>

      <Section id="next" title={page.nextTitle}>
        <p>
          <StatusBadge status="planned" label={t("status.planned")} />
        </p>
        <p>{page.nextBody}</p>
      </Section>

      <Section id="future" title={page.futureTitle}>
        <p>
          <StatusBadge status="unavailable" label={t("status.unavailable")} />
        </p>
        <p>{page.futureBody}</p>
      </Section>
    </div>
  );
}
