default:
  @just --list

fuzz target time="30" jobs="24":
    cargo +nightly fuzz run {{target}} -- -jobs={{jobs}} -workers="$(nproc)" -max_total_time={{time}}

fuzz-del-logs target:
    rm fuzz-*.log

fuzz-merge target:
    TMP="$(mktemp -d)" FTD="fuzz/corpus/{{target}}" && mv -T "$FTD" "$TMP" && cargo +nightly fuzz run {{target}} -- -merge=1 "$FTD" "$TMP" && ls -1 "$TMP" | wc -l && ls -1 "$FTD" | wc -l

fuzz-cov target:
    cargo +nightly fuzz coverage {{target}} && cargo +nightly cov -- show target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/{{target}} --format=html -instr-profile=fuzz/coverage/{{target}}/coverage.profdata > fuzz/index.html
