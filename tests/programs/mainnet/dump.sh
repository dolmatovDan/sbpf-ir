#!/usr/bin/env bash
# Downloads the programs from programs.txt via `solana program dump`
# and writes MANIFEST.md with hashes (on-chain programs get updated).
#
# Usage: ./dump.sh   (RPC: RPC_URL variable, mainnet-beta by default)
set -euo pipefail

cd "$(dirname "$0")"

RPC_URL="${RPC_URL:-https://api.mainnet-beta.solana.com}"
DATE="$(date -u +%Y-%m-%d)"

e_flags() {
    od -An -t u4 -j 48 -N 4 "$1" | tr -d ' '
}

{
    echo "# Контракты из мейннета"
    echo
    echo "Скачаны \`./dump.sh\` $DATE с $RPC_URL."
    echo
    echo "| Имя | Адрес | Тип | Размер | e_flags | sha256 |"
    echo "|-----|-------|-----|-------:|--------:|--------|"
} > MANIFEST.md

grep -vE '^\s*(#|$)' programs.txt | while read -r name address kind; do
    echo "==> $name ($address)"
    solana program dump --url "$RPC_URL" "$address" "$name.so" >/dev/null
    size="$(wc -c < "$name.so" | tr -d ' ')"
    sha="$(shasum -a 256 "$name.so" | cut -d' ' -f1)"
    echo "| $name | \`$address\` | $kind | $size | $(e_flags "$name.so") | \`$sha\` |" >> MANIFEST.md
done

echo
cat MANIFEST.md
