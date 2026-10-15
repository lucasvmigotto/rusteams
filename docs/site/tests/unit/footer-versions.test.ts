import { describe, expect, it } from "vitest";
import { footerVersions } from "../../scripts/footer-versions";

const base = {
  repoUrl: "https://github.com/o/r",
  name: "r",
  docsVersion: "0.2.0",
  docsPath: "docs/site",
};
const sha = "abc1234def5678";

describe("footerVersions", () => {
  it("links a release build to its tag", () => {
    const v = footerVersions({ ...base, git: { exactTag: "0.2.0", lastTag: "0.2.0", sha } });
    expect(v.map((x) => x.label)).toEqual(["r@v0.2.0", "docs@v0.2.0"]);
    expect(v.map((x) => x.href)).toEqual([
      "https://github.com/o/r/tree/0.2.0",
      "https://github.com/o/r/tree/0.2.0/docs/site",
    ]);
  });

  it("marks an untagged build with its commit, linked to the commit", () => {
    const v = footerVersions({ ...base, git: { exactTag: "", lastTag: "0.2.0", sha } });
    expect(v.map((x) => x.label)).toEqual(["r@v0.2.0+abc1234", "docs@v0.2.0+abc1234"]);
    expect(v[0]?.href).toBe(`https://github.com/o/r/tree/${sha}`);
  });

  it("has no link without git", () => {
    const v = footerVersions({ ...base, git: { exactTag: "", lastTag: "0.2.0", sha: "" } });
    expect(v.map((x) => x.label)).toEqual(["r@v0.2.0", "docs@v0.2.0"]);
    expect(v.every((x) => x.href === null)).toBe(true);
  });
});
