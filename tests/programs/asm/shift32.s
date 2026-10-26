# lsh32/rsh32/arsh32: результат расширяется нулями, сдвиг на регистр берётся
# по модулю 32 от младших 32 бит, старшие биты регистра не влияют.
.globl entrypoint
entrypoint:
  mov32 w1, 1
  lsh32 w1, 31
  mov32 w2, w1
  arsh32 w2, 4
  lddw r3, 0xffffffff80000000
  rsh32 w3, 4
  lddw r4, 0x100000024
  mov32 w5, 3
  lsh32 w5, w4
  mov32 w6, -256
  arsh32 w6, w4
  lddw r7, 0x12345678ffffff00
  rsh32 w7, w4
  lddw r8, 0xffffffff00000020
  mov32 w9, 0x1234
  lsh32 w9, w8
  mov64 r0, 0
  exit
