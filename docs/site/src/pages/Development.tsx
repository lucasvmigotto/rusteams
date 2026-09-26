import { Breadcrumbs, CodeBlock, Section } from "../components/ui";
import { useLocale } from "../i18n/useLocale";

export function Development() {
  const { docs, t } = useLocale();
  const page = docs.development;

  return (
    <div className="space-y-8">
      <Breadcrumbs trail={[{ label: t("nav.development") }]} />
      <div className="space-y-2">
        <h1 className="text-2xl font-bold text-slate-50">{t("nav.development")}</h1>
        <p className="max-w-3xl text-sm text-slate-300">{page.intro}</p>
      </div>

      <Section id="setup" title="Setup">
        <CodeBlock
          lang="bash"
          code={[
            "# Pinned toolchain (rust-toolchain.toml)",
            "rustup show active-toolchain  # 1.89.0",
            "cargo build",
            "cargo test",
          ].join("\n")}
        />
        <p>{page.setupBody}</p>
      </Section>

      <Section id="testing" title="Testing">
        <p>{page.testingBody}</p>
        <CodeBlock
          lang="bash"
          code={[
            "cargo test --all-features",
            "cargo clippy --all-targets -- -D warnings",
            "cargo fmt --check",
          ].join("\n")}
        />
      </Section>

      <Section id="cicd" title="CI/CD">
        <p>{page.cicdBody}</p>
      </Section>

      <Section id="contributing" title="Contributing">
        <p>{page.contributingBody}</p>
      </Section>
    </div>
  );
}
