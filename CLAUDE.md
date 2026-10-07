# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Что это

Лифтер байткода смарт-контрактов Solana (sBPF, `.so`-ELF) в собственный IR. Первая часть символьной машины для Solana: здесь только перевод байткода в IR, без модели памяти, солвера и поиска уязвимостей. IR (JSON с версией формата) — контракт с символьной машиной, которая живёт в отдельном проекте на Kotlin, поэтому изменения формата IR ломают потребителя. План работ и статус — чек-лист в `README.md`; описание IR планируется в `docs/ir.md`.

Конвейер: `.so` → загрузка ELF, релокации, верификация (`solana-sbpf`) → CFG (`cfg.rs`, свой: `static_analysis::Analysis` в v3 не видит функций, а у первого блока функции теряет ребро в следующий блок) → лифтер → IR. Загрузку и декодирование намеренно не переписываем: `solana-sbpf` — тот же код, что исполняет контракты в сети, анализатор должен видеть программу так же, как валидатор.

Лифтер обязан делать явными все неявные особенности sBPF: обнуление старших бит у 32-битных операций, двухслотовый `lddw`, деление на ноль, знаковые/беззнаковые сравнения, различия версий sBPF. Поддерживаются только v0 (все контракты в мейннете) и v3 (новые деплои), v1/v2/v4 отклоняются при загрузке. Справочник по семантике инструкций — `bn-ebpf-solana` (ссылки в README).

## Структура

- Cargo-воркспейс (edition 2024, Rust 1.99.0 закреплён в `rust-toolchain.toml`).
  - `crates/sbpf-lift` — библиотека. Реэкспортирует `solana_sbpf`.
    - `load.rs` — `Program::load`: проверка ELF и версии, `Executable::from_elf`, `RequisiteVerifier`.
    - `syscalls.rs` — список syscall'ов (agave v4.3.0) и загрузчик, в котором они зарегистрированы.
    - `cfg.rs` — `Cfg`: функции, блоки, переходы, цели вызовов, пометка `noreturn`.
    - `dot.rs` — экспорт CFG в Graphviz DOT.
  - `crates/sbpf-lift-cli` — CLI, бинарь называется `sbpf-lift`.
- `solana-sbpf` закреплён точно (`=0.25.0`, `default-features = false`, без `jit`) в `[workspace.dependencies]`. Не обновлять и не включать фичи без явной причины.
- `tests/programs/` — **отдельный** Cargo-воркспейс (исключён из основного), собирается только через `cargo build-sbf`, не обычным `cargo build`.
  - `native-basic`, `native-cpi`, `anchor-basic` — исходники тестовых контрактов; собранные `.so` лежат в git в `bin/<имя>.<v0|v3>.so`.
  - `mainnet/` — реальные контракты (p-token, ATA, Marinade), все sBPF v0; хеши в `MANIFEST.md`.

## Неочевидные решения

- Неизвестный syscall — предупреждение (`Cfg::unknown_syscalls`), не ошибка: валидатор (`RequisiteVerifier`) хеши syscall'ов не проверяет ни в v0, ни в v3.
- `CallTarget::Invalid` — вызов, на котором интерпретатор падает (в v3 `src` не 0/1, либо цель не начало инструкции); у блока нет преемников.
- `Block::noreturn` — эвристика (из функции не достижим `exit`, либо `abort`/`sol_panic_`). Ребро возврата при этом **не удаляется**: границы функций приблизительны, а удалённое ребро означало бы потерю путей.
- Входы функций: точка входа, символы и цели `call`. Функция без символа, вызываемая только через `callx`, оказывается внутри предыдущей.

## Команды

```sh
cargo build
cargo test                                        # все тесты
cargo test -p sbpf-lift --test load_smoke         # один тестовый файл
cargo test -p sbpf-lift mainnet_programs_load     # один тест
cargo clippy --all-targets
cargo fmt
cargo run -p sbpf-lift-cli -- cfg <file.so>       # функции, блоки, переходы
cargo run -p sbpf-lift-cli -- cfg <file.so> --dot # то же в Graphviz DOT

tests/programs/build.sh [программа ...]           # пересобрать тестовые контракты под v0 и v3
tests/programs/mainnet/dump.sh                    # перекачать контракты из мейннета (RPC_URL)
```

Тесты читают `.so` из `tests/programs/{bin,mainnet}` относительно `CARGO_MANIFEST_DIR`. `load_smoke` ожидает ровно 6 файлов в `bin/` (3 программы × v0/v3) — при добавлении тестового контракта обновить этот счётчик и список `PROGRAMS` в `build.sh`.

## Окружение для сборки тестовых контрактов

- Solana CLI (Agave) 4.3.0, platform-tools v1.57. `build.sh` передаёт `--tools-version v1.57` явно: старые platform-tools не знают таргет `sbpfv3`. После сборки скрипт сверяет `e_flags` ELF с версией sBPF.
- Anchor CLI для сборки не нужен (`anchor-basic` собирается `cargo build-sbf`). `avm install`/`avm use` может переключить Solana на 3.1.10 — вернуть: `agave-install init 4.3.0`.

## Язык

Документация, комментарии в коде и сообщения коммитов — на русском.
