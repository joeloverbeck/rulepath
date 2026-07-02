# Exact-Commit Git Provenance Guardrail

Target repository:

`joeloverbeck/rulepath`

Target commit:

`e89926ba75eb49e8c257602ba586e6963b5af804`

## Purpose

Use this workflow when Git/GitHub connector metadata, repository selection, default-branch lookup, tool namespace labels, or branch-based fetches may be stale or contaminated.

This workflow does **not** prove that the target commit is the latest `main`.

It proves only this narrower claim:

> The repository analysis uses files fetched from `joeloverbeck/rulepath` at exact commit `e89926ba75eb49e8c257602ba586e6963b5af804`.

If latest-`main` verification is required, the user must provide an independently generated current-`main` SHA and matching manifest.

## Critical distinction: fetch provenance is not file content

This guardrail validates **where repository bytes were fetched from**. It does not require repository files to avoid mentioning other repositories.

The following are two separate things:

1. **Fetch provenance** — the requested URL, resolved/final URL, repository owner, repository name, commit, and file path from which the file was retrieved.
2. **Fetched content** — the text or bytes contained inside that validly fetched file.

Only the first can trigger repository-provenance contamination.

A file fetched from a verified exact-commit URL remains valid exact-commit evidence even when its contents mention:

- another repository;
- a former repository name;
- an upstream project;
- a source from which material was adapted;
- a GitHub issue, pull request, commit, or URL elsewhere;
- an external dependency;
- a research source;
- a historical migration;
- copied or inherited documentation;
- `joeloverbeck/one-more-branch`;
- any other owner or repository.

Those are facts contained in the target repository file. They are **not evidence that the file was fetched from the wrong repository**.

**Never scan fetched file contents for foreign repository names as a provenance check.**

**Never abort merely because validly fetched content mentions another repository.**

If a validly fetched file says that material came from another repository, analyze that statement as file content. Do not reinterpret it as a fetch failure.

## Evidence lanes

Keep these evidence classes separate throughout the work.

### 1. Target-repository evidence

Claims about the state or behavior of `joeloverbeck/rulepath` at the target commit may rely only on:

- paths present in the uploaded manifest; and
- files fetched through verified exact-commit URLs for those paths.

Do not use branch fetches, repository search snippets, memory, prior chats, or files from another commit to establish target-repository state.

### 2. User-supplied control material

The uploaded manifest is a user-supplied path inventory for the stated commit.

The task prompt or research brief defines scope, requirements, and authority order.

Neither one should be treated as a substitute for fetching repository file contents unless the user explicitly supplied the actual file as authoritative source material.

### 3. External research

When the task permits or requires online research, external sources may be fetched normally and cited as external evidence.

External research:

- may mention or reside in other repositories;
- may include academic papers, product documentation, articles, project documentation, or implementation examples;
- does not have to use the target repository or target commit;
- must not be used to assert what exists in the target repository;
- must not be used as a substitute for a target-repository file that failed exact-commit acquisition.

The prohibition on repository search applies to locating or reconstructing target-repository code. It does not prohibit legitimate external research required by the task.

## Hard rules

Do not clone the repository.

Do not use GitHub code search.

Do not use snippet-based search to locate, reconstruct, or analyze target-repository files.

Do not fetch target-repository files by branch name.

Do not ask a connector for the current repository.

Do not ask a connector for the default branch.

Do not use repository-scoped connector arguments when a full file URL is required.

Do not trust connector namespace labels as evidence.

Do not treat connector namespace labels as contamination by themselves.

Do not trust prior chats, memory, old repository names, uploaded filenames, branch names, cached snippets, or tool labels as evidence of repository state.

For each target-repository file, the only trusted fetch target is an exact file URL containing all of:

1. `joeloverbeck`
2. `rulepath`
3. `e89926ba75eb49e8c257602ba586e6963b5af804`
4. the exact repository path from the uploaded manifest

A stale or misleading tool namespace label does not invalidate a fetch when the tool accepts the full URL and the requested and resolved file provenance are verified independently.

Do not follow a repository link found inside fetched content and then treat the linked material as target-repository evidence. A linked path in the target repository must independently appear in the manifest and be fetched through its own exact-commit URL.

## Allowed target-repository fetch forms

Use one of these exact-commit URL forms only:

```text
https://raw.githubusercontent.com/joeloverbeck/rulepath/e89926ba75eb49e8c257602ba586e6963b5af804/<path>
```

or:

```text
https://github.com/joeloverbeck/rulepath/blob/e89926ba75eb49e8c257602ba586e6963b5af804/<path>
```

or:

```text
https://api.github.com/repos/joeloverbeck/rulepath/contents/<path>?ref=e89926ba75eb49e8c257602ba586e6963b5af804
```

Prefer `raw.githubusercontent.com` for plain-text files.

Do not replace the commit with `main`, `master`, `HEAD`, a tag, or an abbreviated SHA.

## Exact URL construction rule

For each required path from the manifest, construct the raw URL mechanically:

```text
base = https://raw.githubusercontent.com/joeloverbeck/rulepath/e89926ba75eb49e8c257602ba586e6963b5af804/
url  = base + <manifest path>
```

Do not normalize, infer, repair, rename, or replace the repository owner, repository name, commit, or path using connector output.

Do not construct paths from search results.

## Fetch-provenance verification

Before using a fetched repository file, verify:

1. The path appears exactly in the uploaded manifest.
2. The requested URL contains the exact owner:
   `joeloverbeck`
3. The requested URL contains the exact repository:
   `rulepath`
4. The requested URL contains the full exact commit:
   `e89926ba75eb49e8c257602ba586e6963b5af804`
5. The requested URL contains the exact manifest path.
6. The fetch tool was called with the full URL rather than repository-scoped metadata.
7. If the tool exposes a final or resolved URL, it still identifies the same owner, repository, commit, and path.
8. The tool returned the requested file rather than a search result, repository overview, branch page, error-page substitute, or unrelated snippet.

Perform these checks against request and transport metadata—not by searching the file body for repository names.

## Required pipeline

### Acquisition phase

1. Treat the uploaded manifest only as the path inventory for the user-supplied target commit.
2. Identify the smallest complete set of repository paths needed for the task.
3. Confirm that every selected repository path is present in the manifest.
4. Construct each exact-commit URL mechanically.
5. Perform an early provenance preflight using representative required files before bulk acquisition.
6. Fetch every selected repository file only through a full URL.
7. Verify each fetch using the fetch-provenance checks above.
8. Maintain an append-only evidence ledger containing every requested exact URL.
9. Complete acquisition of all repository files required for the analysis.
10. Report the acquisition ledger before beginning substantive analysis.

Fetching and provenance verification are acquisition, not substantive analysis.

For a large acquisition set, the complete URL list may be supplied as a downloadable text ledger, provided the inline report states the totals and provenance result.

### Analysis phase

11. Analyze only the contents of successfully verified exact-commit repository files when making claims about the target repository.
12. Treat references to other repositories inside those files as ordinary content.
13. Conduct external research separately when required by the task.
14. Clearly distinguish repository findings from external evidence.
15. Do not fill repository-evidence gaps with memory, snippets, inferred code, external repositories, or prior chat material.
16. Update the evidence ledger if additional manifest paths are fetched during analysis.
17. Include or link the final complete ledger with the result.

## Mandatory evidence ledger

Before substantive analysis, report:

```text
Requested repository: joeloverbeck/rulepath
Target commit: e89926ba75eb49e8c257602ba586e6963b5af804
Freshness claim: user-supplied target commit only; not independently verified as latest main
Manifest role: path inventory only
Repository metadata used: no
Default-branch lookup used: no
Branch-name file fetch used: no
Target-repository code search used: no
Clone used: no
URL fetch method: <tool/function used>
Requested file count: <number>
Successfully verified file count: <number>
Fetched repository files:
- <exact URL 1>
- <exact URL 2>
- ...
Fetch-provenance contamination observed: yes/no
Foreign-repository references inside fetched file contents: permitted; not a provenance check
Connector/tool namespace trusted as evidence: no
External research lane: separate from repository evidence
```

If more repository files are fetched later, append them and provide the completed ledger with the final result.

## Provenance decision table

| Observation | Classification | Required action |
|---|---|---|
| Requested and resolved URL identify the exact target repository, commit, and manifest path | Clean repository provenance | Use the file |
| The fetched file body mentions another repository | Ordinary file content | Continue |
| The fetched file body mentions `joeloverbeck/one-more-branch` | Ordinary file content | Continue |
| The fetched file cites an upstream repository or says content originated elsewhere | Ordinary file content | Continue and evaluate its meaning if relevant |
| An external research source belongs to another repository | External evidence | Continue; do not treat it as target-repository state |
| A connector namespace or label names another repository, but the full requested and resolved URL are correct | Untrusted label only | Ignore the label and continue |
| The requested repository-file URL points to another repository | Provenance contamination | Abort |
| The final/resolved repository-file URL points to another repository | Provenance contamination | Abort |
| The final/resolved repository-file URL changes the commit or path | Provenance contamination | Abort |
| The tool substitutes a search result, snippet, branch page, or unrelated file | Provenance contamination | Abort |
| A required repository path is absent from the manifest | Inventory violation | Abort |
| A required exact-commit repository file cannot be fetched | Acquisition failure | Abort before substantive analysis |
| A target-repository claim would depend on an unfetched file, memory, or snippet | Evidence failure | Do not make the claim; abort if it is required for the deliverable |

## Abort immediately only if

Abort before substantive analysis if any of these happens:

- No full-URL file-fetch tool is available.
- A required repository path is not present in the uploaded manifest.
- A required file cannot be fetched from an exact-commit URL.
- A fetch requires a repository-scoped argument instead of accepting a full URL.
- The requested URL identifies any repository other than `joeloverbeck/rulepath`.
- The requested URL omits or changes the full target commit.
- A redirect or resolved repository-file source identifies another repository, commit, or path.
- The tool returns a search result, branch page, metadata substitute, cached snippet, or unrelated content instead of the requested file.
- A repository file is fetched using `main`, `master`, `HEAD`, another branch, a tag, or an abbreviated commit.
- A required claim about target-repository state would depend on an unfetched repository file, repository snippet, memory, prior chat, or another repository.

If a genuine provenance failure is discovered after analysis has begun, stop further analysis and report the precise transport-level failure. Do not classify ordinary text inside an already verified file as a late provenance failure.

## These are never abort reasons by themselves

Do **not** abort merely because:

- a validly fetched file mentions another repository;
- a validly fetched file mentions `joeloverbeck/one-more-branch`;
- a document cites sources from another repository;
- source code contains an external repository URL;
- a dependency, license, comment, changelog, migration note, or historical record names another project;
- a file says that material was adapted or inherited from somewhere else;
- an external research citation points to another repository or website;
- an untrusted connector namespace label is stale or incorrect while the full URL fetch itself is verified;
- repository content discusses a former repository name;
- repository content contains examples of branch names such as `main`, `master`, or `HEAD`.

The presence of such text may itself be relevant to the requested analysis. It is not fetch-provenance contamination.

## Abort report

If aborting, provide the evidence ledger and state:

1. the exact requested URL;
2. the exact resolved or returned source URL, when available;
3. the specific owner, repository, commit, or path mismatch;
4. the fetch tool or function used;
5. whether any substantive analysis had begun.

Do not generate the requested deliverable after a required repository-evidence acquisition failure.

Do not claim contamination based solely on strings appearing in fetched file contents.

## Private repository note

If the repository is private and raw GitHub URLs cannot be fetched without authentication, use only a URL-fetch connector function that accepts a full GitHub file URL.

Do not fall back to:

- repository-scoped metadata;
- repository-scoped path fetches;
- default-branch lookup;
- branch fetches;
- code search;
- snippet search;
- cached connector repository state.

If exact full-URL fetching is unavailable, abort during acquisition and ask the user to do one of the following:

1. upload the required files directly as an explicitly identified export of the target commit;
2. provide a zip/export of the repository at the target commit;
3. repair the GitHub app connection outside the chat and start a fresh session.

A later analysis based on directly uploaded files must describe them as a user-supplied target-commit export. It must not falsely claim they were fetched by exact GitHub URL.

## User-facing limitation statement

Use this wording when beginning acquisition:

> I am not verifying that this commit is the current `main`. I am using your supplied commit as the target of record and fetching repository files only by exact commit URL from `joeloverbeck/rulepath`. References to other repositories inside those validly fetched files are treated as file content, not as provenance contamination.

## Minimal prompt block

```markdown
Target repository: `joeloverbeck/rulepath`
Target commit: `e89926ba75eb49e8c257602ba586e6963b5af804`

Use exact-commit URL provenance discipline.

Do not clone. Do not use GitHub code search or repository snippets. Do not fetch target-repository files by branch name. Do not use repository metadata or default-branch lookup. Do not trust connector namespace labels.

Use the uploaded manifest only as path inventory. Fetch each needed repository file only by a full exact URL of this form:

`https://raw.githubusercontent.com/joeloverbeck/rulepath/e89926ba75eb49e8c257602ba586e6963b5af804/<path>`

Before substantive analysis, report an evidence ledger listing every exact repository-file URL fetched.

Provenance contamination is determined only from the requested and resolved fetch source—not from words, links, citations, repository names, or historical references contained inside the fetched file.

A validly fetched target-repository file may mention `joeloverbeck/one-more-branch` or any other repository. That is ordinary file content and must not trigger an abort.

Ignore stale connector namespace labels when the full requested and resolved URLs are independently verified. Abort only when the requested or resolved repository-file source, commit, or path is wrong; when a required path is absent from the manifest; when an exact file cannot be fetched; or when a required target-repository claim would depend on snippets, memory, or unfetched files.

External online research is allowed when the task requires it. Keep it separate from target-repository evidence, and never use it as a substitute for an unfetched target-repository file.

State clearly that this workflow does not independently verify latest `main`; it analyzes the user-supplied target commit only.
```