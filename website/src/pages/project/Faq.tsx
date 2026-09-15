import { Accordion, Breadcrumbs, Section } from "../../components/ui";
import { useLocale } from "../../i18n/useLocale";

export function Faq() {
  const { docs, t } = useLocale();
  const page = docs.faq;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.project") }, { label: t("nav.projectFaq") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.projectFaq")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="questions" title={t("nav.projectFaq")}>
        <div className="space-y-2">
          {page.items.map((item) => (
            <Accordion key={item.question} title={item.question}>
              {item.answer}
            </Accordion>
          ))}
        </div>
      </Section>
    </div>
  );
}
