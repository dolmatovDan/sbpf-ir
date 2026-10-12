# Примеры

Тестовый контракт `tests/programs/bin/native-basic.v3.so` (исходник — `tests/programs/native-basic`) в трёх видах. Функции, блоки (`bb_<pc>`) и номера инструкций во всех файлах совпадают.

| Файл | Что это | Как получен |
|---|---|---|
| `native-basic.v3.asm` | дизассемблер sBPF, разбитый на функции и блоки CFG | текст инструкций из `Cfg` (`Instruction::text`) |
| `native-basic.v3.ir` | IR в текстовом виде | `sbpf-lift ir tests/programs/bin/native-basic.v3.so` |
| `native-basic.v3.json` | IR в JSON — формат для символьной машины (`docs/ir.md`) | `sbpf-lift lift tests/programs/bin/native-basic.v3.so --pretty` |

Функция `fn_84` — пример из `docs/ir.md`:

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

Тот же блок в JSON: `jq '.functions[] | select(.entry == 84)' native-basic.v3.json`.

Файлы сгенерированы и после изменений лифтера могут устареть.
