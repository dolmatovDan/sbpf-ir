# Memory: the input region through r1 (header) and r2 (instruction data, then the program id right after it),
# the stack through r10 with stores and loads of every width overlapping each other, and the heap.
# Every store is read back into a register before exit: the runner returns registers, not memory.
# The example passes instruction_data 11 22 33 44 55 66 77 88 99 (9 bytes); r0 collects the reads that must be 0.
.globl entrypoint
entrypoint:
# header and instruction data: account count (0) into r0, instruction data into r7 and r8, the program id
# after it into r9, instruction data length into r1 (last, r1 is the base until then)
  ldxdw r0, [r1+0]
  ldxdw r7, [r2+0]
  ldxb r8, [r2+8]
  ldxdw r9, [r2+9]
  ldxdw r1, [r1+8]
# heap: zero before the first store, a 32-bit store read back as 64 bits
  lddw r2, 0x300000100
  ldxdw r3, [r2+0]
  add64 r0, r3
  stw [r2+0], 0xcafe
  ldxdw r2, [r2+0]
# stack: a 64-bit store, narrower loads inside it, a 16-bit store over it, narrow stores read as one word
  lddw r6, 0x1122334455667788
  stxdw [r10-8], r6
  ldxh r4, [r10-8]
  ldxw r5, [r10-6]
  sth [r10-4], 0xabcd
  ldxdw r6, [r10-8]
  stb [r10-16], 0x7f
  stxh [r10-14], r4
  stxw [r10-12], r5
  ldxdw r3, [r10-16]
  exit
