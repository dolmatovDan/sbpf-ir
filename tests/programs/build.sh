#!/usr/bin/env bash
# Собирает тестовые контракты под каждую версию sBPF из ARCHES
# и раскладывает их в bin/<имя>.<arch>.so.
#
# Использование: ./build.sh [программа ...]   (по умолчанию все)
set -euo pipefail

cd "$(dirname "$0")"

# Без явной версии cargo-build-sbf может взять старые platform-tools, где нет таргета sbpfv3.
TOOLS_VERSION="${TOOLS_VERSION:-v1.57}"
ARCHES=(v0 v3)
PROGRAMS=("$@")
if [ ${#PROGRAMS[@]} -eq 0 ]; then
    PROGRAMS=(native-basic native-cpi anchor-basic)
fi

# e_flags лежит по смещению 48 в ELF64-заголовке (little endian, u32).
e_flags() {
    od -An -t u4 -j 48 -N 4 "$1" | tr -d ' '
}

mkdir -p bin
for arch in "${ARCHES[@]}"; do
    out="out/$arch"
    for prog in "${PROGRAMS[@]}"; do
        echo "==> $prog ($arch)"
        cargo build-sbf \
            --tools-version "$TOOLS_VERSION" \
            --arch "$arch" \
            --manifest-path "$prog/Cargo.toml" \
            --sbf-out-dir "$out"

        lib="${prog//-/_}"
        dst="bin/$prog.$arch.so"
        cp "$out/$lib.so" "$dst"

        expected="${arch#v}"
        actual="$(e_flags "$dst")"
        if [ "$actual" != "$expected" ]; then
            echo "ошибка: $dst имеет e_flags=$actual, ожидалось $expected" >&2
            exit 1
        fi
    done
done
rm -f out/*/*-keypair.json

echo
ls -l bin/
