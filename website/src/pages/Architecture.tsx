import { Breadcrumbs, Section, Table } from "../components/ui";
import { GRAPH_ENDPOINTS } from "../content/shared";
import { useLocale } from "../i18n/useLocale";

export function Architecture() {
  const { docs, t } = useLocale();
  const page = docs.architecture;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.architecture") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.architecture")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="layers" title={page.layersTitle}>
        <p className="font-mono text-xs text-sky-200">tui → app → provider traits → infra/graph</p>
        <p>{page.layersBody}</p>
      </Section>

      <Section id="graph" title={page.graphTitle}>
        <Table
          caption="Graph endpoints"
          headers={["Endpoint", "Used for"]}
          rows={GRAPH_ENDPOINTS.map(([endpoint, use]) => [endpoint, use])}
        />
        <p>{page.graphBody}</p>
      </Section>

      <Section id="sync" title={page.syncTitle}>
        <p>{page.syncBody}</p>
      </Section>

      <Section id="persistence" title={page.persistenceTitle}>
        <p>{page.persistenceBody}</p>
      </Section>
    </div>
  );
}
