#!/usr/bin/env bash
# Собирает тестовые контракты под каждую версию sBPF из ARCHES
# и раскладывает их в bin/<имя>.<arch>.so. Программы asm-<имя> собираются
# из asm/<имя>.s напрямую clang и ld.lld из platform-tools, без cargo build-sbf.
#
# Использование: ./build.sh [программа ...]   (по умолчанию все)
set -euo pipefail

cd "$(dirname "$0")"

# Без явной версии cargo-build-sbf может взять старые platform-tools, где нет таргета sbpfv3.
TOOLS_VERSION="${TOOLS_VERSION:-v1.57}"
PLATFORM_TOOLS="${PLATFORM_TOOLS:-$HOME/.cache/solana/$TOOLS_VERSION/platform-tools}"
ARCHES=(v0 v3)
ASM=(mov alu64 shift64 alu32 shift32 divmod endian)
PROGRAMS=("$@")
if [ ${#PROGRAMS[@]} -eq 0 ]; then
    PROGRAMS=(native-basic native-cpi anchor-basic "${ASM[@]/#/asm-}")
fi

# e_flags лежит по смещению 48 в ELF64-заголовке (little endian, u32).
e_flags() {
    od -An -t u4 -j 48 -N 4 "$1" | tr -d ' '
}

# Линкер-скрипт и флаги линковки — из спецификации таргета rustc в platform-tools.
build_asm() {
    local src="$1" arch="$2" dst="$3"
    local out="out/$arch" triple cpu tool arg args=()
    case "$arch" in
        v0) triple=sbpf-solana-solana cpu=generic ;;
        v3) triple=sbpfv3-solana-solana cpu=v3 ;;
    esac
    for tool in llvm/bin/clang llvm/bin/ld.lld rust/bin/rustc; do
        if [ ! -x "$PLATFORM_TOOLS/$tool" ]; then
            echo "ошибка: нет $PLATFORM_TOOLS/$tool (platform-tools ставит cargo build-sbf: собрать этим скриптом любой Rust-контракт)" >&2
            exit 1
        fi
    done
    if ! command -v python3 > /dev/null; then
        echo "ошибка: для сборки asm нужен python3" >&2
        exit 1
    fi
    mkdir -p "$out"
    "$PLATFORM_TOOLS/rust/bin/rustc" --print target-spec-json -Z unstable-options \
        --target "$triple" > "$out/$triple.json"
    python3 -c '
import json, sys
spec = json.load(open(sys.argv[1]))
open(sys.argv[2], "w").write(spec["link-script"])
print("\n".join(spec["pre-link-args"]["gnu-lld"]))
' "$out/$triple.json" "$out/$triple.ld" > "$out/$triple.args"
    while IFS= read -r arg; do
        args+=("$arg")
    done < "$out/$triple.args"
    "$PLATFORM_TOOLS/llvm/bin/clang" --target=sbf -mcpu="$cpu" -c "$src" -o "$out/asm.o"
    "$PLATFORM_TOOLS/llvm/bin/ld.lld" "${args[@]}" -shared -e entrypoint \
        -T "$out/$triple.ld" "$out/asm.o" -o "$dst"
}

mkdir -p bin
for arch in "${ARCHES[@]}"; do
    out="out/$arch"
    for prog in "${PROGRAMS[@]}"; do
        echo "==> $prog ($arch)"
        dst="bin/$prog.$arch.so"
        if [[ "$prog" == asm-* ]]; then
            build_asm "asm/${prog#asm-}.s" "$arch" "$dst"
        else
            cargo build-sbf \
                --tools-version "$TOOLS_VERSION" \
                --arch "$arch" \
                --manifest-path "$prog/Cargo.toml" \
                --sbf-out-dir "$out"
            cp "$out/${prog//-/_}.so" "$dst"
        fi

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
