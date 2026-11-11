# All 64-bit conditional jumps (jeq jne jgt jge jlt jle jsgt jsge jslt jsle jset), imm and reg forms,
# each taken once and not taken once. Operands are r8 (and r9 for reg); the imm is sign-extended to 64 bits.
# Cases are picked so that a wrong reading (other signedness, zero-extended imm) gives the other outcome.
# Each case sets its own bit in r1-r7: the lower bit if the jump is not taken, the higher bit if it is.
# The not-taken side skips the taken one with ja, so blocks are entered both by a jump and by falling through.
# The LLVM SBF assembler has no jset mnemonic, so jset is written as bytes (opcode 0x45 / 0x4d, off +2).
.globl entrypoint
entrypoint:
  mov64 r1, 0
  mov64 r2, 0
  mov64 r3, 0
  mov64 r4, 0
  mov64 r5, 0
  mov64 r6, 0
  mov64 r7, 0
  # jeq imm: 0xffffffffffffffff, 0xffffffffffffffff -> taken
  lddw r8, 0xffffffffffffffff
  jeq r8, -1, .Ltaken0
  or64 r1, 0x1
  ja .Lend0
.Ltaken0:
  or64 r1, 0x2
.Lend0:
  # jeq imm: 0x80000000, 0xffffffff80000000 -> not taken
  lddw r8, 0x80000000
  jeq r8, -2147483648, .Ltaken1
  or64 r1, 0x4
  ja .Lend1
.Ltaken1:
  or64 r1, 0x8
.Lend1:
  # jeq reg: 0x1, 0x1 -> taken
  lddw r8, 0x1
  lddw r9, 0x1
  jeq r8, r9, .Ltaken2
  or64 r1, 0x10
  ja .Lend2
.Ltaken2:
  or64 r1, 0x20
.Lend2:
  # jeq reg: 0x1, 0x5 -> not taken
  lddw r8, 0x1
  lddw r9, 0x5
  jeq r8, r9, .Ltaken3
  or64 r1, 0x40
  ja .Lend3
.Ltaken3:
  or64 r1, 0x80
.Lend3:
  # jne imm: 0x80000000, 0xffffffff80000000 -> taken
  lddw r8, 0x80000000
  jne r8, -2147483648, .Ltaken4
  or64 r2, 0x1
  ja .Lend4
.Ltaken4:
  or64 r2, 0x2
.Lend4:
  # jne imm: 0xffffffffffffffff, 0xffffffffffffffff -> not taken
  lddw r8, 0xffffffffffffffff
  jne r8, -1, .Ltaken5
  or64 r2, 0x4
  ja .Lend5
.Ltaken5:
  or64 r2, 0x8
.Lend5:
  # jne reg: 0x1, 0x5 -> taken
  lddw r8, 0x1
  lddw r9, 0x5
  jne r8, r9, .Ltaken6
  or64 r2, 0x10
  ja .Lend6
.Ltaken6:
  or64 r2, 0x20
.Lend6:
  # jne reg: 0x1, 0x1 -> not taken
  lddw r8, 0x1
  lddw r9, 0x1
  jne r8, r9, .Ltaken7
  or64 r2, 0x40
  ja .Lend7
.Ltaken7:
  or64 r2, 0x80
.Lend7:
  # jgt imm: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  jgt r8, 1, .Ltaken8
  or64 r3, 0x1
  ja .Lend8
.Ltaken8:
  or64 r3, 0x2
.Lend8:
  # jgt imm: 0xffffffff, 0xfffffffffffffffe -> not taken
  lddw r8, 0xffffffff
  jgt r8, -2, .Ltaken9
  or64 r3, 0x4
  ja .Lend9
.Ltaken9:
  or64 r3, 0x8
.Lend9:
  # jgt reg: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jgt r8, r9, .Ltaken10
  or64 r3, 0x10
  ja .Lend10
.Ltaken10:
  or64 r3, 0x20
.Lend10:
  # jgt reg: 0x1, 0x8000000000000000 -> not taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jgt r8, r9, .Ltaken11
  or64 r3, 0x40
  ja .Lend11
.Ltaken11:
  or64 r3, 0x80
.Lend11:
  # jge imm: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  jge r8, 1, .Ltaken12
  or64 r3, 0x100
  ja .Lend12
.Ltaken12:
  or64 r3, 0x200
.Lend12:
  # jge imm: 0x80000000, 0xffffffff80000000 -> not taken
  lddw r8, 0x80000000
  jge r8, -2147483648, .Ltaken13
  or64 r3, 0x400
  ja .Lend13
.Ltaken13:
  or64 r3, 0x800
.Lend13:
  # jge reg: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jge r8, r9, .Ltaken14
  or64 r3, 0x1000
  ja .Lend14
.Ltaken14:
  or64 r3, 0x2000
.Lend14:
  # jge reg: 0x1, 0x8000000000000000 -> not taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jge r8, r9, .Ltaken15
  or64 r3, 0x4000
  ja .Lend15
.Ltaken15:
  or64 r3, 0x8000
.Lend15:
  # jlt imm: 0x80000000, 0xffffffff80000000 -> taken
  lddw r8, 0x80000000
  jlt r8, -2147483648, .Ltaken16
  or64 r4, 0x1
  ja .Lend16
.Ltaken16:
  or64 r4, 0x2
.Lend16:
  # jlt imm: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  jlt r8, 1, .Ltaken17
  or64 r4, 0x4
  ja .Lend17
.Ltaken17:
  or64 r4, 0x8
.Lend17:
  # jlt reg: 0x1, 0x8000000000000000 -> taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jlt r8, r9, .Ltaken18
  or64 r4, 0x10
  ja .Lend18
.Ltaken18:
  or64 r4, 0x20
.Lend18:
  # jlt reg: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jlt r8, r9, .Ltaken19
  or64 r4, 0x40
  ja .Lend19
.Ltaken19:
  or64 r4, 0x80
.Lend19:
  # jle imm: 0xffffffff, 0xfffffffffffffffe -> taken
  lddw r8, 0xffffffff
  jle r8, -2, .Ltaken20
  or64 r4, 0x100
  ja .Lend20
.Ltaken20:
  or64 r4, 0x200
.Lend20:
  # jle imm: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  jle r8, 1, .Ltaken21
  or64 r4, 0x400
  ja .Lend21
.Ltaken21:
  or64 r4, 0x800
.Lend21:
  # jle reg: 0x1, 0x8000000000000000 -> taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jle r8, r9, .Ltaken22
  or64 r4, 0x1000
  ja .Lend22
.Ltaken22:
  or64 r4, 0x2000
.Lend22:
  # jle reg: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jle r8, r9, .Ltaken23
  or64 r4, 0x4000
  ja .Lend23
.Ltaken23:
  or64 r4, 0x8000
.Lend23:
  # jsgt imm: 0x1, 0xffffffffffffffff -> taken
  lddw r8, 0x1
  jsgt r8, -1, .Ltaken24
  or64 r5, 0x1
  ja .Lend24
.Ltaken24:
  or64 r5, 0x2
.Lend24:
  # jsgt imm: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  jsgt r8, 1, .Ltaken25
  or64 r5, 0x4
  ja .Lend25
.Ltaken25:
  or64 r5, 0x8
.Lend25:
  # jsgt reg: 0x1, 0x8000000000000000 -> taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jsgt r8, r9, .Ltaken26
  or64 r5, 0x10
  ja .Lend26
.Ltaken26:
  or64 r5, 0x20
.Lend26:
  # jsgt reg: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jsgt r8, r9, .Ltaken27
  or64 r5, 0x40
  ja .Lend27
.Ltaken27:
  or64 r5, 0x80
.Lend27:
  # jsge imm: 0x1, 0xffffffffffffffff -> taken
  lddw r8, 0x1
  jsge r8, -1, .Ltaken28
  or64 r5, 0x100
  ja .Lend28
.Ltaken28:
  or64 r5, 0x200
.Lend28:
  # jsge imm: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  jsge r8, 1, .Ltaken29
  or64 r5, 0x400
  ja .Lend29
.Ltaken29:
  or64 r5, 0x800
.Lend29:
  # jsge reg: 0x1, 0x8000000000000000 -> taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jsge r8, r9, .Ltaken30
  or64 r5, 0x1000
  ja .Lend30
.Ltaken30:
  or64 r5, 0x2000
.Lend30:
  # jsge reg: 0x8000000000000000, 0x1 -> not taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jsge r8, r9, .Ltaken31
  or64 r5, 0x4000
  ja .Lend31
.Ltaken31:
  or64 r5, 0x8000
.Lend31:
  # jslt imm: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  jslt r8, 1, .Ltaken32
  or64 r6, 0x1
  ja .Lend32
.Ltaken32:
  or64 r6, 0x2
.Lend32:
  # jslt imm: 0x1, 0xffffffffffffffff -> not taken
  lddw r8, 0x1
  jslt r8, -1, .Ltaken33
  or64 r6, 0x4
  ja .Lend33
.Ltaken33:
  or64 r6, 0x8
.Lend33:
  # jslt reg: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jslt r8, r9, .Ltaken34
  or64 r6, 0x10
  ja .Lend34
.Ltaken34:
  or64 r6, 0x20
.Lend34:
  # jslt reg: 0x1, 0x8000000000000000 -> not taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jslt r8, r9, .Ltaken35
  or64 r6, 0x40
  ja .Lend35
.Ltaken35:
  or64 r6, 0x80
.Lend35:
  # jsle imm: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  jsle r8, 1, .Ltaken36
  or64 r6, 0x100
  ja .Lend36
.Ltaken36:
  or64 r6, 0x200
.Lend36:
  # jsle imm: 0x1, 0xffffffffffffffff -> not taken
  lddw r8, 0x1
  jsle r8, -1, .Ltaken37
  or64 r6, 0x400
  ja .Lend37
.Ltaken37:
  or64 r6, 0x800
.Lend37:
  # jsle reg: 0x8000000000000000, 0x1 -> taken
  lddw r8, 0x8000000000000000
  lddw r9, 0x1
  jsle r8, r9, .Ltaken38
  or64 r6, 0x1000
  ja .Lend38
.Ltaken38:
  or64 r6, 0x2000
.Lend38:
  # jsle reg: 0x1, 0x8000000000000000 -> not taken
  lddw r8, 0x1
  lddw r9, 0x8000000000000000
  jsle r8, r9, .Ltaken39
  or64 r6, 0x4000
  ja .Lend39
.Ltaken39:
  or64 r6, 0x8000
.Lend39:
  # jset imm: 0x300000000, 0xffffffffffffffff -> taken
  lddw r8, 0x300000000
  .byte 0x45, 0x08, 0x02, 0x00, 0xff, 0xff, 0xff, 0xff  # jset +2
  or64 r7, 0x1
  ja .Lend40
  or64 r7, 0x2
.Lend40:
  # jset imm: 0x1, 0x6 -> not taken
  lddw r8, 0x1
  .byte 0x45, 0x08, 0x02, 0x00, 0x06, 0x00, 0x00, 0x00  # jset +2
  or64 r7, 0x4
  ja .Lend41
  or64 r7, 0x8
.Lend41:
  # jset reg: 0x1, 0x1 -> taken
  lddw r8, 0x1
  lddw r9, 0x1
  .byte 0x4d, 0x98, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00  # jset +2
  or64 r7, 0x10
  ja .Lend42
  or64 r7, 0x20
.Lend42:
  # jset reg: 0x1, 0x6 -> not taken
  lddw r8, 0x1
  lddw r9, 0x6
  .byte 0x4d, 0x98, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00  # jset +2
  or64 r7, 0x40
  ja .Lend43
  or64 r7, 0x80
.Lend43:
  mov64 r0, 0
  exit
