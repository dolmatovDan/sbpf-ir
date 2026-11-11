# Loops: a do-while with a backward conditional jump, a while with the test at the top and ja back,
# and a nested loop. All registers are written before the first jump.
.globl entrypoint
entrypoint:
  mov64 r1, 0
  mov64 r2, 0
  mov64 r3, 1
  mov64 r4, 0
  mov64 r5, 0
  mov64 r6, 0
  mov64 r7, 0
  mov64 r8, 0
  mov64 r9, 0
  mov64 r0, 0
# do { r1 += 1; r2 += r1; r3 *= r1 } while (r1 < 10): r2 = 55, r3 = 10!
.Ldo:
  add64 r1, 1
  add64 r2, r1
  mul64 r3, r1
  jlt r1, 10, .Ldo
# while (r4 != 7) { r4 += 1; r8 += r4 }: r8 = 28
.Lwhile:
  jeq r4, 7, .Lwhile_end
  add64 r4, 1
  add64 r8, r4
  ja .Lwhile
.Lwhile_end:
# for (r6 = 0; r6 < 3; r6++) for (r7 = 0; r7 != 4; r7++) { r5 += 1; r9 += r6 * r7 }: r5 = 12, r9 = 18
.Louter:
  mov64 r7, 0
.Linner:
  add64 r5, 1
  mov64 r0, r6
  mul64 r0, r7
  add64 r9, r0
  add64 r7, 1
  jne r7, 4, .Linner
  add64 r6, 1
  jslt r6, 3, .Louter
  mov64 r0, 0
  exit
