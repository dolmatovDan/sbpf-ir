# lsh/rsh/arsh: константа < 64, сдвиг на регистр берётся по модулю 64.
.globl entrypoint
entrypoint:
  mov64 r1, 1
  lsh64 r1, 63
  mov64 r2, r1
  arsh64 r2, 4
  mov64 r3, r1
  rsh64 r3, 4
  mov64 r4, 70
  mov64 r5, 3
  lsh64 r5, r4
  mov64 r6, -256
  arsh64 r6, r4
  mov64 r7, -256
  rsh64 r7, r4
  mov64 r8, 64
  mov64 r9, 0x1234
  lsh64 r9, r8
  mov64 r0, 0
  exit
