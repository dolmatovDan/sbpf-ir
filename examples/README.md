# Примеры

Пять небольших функций тестового контракта `tests/programs/bin/native-basic.v3.so` (исходник — `tests/programs/native-basic`) в трёх видах. Блоки (`bb_<pc>`) и номера инструкций во всех файлах совпадают.

| Файл | Что это |
|---|---|
| `native-basic.v3.asm` | дизассемблер sBPF, разбитый на блоки CFG |
| `native-basic.v3.ir` | IR в текстовом виде: `sbpf-lift ir <file.so> -f <функция>` |
| `native-basic.v3.json` | те же функции в JSON — формат для символьной машины (`docs/ir.md`); полный файл даёт `sbpf-lift lift <file.so>` |

| Функция | Что показывает |
|---|---|
| `fn_76` | знаковые сравнения `jslt`/`jsgt` → `slt`/`sgt` |
| `fn_84` | деление с явным `check`, `ja`, сохранения разной ширины (пример из `docs/ir.md`) |
| `fn_173` | 64-битная арифметика, двухслотовый `lddw` |
| `fn_769` | загрузки и сохранения, `add64 r, -1` со знаково расширенной константой, вызов |
| `fn_845` | syscall `sol_log_` и невозвращающийся вызов (`noreturn`) |

Функция `fn_84`:

```
asm                                IR
84: jeq r3, 0, lbb_90              [84] branch eq(r3, 0) → bb_90, bb_85
85: div64 r2, r3                   [85] check ne(r3, 0) DivideByZero
                                   [85] r2 := udiv(r2, r3)
86: stxdw [r1+0x8], r2             [86] store.64 add(r1, 8), r2
87: mov64 r2, 0                    [87] r2 := 0
                                   → bb_88
88: stxw [r1+0x0], r2              [88] store.32 add(r1, 0), trunc32(r2)
89: exit                           [89] return
90: stw [r1+0x4], 23               [90] store.32 add(r1, 4), 23:32
91: mov64 r2, 1                    [91] r2 := 1
92: ja lbb_88                      [92] next → bb_88
```
