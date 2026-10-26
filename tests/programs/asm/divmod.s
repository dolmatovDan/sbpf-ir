# Беззнаковые div/mod 64 и 32 с константой и регистром, делители не 0.
# Отрицательная константа в div64 расширяется знаком, в div32 берутся её младшие 32 бита.
# У 32-битных операндов-регистров есть старшие биты, они не должны влиять.
.globl entrypoint
entrypoint:
  mov64 r1, 100
  div64 r1, 7
  mov64 r2, 100
  mod64 r2, 7
  mov64 r3, -1
  div64 r3, -2
  mov64 r4, -7
  mov64 r5, 10
  mod64 r4, r5
  mov64 r5, 1000
  div64 r5, r1
  mov32 w6, -1
  div32 w6, -2
  lddw r7, 0x100000064
  lddw r9, 0xfffffffffffffff9
  mod32 w9, w7
  mod32 w7, 7
  mov32 w8, -7
  div32 w8, w1
  mov64 r0, 0
  exit
