# add/sub/mul/and/or/xor с константой и регистром, neg64; переполнение по модулю 2^64.
.globl entrypoint
entrypoint:
  lddw r1, 0xfffffffffffffffe
  add64 r1, 5
  mov64 r2, 10
  add64 r2, r1
  sub64 r2, 20
  mov64 r3, 7
  sub64 r3, r2
  mul64 r3, -3
  lddw r4, 0x100000001
  mul64 r4, r4
  lddw r5, 0xff00ff00ff00ff00
  mov64 r6, r5
  and64 r5, 0x0ff0
  or64 r6, 0x0f
  xor64 r6, r4
  mov64 r7, r6
  and64 r7, r5
  or64 r5, r4
  mov64 r8, 0x1234
  xor64 r8, -1
  mov64 r9, 5
  neg64 r9
  mov64 r0, 0
  exit
