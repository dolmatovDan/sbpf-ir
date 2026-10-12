fn_76:
  bb_76:
       76: mov64 r0, 1
       77: mov64 r3, 1
       78: jslt r1, r2, lbb_80
  bb_79:
       79: mov64 r3, 0
  bb_80:
       80: jsgt r1, r2, lbb_82
  bb_81:
       81: mov64 r0, 0
  bb_82:
       82: sub64 r0, r3
       83: exit

fn_84:
  bb_84:
       84: jeq r3, 0, lbb_90
  bb_85:
       85: div64 r2, r3
       86: stxdw [r1+0x8], r2
       87: mov64 r2, 0
  bb_88:
       88: stxw [r1+0x0], r2
       89: exit
  bb_90:
       90: stw [r1+0x4], 23
       91: mov64 r2, 1
       92: ja lbb_88

fn_173:
  bb_173:
      173: mov64 r0, r2
      174: add64 r0, r1
      175: mul64 r0, 3
      176: lsh64 r1, 5
      177: xor64 r0, r1
      178: lddw r1, 0xfffffffe
      180: and64 r2, r1
      181: rsh64 r2, 1
      182: or64 r0, r2
      183: exit

fn_769:
  bb_769:
      769: ldxdw r1, [r1+0x0]
      770: jeq r1, -1, lbb_778
  bb_771:
      771: ldxdw r2, [r1+0x8]
      772: add64 r2, -1
      773: stxdw [r1+0x8], r2
      774: jne r2, 0, lbb_778
  bb_775:
      775: mov64 r2, 40
      776: mov64 r3, 8
      777: call fn_34
  bb_778:
      778: exit

fn_845:
  bb_845:
      845: lddw r1, 0xc8
      847: mov64 r2, 46
      848: syscall sol_log_
  bb_849:
      849: call fn_850
