# 32-битная арифметика: add32/sub32/mul32 расширяют результат знаком, остальные — нулями.
.globl entrypoint
entrypoint:
  mov32 w1, 0x7fffffff
  add32 w1, 1
  mov32 w2, 5
  sub32 w2, 7
  mov32 w3, 3
  add32 w3, w2
  mov32 w4, 0x10001
  mul32 w4, 0x8000
  mov32 w5, 1000
  mul32 w5, w2
  lddw r6, 0xffffffff0000ffff
  and32 w6, 0x0ff0
  or32 w6, w3
  sub32 w3, w1
  mov32 w7, 1
  or32 w7, -2147483648
  and32 w7, w4
  mov32 w8, -1
  xor32 w8, w5
  xor32 w8, 0x7ff
  mov32 w9, 5
  neg32 w9
  mov64 r0, 0
  exit
