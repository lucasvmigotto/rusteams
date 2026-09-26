import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { TuiShot } from "@/components/TuiShot";
import { LocaleProvider } from "@/i18n/useLocale";

function renderWithLocale(children: React.ReactNode) {
  return render(<LocaleProvider>{children}</LocaleProvider>);
}

beforeEach(() => {
  window.localStorage.clear();
});

describe("TuiShot terminal print", () => {
  it("renders real cells as an image with caption and transcript", () => {
    renderWithLocale(
      <TuiShot id="read" alt="Read view" caption="A caption" transcript="A transcript" />,
    );
    expect(screen.getByRole("img", { name: "Read view" })).toHaveTextContent("Engineering");
    expect(screen.getByText("A caption")).toBeInTheDocument();
    expect(screen.getByText("A transcript")).toBeInTheDocument();
  });

  it("renders nothing for an unknown shot id", () => {
    const { container } = renderWithLocale(
      <TuiShot id="nope" alt="x" caption="x" transcript="x" />,
    );
    expect(container).toBeEmptyDOMElement();
  });
});
