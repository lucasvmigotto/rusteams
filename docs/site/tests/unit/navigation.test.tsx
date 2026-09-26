import { fireEvent, render, screen, within } from "@testing-library/react";
import { HashRouter } from "react-router-dom";
import { beforeEach, describe, expect, it } from "vitest";
import { Layout } from "@/components/Layout";
import { LocaleProvider } from "@/i18n/useLocale";

const INDEX = [
  { title: "Overview", keywords: "quickstart", to: "#/" },
  { title: "CLI", keywords: "login logout", to: "#/reference/cli" },
  { title: "Keybindings", keywords: "shortcuts navigation", to: "#/reference/keybindings" },
];

function renderLayout() {
  return render(
    <LocaleProvider>
      <HashRouter>
        <Layout searchIndex={INDEX} />
      </HashRouter>
    </LocaleProvider>,
  );
}

beforeEach(() => {
  window.localStorage.clear();
  Object.defineProperty(window.navigator, "language", { value: "en-US", configurable: true });
});

describe("site chrome and navigation", () => {
  it("renders brand, skip link, and main landmark", () => {
    renderLayout();
    expect(screen.getByRole("link", { name: "Skip to main content" })).toHaveAttribute(
      "href",
      "#main-content",
    );
    expect(screen.getByRole("link", { name: "rusteams home" })).toBeInTheDocument();
    expect(screen.getByRole("main")).toBeInTheDocument();
  });

  it("exposes desktop nav with reference and project links", () => {
    renderLayout();
    const nav = screen.getByRole("navigation", { name: "Main navigation" });
    expect(within(nav).getByRole("link", { name: "Overview" })).toBeInTheDocument();
    expect(within(nav).getByRole("link", { name: "CLI" })).toBeInTheDocument();
    expect(within(nav).getByRole("link", { name: "Security" })).toBeInTheDocument();
  });

  it("mobile menu opens, focuses content, and closes on Escape", () => {
    renderLayout();
    const toggle = screen.getByRole("button", { name: "Open navigation menu" });
    fireEvent.click(toggle);
    expect(screen.getByRole("button", { name: "Close navigation menu" })).toBeInTheDocument();
    expect(document.activeElement?.tagName).toBe("A");
    fireEvent.keyDown(document, { key: "Escape" });
    expect(screen.getByRole("button", { name: "Open navigation menu" })).toBeInTheDocument();
  });

  it("language switcher re-renders navigation in pt-BR", () => {
    renderLayout();
    const selects = screen.getAllByLabelText("Language");
    fireEvent.change(selects[0] as HTMLSelectElement, { target: { value: "pt-BR" } });
    expect(screen.getByRole("navigation", { name: "Navegação principal" })).toBeInTheDocument();
    expect(window.localStorage.getItem("rusteams-locale")).toBe("pt-BR");
  });

  it("search filters the index and closes on Escape", () => {
    renderLayout();
    fireEvent.click(
      screen.getAllByRole("button", { name: "Search documentation" })[0] as HTMLElement,
    );
    const box = screen.getByPlaceholderText("Search…");
    fireEvent.change(box, { target: { value: "logi" } });
    const panel = document.getElementById("site-search") as HTMLElement;
    expect(within(panel).getByRole("link", { name: "CLI" })).toHaveAttribute(
      "href",
      "#/reference/cli",
    );
    expect(screen.getByText("1 results")).toBeInTheDocument();
    fireEvent.change(box, { target: { value: "zzz-no-match" } });
    expect(screen.getByText(/No results/)).toBeInTheDocument();
    fireEvent.keyDown(box, { key: "Escape" });
    expect(screen.queryByPlaceholderText("Search…")).not.toBeInTheDocument();
  });

  it("footer carries version, GitHub, and license links", () => {
    renderLayout();
    expect(screen.getByText("0.0.0-test", { exact: false })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "GitHub" })).toHaveAttribute(
      "href",
      "https://github.com/lucasvmigotto/rusteams",
    );
    expect(screen.getByRole("link", { name: /License/ })).toBeInTheDocument();
  });
});
