# le16/le32/le64 и be16/be32/be64.
# Мнемоник le* в ассемблере LLVM SBF нет: они записаны байтами
# (опкод 0xd4, dst, imm = 16/32/64).
.globl entrypoint
entrypoint:
  lddw r1, 0x1122334455667788
  mov64 r2, r1
  mov64 r3, r1
  mov64 r4, r1
  mov64 r5, r1
  mov64 r6, r1
  .byte 0xd4, 0x01, 0, 0, 16, 0, 0, 0
  .byte 0xd4, 0x02, 0, 0, 32, 0, 0, 0
  .byte 0xd4, 0x03, 0, 0, 64, 0, 0, 0
  be16 r4
  be32 r5
  be64 r6
  lddw r7, 0xffffffffffff0102
  be16 r7
  mov64 r8, -1
  .byte 0xd4, 0x08, 0, 0, 32, 0, 0, 0
  mov64 r9, 0x1234
  be64 r9
  mov64 r0, 0
  exit
