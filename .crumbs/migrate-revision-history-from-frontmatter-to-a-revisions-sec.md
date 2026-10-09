---
id: fit-v72
title: Migrate revision history from frontmatter to a Revisions section
status: open
type: task
priority: 3
tags:
- docs
- frontmatter
created: 2026-10-09
updated: 2026-10-09
phase: ''
---

# Migrate revision history from frontmatter to a Revisions section

Docs must keep revision history in a '## Revisions' table as the last section, not in frontmatter (standard: /Volumes/SSD/Source/_Common/Frontmatter.md). Found: 2 doc(s) with a revision_history frontmatter field; 0 doc(s) with a '## Revision History' heading. For each: move the revision_history list into a '## Revisions' table (Revision | Date | Notes), delete the field, rename any '## Revision History' heading to '## Revisions', and check revision matches the last row. Verify each file with: yq --front-matter=extract '.' <file>. Find them with: rg -l '^revision_history:|^## Revision History' docs
