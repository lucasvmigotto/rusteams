import { render } from "@testing-library/react";
import { axe, toHaveNoViolations } from "jest-axe";
import { beforeEach, describe, expect, it } from "vitest";
import { Accordion, Callout, Card, CodeBlock, Section, StatusBadge, Table } from "@/components/ui";
import { LocaleProvider } from "@/i18n/useLocale";
import { Overview } from "@/pages/Overview";
import { Faq } from "@/pages/project/Faq";

expect.extend(toHaveNoViolations);

function renderWithLocale(children: React.ReactNode) {
  return render(<LocaleProvider>{children}</LocaleProvider>);
}

beforeEach(() => {
  window.localStorage.clear();
  Object.defineProperty(window.navigator, "language", { value: "en-US", configurable: true });
});

// Definition of done: every component and page passes automated axe with no violations.
describe("accessibility (automated axe)", () => {
  it("design-system primitives have no violations", async () => {
    const { container } = renderWithLocale(
      <main>
        <Section id="s" title="Section">
          <p>body</p>
        </Section>
        <Card title="Card">body</Card>
        <StatusBadge status="implemented" label="Implemented" />
        <Callout title="Note">body</Callout>
        <Callout kind="warn" title="Warning">
          body
        </Callout>
        <CodeBlock lang="bash" code="rusteams status" />
        <Table caption="Keys" headers={["Keys", "Action"]} rows={[["j", "Next"]]} />
        <Accordion title="Question">answer</Accordion>
      </main>,
    );
    expect(await axe(container)).toHaveNoViolations();
  });

  it("Overview page has no violations", async () => {
    const { container } = renderWithLocale(<Overview />);
    expect(await axe(container)).toHaveNoViolations();
  });

  it("FAQ page with accordions has no violations", async () => {
    const { container } = renderWithLocale(<Faq />);
    expect(await axe(container)).toHaveNoViolations();
  });
});
