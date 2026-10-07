#!/usr/bin/env bash
# Files the issue drafts from docs/case-studies/issue-29/issues/ in the right
# repository and rewrites WS-nn/WC-nn dependency ids into real issue URLs.
#
# Dry run (default):   bash experiments/issue-29/file-issues.sh
# Real run:            bash experiments/issue-29/file-issues.sh --apply
#
# Requires: gh (authenticated), python3. Nothing is created without --apply.
set -euo pipefail

DRAFTS="${DRAFTS:-docs/case-studies/issue-29/issues}"
APPLY=0
[[ "${1:-}" == "--apply" ]] && APPLY=1

MAP="$(mktemp)"
trap 'rm -f "$MAP"' EXIT

# Pass 1: create issues in dependency order (file names are already ordered so
# that web-capture drafts come first and lower numbers precede higher ones).
for f in "$DRAFTS"/web-capture-*.md "$DRAFTS"/web-search-*.md; do
  id="$(sed -n 's/^id: //p' "$f" | head -n1)"
  repo="$(sed -n 's/^repo: //p' "$f" | head -n1)"
  title="$(sed -n 's/^title: //p' "$f" | head -n1)"
  labels="$(sed -n 's/^labels: //p' "$f" | head -n1 | tr -d ' ')"
  body="$(python3 -I - "$f" <<'PY'
import sys, re
text = open(sys.argv[1], encoding="utf-8").read()
# strip front matter
text = re.sub(r"\A---\n.*?\n---\n", "", text, count=1, flags=re.S)
print(text.strip())
PY
)"
  if [[ $APPLY -eq 1 ]]; then
    url="$(gh issue create --repo "$repo" --title "$title" --label "$labels" --body "$body")"
  else
    url="https://github.com/$repo/issues/<pending>"
    echo "[dry-run] would create in $repo: $title"
  fi
  echo "$id $url" >> "$MAP"
done

# Pass 2: append a "Depends on" section with real URLs.
while read -r id url; do
  f="$(grep -l "^id: $id$" "$DRAFTS"/*.md)"
  deps="$(sed -n 's/^depends_on: \[\(.*\)\]/\1/p' "$f" | tr ',' '\n' | sed 's/^ *//; s/^web-capture //; s/^web-search //' | grep -v '^$' || true)"
  [[ -z "$deps" ]] && continue
  lines=""
  for d in $deps; do
    dep_url="$(awk -v k="$d" '$1==k {print $2}' "$MAP")"
    lines+="- $d: ${dep_url:-unknown}"$'\n'
  done
  if [[ $APPLY -eq 1 ]]; then
    gh issue comment "$url" --body "Depends on:"$'\n'"$lines"
  else
    echo "[dry-run] $id depends on:"; echo "$lines"
  fi
done < "$MAP"

echo "Mapping (id -> url):"; cat "$MAP"
