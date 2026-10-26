# mov64/mov32 с константой и регистром, lddw.
.globl entrypoint
entrypoint:
  mov64 r1, 42
  mov64 r2, -1
  mov64 r3, r2
  mov32 w4, -1
  lddw r5, 0x8000000000000001
  mov32 w6, w5
  lddw r7, 0xdeadbeefcafebabe
  mov64 r8, r7
  mov32 w9, w7
  mov64 r0, 0
  exit
