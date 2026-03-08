# PR Summary Skill

## Description

Reviews local git changes and generates a pull request summary following the project template.

## Steps

1. **Gather change context** by running the following git commands:
   - `git diff HEAD` — full diff of staged and unstaged changes
   - `git diff --name-only HEAD` — list of changed files
   - `git log --oneline origin/main..HEAD` — list of local commits not yet on main

2. **Infer the type of change** from the diff and file list:
   - `feat` — new feature or capability added
   - `fix` — bug fix
   - `docs` — documentation-only changes
   - `refactor` — code restructuring without behavior change
   - `test` — new or updated tests
   - `chore` — build, CI, or tooling changes
   - `perf` — performance improvement

3. **Write the summary** as topic-based bullet points describing *what* changed and *why*, grouped by area of the codebase.

4. **Output in the following template format**:

```markdown
## Description

<!-- Brief description of the changes -->

## Type of change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Refactor
- [ ] Test
- [ ] Chore

## Changes

<!-- Topic bullets summarising what was changed -->

## How to test

<!-- Steps to verify the changes work as expected -->
```

5. **Deliver the result** inside a fenced markdown code block in the chat, then write the generated content to `pr-description.md` in the repository root.
