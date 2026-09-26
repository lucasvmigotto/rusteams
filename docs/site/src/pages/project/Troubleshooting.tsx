import { Breadcrumbs, Card, Section } from "../../components/ui";
import { useLocale } from "../../i18n/useLocale";

export function Troubleshooting() {
  const { docs, t } = useLocale();
  const page = docs.troubleshooting;

  return (
    <div className="space-y-8">
      <Breadcrumbs
        trail={[{ label: t("nav.project") }, { label: t("nav.projectTroubleshooting") }]}
      />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.projectTroubleshooting")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="issues" title={t("nav.projectTroubleshooting")}>
        <div className="grid gap-3">
          {page.items.map((item) => (
            <Card key={item.title} title={item.title}>
              <code className="font-mono text-xs text-amber-200">{item.title}</code>
              <p className="mt-2">{item.body}</p>
            </Card>
          ))}
        </div>
      </Section>
    </div>
  );
}
