import { fireEvent, render, screen, within } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import {
  Accordion,
  Breadcrumbs,
  Callout,
  Card,
  CodeBlock,
  Section,
  StatusBadge,
  Table,
} from "@/components/ui";
import { LocaleProvider } from "@/i18n/useLocale";

function renderWithLocale(children: React.ReactNode) {
  return render(<LocaleProvider>{children}</LocaleProvider>);
}

beforeEach(() => {
  window.localStorage.clear();
});

describe("design-system primitives", () => {
  it("Section renders a labelled heading", () => {
    render(
      <Section id="install" title="Installation">
        <p>body</p>
      </Section>,
    );
    const heading = screen.getByRole("heading", { level: 2, name: "Installation" });
    expect(heading).toHaveAttribute("id", "install-heading");
    expect(heading.closest("section")).toHaveAttribute("id", "install");
  });

  it("Card renders an optional title", () => {
    const { rerender } = render(<Card title="Prereqs">body</Card>);
    expect(screen.getByRole("heading", { level: 3, name: "Prereqs" })).toBeInTheDocument();
    rerender(<Card>body</Card>);
    expect(screen.queryByRole("heading")).not.toBeInTheDocument();
  });

  it("StatusBadge pairs every status with text, never color alone", () => {
    const { rerender } = render(<StatusBadge status="implemented" label="Implemented" />);
    expect(screen.getByText("Implemented")).toBeInTheDocument();
    rerender(<StatusBadge status="planned" label="Planned" />);
    expect(screen.getByText("Planned")).toBeInTheDocument();
    rerender(<StatusBadge status="graphLimited" label="Graph limitation" />);
    expect(screen.getByText("Graph limitation")).toBeInTheDocument();
  });

  it("Callout uses alert role for warnings, no landmark for plain notes", () => {
    renderWithLocale(
      <>
        <Callout kind="warn" title="Heads up">
          careful
        </Callout>
        <Callout title="FYI">plain</Callout>
      </>,
    );
    const alert = screen.getByRole("alert");
    expect(within(alert).getByText("Heads up")).toBeInTheDocument();
    expect(screen.getByText("FYI")).toBeInTheDocument();
    expect(screen.queryByRole("note")).not.toBeInTheDocument();
  });

  it("CodeBlock shows the language and a copy button", () => {
    renderWithLocale(<CodeBlock lang="bash" code="rusteams status" />);
    expect(screen.getByText("bash")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Copy code" })).toBeInTheDocument();
    expect(screen.getByText("rusteams status")).toBeInTheDocument();
  });

  it("Table exposes a caption and column scopes", () => {
    render(<Table caption="Keybindings" headers={["Keys", "Action"]} rows={[["j", "Next"]]} />);
    expect(screen.getByRole("table", { name: "Keybindings" })).toBeInTheDocument();
    expect(screen.getByRole("columnheader", { name: "Keys" })).toHaveAttribute("scope", "col");
    expect(screen.getByRole("cell", { name: "j" })).toBeInTheDocument();
  });

  it("Accordion expands and collapses with aria-expanded", () => {
    render(
      <Accordion title="Why an Entra app?">
        <p>Because BYO.</p>
      </Accordion>,
    );
    const button = screen.getByRole("button", { name: /Why an Entra app/ });
    expect(button).toHaveAttribute("aria-expanded", "false");
    fireEvent.click(button);
    expect(button).toHaveAttribute("aria-expanded", "true");
    expect(screen.getByText("Because BYO.")).toBeVisible();
    fireEvent.click(button);
    expect(button).toHaveAttribute("aria-expanded", "false");
  });

  it("Breadcrumbs mark the current page", () => {
    renderWithLocale(<Breadcrumbs trail={[{ label: "Reference" }, { label: "CLI" }]} />);
    expect(screen.getByRole("link", { name: "Home" })).toBeInTheDocument();
    expect(screen.getByText("CLI")).toHaveAttribute("aria-current", "page");
  });
});
