default:
  @just --list

fuzz time="30" jobs="24":
    cargo +nightly fuzz run parse_walk -- -jobs={{jobs}} -workers="$(nproc)" -max_total_time={{time}}

fuzz-del-logs:
    rm fuzz-*.log

fuzz-merge:
    TMP="$(mktemp -d)" FTD="fuzz/corpus/parse_walk" && mv -T "$FTD" "$TMP" && cargo +nightly fuzz run parse_walk -- -merge=1 "$FTD" "$TMP" && ls -1 "$TMP" | wc -l && ls -1 "$FTD" | wc -l

fuzz-cov:
    cargo +nightly fuzz coverage parse_walk && cargo +nightly cov -- show target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/parse_walk --format=html -instr-profile=fuzz/coverage/parse_walk/coverage.profdata > fuzz/index.html
