import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it } from "vitest";
import { detectLocale, isLocale, STRINGS } from "@/i18n/locales";
import { LocaleProvider, useLocale } from "@/i18n/useLocale";

function Probe() {
  const { locale, t, docs } = useLocale();
  return (
    <div>
      <p data-testid="locale">{locale}</p>
      <p data-testid="nav">{t("nav.overview")}</p>
      <p data-testid="intro">{docs.overview.intro}</p>
    </div>
  );
}

function renderProbe() {
  return render(
    <LocaleProvider>
      <Probe />
    </LocaleProvider>,
  );
}

beforeEach(() => {
  window.localStorage.clear();
});

describe("locale detection, persistence, and fallback", () => {
  it("detects pt-BR from the browser and syncs <html lang>", () => {
    Object.defineProperty(window.navigator, "language", { value: "pt-BR", configurable: true });
    renderProbe();
    expect(screen.getByTestId("locale")).toHaveTextContent("pt-BR");
    expect(screen.getByTestId("nav")).toHaveTextContent("Visão geral");
    expect(document.documentElement.lang).toBe("pt-BR");
  });

  it("falls back to English for unsupported browser languages", () => {
    Object.defineProperty(window.navigator, "language", { value: "fr-FR", configurable: true });
    renderProbe();
    expect(screen.getByTestId("locale")).toHaveTextContent("en");
    expect(screen.getByTestId("nav")).toHaveTextContent("Overview");
  });

  it("prefers a stored locale over the browser language", () => {
    window.localStorage.setItem("rusteams-locale", "pt-BR");
    Object.defineProperty(window.navigator, "language", { value: "en-US", configurable: true });
    renderProbe();
    expect(screen.getByTestId("locale")).toHaveTextContent("pt-BR");
  });

  it("rejects unknown stored values", () => {
    expect(isLocale("en")).toBe(true);
    expect(isLocale("pt-BR")).toBe(true);
    expect(isLocale("es")).toBe(false);
    expect(isLocale(null)).toBe(false);
    expect(detectLocale()).toBe("en");
  });

  it("switching language persists and re-renders docs content", () => {
    Object.defineProperty(window.navigator, "language", { value: "en-US", configurable: true });
    render(
      <LocaleProvider>
        <Probe />
        <LanguageSwitchForTest />
      </LocaleProvider>,
    );
    expect(screen.getByTestId("intro")).toHaveTextContent("terminal user interface");
    fireEvent.click(screen.getByRole("button", { name: "switch to pt-BR" }));
    expect(window.localStorage.getItem("rusteams-locale")).toBe("pt-BR");
    expect(screen.getByTestId("intro")).toHaveTextContent("interface de terminal");
    expect(document.documentElement.lang).toBe("pt-BR");
  });

  it("en and pt-BR expose the same UI keys", () => {
    expect(Object.keys(STRINGS["pt-BR"]).sort()).toEqual(Object.keys(STRINGS.en).sort());
  });
});

function LanguageSwitchForTest() {
  const { setLocale } = useLocale();
  return (
    <button type="button" onClick={() => setLocale("pt-BR")}>
      switch to pt-BR
    </button>
  );
}
