import { Breadcrumbs, Callout, CodeBlock, Section, Table } from "../../components/ui";
import { CLI_COMMANDS } from "../../content/shared";
import { useLocale } from "../../i18n/useLocale";

export function Cli() {
  const { docs, t } = useLocale();
  const page = docs.cli;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.reference") }, { label: t("nav.referenceCli") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.referenceCli")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="commands" title={page.commandsTitle}>
        <Table
          caption="CLI commands"
          headers={["Command", "Behavior"]}
          rows={CLI_COMMANDS.map(([command, behavior]) => [command, behavior])}
        />
      </Section>

      <Section id="flags" title={page.flagsTitle}>
        <Table
          caption="Global flags"
          headers={["Flag", "Effect"]}
          rows={[
            ["-v, --verbose", "Parsed, currently unused."],
            ["--tenant-id", "Parsed, currently not applied."],
            ["--help / --version", "Clap built-ins."],
          ]}
        />
        <Callout kind="warn" title={page.missingFlags.title}>
          {page.missingFlags.body}
        </Callout>
      </Section>

      <Section id="examples" title={page.examplesTitle}>
        <CodeBlock
          lang="bash"
          code={[
            "rusteams --help",
            "rusteams version",
            "RUSTEAMS_CLIENT_ID=<id> rusteams login",
            "rusteams status && rusteams config && rusteams doctor",
          ].join("\n")}
        />
      </Section>
    </div>
  );
}
