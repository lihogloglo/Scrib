#!/usr/bin/env bash
# doccheck.sh: no-loss check for doc cleanup slices (doc audit 2026-10-25, s.5). Bash + coreutils + awk only.
#
#   tools/doccheck.sh snapshot <name> <file>...   before the move: saves stripped lines, headings, IDs
#                                                 to .doccheck/<name>.* (git-ignored by the root whitelist)
#   tools/doccheck.sh verify   <name> <file>...   after: pass the whole new set (main + archive/history files).
#                                                 FAIL if any original line (multiset) or ID is gone. Added lines
#                                                 are fine if headings, .md pointer links, or inside
#                                                 <!-- agent-summary --> ... <!-- /agent-summary -->; others WARN.
#   tools/doccheck.sh links [root]                every relative link in docs/**, **/CLAUDE.md, **/README.md
#                                                 resolves (file, and heading anchor for .md). Exit 1 if broken.
#
# "Stripped" = CRLF normalized, leading/trailing whitespace trimmed, blank lines dropped. IDs = \b[A-Z]{1,3}-[0-9]+\b.
# Paste the final "doccheck: ..." line into the commit message. Run from the repo root (or any dir: paths are
# taken as given; .doccheck/ lives at the repo root).
set -u
export LC_ALL=C

die() { echo "doccheck: $*" >&2; exit 2; }

repo_root() { git rev-parse --show-toplevel 2>/dev/null || (cd "$(dirname "$0")/.." && pwd); }
STORE="$(repo_root)/.doccheck"

# stripped lines of the given files, one per line (unsorted)
strip_lines() {
  cat -- "$@" | tr -d '\r' | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' | grep -v '^$'
}
ids_of() { grep -oE '\b[A-Z]{1,3}-[0-9]+\b' | sort -u; }

check_files() { for f in "$@"; do [ -f "$f" ] || die "no such file: $f"; done; }

cmd_snapshot() {
  [ $# -ge 2 ] || die "usage: snapshot <name> <file>..."
  local name=$1; shift; check_files "$@"
  mkdir -p "$STORE"
  strip_lines "$@" | sort > "$STORE/$name.lines"
  grep '^#' "$STORE/$name.lines" > "$STORE/$name.headings" || true
  ids_of < "$STORE/$name.lines" > "$STORE/$name.ids"
  # location index (file:line<TAB>stripped) to name where a missing line came from
  for f in "$@"; do
    tr -d '\r' < "$f" | awk -v f="$f" '{ s=$0; sub(/^[[:space:]]+/,"",s); sub(/[[:space:]]+$/,"",s);
      if (s!="") print f ":" NR "\t" s }'
  done > "$STORE/$name.loc"
  printf '%s\n' "$@" > "$STORE/$name.files"
  echo "doccheck: snapshot $name saved ($(wc -l < "$STORE/$name.lines") lines, $(wc -l < "$STORE/$name.headings") headings, $(wc -l < "$STORE/$name.ids") IDs, $(cat -- "$@" | wc -c) bytes)"
}

cmd_verify() {
  [ $# -ge 2 ] || die "usage: verify <name> <file>..."
  local name=$1; shift; check_files "$@"
  [ -f "$STORE/$name.lines" ] || die "no snapshot '$name' (run snapshot first)"
  tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
  strip_lines "$@" | sort > "$tmp/new"
  # stripped lines inside agent-summary blocks (markers included)
  for f in "$@"; do
    tr -d '\r' < "$f" | awk '{ s=$0; sub(/^[[:space:]]+/,"",s); sub(/[[:space:]]+$/,"",s) }
      s ~ /^<!-- agent-summary -->/ { inb=1 }
      inb && s!="" { print s }
      s ~ /<!-- \/agent-summary -->/ { inb=0 }'
  done | sort > "$tmp/summary"
  comm -23 "$STORE/$name.lines" "$tmp/new" > "$tmp/missing"
  comm -13 "$STORE/$name.lines" "$tmp/new" > "$tmp/added"
  ids_of < "$tmp/new" > "$tmp/ids"
  comm -23 "$STORE/$name.ids" "$tmp/ids" > "$tmp/lostids"
  # unexplained additions: not a heading, not a .md pointer link, not (multiset) in a summary block
  comm -23 "$tmp/added" "$tmp/summary" | grep -vE '^#|\]\([^)]*\.md(#[^)]*)?\)' > "$tmp/warn" || true

  local kept missing added warn lost
  kept=$(( $(wc -l < "$STORE/$name.lines") - $(wc -l < "$tmp/missing") ))
  missing=$(wc -l < "$tmp/missing"); added=$(wc -l < "$tmp/added")
  warn=$(wc -l < "$tmp/warn"); lost=$(wc -l < "$tmp/lostids")
  echo "before: $(wc -l < "$STORE/$name.lines") lines; after: $(wc -l < "$tmp/new") lines, $(cat -- "$@" | wc -c) bytes in $# files"
  if [ "$warn" -gt 0 ]; then
    echo "WARN: $warn added lines that are not headings, .md pointers or agent-summary lines (review them):"
    uniq -c "$tmp/warn" | head -50 | sed 's/^/  /'
  fi
  if [ "$missing" -gt 0 ]; then
    echo "FAIL: $missing original lines missing (count, first original location, text):"
    uniq -c "$tmp/missing" | head -50 | awk -v loc="$STORE/$name.loc" '
      BEGIN { FS="\t"; while ((getline l < loc) > 0) { t=index(l,"\t"); k=substr(l,t+1); if (!(k in at)) at[k]=substr(l,1,t-1) } FS=" " }
      { n=$1; s=$0; sub(/^ *[0-9]+ /,"",s); printf "  %3d  %s  %s\n", n, (s in at ? at[s] : "?"), s }'
    [ "$missing" -gt 50 ] && echo "  ... ($((missing-50)) more lines not shown)"
  fi
  if [ "$lost" -gt 0 ]; then
    echo "FAIL: $lost IDs missing: $(tr '\n' ' ' < "$tmp/lostids")"
  fi
  if [ "$missing" -gt 0 ] || [ "$lost" -gt 0 ]; then
    echo "doccheck: $name FAIL ($missing lines missing, $lost IDs missing, $added added, $warn warnings)"
    return 1
  fi
  echo "doccheck: $name OK ($kept lines kept, $added added, $warn warnings)"
}

cmd_links() {
  local root=${1:-$(repo_root)}
  cd "$root" || die "bad root: $root"
  tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
  if git rev-parse --git-dir >/dev/null 2>&1; then
    git -c core.quotepath=off ls-files -co --exclude-standard | tr -d '\r' > "$tmp/all"
  else
    find . -type f | sed 's|^\./||' > "$tmp/all"
  fi
  grep -vE '(^|/)(target|\.git|sdk|shots)/' "$tmp/all" > "$tmp/all2"; mv "$tmp/all2" "$tmp/all"
  grep -E '((^|/)docs/.*\.md|(^|/)CLAUDE\.md|(^|/)README\.md)$' "$tmp/all" > "$tmp/docs" || true
  [ -s "$tmp/docs" ] || die "no docs found under $root"
  awk -v listf="$tmp/all" -v docsf="$tmp/docs" '
    function norm(p,   n, a, i, out, k) {
      n = split(p, a, "/"); k = 0
      for (i = 1; i <= n; i++) {
        if (a[i] == "" || a[i] == ".") continue
        if (a[i] == ".." && k > 0 && out[k] != "..") { k--; continue }
        out[++k] = a[i]
      }
      p = ""; for (i = 1; i <= k; i++) p = p (i > 1 ? "/" : "") out[i]
      return p
    }
    function slug(h,   s, i, c, r, n) {
      s = h
      while (match(s, /\[[^]]*\]\([^)]*\)/)) {           # [text](url) -> text
        r = substr(s, RSTART, RLENGTH); sub(/^\[/, "", r); sub(/\]\(.*$/, "", r)
        s = substr(s, 1, RSTART - 1) r substr(s, RSTART + RLENGTH)
      }
      s = tolower(s); r = ""
      for (i = 1; i <= length(s); i++) {
        c = substr(s, i, 1)
        if (c ~ /[a-z0-9_-]/) r = r c
        else if (c == " ") r = r "-"
        else if (c > "~") {                                 # UTF-8: keep letters, drop symbols
          n = (c >= "\360") ? 4 : (c >= "\340") ? 3 : 2     # U+0080-00BF (section sign, degree...),
          if (c != "\302" && c != "\342" && c != "\360") r = r substr(s, i, n)  # U+2000-2FFF (arrows,
          i += n - 1                                        # dashes, math), emoji: GitHub drops them
        }
      }
      return r
    }
    function load(f,   l, fence, h, sl, ok) {
      loaded[f] = 1; fence = 0
      while ((ok = (getline l < f)) > 0) {
        sub(/\r$/, "", l)
        if (l ~ /^[ ]{0,3}(```|~~~)/) { fence = !fence; continue }
        if (fence) continue
        while (match(l, /(id|name)="[^"]+"/)) {            # <a id="x"> / <a name="x">
          h = substr(l, RSTART, RLENGTH); sub(/^[a-z]+="/, "", h); sub(/"$/, "", h)
          anch[f, tolower(h)] = 1; l = substr(l, RSTART + RLENGTH)
        }
        if (l ~ /^#+[ \t]/) {
          h = l; sub(/^#+[ \t]+/, "", h); sub(/[ \t]+#+[ \t]*$/, "", h); sub(/[ \t]+$/, "", h)
          sl = slug(h)
          if ((f, sl) in anch) { dup[f, sl]++; sl = sl "-" dup[f, sl] }
          anch[f, sl] = 1
        }
      }
      close(f)
    }
    function exists(p) {
      if (p in have || p in dirs || p == "") return 1
      return system("test -e \"" p "\"") == 0
    }
    function check(t, src, ln,   path, a, i, d, full) {
      sub(/^</, "", t); sub(/>$/, "", t)
      if (t ~ /^[a-zA-Z][a-zA-Z0-9+.-]*:/ || t ~ /^\//) return     # URL, mailto:, absolute
      i = index(t, "#")
      if (i) { path = substr(t, 1, i - 1); a = substr(t, i + 1) } else { path = t; a = "" }
      gsub(/%20/, " ", path)
      d = src; if (d ~ /\//) sub(/\/[^\/]*$/, "", d); else d = ""
      full = (path == "") ? src : norm((d == "" ? "" : d "/") path)
      nlinks++
      if (!exists(full)) { printf "%s:%d: missing file -> %s\n", src, ln, t; nbad++; return }
      if (a != "" && full ~ /\.md$/) {
        if (!(full in loaded)) load(full)
        if (!((full, tolower(a)) in anch)) { printf "%s:%d: missing anchor -> %s\n", src, ln, t; nbad++ }
      }
    }
    BEGIN {
      while ((getline l < listf) > 0) {
        have[l] = 1
        while (sub(/\/[^\/]*$/, "", l)) dirs[l] = 1
      }
      close(listf)
      while ((getline l < docsf) > 0) ARGV[ARGC++] = l
      close(docsf)
    }
    FNR == 1 { fence = 0; nfiles++ }
    { sub(/\r$/, "") }
    /^[ ]{0,3}(```|~~~)/ { fence = !fence; next }
    fence { next }
    {
      s = $0; gsub(/`[^`]*`/, "", s)                        # drop inline code spans
      if (match(s, /^[ ]{0,3}\[[^]]+\]:[ \t]+[^ \t]+/)) {   # reference definition
        r = substr(s, RSTART, RLENGTH); sub(/^[^:]*:[ \t]+/, "", r); check(r, FILENAME, FNR)
      }
      while (match(s, /\]\([^)]*\)/)) {
        r = substr(s, RSTART + 2, RLENGTH - 3); s = substr(s, RSTART + RLENGTH)
        sub(/^[ \t]+/, "", r); sub(/[ \t].*$/, "", r)
        if (r != "") check(r, FILENAME, FNR)
      }
    }
    END {
      if (nbad) { printf "doccheck: links FAIL (%d broken of %d relative links in %d files)\n", nbad, nlinks, nfiles; exit 1 }
      printf "doccheck: links OK (%d relative links in %d files)\n", nlinks, nfiles
    }' /dev/null
}

case "${1:-}" in
  snapshot) shift; cmd_snapshot "$@" ;;
  verify)   shift; cmd_verify "$@" ;;
  links)    shift; cmd_links "$@" ;;
  *) sed -n '2,16p' "$0"; exit 2 ;;
esac
