# sbpf-ir

Лифтер байткода смарт-контрактов Solana (sBPF) в собственное промежуточное представление (IR).

Это первая часть проекта символьной машины для Solana. Задача этого репозитория — взять скомпилированный контракт (`.so`, ELF с sBPF-байткодом) и получить из него простой и однозначный IR, с которым дальше работает символьная машина. Модели памяти, солвера и поиска уязвимостей здесь нет: только перевод байткода в IR.

## Как это устроено

```
файл .so (ELF)
   │  загрузка + релокации      — solana-sbpf
   ▼
инструкции sBPF
   │  CFG                       — этот репозиторий
   ▼
функции, базовые блоки, переходы
   │  лифтер                    — этот репозиторий
   ▼
IR (файл JSON)  →  символьная машина (отдельный проект, Kotlin)
```

- **Загрузка ELF и релокации.** Делает `solana-sbpf`, тот же код, что исполняет контракты в реальной сети. Так анализатор видит программу ровно так же, как валидатор.
- **CFG.** Свой: функции, базовые блоки, переходы, цели вызовов. Инструкции декодирует `solana-sbpf`, но его `static_analysis::Analysis` в v3 не видит функций и теряет ребро в следующий блок у первого блока функции.
- **Лифтер.** Переводит каждую инструкцию sBPF в операции IR, где все неявные особенности записаны явно: расширение результата 32-битных операций (знаком у add/sub/mul, нулями у остальных), двухслотовый `lddw`, деление на ноль, знаковые и беззнаковые сравнения, различия версий sBPF.
- **IR.** Одна инструкция sBPF — один statement с деревом битвекторных выражений (переводятся в термы KSMT), функции и базовые блоки, переходы — выходами блоков. Сериализуется в JSON с номером версии формата. Этот формат — контракт между лифтером и символьной машиной на базе [usvm](https://github.com/UnitTestBot/usvm), описан в [`docs/ir.md`](docs/ir.md).

## Использование

Все команды — подкоманды одного бинаря `sbpf-lift`. Собрать: `cargo build -p sbpf-lift-cli`, бинарь окажется в `target/debug/sbpf-lift` (или запускать через `cargo run -p sbpf-lift-cli -- …`). Вывод по умолчанию в stdout, `-o FILE` — в файл.

### CFG: функции, блоки, переходы

```sh
sbpf-lift cfg tests/programs/bin/native-basic.v3.so
sbpf-lift cfg tests/programs/bin/native-basic.v3.so -f entrypoint --dot | dot -Tsvg > entrypoint.svg
```

```
sBPF V3, всего в программе функций: 55, блоков: 615, точка входа: entrypoint (pc 233)

fn fn_0 @ 0
  lbb_0 [0..=5] -> lbb_6, lbb_7
  lbb_6 [6..=6] -> lbb_7
```

`-f` — одна функция по имени или pc входа, `--dot` — граф для Graphviz.

### IR

```sh
sbpf-lift ir tests/programs/bin/native-basic.v3.so -f entrypoint   # текстом, для чтения
sbpf-lift lift tests/programs/bin/native-basic.v3.so --pretty       # JSON для символьной машины
```

```
entrypoint:
  bb_233:
    [233] r2 := r1
    [234] r1 := r10
    [235] r1 := add(r1, 0xfffffffffffff000)
    [236] call fn_378 → bb_237
```

Операции и формат JSON — [`docs/ir.md`](docs/ir.md), разобранные примеры функций — [`examples/`](examples/README.md).

### Конкретный запуск контракта

`sbpf-lift run` исполняет одну инструкцию контракта так, как её исполнил бы валидатор agave 4.3 (через [Mollusk](https://github.com/anza-xyz/mollusk)): загрузка `.so`, аккаунты, syscall'ы, CPI, учёт CU. Это эталон, с которым сравнивается символьная машина.

Вход — JSON: какой `.so` запустить, аккаунты и байты данных инструкции.

```json
{
  "program": "../../programs/bin/native-basic.v0.so",
  "program_id": "Basic11111111111111111111111111111111111111",
  "instruction_data": "0007000000000000000300000000000000"
}
```

`program` — путь относительно JSON-файла. Смысл байтов `instruction_data` знает только контракт: здесь у `native-basic` это `[операция: 1 байт][a: u64][b: u64]`, то есть `alu64(7, 3)`.

```sh
sbpf-lift run tests/examples/native-basic/alu64.json
sbpf-lift run tests/examples/native-basic/alu64.json --registers   # + регистры в конце
sbpf-lift run tests/examples/native-basic/alu64.json --trace       # + регистры на каждом шаге
```

```json
{
  "status": "success",
  "compute_units": 781,
  "return_data": "",
  "accounts": [],
  "logs": [
    "Program Basic11111111111111111111111111111111111111 invoke [1]",
    "Program log: op 0 -> 2688",
    "Program Basic11111111111111111111111111111111111111 consumed 781 of 200000 compute units",
    "Program Basic11111111111111111111111111111111111111 success"
  ]
}
```

Ошибка контракта — `"status": "failure"` и `"error"` (например `tests/examples/native-basic/div-by-zero.json` → `ArithmeticOverflow`). Готовые примеры с аккаунтами, CPI и Anchor — в [`tests/examples/`](tests/examples), полный формат входа и выхода — [`docs/runner.md`](docs/runner.md).

## Структура

```
sbpf-ir/
├── Cargo.toml              # Cargo-воркспейс
├── crates/
│   ├── sbpf-lift/          # библиотека: загрузка, CFG, лифтинг, IR
│   ├── sbpf-run/           # раннер: исполнение контракта через Mollusk
│   └── sbpf-lift-cli/      # CLI: cfg, lift, ir, run
├── docs/
│   ├── ir.md               # описание операций IR и формата файла
│   └── runner.md           # формат входа и выхода раннера
├── examples/               # разобранные функции native-basic: asm, IR, JSON
└── tests/
    ├── examples/           # примеры для раннера (JSON), генерирует gen.py
    └── programs/           # тестовые контракты (отдельный Cargo-воркспейс)
        ├── native-basic/   # нативный Rust: ветвления, ALU 32/64, деление, циклы
        ├── native-cpi/     # нативный Rust: invoke / invoke_signed в System Program
        ├── anchor-basic/   # Anchor: счётчик с constraints
        ├── asm/            # ассемблер sBPF: только регистры и арифметика (для первой версии машины)
        ├── build.sh        # сборка под v0 и v3 → bin/<имя>.<v0|v3>.so (asm → bin/asm-<имя>.…)
        ├── bin/            # собранные .so (в git)
        └── mainnet/        # контракты из мейннета: dump.sh, programs.txt, MANIFEST.md
```

## Окружение

| Инструмент | Версия | Установка |
|---|---|---|
| Rust | 1.99.0 (закреплён в `rust-toolchain.toml`) | `rustup` |
| Solana CLI (Agave) | 4.3.0 | `sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"` |
| platform-tools | v1.57 | ставит `cargo build-sbf` при первой сборке |
| Anchor CLI | 1.2.1 | `cargo install --git https://github.com/otter-sec/anchor --tag v1.2.1 avm --locked`, затем `avm install 1.2.1` |

- `cargo build-sbf` без `--tools-version v1.57` может взять старые platform-tools, где нет таргета `sbpfv3`. Поэтому `build.sh` указывает версию явно.
- `avm install`/`avm use` может сам переключить активную версию Solana на рекомендованную для Anchor (3.1.10). Вернуть обратно: `agave-install init 4.3.0`.
- Для сборки тестовых контрактов Anchor CLI не нужен: `anchor-basic` собирается обычным `cargo build-sbf`.

Пересобрать тестовые контракты: `tests/programs/build.sh`. Обновить контракты из мейннета: `tests/programs/mainnet/dump.sh`.

## План

### Подготовка
- [x] Инициализировать Cargo-воркспейс
- [x] Подключить `solana-sbpf`, закрепив версию (`=0.23.0`, как в agave 4.3.0)
- [x] Собрать несколько тестовых контрактов: нативный Rust, Anchor, контракт с CPI (под v0 и v3)
- [x] Скачать несколько реальных контрактов через `solana program dump`: p-token (pinocchio), ATA (нативный Rust), Marinade (Anchor)

### Загрузка и CFG
- [x] Загрузить `.so` через `solana-sbpf` (`Executable::from_elf`) и прогнать верификатор
- [x] Построить CFG: функции, базовые блоки, переходы, цели вызовов (свой, `Analysis` в v3 не видит функций)
- [x] Проверить, что syscall'ы разрешаются в имена (`sol_log_`, `sol_invoke_signed_rust` и т.д.)
- [x] Загружать v0 и v3, отклонять v1/v2/v4 (в мейннете их нет)
- [x] Экспортировать CFG в DOT для визуальной проверки (`sbpf-lift cfg <file.so> --dot`)

### IR
- [x] Спроектировать набор операций IR
- [x] Описать формат файла (JSON, с версией) в `docs/ir.md`
- [x] Реализовать типы IR и сериализацию (`serde`)

### Лифтер
- [x] ALU-операции (64 и 32 бита)
- [x] Загрузки и сохранения в память (1/2/4/8 байт), `lddw`
- [x] Условные и безусловные переходы
- [x] Вызовы: `call` внутренних функций, `callx`, syscall'ы
- [x] `exit` / возврат из функции
- [x] Учесть различия между v0 и v3
- [x] Человекочитаемая печать IR

### Проверка
- [x] Конкретный раннер: `sbpf-lift run <example.json>` исполняет контракт через Mollusk (agave 4.3), формат — `docs/runner.md`
- [ ] Проверить отсев трасс CPI в раннере на CPI в BPF-программу
- [ ] Написать конкретный интерпретатор IR
- [ ] Дифференциальные тесты против интерпретатора `solana-sbpf`
- [x] Прогнать лифтер на реальных контрактах из мейннета

### CLI
- [x] Команда `.so → IR.json` (`sbpf-lift lift <file.so>`)
- [x] Команда печати IR и экспорта CFG (`sbpf-lift ir`, `sbpf-lift cfg --dot`)

## Ссылки

- [solana-sbpf](https://github.com/anza-xyz/sbpf) — VM, загрузчик ELF, верификатор, дизассемблер
- [SSEREX: Practical Symbolic Execution of Solana Smart Contracts](https://arxiv.org/abs/2603.16349) — статья, на которую опирается проект
- [bn-ebpf-solana](https://github.com/otter-sec/bn-ebpf-solana) — лифтер sBPF для Binary Ninja, справочник по семантике инструкций
