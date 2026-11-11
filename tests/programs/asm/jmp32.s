# All jmp32 conditional jumps (only in sBPF v3), imm and reg forms, each taken once and not taken once.
# jmp32 compares the low 32 bits of r8 (and r9 for reg) with the low 32 bits of the imm. Cases are picked
# so that a 64-bit compare or the other signedness gives the other outcome.
# Each case sets its own bit in r1-r7: the lower bit if the jump is not taken, the higher bit if it is.
# The LLVM SBF assembler has no jset mnemonic, so jset32 is written as bytes (opcode 0x46 / 0x4e, off +2).
# Built only for v3: the v0 verifier rejects the jmp32 class.
.globl entrypoint
entrypoint:
  mov64 r1, 0
  mov64 r2, 0
  mov64 r3, 0
  mov64 r4, 0
  mov64 r5, 0
  mov64 r6, 0
  mov64 r7, 0
  # jeq32 imm: 0x80000000, 0xffffffff80000000 -> taken
  lddw r8, 0x80000000
  jeq32 w8, -2147483648, .Ltaken0
  or64 r1, 0x1
  ja .Lend0
.Ltaken0:
  or64 r1, 0x2
.Lend0:
  # jeq32 imm: 0x1, 0x5 -> not taken
  lddw r8, 0x1
  jeq32 w8, 5, .Ltaken1
  or64 r1, 0x4
  ja .Lend1
.Ltaken1:
  or64 r1, 0x8
.Lend1:
  # jeq32 reg: 0x5, 0x300000005 -> taken
  lddw r8, 0x5
  lddw r9, 0x300000005
  jeq32 w8, w9, .Ltaken2
  or64 r1, 0x10
  ja .Lend2
.Ltaken2:
  or64 r1, 0x20
.Lend2:
  # jeq32 reg: 0x1, 0x5 -> not taken
  lddw r8, 0x1
  lddw r9, 0x5
  jeq32 w8, w9, .Ltaken3
  or64 r1, 0x40
  ja .Lend3
.Ltaken3:
  or64 r1, 0x80
.Lend3:
  # jne32 imm: 0x1, 0x5 -> taken
  lddw r8, 0x1
  jne32 w8, 5, .Ltaken4
  or64 r2, 0x1
  ja .Lend4
.Ltaken4:
  or64 r2, 0x2
.Lend4:
  # jne32 imm: 0x80000000, 0xffffffff80000000 -> not taken
  lddw r8, 0x80000000
  jne32 w8, -2147483648, .Ltaken5
  or64 r2, 0x4
  ja .Lend5
.Ltaken5:
  or64 r2, 0x8
.Lend5:
  # jne32 reg: 0x1, 0x5 -> taken
  lddw r8, 0x1
  lddw r9, 0x5
  jne32 w8, w9, .Ltaken6
  or64 r2, 0x10
  ja .Lend6
.Ltaken6:
  or64 r2, 0x20
.Lend6:
  # jne32 reg: 0x5, 0x300000005 -> not taken
  lddw r8, 0x5
  lddw r9, 0x300000005
  jne32 w8, w9, .Ltaken7
  or64 r2, 0x40
  ja .Lend7
.Ltaken7:
  or64 r2, 0x80
.Lend7:
  # jgt32 imm: 0x80000000, 0x1 -> taken
  lddw r8, 0x80000000
  jgt32 w8, 1, .Ltaken8
  or64 r3, 0x1
  ja .Lend8
.Ltaken8:
  or64 r3, 0x2
.Lend8:
  # jgt32 imm: 0x1, 0xffffffffffffffff -> not taken
  lddw r8, 0x1
  jgt32 w8, -1, .Ltaken9
  or64 r3, 0x4
  ja .Lend9
.Ltaken9:
  or64 r3, 0x8
.Lend9:
  # jgt32 reg: 0x80000000, 0x300000000 -> taken
  lddw r8, 0x80000000
  lddw r9, 0x300000000
  jgt32 w8, w9, .Ltaken10
  or64 r3, 0x10
  ja .Lend10
.Ltaken10:
  or64 r3, 0x20
.Lend10:
  # jgt32 reg: 0x300000000, 0x80000000 -> not taken
  lddw r8, 0x300000000
  lddw r9, 0x80000000
  jgt32 w8, w9, .Ltaken11
  or64 r3, 0x40
  ja .Lend11
.Ltaken11:
  or64 r3, 0x80
.Lend11:
  # jge32 imm: 0x80000000, 0x1 -> taken
  lddw r8, 0x80000000
  jge32 w8, 1, .Ltaken12
  or64 r3, 0x100
  ja .Lend12
.Ltaken12:
  or64 r3, 0x200
.Lend12:
  # jge32 imm: 0x1, 0xffffffffffffffff -> not taken
  lddw r8, 0x1
  jge32 w8, -1, .Ltaken13
  or64 r3, 0x400
  ja .Lend13
.Ltaken13:
  or64 r3, 0x800
.Lend13:
  # jge32 reg: 0x80000000, 0x300000000 -> taken
  lddw r8, 0x80000000
  lddw r9, 0x300000000
  jge32 w8, w9, .Ltaken14
  or64 r3, 0x1000
  ja .Lend14
.Ltaken14:
  or64 r3, 0x2000
.Lend14:
  # jge32 reg: 0x300000000, 0x80000000 -> not taken
  lddw r8, 0x300000000
  lddw r9, 0x80000000
  jge32 w8, w9, .Ltaken15
  or64 r3, 0x4000
  ja .Lend15
.Ltaken15:
  or64 r3, 0x8000
.Lend15:
  # jlt32 imm: 0x1, 0xffffffffffffffff -> taken
  lddw r8, 0x1
  jlt32 w8, -1, .Ltaken16
  or64 r4, 0x1
  ja .Lend16
.Ltaken16:
  or64 r4, 0x2
.Lend16:
  # jlt32 imm: 0x80000000, 0x1 -> not taken
  lddw r8, 0x80000000
  jlt32 w8, 1, .Ltaken17
  or64 r4, 0x4
  ja .Lend17
.Ltaken17:
  or64 r4, 0x8
.Lend17:
  # jlt32 reg: 0x300000000, 0x80000000 -> taken
  lddw r8, 0x300000000
  lddw r9, 0x80000000
  jlt32 w8, w9, .Ltaken18
  or64 r4, 0x10
  ja .Lend18
.Ltaken18:
  or64 r4, 0x20
.Lend18:
  # jlt32 reg: 0x80000000, 0x300000000 -> not taken
  lddw r8, 0x80000000
  lddw r9, 0x300000000
  jlt32 w8, w9, .Ltaken19
  or64 r4, 0x40
  ja .Lend19
.Ltaken19:
  or64 r4, 0x80
.Lend19:
  # jle32 imm: 0x1, 0xffffffffffffffff -> taken
  lddw r8, 0x1
  jle32 w8, -1, .Ltaken20
  or64 r4, 0x100
  ja .Lend20
.Ltaken20:
  or64 r4, 0x200
.Lend20:
  # jle32 imm: 0x80000000, 0x1 -> not taken
  lddw r8, 0x80000000
  jle32 w8, 1, .Ltaken21
  or64 r4, 0x400
  ja .Lend21
.Ltaken21:
  or64 r4, 0x800
.Lend21:
  # jle32 reg: 0x300000000, 0x80000000 -> taken
  lddw r8, 0x300000000
  lddw r9, 0x80000000
  jle32 w8, w9, .Ltaken22
  or64 r4, 0x1000
  ja .Lend22
.Ltaken22:
  or64 r4, 0x2000
.Lend22:
  # jle32 reg: 0x80000000, 0x300000000 -> not taken
  lddw r8, 0x80000000
  lddw r9, 0x300000000
  jle32 w8, w9, .Ltaken23
  or64 r4, 0x4000
  ja .Lend23
.Ltaken23:
  or64 r4, 0x8000
.Lend23:
  # jsgt32 imm: 0x8000000000000000, 0xffffffffffffffff -> taken
  lddw r8, 0x8000000000000000
  jsgt32 w8, -1, .Ltaken24
  or64 r5, 0x1
  ja .Lend24
.Ltaken24:
  or64 r5, 0x2
.Lend24:
  # jsgt32 imm: 0x80000000, 0x1 -> not taken
  lddw r8, 0x80000000
  jsgt32 w8, 1, .Ltaken25
  or64 r5, 0x4
  ja .Lend25
.Ltaken25:
  or64 r5, 0x8
.Lend25:
  # jsgt32 reg: 0x1, 0x80000000 -> taken
  lddw r8, 0x1
  lddw r9, 0x80000000
  jsgt32 w8, w9, .Ltaken26
  or64 r5, 0x10
  ja .Lend26
.Ltaken26:
  or64 r5, 0x20
.Lend26:
  # jsgt32 reg: 0x80000000, 0x1 -> not taken
  lddw r8, 0x80000000
  lddw r9, 0x1
  jsgt32 w8, w9, .Ltaken27
  or64 r5, 0x40
  ja .Lend27
.Ltaken27:
  or64 r5, 0x80
.Lend27:
  # jsge32 imm: 0x8000000000000000, 0xffffffffffffffff -> taken
  lddw r8, 0x8000000000000000
  jsge32 w8, -1, .Ltaken28
  or64 r5, 0x100
  ja .Lend28
.Ltaken28:
  or64 r5, 0x200
.Lend28:
  # jsge32 imm: 0x80000000, 0x1 -> not taken
  lddw r8, 0x80000000
  jsge32 w8, 1, .Ltaken29
  or64 r5, 0x400
  ja .Lend29
.Ltaken29:
  or64 r5, 0x800
.Lend29:
  # jsge32 reg: 0x1, 0x80000000 -> taken
  lddw r8, 0x1
  lddw r9, 0x80000000
  jsge32 w8, w9, .Ltaken30
  or64 r5, 0x1000
  ja .Lend30
.Ltaken30:
  or64 r5, 0x2000
.Lend30:
  # jsge32 reg: 0x80000000, 0x1 -> not taken
  lddw r8, 0x80000000
  lddw r9, 0x1
  jsge32 w8, w9, .Ltaken31
  or64 r5, 0x4000
  ja .Lend31
.Ltaken31:
  or64 r5, 0x8000
.Lend31:
  # jslt32 imm: 0x80000000, 0x1 -> taken
  lddw r8, 0x80000000
  jslt32 w8, 1, .Ltaken32
  or64 r6, 0x1
  ja .Lend32
.Ltaken32:
  or64 r6, 0x2
.Lend32:
  # jslt32 imm: 0x8000000000000000, 0xffffffffffffffff -> not taken
  lddw r8, 0x8000000000000000
  jslt32 w8, -1, .Ltaken33
  or64 r6, 0x4
  ja .Lend33
.Ltaken33:
  or64 r6, 0x8
.Lend33:
  # jslt32 reg: 0x80000000, 0x1 -> taken
  lddw r8, 0x80000000
  lddw r9, 0x1
  jslt32 w8, w9, .Ltaken34
  or64 r6, 0x10
  ja .Lend34
.Ltaken34:
  or64 r6, 0x20
.Lend34:
  # jslt32 reg: 0x1, 0x80000000 -> not taken
  lddw r8, 0x1
  lddw r9, 0x80000000
  jslt32 w8, w9, .Ltaken35
  or64 r6, 0x40
  ja .Lend35
.Ltaken35:
  or64 r6, 0x80
.Lend35:
  # jsle32 imm: 0x80000000, 0x1 -> taken
  lddw r8, 0x80000000
  jsle32 w8, 1, .Ltaken36
  or64 r6, 0x100
  ja .Lend36
.Ltaken36:
  or64 r6, 0x200
.Lend36:
  # jsle32 imm: 0x8000000000000000, 0xffffffffffffffff -> not taken
  lddw r8, 0x8000000000000000
  jsle32 w8, -1, .Ltaken37
  or64 r6, 0x400
  ja .Lend37
.Ltaken37:
  or64 r6, 0x800
.Lend37:
  # jsle32 reg: 0x80000000, 0x1 -> taken
  lddw r8, 0x80000000
  lddw r9, 0x1
  jsle32 w8, w9, .Ltaken38
  or64 r6, 0x1000
  ja .Lend38
.Ltaken38:
  or64 r6, 0x2000
.Lend38:
  # jsle32 reg: 0x1, 0x80000000 -> not taken
  lddw r8, 0x1
  lddw r9, 0x80000000
  jsle32 w8, w9, .Ltaken39
  or64 r6, 0x4000
  ja .Lend39
.Ltaken39:
  or64 r6, 0x8000
.Lend39:
  # jset32 imm: 0x1, 0x1 -> taken
  lddw r8, 0x1
  .byte 0x46, 0x08, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00  # jset32 +2
  or64 r7, 0x1
  ja .Lend40
  or64 r7, 0x2
.Lend40:
  # jset32 imm: 0x300000000, 0xffffffffffffffff -> not taken
  lddw r8, 0x300000000
  .byte 0x46, 0x08, 0x02, 0x00, 0xff, 0xff, 0xff, 0xff  # jset32 +2
  or64 r7, 0x4
  ja .Lend41
  or64 r7, 0x8
.Lend41:
  # jset32 reg: 0x1, 0x1 -> taken
  lddw r8, 0x1
  lddw r9, 0x1
  .byte 0x4e, 0x98, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00  # jset32 +2
  or64 r7, 0x10
  ja .Lend42
  or64 r7, 0x20
.Lend42:
  # jset32 reg: 0x300000000, 0x300000000 -> not taken
  lddw r8, 0x300000000
  lddw r9, 0x300000000
  .byte 0x4e, 0x98, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00  # jset32 +2
  or64 r7, 0x40
  ja .Lend43
  or64 r7, 0x80
.Lend43:
  mov64 r0, 0
  exit
