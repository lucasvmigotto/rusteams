import { Breadcrumbs, Section, Table } from "../../components/ui";
import { KEYBINDINGS } from "../../content/shared";
import { useLocale } from "../../i18n/useLocale";

export function Keybindings() {
  const { docs, t } = useLocale();
  const page = docs.keybindings;

  return (
    <div className="space-y-8">
      <Breadcrumbs
        trail={[{ label: t("nav.reference") }, { label: t("nav.referenceKeybindings") }]}
      />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.referenceKeybindings")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="all" title={page.allTitle}>
        <Table
          caption="Keybindings"
          headers={["Keys", "Action", "Status"]}
          rows={KEYBINDINGS.map(([keys, action, status]) => [keys, action, status])}
        />
      </Section>
    </div>
  );
}
