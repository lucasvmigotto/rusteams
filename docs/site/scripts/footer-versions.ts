// The footer's versions: rusteams (Cargo.toml, whose X.Y.Z release.yml
// tags), then the site's own (package.json), each linked to the repository
// tree it was built from — `rusteams@v0.2.0 · docs@v0.2.0` on the release
// tag, `rusteams@v0.2.0+abc1234` linked to the commit otherwise, plain text
// without git. Pure: git facts in, entries out.
export interface GitInfo {
  /** The release tag on HEAD equal to the Cargo version, or "". */
  exactTag: string;
  /** The version to show past the tag: Cargo.toml's. */
  lastTag: string;
  sha: string;
}

export interface FooterVersion {
  name: string;
  version: string;
  label: string;
  href: string | null;
}

// `name@vX.Y.Z` (the "v" is display-only), or `name@dev…` before any release.
export function label(name: string, version: string): string {
  return /^\d/.test(version) ? `${name}@v${version}` : `${name}@${version}`;
}

export function footerVersions(opts: {
  repoUrl: string;
  name: string;
  docsVersion: string;
  docsPath: string;
  git: GitInfo;
}): FooterVersion[] {
  const { exactTag, lastTag, sha } = opts.git;
  const short = sha.slice(0, 7);
  const ref = exactTag || sha;
  const project = exactTag || (sha ? `${lastTag || "dev"}+${short}` : lastTag || "dev");
  const docs = exactTag || !sha ? opts.docsVersion : `${opts.docsVersion}+${short}`;
  const tree = ref ? `${opts.repoUrl}/tree/${ref}` : null;
  return [
    { name: opts.name, version: project, label: label(opts.name, project), href: tree },
    {
      name: "docs",
      version: docs,
      label: label("docs", docs),
      href: tree && `${tree}/${opts.docsPath}`,
    },
  ];
}
