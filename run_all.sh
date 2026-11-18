#!/usr/bin/env bash
# Compiles every example with rustc.
#   working/*.rs  must compile and run.
#   errors/*.rs   must FAIL to compile with the error code named in the file.
# Usage: ./run_all.sh        (add -v to show full compiler errors)
set -u
cd "$(dirname "$0")/examples"
VERBOSE=0; [ "${1:-}" = "-v" ] && VERBOSE=1
OUT=$(mktemp -d); trap 'rm -rf "$OUT"' EXIT
fail=0

echo "=== working examples (should compile and run) ==="
for f in working/*.rs; do
  n=$(basename "$f" .rs)
  if rustc "$f" -o "$OUT/$n" 2>"$OUT/err.txt"; then
    echo "PASS  $f"
    "$OUT/$n" | sed 's/^/        > /'
  else
    echo "FAIL  $f (expected it to compile)"; cat "$OUT/err.txt"; fail=1
  fi
done

echo
echo "=== error examples (should NOT compile) ==="
for f in errors/*.rs; do
  expected=$(grep -o 'E0[0-9]\{3\}' "$f" | head -1)
  if rustc "$f" -o "$OUT/x" 2>"$OUT/err.txt"; then
    echo "FAIL  $f (compiled, but an error was expected)"; fail=1
  elif grep -q "error\[$expected\]" "$OUT/err.txt"; then
    echo "PASS  $f -> error[$expected] as expected"
    [ $VERBOSE -eq 1 ] && cat "$OUT/err.txt"
  else
    echo "FAIL  $f (failed, but not with $expected)"; cat "$OUT/err.txt"; fail=1
  fi
done

echo
[ $fail -eq 0 ] && echo "All examples behaved as expected." || echo "Some examples did not behave as expected."
exit $fail
