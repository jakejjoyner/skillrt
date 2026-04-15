---
name: pr-review
version: 0.1.0
description: Review a GitHub pull request and report findings as structured JSON.
authors:
  - jakejjoyner
license: MIT
runtime:
  type: markdown-skill
  min-version: 0.1.0
inputs:
  - name: pr_url
    type: url
    required: true
    description: The URL of the pull request to review.
  - name: depth
    type: string
    required: false
    default: normal
    description: One of "quick", "normal", "thorough".
outputs:
  type: json
  schema:
    type: object
    required: [findings, summary]
    properties:
      findings:
        type: array
        items:
          type: object
          required: [file, line, severity, issue]
          properties:
            file: { type: string }
            line: { type: integer }
            severity: { type: string, enum: [info, warning, error] }
            issue: { type: string }
      summary:
        type: string
dependencies:
  tools: [gh, jq]
permissions:
  network:
    - api.github.com
  env:
    - GH_TOKEN
tags: [github, code-review, ci]
---

# Review a GitHub pull request

You are a code reviewer. Examine the pull request at `{{ pr_url }}` and return findings as JSON matching the output schema.

## Procedure

1. Fetch PR metadata:

   ```bash
   gh pr view {{ pr_url }} --json title,body,files,additions,deletions
   ```

2. For each file in the diff, fetch the patch:

   ```bash
   gh pr diff {{ pr_url }} --patch
   ```

3. Apply the following checks based on `{{ depth }}`:

   - `quick`: flag only `TODO`, `FIXME`, debug prints (`console.log`, `println!`, `dbg!`).
   - `normal` (default): add missing tests, new `any`/`unknown` TypeScript types, unhandled errors.
   - `thorough`: add race conditions, N+1 queries, missing input validation at trust boundaries.

4. Emit findings as JSON. Include a short natural-language `summary` (≤3 sentences).

## Output format

```json
{
  "findings": [
    { "file": "src/foo.ts", "line": 42, "severity": "warning", "issue": "unhandled promise rejection" }
  ],
  "summary": "PR is a small bug fix; one unhandled promise in foo.ts needs a catch."
}
```

## Notes for the agent

- Do not post comments to the PR. This skill is read-only.
- If the PR URL is invalid or inaccessible, exit with a clear error rather than fabricating findings.
