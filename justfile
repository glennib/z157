default:
  @just --list

fuzz time="30" jobs="24":
    cargo +nightly fuzz run fuzz_target_1 -- -jobs={{jobs}} -workers="$(nproc)" -max_total_time={{time}}

fuzz-del-logs:
    rm fuzz-*.log

fuzz-merge:
    TMP="$(mktemp -d)" FTD="fuzz/corpus/fuzz_target_1" && mv -T "$FTD" "$TMP" && cargo +nightly fuzz run fuzz_target_1 -- -merge=1 "$FTD" "$TMP" && ls -1 "$TMP" | wc -l && ls -1 "$FTD" | wc -l

fuzz-cov:
    cargo +nightly fuzz coverage fuzz_target_1 && cargo +nightly cov -- show fuzz/target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/fuzz_target_1 --format=html -instr-profile=fuzz/coverage/fuzz_target_1/coverage.profdata > fuzz/index.html
