fn_0:
  bb_0:
        0: lddw r3, 0x300000000
        2: ldxdw r3, [r3+0x0]
        3: lddw r4, 0x300008000
        5: jeq r3, 0, lbb_7
  bb_6:
        6: mov64 r4, r3
  bb_7:
        7: mov64 r3, r4
        8: sub64 r3, r1
        9: mov64 r0, 0
       10: mov64 r5, 1
       11: jle r3, r4, lbb_24
  bb_12:
       12: mov64 r1, 0
       13: jeq r5, 0, lbb_27
  bb_14:
       14: neg64 r2
       15: and64 r1, r2
       16: lddw r2, 0x300000008
       18: jlt r1, r2, lbb_23
  bb_19:
       19: lddw r2, 0x300000000
       21: stxdw [r2+0x0], r1
       22: mov64 r0, r1
  bb_23:
       23: exit
  bb_24:
       24: mov64 r5, 0
       25: mov64 r1, 0
       26: jne r5, 0, lbb_14
  bb_27:
       27: mov64 r1, r3
       28: neg64 r2
       29: and64 r1, r2
       30: lddw r2, 0x300000008
       32: jge r1, r2, lbb_19
  bb_33:
       33: ja lbb_23

fn_34:
  bb_34:
       34: exit

fn_35:
  bb_35:
       35: mov64 r5, r1
       36: lddw r1, 0x300000000
       38: ldxdw r1, [r1+0x0]
       39: lddw r6, 0x300008000
       41: jeq r1, 0, lbb_43
  bb_42:
       42: mov64 r6, r1
  bb_43:
       43: mov64 r1, r6
       44: sub64 r1, r4
       45: mov64 r0, 0
       46: mov64 r7, 1
       47: jle r1, r6, lbb_56
  bb_48:
       48: mov64 r6, 0
       49: jeq r7, 0, lbb_59
  bb_50:
       50: neg64 r3
       51: and64 r6, r3
       52: lddw r1, 0x300000008
       54: jge r6, r1, lbb_65
  bb_55:
       55: ja lbb_75
  bb_56:
       56: mov64 r7, 0
       57: mov64 r6, 0
       58: jne r7, 0, lbb_50
  bb_59:
       59: mov64 r6, r1
       60: neg64 r3
       61: and64 r6, r3
       62: lddw r1, 0x300000008
       64: jlt r6, r1, lbb_75
  bb_65:
       65: lddw r1, 0x300000000
       67: stxdw [r1+0x0], r6
       68: jlt r4, r2, lbb_70
  bb_69:
       69: mov64 r4, r2
  bb_70:
       70: mov64 r1, r6
       71: mov64 r2, r5
       72: mov64 r3, r4
       73: call fn_2072
  bb_74:
       74: mov64 r0, r6
  bb_75:
       75: exit

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

fn_93:
  bb_93:
       93: mov64 r0, 2
       94: jeq r5, 0, lbb_113
  bb_95:
       95: ldxb r6, [r4+0x0]
       96: stxb [r10-0xfc0], r6
       97: jlt r5, 9, lbb_172
  bb_98:
       98: jlt r5, 17, lbb_172
  bb_99:
       99: ldxdw r2, [r4+0x1]
      100: ldxdw r3, [r4+0x9]
      101: jsgt r6, 2, lbb_115
  bb_102:
      102: jeq r6, 0, lbb_128
  bb_103:
      103: jeq r6, 1, lbb_136
  bb_104:
      104: jne r6, 2, lbb_172
  bb_105:
      105: mov64 r1, r10
      106: add64 r1, -4096
      107: call fn_84
  bb_108:
      108: ldxw r1, [r10-0x1000]
      109: jeq r1, 0, lbb_147
  bb_110:
      110: ldxw r1, [r10-0xff8]
      111: ldxw r0, [r10-0xffc]
      112: ja lbb_172
  bb_113:
      113: mov64 r1, 0
      114: ja lbb_172
  bb_115:
      115: jeq r6, 3, lbb_132
  bb_116:
      116: jeq r6, 4, lbb_142
  bb_117:
      117: jne r6, 5, lbb_172
  bb_118:
      118: lsh64 r3, 32
      119: arsh64 r3, 32
      120: jsgt r3, 1, lbb_122
  bb_121:
      121: mov64 r3, 1
  bb_122:
      122: lsh64 r2, 32
      123: arsh64 r2, 32
      124: mov64 r1, r2
      125: mov64 r2, r3
      126: call fn_2119
  bb_127:
      127: ja lbb_148
  bb_128:
      128: mov64 r1, r2
      129: mov64 r2, r3
      130: call fn_184
  bb_131:
      131: ja lbb_148
  bb_132:
      132: mov64 r1, r2
      133: mov64 r2, r3
      134: call fn_76
  bb_135:
      135: ja lbb_148
  bb_136:
      136: mov64 r1, r2
      137: mov64 r2, r3
      138: call fn_173
  bb_139:
      139: lsh64 r0, 32
      140: rsh64 r0, 32
      141: ja lbb_148
  bb_142:
      142: jlt r2, 1000, lbb_144
  bb_143:
      143: mov64 r2, 1000
  bb_144:
      144: mov64 r1, r2
      145: call fn_195
  bb_146:
      146: ja lbb_148
  bb_147:
      147: ldxdw r0, [r10-0xff8]
  bb_148:
      148: stxdw [r10-0xfc8], r0
      149: lddw r1, 0x100003de8
      151: stxdw [r10-0xfe8], r1
      152: mov64 r1, r10
      153: add64 r1, -4040
      154: stxdw [r10-0xff0], r1
      155: lddw r1, 0x100003c20
      157: stxdw [r10-0xff8], r1
      158: mov64 r1, r10
      159: add64 r1, -4032
      160: stxdw [r10-0x1000], r1
      161: mov64 r1, r10
      162: add64 r1, -4064
      163: mov64 r3, r10
      164: add64 r3, -4096
      165: lddw r2, 0xb8
      167: call fn_945
  bb_168:
      168: ldxdw r1, [r10-0xfd8]
      169: ldxdw r2, [r10-0xfd0]
      170: syscall sol_log_
  bb_171:
      171: mov64 r0, 26
  bb_172:
      172: exit

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

fn_184:
  bb_184:
      184: mov64 r0, r2
      185: mul64 r0, r1
      186: rsh64 r1, 3
      187: xor64 r0, r1
      188: arsh64 r2, 2
      189: add64 r0, r2
      190: mov64 r1, r0
      191: rsh64 r1, 57
      192: lsh64 r0, 7
      193: or64 r0, r1
      194: exit

fn_195:
  bb_195:
      195: mov64 r6, r1
      196: jeq r6, 0, lbb_213
  bb_197:
      197: mov64 r8, 0
      198: mov64 r7, 0
  bb_199:
      199: mov64 r1, r7
      200: mov64 r2, 0
      201: mov64 r3, r7
      202: mov64 r4, 0
      203: call fn_2076
  bb_204:
      204: mov64 r2, 1
      205: jne r1, 0, lbb_207
  bb_206:
      206: mov64 r2, 0
  bb_207:
      207: jeq r2, 1, lbb_215
  bb_208:
      208: add64 r0, r8
      209: add64 r7, 1
      210: mov64 r8, r0
      211: jlt r7, r6, lbb_199
  bb_212:
      212: ja lbb_214
  bb_213:
      213: mov64 r0, 0
  bb_214:
      214: exit
  bb_215:
      215: lddw r1, 0x280
      217: call fn_1896

fn_218:
  bb_218:
      218: mov64 r6, r1
      219: ldxdw r1, [r6+0x0]
      220: ldxdw r2, [r1+0x8]
      221: mov64 r3, r2
      222: and64 r3, 1
      223: jeq r3, 0, lbb_227
  bb_224:
      224: ldxdw r1, [r1+0x0]
      225: rsh64 r2, 1
      226: syscall sol_log_
  bb_227:
      227: ldxdw r1, [r6+0x8]
      228: ldxw r4, [r1+0x14]
      229: ldxw r3, [r1+0x10]
      230: ldxdw r2, [r1+0x8]
      231: ldxdw r1, [r1+0x0]
      232: syscall sol_panic_

entrypoint:
  bb_233:
      233: mov64 r2, r1
      234: mov64 r1, r10
      235: add64 r1, -4096
      236: call fn_378
  bb_237:
      237: ldxdw r8, [r10-0xff0]
      238: ldxdw r6, [r10-0xff8]
      239: ldxdw r5, [r10-0xfd8]
      240: ldxdw r4, [r10-0xfe0]
      241: call fn_93
  bb_242:
      242: mov64 r7, 0
      243: lsh64 r0, 32
      244: rsh64 r0, 32
      245: jne r0, 26, lbb_270
  bb_246:
      246: jeq r8, 0, lbb_268
  bb_247:
      247: add64 r6, 16
      248: ja lbb_252
  bb_249:
      249: add64 r6, 48
      250: add64 r8, -1
      251: jeq r8, 0, lbb_268
  bb_252:
      252: ldxdw r1, [r6-0x8]
      253: ldxdw r2, [r1+0x0]
      254: add64 r2, -1
      255: stxdw [r1+0x0], r2
      256: jne r2, 0, lbb_260
  bb_257:
      257: mov64 r1, r6
      258: add64 r1, -8
      259: call fn_779
  bb_260:
      260: ldxdw r1, [r6+0x0]
      261: ldxdw r2, [r1+0x0]
      262: add64 r2, -1
      263: stxdw [r1+0x0], r2
      264: jne r2, 0, lbb_249
  bb_265:
      265: mov64 r1, r6
      266: call fn_769
  bb_267:
      267: ja lbb_249
  bb_268:
      268: mov64 r0, r7
      269: exit
  bb_270:
      270: jsgt r0, 12, lbb_278
  bb_271:
      271: jsle r0, 5, lbb_285
  bb_272:
      272: jsle r0, 8, lbb_299
  bb_273:
      273: jsgt r0, 10, lbb_319
  bb_274:
      274: jne r0, 9, lbb_353
  bb_275:
      275: lddw r7, 0xa00000000
      277: ja lbb_246
  bb_278:
      278: jsle r0, 18, lbb_293
  bb_279:
      279: jsle r0, 21, lbb_304
  bb_280:
      280: jsgt r0, 23, lbb_323
  bb_281:
      281: jne r0, 22, lbb_356
  bb_282:
      282: lddw r7, 0x1700000000
      284: ja lbb_246
  bb_285:
      285: jsgt r0, 2, lbb_309
  bb_286:
      286: jeq r0, 0, lbb_333
  bb_287:
      287: lddw r7, 0x200000000
      289: jeq r0, 1, lbb_246
  bb_290:
      290: lddw r7, 0x300000000
      292: ja lbb_246
  bb_293:
      293: jsgt r0, 15, lbb_314
  bb_294:
      294: jeq r0, 13, lbb_338
  bb_295:
      295: jne r0, 14, lbb_365
  bb_296:
      296: lddw r7, 0xf00000000
      298: ja lbb_246
  bb_299:
      299: jeq r0, 6, lbb_327
  bb_300:
      300: jne r0, 7, lbb_347
  bb_301:
      301: lddw r7, 0x800000000
      303: ja lbb_246
  bb_304:
      304: jeq r0, 19, lbb_330
  bb_305:
      305: jne r0, 20, lbb_350
  bb_306:
      306: lddw r7, 0x1500000000
      308: ja lbb_246
  bb_309:
      309: jeq r0, 3, lbb_341
  bb_310:
      310: jne r0, 4, lbb_368
  bb_311:
      311: lddw r7, 0x500000000
      313: ja lbb_246
  bb_314:
      314: jeq r0, 16, lbb_344
  bb_315:
      315: jne r0, 17, lbb_371
  bb_316:
      316: lddw r7, 0x1200000000
      318: ja lbb_246
  bb_319:
      319: jne r0, 11, lbb_359
  bb_320:
      320: lddw r7, 0xc00000000
      322: ja lbb_246
  bb_323:
      323: jne r0, 24, lbb_362
  bb_324:
      324: lddw r7, 0x1900000000
      326: ja lbb_246
  bb_327:
      327: lddw r7, 0x700000000
      329: ja lbb_246
  bb_330:
      330: lddw r7, 0x1400000000
      332: ja lbb_246
  bb_333:
      333: lsh64 r1, 32
      334: rsh64 r1, 32
      335: jeq r1, 0, lbb_374
  bb_336:
      336: mov64 r7, r1
      337: ja lbb_246
  bb_338:
      338: lddw r7, 0xe00000000
      340: ja lbb_246
  bb_341:
      341: lddw r7, 0x400000000
      343: ja lbb_246
  bb_344:
      344: lddw r7, 0x1100000000
      346: ja lbb_246
  bb_347:
      347: lddw r7, 0x900000000
      349: ja lbb_246
  bb_350:
      350: lddw r7, 0x1600000000
      352: ja lbb_246
  bb_353:
      353: lddw r7, 0xb00000000
      355: ja lbb_246
  bb_356:
      356: lddw r7, 0x1800000000
      358: ja lbb_246
  bb_359:
      359: lddw r7, 0xd00000000
      361: ja lbb_246
  bb_362:
      362: lddw r7, 0x1a00000000
      364: ja lbb_246
  bb_365:
      365: lddw r7, 0x1000000000
      367: ja lbb_246
  bb_368:
      368: lddw r7, 0x600000000
      370: ja lbb_246
  bb_371:
      371: lddw r7, 0x1300000000
      373: ja lbb_246
  bb_374:
      374: lddw r7, 0x100000000
      376: ja lbb_246

fn_377:
  bb_377:
      377: exit

fn_378:
  bb_378:
      378: mov64 r6, r2
      379: stxdw [r10-0xf70], r1
      380: mov64 r7, 0
      381: ldxdw r4, [r6+0x0]
      382: mov64 r8, r4
      383: mul64 r8, 48
      384: lddw r1, 0x2aaaaaaaaaaaaaa
      386: jle r4, r1, lbb_390
  bb_387:
      387: mov64 r1, r7
      388: mov64 r2, r8
      389: call fn_936
  bb_390:
      390: stxdw [r10-0xf78], r4
      391: jeq r8, 0, lbb_401
  bb_392:
      392: call fn_377
  bb_393:
      393: mov64 r7, 8
      394: mov64 r1, r8
      395: mov64 r2, 8
      396: call fn_0
  bb_397:
      397: ldxdw r4, [r10-0xf78]
      398: jeq r0, 0, lbb_387
  bb_399:
      399: mov64 r1, r4
      400: ja lbb_403
  bb_401:
      401: mov64 r0, 8
      402: mov64 r1, 0
  bb_403:
      403: stxdw [r10-0xfc8], r0
      404: stxdw [r10-0xfd0], r1
      405: stdw [r10-0xfc0], 0
      406: mov64 r9, 8
      407: jeq r4, 0, lbb_616
  bb_408:
      408: mov64 r2, 0
      409: mov64 r8, 42
      410: stxdw [r10-0xfb0], r6
      411: ja lbb_432
  bb_412:
      412: mov64 r1, r0
      413: add64 r1, r8
      414: ldxdw r5, [r10-0xf88]
      415: stxb [r1+0x0], r5
      416: ldxdw r5, [r10-0xf90]
      417: stxb [r1-0x1], r5
      418: ldxdw r5, [r10-0xf98]
      419: stxb [r1-0x2], r5
      420: stxdw [r1-0xa], r6
      421: stxdw [r1-0x12], r3
      422: stxdw [r1-0x1a], r7
      423: ldxdw r3, [r10-0xfa0]
      424: stxdw [r1-0x22], r3
      425: stxdw [r1-0x2a], r9
      426: ldxdw r9, [r10-0xfb8]
      427: ldxdw r6, [r10-0xfb0]
  bb_428:
      428: add64 r8, 48
      429: add64 r2, 1
      430: stxdw [r10-0xfc0], r2
      431: jge r2, r4, lbb_612
  bb_432:
      432: mov64 r3, r9
      433: add64 r3, 1
      434: mov64 r1, 1
      435: jeq r3, 0, lbb_437
  bb_436:
      436: mov64 r1, 0
  bb_437:
      437: and64 r1, 1
      438: jne r1, 0, lbb_642
  bb_439:
      439: mov64 r1, r6
      440: add64 r1, r9
      441: ldxb r1, [r1+0x0]
      442: jeq r1, 255, lbb_492
  bb_443:
      443: add64 r9, 8
      444: jlt r9, r3, lbb_648
  bb_445:
      445: jle r2, r1, lbb_651
  bb_446:
      446: stxdw [r10-0xfb8], r9
      447: mul64 r1, 48
      448: stxdw [r10-0xfa8], r0
      449: mov64 r5, r0
      450: add64 r5, r1
      451: ldxdw r0, [r5+0x8]
      452: ldxdw r1, [r0+0x0]
      453: add64 r1, 1
      454: mov64 r3, 1
      455: jeq r1, 0, lbb_457
  bb_456:
      456: mov64 r3, 0
  bb_457:
      457: ldxdw r9, [r5+0x0]
      458: stxdw [r0+0x0], r1
      459: jeq r3, 1, lbb_641
  bb_460:
      460: ldxdw r7, [r5+0x10]
      461: ldxdw r1, [r7+0x0]
      462: add64 r1, 1
      463: mov64 r3, 1
      464: jeq r1, 0, lbb_466
  bb_465:
      465: mov64 r3, 0
  bb_466:
      466: stxdw [r7+0x0], r1
      467: jeq r3, 1, lbb_641
  bb_468:
      468: stxdw [r10-0xfa0], r0
      469: ldxb r1, [r5+0x2a]
      470: stxdw [r10-0xf88], r1
      471: ldxb r1, [r5+0x29]
      472: stxdw [r10-0xf90], r1
      473: ldxb r1, [r5+0x28]
      474: stxdw [r10-0xf98], r1
      475: ldxdw r6, [r5+0x20]
      476: ldxdw r3, [r5+0x18]
      477: ldxdw r1, [r10-0xfd0]
      478: ldxdw r0, [r10-0xfa8]
      479: jne r2, r1, lbb_412
  bb_480:
      480: mov64 r1, r10
      481: add64 r1, -4048
      482: stxdw [r10-0xfa8], r2
      483: stxdw [r10-0xf80], r8
      484: mov64 r8, r3
      485: call fn_690
  bb_486:
      486: mov64 r3, r8
      487: ldxdw r8, [r10-0xf80]
      488: ldxdw r2, [r10-0xfa8]
      489: ldxdw r4, [r10-0xf78]
      490: ldxdw r0, [r10-0xfc8]
      491: ja lbb_412
  bb_492:
      492: jeq r3, -1, lbb_669
  bb_493:
      493: mov64 r4, r9
      494: add64 r4, 2
      495: mov64 r5, r9
      496: add64 r5, 3
      497: jlt r5, r4, lbb_681
  bb_498:
      498: mov64 r7, r9
      499: add64 r7, 4
      500: jlt r7, r5, lbb_657
  bb_501:
      501: mov64 r1, r9
      502: add64 r1, 8
      503: jlt r1, r7, lbb_678
  bb_504:
      504: stxdw [r10-0xf80], r8
      505: stxdw [r10-0xfa8], r2
      506: mov64 r2, r9
      507: add64 r2, 40
      508: jlt r2, r1, lbb_672
  bb_509:
      509: mov64 r0, r9
      510: add64 r0, 72
      511: jlt r0, r2, lbb_660
  bb_512:
      512: mov64 r6, r9
      513: add64 r6, 80
      514: jlt r6, r0, lbb_663
  bb_515:
      515: mov64 r8, r1
      516: add64 r9, 88
      517: stxdw [r10-0xfb8], r9
      518: jlt r9, r6, lbb_675
  bb_519:
      519: ldxdw r1, [r10-0xfb0]
      520: mov64 r9, r1
      521: add64 r9, r3
      522: ldxb r3, [r9+0x0]
      523: mov64 r9, r1
      524: add64 r9, r4
      525: ldxb r4, [r9+0x0]
      526: stxdw [r10-0xfa0], r4
      527: mov64 r4, r1
      528: add64 r4, r5
      529: ldxb r9, [r4+0x0]
      530: mov64 r4, r1
      531: add64 r4, r6
      532: ldxdw r5, [r4+0x0]
      533: add64 r1, r7
      534: stxw [r1+0x0], r5
      535: jgt r5, -10241, lbb_645
  bb_536:
      536: mov64 r7, r5
      537: add64 r7, 10248
      538: mov64 r4, r5
      539: add64 r4, 10240
      540: jlt r7, r4, lbb_645
  bb_541:
      541: mov64 r6, r7
      542: ldxdw r1, [r10-0xfb8]
      543: add64 r6, r1
      544: mov64 r4, 1
      545: mov64 r1, r8
      546: jlt r6, r7, lbb_548
  bb_547:
      547: mov64 r4, 0
  bb_548:
      548: and64 r4, 1
      549: jne r4, 0, lbb_666
  bb_550:
      550: mov64 r7, r6
      551: add64 r7, 7
      552: and64 r7, -8
      553: jlt r7, r6, lbb_654
  bb_554:
      554: mov64 r4, r9
      555: mov64 r8, 1
      556: mov64 r9, 1
      557: jeq r4, 0, lbb_599
  bb_558:
      558: mov64 r4, r3
      559: mov64 r3, 1
      560: ldxdw r6, [r10-0xfb0]
      561: jeq r4, 0, lbb_604
  bb_562:
      562: ldxdw r4, [r10-0xfa0]
      563: jne r4, 0, lbb_565
  bb_564:
      564: mov64 r8, 0
  bb_565:
      565: stxdw [r10+0x8], r5
      566: mov64 r4, r6
      567: add64 r4, r2
      568: stxdw [r10+0x10], r4
      569: mov64 r2, r6
      570: ldxdw r4, [r10-0xfb8]
      571: add64 r2, r4
      572: stxdw [r10+0x0], r2
      573: stxdw [r10+0x18], r9
      574: mov64 r2, r6
      575: add64 r2, r1
      576: mov64 r5, r6
      577: add64 r5, r0
      578: mov64 r1, r10
      579: add64 r1, -4096
      580: mov64 r4, r8
      581: call fn_789
  bb_582:
      582: ldxdw r1, [r10-0xfd0]
      583: ldxdw r2, [r10-0xfa8]
      584: jeq r2, r1, lbb_608
  bb_585:
      585: ldxdw r9, [r10-0xfc8]
      586: mov64 r1, r9
      587: ldxdw r8, [r10-0xf80]
      588: add64 r1, r8
      589: add64 r1, -42
      590: mov64 r2, r10
      591: add64 r2, -4096
      592: mov64 r3, 48
      593: call fn_2072
  bb_594:
      594: mov64 r0, r9
      595: mov64 r9, r7
      596: ldxdw r4, [r10-0xf78]
      597: ldxdw r2, [r10-0xfa8]
      598: ja lbb_428
  bb_599:
      599: mov64 r9, 0
      600: mov64 r4, r3
      601: mov64 r3, 1
      602: ldxdw r6, [r10-0xfb0]
      603: jne r4, 0, lbb_562
  bb_604:
      604: mov64 r3, 0
      605: ldxdw r4, [r10-0xfa0]
      606: jeq r4, 0, lbb_564
  bb_607:
      607: ja lbb_565
  bb_608:
      608: mov64 r1, r10
      609: add64 r1, -4048
      610: call fn_690
  bb_611:
      611: ja lbb_585
  bb_612:
      612: jgt r9, -9, lbb_687
  bb_613:
      613: mov64 r1, r6
      614: add64 r1, r9
      615: ja lbb_618
  bb_616:
      616: mov64 r1, r6
      617: add64 r1, 8
  bb_618:
      618: add64 r9, 8
      619: ldxdw r2, [r1+0x0]
      620: mov64 r1, r2
      621: add64 r1, r9
      622: mov64 r3, 1
      623: jlt r1, r2, lbb_625
  bb_624:
      624: mov64 r3, 0
  bb_625:
      625: and64 r3, 1
      626: jne r3, 0, lbb_684
  bb_627:
      627: ldxdw r3, [r10-0xfc0]
      628: ldxdw r4, [r10-0xf70]
      629: stxdw [r4+0x10], r3
      630: ldxdw r3, [r10-0xfc8]
      631: stxdw [r4+0x8], r3
      632: ldxdw r3, [r10-0xfd0]
      633: stxdw [r4+0x0], r3
      634: stxdw [r4+0x28], r2
      635: mov64 r2, r6
      636: add64 r2, r9
      637: stxdw [r4+0x20], r2
      638: add64 r6, r1
      639: stxdw [r4+0x18], r6
      640: exit
  bb_641:
      641: call fn_854
  bb_642:
      642: lddw r1, 0x2c8
      644: call fn_1891
  bb_645:
      645: lddw r1, 0x3d0
      647: call fn_1891
  bb_648:
      648: lddw r1, 0x2e0
      650: call fn_1891
  bb_651:
      651: lddw r3, 0x2f8
      653: call fn_1678
  bb_654:
      654: lddw r1, 0x400
      656: call fn_1891
  bb_657:
      657: lddw r1, 0x340
      659: call fn_1891
  bb_660:
      660: lddw r1, 0x388
      662: call fn_1891
  bb_663:
      663: lddw r1, 0x3a0
      665: call fn_1891
  bb_666:
      666: lddw r1, 0x3e8
      668: call fn_1891
  bb_669:
      669: lddw r1, 0x310
      671: call fn_1891
  bb_672:
      672: lddw r1, 0x370
      674: call fn_1891
  bb_675:
      675: lddw r1, 0x3b8
      677: call fn_1891
  bb_678:
      678: lddw r1, 0x358
      680: call fn_1891
  bb_681:
      681: lddw r1, 0x328
      683: call fn_1891
  bb_684:
      684: lddw r1, 0x2b0
      686: call fn_1891
  bb_687:
      687: lddw r1, 0x298
      689: call fn_1891

fn_690:
  bb_690:
      690: ldxdw r2, [r1+0x0]
      691: call fn_743
  bb_692:
      692: mov64 r2, r1
      693: lddw r1, 0x8000000000000001
      695: jne r0, r1, lbb_697
  bb_696:
      696: exit
  bb_697:
      697: mov64 r1, r0
      698: call fn_936

fn_699:
  bb_699:
      699: mov64 r7, r4
      700: mov64 r6, r1
      701: lddw r1, 0x2aaaaaaaaaaaaaa
      703: jle r7, r1, lbb_706
  bb_704:
      704: stdw [r6+0x8], 0
      705: ja lbb_731
  bb_706:
      706: mul64 r7, 48
      707: jeq r2, 0, lbb_719
  bb_708:
      708: mul64 r2, 48
      709: mov64 r1, r3
      710: mov64 r3, 8
      711: mov64 r4, r7
      712: call fn_35
  bb_713:
      713: mov64 r1, r6
      714: add64 r1, 16
      715: mov64 r2, r6
      716: add64 r2, 8
      717: jne r0, 0, lbb_738
  bb_718:
      718: ja lbb_729
  bb_719:
      719: jeq r7, 0, lbb_733
  bb_720:
      720: call fn_377
  bb_721:
      721: mov64 r1, r7
      722: mov64 r2, 8
      723: call fn_0
  bb_724:
      724: jne r0, 0, lbb_734
  bb_725:
      725: mov64 r1, r6
      726: add64 r1, 16
      727: mov64 r2, r6
      728: add64 r2, 8
  bb_729:
      729: stdw [r2+0x0], 8
      730: stxdw [r1+0x0], r7
  bb_731:
      731: mov64 r1, 1
      732: ja lbb_741
  bb_733:
      733: mov64 r0, 8
  bb_734:
      734: mov64 r1, r6
      735: add64 r1, 16
      736: mov64 r2, r6
      737: add64 r2, 8
  bb_738:
      738: stxdw [r2+0x0], r0
      739: stxdw [r1+0x0], r7
      740: mov64 r1, 0
  bb_741:
      741: stxdw [r6+0x0], r1
      742: exit

fn_743:
  bb_743:
      743: mov64 r7, r2
      744: mov64 r6, r1
      745: add64 r7, 1
      746: ldxdw r2, [r6+0x0]
      747: mov64 r1, r2
      748: lsh64 r1, 1
      749: jgt r7, r1, lbb_751
  bb_750:
      750: mov64 r7, r1
  bb_751:
      751: jgt r7, 4, lbb_753
  bb_752:
      752: mov64 r7, 4
  bb_753:
      753: ldxdw r3, [r6+0x8]
      754: mov64 r1, r10
      755: add64 r1, -4096
      756: mov64 r4, r7
      757: call fn_699
  bb_758:
      758: ldxdw r1, [r10-0x1000]
      759: jeq r1, 0, lbb_763
  bb_760:
      760: ldxdw r1, [r10-0xff0]
      761: ldxdw r0, [r10-0xff8]
      762: ja lbb_768
  bb_763:
      763: ldxdw r1, [r10-0xff8]
      764: stxdw [r6+0x0], r7
      765: stxdw [r6+0x8], r1
      766: lddw r0, 0x8000000000000001
  bb_768:
      768: exit

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

fn_779:
  bb_779:
      779: ldxdw r1, [r1+0x0]
      780: jeq r1, -1, lbb_788
  bb_781:
      781: ldxdw r2, [r1+0x8]
      782: add64 r2, -1
      783: stxdw [r1+0x8], r2
      784: jne r2, 0, lbb_788
  bb_785:
      785: mov64 r2, 32
      786: mov64 r3, 8
      787: call fn_34
  bb_788:
      788: exit

fn_789:
  bb_789:
      789: mov64 r8, r5
      790: stxdw [r10-0xfd8], r4
      791: stxdw [r10-0xfe0], r3
      792: mov64 r9, r2
      793: mov64 r6, r1
      794: call fn_377
  bb_795:
      795: mov64 r1, 32
      796: mov64 r2, 8
      797: call fn_0
  bb_798:
      798: mov64 r7, r0
      799: jeq r7, 0, lbb_837
  bb_800:
      800: stxdw [r10-0xfd0], r9
      801: ldxdw r1, [r10-0xfe8]
      802: stxdw [r10-0xfc0], r1
      803: ldxdw r1, [r10-0xff0]
      804: stxdw [r10-0xfc8], r1
      805: ldxdw r1, [r10-0xff8]
      806: stxdw [r10-0xfb8], r1
      807: ldxdw r9, [r10-0x1000]
      808: stxdw [r7+0x18], r8
      809: stdw [r7+0x10], 0
      810: stdw [r7+0x8], 1
      811: stdw [r7+0x0], 1
      812: call fn_377
  bb_813:
      813: mov64 r1, 40
      814: mov64 r2, 8
      815: call fn_0
  bb_816:
      816: jeq r0, 0, lbb_840
  bb_817:
      817: ldxdw r1, [r10-0xfb8]
      818: stxdw [r0+0x20], r1
      819: stxdw [r0+0x18], r9
      820: stdw [r0+0x10], 0
      821: stdw [r0+0x8], 1
      822: stdw [r0+0x0], 1
      823: ldxdw r1, [r10-0xfc0]
      824: stxb [r6+0x2a], r1
      825: ldxdw r1, [r10-0xfd8]
      826: stxb [r6+0x29], r1
      827: ldxdw r1, [r10-0xfe0]
      828: stxb [r6+0x28], r1
      829: ldxdw r1, [r10-0xfc8]
      830: stxdw [r6+0x18], r1
      831: stxdw [r6+0x10], r0
      832: stxdw [r6+0x8], r7
      833: ldxdw r1, [r10-0xfd0]
      834: stxdw [r6+0x0], r1
      835: stdw [r6+0x20], 0
      836: exit
  bb_837:
      837: mov64 r1, 8
      838: mov64 r2, 32
      839: call fn_932
  bb_840:
      840: mov64 r1, 8
      841: mov64 r2, 40
      842: call fn_932

fn_843:
  bb_843:
      843: call fn_852

fn_844:
  bb_844:
      844: call fn_845

fn_845:
  bb_845:
      845: lddw r1, 0xc8
      847: mov64 r2, 46
      848: syscall sol_log_
  bb_849:
      849: call fn_850

fn_850:
  bb_850:
      850: call fn_851

fn_851:
  bb_851:
      851: call fn_854

fn_852:
  bb_852:
      852: call fn_218
  bb_853:
      853: call fn_854

fn_854:
  bb_854:
      854: syscall abort

fn_855:
  bb_855:
      855: ldxdw r2, [r1+0x0]
      856: jeq r2, 0, lbb_860
  bb_857:
      857: ldxdw r1, [r1+0x8]
      858: mov64 r3, 1
      859: call fn_34
  bb_860:
      860: exit

fn_861:
  bb_861:
      861: mov64 r6, r1
      862: mov64 r7, r3
      863: add64 r7, r2
      864: mov64 r1, 0
      865: mov64 r2, 1
      866: jlt r7, r3, lbb_868
  bb_867:
      867: mov64 r2, 0
  bb_868:
      868: and64 r2, 1
      869: jne r2, 0, lbb_888
  bb_870:
      870: ldxdw r2, [r6+0x0]
      871: mov64 r1, r2
      872: lsh64 r1, 1
      873: jgt r7, r1, lbb_875
  bb_874:
      874: mov64 r7, r1
  bb_875:
      875: jgt r7, 8, lbb_877
  bb_876:
      876: mov64 r7, 8
  bb_877:
      877: ldxdw r3, [r6+0x8]
      878: mov64 r1, r10
      879: add64 r1, -4096
      880: mov64 r4, r7
      881: call fn_892
  bb_882:
      882: ldxdw r1, [r10-0x1000]
      883: jeq r1, 1, lbb_889
  bb_884:
      884: ldxdw r1, [r10-0xff8]
      885: stxdw [r6+0x0], r7
      886: stxdw [r6+0x8], r1
      887: exit
  bb_888:
      888: call fn_936
  bb_889:
      889: ldxdw r2, [r10-0xff0]
      890: ldxdw r1, [r10-0xff8]
      891: call fn_936

fn_892:
  bb_892:
      892: mov64 r7, r4
      893: mov64 r6, r1
      894: jslt r7, 0, lbb_906
  bb_895:
      895: jeq r2, 0, lbb_908
  bb_896:
      896: mov64 r1, r3
      897: mov64 r3, 1
      898: mov64 r4, r7
      899: call fn_35
  bb_900:
      900: mov64 r1, r6
      901: add64 r1, 16
      902: mov64 r2, r6
      903: add64 r2, 8
      904: jne r0, 0, lbb_927
  bb_905:
      905: ja lbb_918
  bb_906:
      906: stdw [r6+0x8], 0
      907: ja lbb_920
  bb_908:
      908: jeq r7, 0, lbb_922
  bb_909:
      909: call fn_377
  bb_910:
      910: mov64 r1, r7
      911: mov64 r2, 1
      912: call fn_0
  bb_913:
      913: jne r0, 0, lbb_923
  bb_914:
      914: mov64 r1, r6
      915: add64 r1, 16
      916: mov64 r2, r6
      917: add64 r2, 8
  bb_918:
      918: stdw [r2+0x0], 1
      919: stxdw [r1+0x0], r7
  bb_920:
      920: mov64 r1, 1
      921: ja lbb_930
  bb_922:
      922: mov64 r0, 1
  bb_923:
      923: mov64 r1, r6
      924: add64 r1, 16
      925: mov64 r2, r6
      926: add64 r2, 8
  bb_927:
      927: stxdw [r2+0x0], r0
      928: stxdw [r1+0x0], r7
      929: mov64 r1, 0
  bb_930:
      930: stxdw [r6+0x0], r1
      931: exit

fn_932:
  bb_932:
      932: mov64 r3, r1
      933: mov64 r1, r2
      934: mov64 r2, r3
      935: call fn_844

fn_936:
  bb_936:
      936: jne r1, 0, lbb_938
  bb_937:
      937: call fn_939
  bb_938:
      938: call fn_932

fn_939:
  bb_939:
      939: lddw r1, 0xf8
      941: mov64 r2, 35
      942: lddw r3, 0x418
      944: call fn_1695

fn_945:
  bb_945:
      945: mov64 r7, r3
      946: mov64 r3, r2
      947: mov64 r6, r1
      948: mov64 r1, r7
      949: and64 r1, 1
      950: jne r1, 0, lbb_998
  bb_951:
      951: ldxb r5, [r3+0x0]
      952: jeq r5, 0, lbb_1021
  bb_953:
      953: mov64 r4, 0
      954: stxdw [r10-0xfe0], r3
      955: mov64 r2, 0
      956: lddw r8, 0x4000000000000000
      958: ja lbb_963
  bb_959:
      959: add64 r3, r0
      960: add64 r2, r0
  bb_961:
      961: ldxb r5, [r3+0x0]
      962: jeq r5, 0, lbb_1001
  bb_963:
      963: add64 r3, 1
      964: mov64 r9, r5
      965: lsh64 r9, 56
      966: arsh64 r9, 56
      967: mov64 r0, r5
      968: jsgt r9, -1, lbb_959
  bb_969:
      969: jne r0, 128, lbb_975
  bb_970:
      970: ldxh r5, [r3+0x0]
      971: add64 r2, r5
      972: add64 r3, r5
      973: add64 r3, 2
      974: ja lbb_961
  bb_975:
      975: mov64 r0, 1
      976: jeq r2, 0, lbb_978
  bb_977:
      977: mov64 r0, 0
  bb_978:
      978: mov64 r1, r5
      979: and64 r1, 3
      980: be64 r1
      981: mov64 r9, r1
      982: lsh64 r9, 5
      983: and64 r9, r8
      984: lsh64 r1, 7
      985: or64 r9, r1
      986: or64 r0, r4
      987: rsh64 r9, 61
      988: add64 r3, r9
      989: mov64 r1, r5
      990: rsh64 r1, 1
      991: and64 r1, 2
      992: add64 r3, r1
      993: rsh64 r5, 2
      994: and64 r5, 2
      995: add64 r3, r5
      996: mov64 r4, r0
      997: ja lbb_961
  bb_998:
      998: mov64 r9, r7
      999: rsh64 r9, 1
     1000: ja lbb_1012
  bb_1001:
     1001: mov64 r9, 0
     1002: mov64 r1, 1
     1003: jlt r2, 16, lbb_1005
  bb_1004:
     1004: mov64 r1, 0
  bb_1005:
     1005: and64 r4, r1
     1006: and64 r4, 1
     1007: ldxdw r3, [r10-0xfe0]
     1008: jne r4, 0, lbb_1012
  bb_1009:
     1009: lsh64 r2, 1
     1010: jslt r2, 0, lbb_1053
  bb_1011:
     1011: mov64 r9, r2
  bb_1012:
     1012: jeq r9, 0, lbb_1021
  bb_1013:
     1013: mov64 r8, r3
     1014: call fn_377
  bb_1015:
     1015: mov64 r1, r9
     1016: mov64 r2, 1
     1017: call fn_0
  bb_1018:
     1018: jeq r0, 0, lbb_1050
  bb_1019:
     1019: mov64 r3, r8
     1020: ja lbb_1023
  bb_1021:
     1021: mov64 r0, 1
     1022: mov64 r9, 0
  bb_1023:
     1023: stxdw [r10-0xff0], r0
     1024: stxdw [r10-0xff8], r9
     1025: stdw [r10-0xfe8], 0
     1026: mov64 r1, r10
     1027: add64 r1, -4088
     1028: lddw r2, 0x430
     1030: mov64 r4, r7
     1031: call fn_1519
  bb_1032:
     1032: jne r0, 0, lbb_1040
  bb_1033:
     1033: ldxdw r1, [r10-0xfe8]
     1034: stxdw [r6+0x10], r1
     1035: ldxdw r1, [r10-0xff0]
     1036: stxdw [r6+0x8], r1
     1037: ldxdw r1, [r10-0xff8]
     1038: stxdw [r6+0x0], r1
     1039: exit
  bb_1040:
     1040: mov64 r3, r10
     1041: add64 r3, -4096
     1042: lddw r1, 0x110
     1044: mov64 r2, 86
     1045: lddw r4, 0x460
     1047: lddw r5, 0x480
     1049: call fn_1656
  bb_1050:
     1050: mov64 r1, 1
     1051: mov64 r2, r9
     1052: call fn_936
  bb_1053:
     1053: mov64 r1, 0
     1054: call fn_936

fn_1055:
  bb_1055:
     1055: mov64 r1, r2
     1056: lddw r2, 0x168
     1058: mov64 r3, 5
     1059: call fn_1513
  bb_1060:
     1060: exit

fn_1061:
  bb_1061:
     1061: mov64 r7, 1
     1062: ldxdw r6, [r1+0x10]
     1063: mov64 r9, r2
     1064: lsh64 r9, 32
     1065: rsh64 r9, 32
     1066: jlt r9, 128, lbb_1072
  bb_1067:
     1067: mov64 r7, 2
     1068: jlt r9, 2048, lbb_1072
  bb_1069:
     1069: mov64 r7, 3
     1070: jlt r9, 65536, lbb_1072
  bb_1071:
     1071: mov64 r7, 4
  bb_1072:
     1072: ldxdw r3, [r1+0x0]
     1073: sub64 r3, r6
     1074: mov64 r4, r6
     1075: jgt r7, r3, lbb_1081
  bb_1076:
     1076: ldxdw r3, [r1+0x8]
     1077: add64 r3, r4
     1078: jge r9, 128, lbb_1092
  bb_1079:
     1079: stxb [r3+0x0], r2
     1080: ja lbb_1111
  bb_1081:
     1081: mov64 r8, r1
     1082: stxdw [r10-0x1000], r2
     1083: mov64 r2, r6
     1084: mov64 r3, r7
     1085: call fn_861
  bb_1086:
     1086: ldxdw r2, [r10-0x1000]
     1087: mov64 r1, r8
     1088: ldxdw r4, [r1+0x10]
     1089: ldxdw r3, [r1+0x8]
     1090: add64 r3, r4
     1091: jlt r9, 128, lbb_1079
  bb_1092:
     1092: mov64 r5, r2
     1093: and64 r5, 63
     1094: or64 r5, -128
     1095: mov64 r4, r9
     1096: rsh64 r4, 6
     1097: jge r9, 2048, lbb_1102
  bb_1098:
     1098: stxb [r3+0x1], r5
     1099: or64 r4, 192
     1100: stxb [r3+0x0], r4
     1101: ja lbb_1111
  bb_1102:
     1102: mov64 r0, r9
     1103: rsh64 r0, 12
     1104: and64 r4, 63
     1105: or64 r4, -128
     1106: jgt r9, 65535, lbb_1115
  bb_1107:
     1107: stxb [r3+0x2], r5
     1108: stxb [r3+0x1], r4
     1109: or64 r0, 224
     1110: stxb [r3+0x0], r0
  bb_1111:
     1111: add64 r7, r6
     1112: stxdw [r1+0x10], r7
     1113: mov64 r0, 0
     1114: exit
  bb_1115:
     1115: rsh64 r2, 18
     1116: or64 r2, -16
     1117: and64 r0, 63
     1118: or64 r0, -128
     1119: stxb [r3+0x3], r5
     1120: stxb [r3+0x2], r4
     1121: stxb [r3+0x1], r0
     1122: ja lbb_1079

fn_1123:
  bb_1123:
     1123: mov64 r6, r3
     1124: mov64 r7, r1
     1125: ldxdw r8, [r7+0x10]
     1126: ldxdw r1, [r7+0x0]
     1127: sub64 r1, r8
     1128: jgt r6, r1, lbb_1138
  bb_1129:
     1129: jeq r6, 0, lbb_1134
  bb_1130:
     1130: ldxdw r1, [r7+0x8]
     1131: add64 r1, r8
     1132: mov64 r3, r6
     1133: call fn_2072
  bb_1134:
     1134: add64 r8, r6
     1135: stxdw [r7+0x10], r8
     1136: mov64 r0, 0
     1137: exit
  bb_1138:
     1138: mov64 r1, r7
     1139: mov64 r9, r2
     1140: mov64 r2, r8
     1141: mov64 r3, r6
     1142: call fn_861
  bb_1143:
     1143: mov64 r2, r9
     1144: ldxdw r8, [r7+0x10]
     1145: ja lbb_1130

fn_1146:
  bb_1146:
     1146: mov64 r4, r3
     1147: mov64 r3, r2
     1148: lddw r2, 0x430
     1150: call fn_1519
  bb_1151:
     1151: exit

fn_1152:
  bb_1152:
     1152: mov64 r0, r1
     1153: ldxdw r8, [r10-0x1000]
     1154: jeq r2, 0, lbb_1162
  bb_1155:
     1155: mov64 r2, 1114112
     1156: ldxw r6, [r0+0x10]
     1157: mov64 r1, r6
     1158: and64 r1, 2097152
     1159: jne r1, 0, lbb_1165
  bb_1160:
     1160: mov64 r9, r8
     1161: ja lbb_1168
  bb_1162:
     1162: mov64 r2, 45
     1163: ldxw r6, [r0+0x10]
     1164: ja lbb_1166
  bb_1165:
     1165: mov64 r2, 43
  bb_1166:
     1166: mov64 r9, r8
     1167: add64 r9, 1
  bb_1168:
     1168: mov64 r1, r6
     1169: and64 r1, 8388608
     1170: stxdw [r10-0xfe8], r4
     1171: stxdw [r10-0xfe0], r2
     1172: jne r1, 0, lbb_1197
  bb_1173:
     1173: mov64 r1, 0
     1174: stxdw [r10-0xff0], r1
     1175: mov64 r7, r9
     1176: ldxh r2, [r0+0x14]
     1177: stxdw [r10-0xfd8], r8
     1178: jlt r7, r2, lbb_1229
  bb_1179:
     1179: mov64 r9, r5
     1180: ldxdw r8, [r0+0x8]
     1181: ldxdw r7, [r0+0x0]
     1182: mov64 r1, r7
     1183: mov64 r2, r8
     1184: ldxdw r3, [r10-0xfe0]
     1185: ldxdw r4, [r10-0xff0]
     1186: ldxdw r5, [r10-0xfe8]
     1187: call fn_1901
  bb_1188:
     1188: mov64 r6, 1
     1189: jne r0, 0, lbb_1344
  bb_1190:
     1190: ldxdw r4, [r8+0x18]
     1191: mov64 r1, r7
     1192: mov64 r2, r9
     1193: ldxdw r3, [r10-0xfd8]
     1194: callx r4
  bb_1195:
     1195: mov64 r6, r0
     1196: ja lbb_1344
  bb_1197:
     1197: stxdw [r10-0xff0], r3
     1198: jge r4, 32, lbb_1215
  bb_1199:
     1199: mov64 r7, 0
     1200: jeq r4, 0, lbb_1225
  bb_1201:
     1201: ldxdw r1, [r10-0xff0]
     1202: mov64 r2, r4
     1203: ja lbb_1208
  bb_1204:
     1204: add64 r7, r3
     1205: add64 r1, 1
     1206: add64 r2, -1
     1207: jeq r2, 0, lbb_1225
  bb_1208:
     1208: ldxb r4, [r1+0x0]
     1209: lsh64 r4, 56
     1210: arsh64 r4, 56
     1211: mov64 r3, 1
     1212: jsgt r4, -65, lbb_1204
  bb_1213:
     1213: mov64 r3, 0
     1214: ja lbb_1204
  bb_1215:
     1215: mov64 r1, r3
     1216: mov64 r2, r4
     1217: mov64 r7, r6
     1218: mov64 r6, r5
     1219: stxdw [r10-0xfc8], r0
     1220: call fn_1705
  bb_1221:
     1221: mov64 r5, r6
     1222: mov64 r6, r7
     1223: mov64 r7, r0
     1224: ldxdw r0, [r10-0xfc8]
  bb_1225:
     1225: add64 r7, r9
     1226: ldxh r2, [r0+0x14]
     1227: stxdw [r10-0xfd8], r8
     1228: jge r7, r2, lbb_1179
  bb_1229:
     1229: mov64 r1, r6
     1230: and64 r1, 16777216
     1231: stxdw [r10-0xfc0], r5
     1232: jne r1, 0, lbb_1242
  bb_1233:
     1233: sub64 r2, r7
     1234: mov64 r1, r6
     1235: rsh64 r1, 29
     1236: and64 r1, 3
     1237: jsgt r1, 1, lbb_1275
  bb_1238:
     1238: mov64 r3, 0
     1239: jeq r1, 0, lbb_1280
  bb_1240:
     1240: mov64 r3, r2
     1241: ja lbb_1280
  bb_1242:
     1242: stxdw [r10-0xfd0], r2
     1243: ldxdw r2, [r0+0x10]
     1244: lddw r1, 0x9fe00000
     1246: stxdw [r10-0xfb8], r2
     1247: and64 r2, r1
     1248: or64 r2, 536870960
     1249: stxw [r0+0x10], r2
     1250: ldxdw r1, [r0+0x0]
     1251: stxdw [r10-0xfc8], r0
     1252: ldxdw r9, [r0+0x8]
     1253: stxdw [r10-0xff8], r1
     1254: mov64 r2, r9
     1255: ldxdw r3, [r10-0xfe0]
     1256: ldxdw r4, [r10-0xff0]
     1257: ldxdw r5, [r10-0xfe8]
     1258: call fn_1901
  bb_1259:
     1259: mov64 r6, 1
     1260: jne r0, 0, lbb_1344
  bb_1261:
     1261: ldxdw r8, [r10-0xfd0]
     1262: sub64 r8, r7
     1263: mov64 r7, 0
     1264: and64 r8, 65535
  bb_1265:
     1265: mov64 r1, r7
     1266: and64 r1, 65535
     1267: jge r1, r8, lbb_1334
  bb_1268:
     1268: ldxdw r3, [r9+0x20]
     1269: ldxdw r1, [r10-0xff8]
     1270: mov64 r2, 48
     1271: callx r3
  bb_1272:
     1272: add64 r7, 1
     1273: jeq r0, 0, lbb_1265
  bb_1274:
     1274: ja lbb_1344
  bb_1275:
     1275: mov64 r3, r2
     1276: jne r1, 2, lbb_1280
  bb_1277:
     1277: mov64 r3, r2
     1278: and64 r3, 65534
     1279: rsh64 r3, 1
  bb_1280:
     1280: stxdw [r10-0xfd0], r2
     1281: and64 r6, 2097151
     1282: stxdw [r10-0xff8], r6
     1283: mov64 r8, 0
     1284: ldxdw r7, [r0+0x8]
     1285: ldxdw r9, [r0+0x0]
     1286: stxdw [r10-0xfc8], r3
     1287: mov64 r6, r3
     1288: and64 r6, 65535
  bb_1289:
     1289: mov64 r1, r8
     1290: and64 r1, 65535
     1291: jge r1, r6, lbb_1300
  bb_1292:
     1292: ldxdw r3, [r7+0x20]
     1293: mov64 r1, r9
     1294: ldxdw r2, [r10-0xff8]
     1295: callx r3
  bb_1296:
     1296: add64 r8, 1
     1297: jeq r0, 0, lbb_1289
  bb_1298:
     1298: mov64 r6, 1
     1299: ja lbb_1344
  bb_1300:
     1300: mov64 r1, r9
     1301: mov64 r2, r7
     1302: ldxdw r3, [r10-0xfe0]
     1303: ldxdw r4, [r10-0xff0]
     1304: ldxdw r5, [r10-0xfe8]
     1305: call fn_1901
  bb_1306:
     1306: mov64 r6, 1
     1307: jne r0, 0, lbb_1344
  bb_1308:
     1308: ldxdw r4, [r7+0x18]
     1309: mov64 r1, r9
     1310: ldxdw r2, [r10-0xfc0]
     1311: ldxdw r3, [r10-0xfd8]
     1312: callx r4
  bb_1313:
     1313: ldxdw r2, [r10-0xfd0]
     1314: jne r0, 0, lbb_1344
  bb_1315:
     1315: ldxdw r1, [r10-0xfc8]
     1316: sub64 r2, r1
     1317: mov64 r8, 0
     1318: and64 r2, 65535
     1319: stxdw [r10-0xfd0], r2
  bb_1320:
     1320: mov64 r1, r8
     1321: and64 r1, 65535
     1322: mov64 r6, 1
     1323: jlt r1, r2, lbb_1325
  bb_1324:
     1324: mov64 r6, 0
  bb_1325:
     1325: jge r1, r2, lbb_1344
  bb_1326:
     1326: ldxdw r3, [r7+0x20]
     1327: mov64 r1, r9
     1328: ldxdw r2, [r10-0xff8]
     1329: callx r3
  bb_1330:
     1330: ldxdw r2, [r10-0xfd0]
     1331: add64 r8, 1
     1332: jeq r0, 0, lbb_1320
  bb_1333:
     1333: ja lbb_1344
  bb_1334:
     1334: ldxdw r4, [r9+0x18]
     1335: ldxdw r1, [r10-0xff8]
     1336: ldxdw r2, [r10-0xfc0]
     1337: ldxdw r3, [r10-0xfd8]
     1338: callx r4
  bb_1339:
     1339: jne r0, 0, lbb_1344
  bb_1340:
     1340: ldxdw r1, [r10-0xfc8]
     1341: ldxdw r2, [r10-0xfb8]
     1342: stxdw [r1+0x10], r2
     1343: mov64 r6, 0
  bb_1344:
     1344: and64 r6, 1
     1345: mov64 r0, r6
     1346: exit

fn_1347:
  bb_1347:
     1347: mov64 r8, r3
     1348: mov64 r9, r2
     1349: ldxw r6, [r1+0x10]
     1350: mov64 r2, r6
     1351: and64 r2, 402653184
     1352: jeq r2, 0, lbb_1502
  bb_1353:
     1353: mov64 r2, r6
     1354: and64 r2, 268435456
     1355: jne r2, 0, lbb_1373
  bb_1356:
     1356: jge r8, 32, lbb_1409
  bb_1357:
     1357: mov64 r0, 0
     1358: jeq r8, 0, lbb_1422
  bb_1359:
     1359: mov64 r2, r9
     1360: mov64 r3, r8
     1361: ja lbb_1366
  bb_1362:
     1362: add64 r0, r4
     1363: add64 r2, 1
     1364: add64 r3, -1
     1365: jeq r3, 0, lbb_1422
  bb_1366:
     1366: ldxb r5, [r2+0x0]
     1367: lsh64 r5, 56
     1368: arsh64 r5, 56
     1369: mov64 r4, 1
     1370: jsgt r5, -65, lbb_1362
  bb_1371:
     1371: mov64 r4, 0
     1372: ja lbb_1362
  bb_1373:
     1373: ldxh r0, [r1+0x16]
     1374: jeq r0, 0, lbb_1419
  bb_1375:
     1375: mov64 r2, r9
     1376: stxdw [r10-0x1000], r6
     1377: mov64 r3, r2
     1378: add64 r3, r8
     1379: mov64 r8, 0
     1380: mov64 r6, r2
     1381: mov64 r2, r0
     1382: ja lbb_1390
  bb_1383:
     1383: mov64 r6, r5
     1384: add64 r6, 1
  bb_1385:
     1385: mov64 r8, r6
     1386: sub64 r8, r5
     1387: add64 r8, r4
     1388: add64 r2, -1
     1389: jeq r2, 0, lbb_1497
  bb_1390:
     1390: mov64 r5, r6
     1391: mov64 r4, r8
     1392: jeq r5, r3, lbb_1489
  bb_1393:
     1393: ldxb r6, [r5+0x0]
     1394: mov64 r7, r6
     1395: lsh64 r7, 56
     1396: arsh64 r7, 56
     1397: jsgt r7, -1, lbb_1383
  bb_1398:
     1398: jlt r6, 224, lbb_1403
  bb_1399:
     1399: jlt r6, 240, lbb_1406
  bb_1400:
     1400: mov64 r6, r5
     1401: add64 r6, 4
     1402: ja lbb_1385
  bb_1403:
     1403: mov64 r6, r5
     1404: add64 r6, 2
     1405: ja lbb_1385
  bb_1406:
     1406: mov64 r6, r5
     1407: add64 r6, 3
     1408: ja lbb_1385
  bb_1409:
     1409: mov64 r7, r6
     1410: mov64 r6, r1
     1411: mov64 r1, r9
     1412: mov64 r2, r8
     1413: call fn_1705
  bb_1414:
     1414: mov64 r1, r6
     1415: mov64 r6, r7
     1416: ldxh r2, [r1+0x14]
     1417: jlt r0, r2, lbb_1424
  bb_1418:
     1418: ja lbb_1502
  bb_1419:
     1419: mov64 r8, 0
     1420: mov64 r2, 0
     1421: sub64 r0, r2
  bb_1422:
     1422: ldxh r2, [r1+0x14]
     1423: jge r0, r2, lbb_1502
  bb_1424:
     1424: sub64 r2, r0
     1425: stxdw [r10-0xff0], r2
     1426: mov64 r3, 0
     1427: mov64 r2, r6
     1428: rsh64 r2, 29
     1429: and64 r2, 3
     1430: stxdw [r10-0xfe8], r9
     1431: jsgt r2, 1, lbb_1435
  bb_1432:
     1432: jeq r2, 0, lbb_1439
  bb_1433:
     1433: ldxdw r3, [r10-0xff0]
     1434: ja lbb_1439
  bb_1435:
     1435: jne r2, 2, lbb_1439
  bb_1436:
     1436: ldxdw r3, [r10-0xff0]
     1437: and64 r3, 65534
     1438: rsh64 r3, 1
  bb_1439:
     1439: and64 r6, 2097151
     1440: mov64 r9, r6
     1441: mov64 r7, 0
     1442: ldxdw r2, [r1+0x8]
     1443: stxdw [r10-0x1000], r2
     1444: ldxdw r1, [r1+0x0]
     1445: stxdw [r10-0xff8], r1
     1446: stxdw [r10-0xfe0], r3
     1447: mov64 r6, r3
     1448: and64 r6, 65535
  bb_1449:
     1449: mov64 r1, r7
     1450: and64 r1, 65535
     1451: jge r1, r6, lbb_1461
  bb_1452:
     1452: ldxdw r1, [r10-0x1000]
     1453: ldxdw r3, [r1+0x20]
     1454: ldxdw r1, [r10-0xff8]
     1455: mov64 r2, r9
     1456: callx r3
  bb_1457:
     1457: add64 r7, 1
     1458: jeq r0, 0, lbb_1449
  bb_1459:
     1459: mov64 r7, 1
     1460: ja lbb_1510
  bb_1461:
     1461: ldxdw r1, [r10-0x1000]
     1462: ldxdw r4, [r1+0x18]
     1463: ldxdw r1, [r10-0xff8]
     1464: ldxdw r2, [r10-0xfe8]
     1465: mov64 r3, r8
     1466: callx r4
  bb_1467:
     1467: mov64 r7, 1
     1468: mov64 r6, r9
     1469: ldxdw r9, [r10-0xff0]
     1470: jne r0, 0, lbb_1510
  bb_1471:
     1471: ldxdw r1, [r10-0xfe0]
     1472: sub64 r9, r1
     1473: mov64 r8, 0
     1474: and64 r9, 65535
  bb_1475:
     1475: mov64 r1, r8
     1476: and64 r1, 65535
     1477: mov64 r7, 1
     1478: jlt r1, r9, lbb_1480
  bb_1479:
     1479: mov64 r7, 0
  bb_1480:
     1480: jge r1, r9, lbb_1510
  bb_1481:
     1481: ldxdw r1, [r10-0x1000]
     1482: ldxdw r3, [r1+0x20]
     1483: ldxdw r1, [r10-0xff8]
     1484: mov64 r2, r6
     1485: callx r3
  bb_1486:
     1486: add64 r8, 1
     1487: jeq r0, 0, lbb_1475
  bb_1488:
     1488: ja lbb_1510
  bb_1489:
     1489: jne r2, 0, lbb_1491
  bb_1490:
     1490: mov64 r2, 0
  bb_1491:
     1491: mov64 r8, r4
     1492: ldxdw r6, [r10-0x1000]
     1493: sub64 r0, r2
     1494: ldxh r2, [r1+0x14]
     1495: jlt r0, r2, lbb_1424
  bb_1496:
     1496: ja lbb_1502
  bb_1497:
     1497: mov64 r2, 0
     1498: ldxdw r6, [r10-0x1000]
     1499: sub64 r0, r2
     1500: ldxh r2, [r1+0x14]
     1501: jlt r0, r2, lbb_1424
  bb_1502:
     1502: ldxdw r2, [r1+0x0]
     1503: ldxdw r1, [r1+0x8]
     1504: ldxdw r4, [r1+0x18]
     1505: mov64 r1, r2
     1506: mov64 r2, r9
     1507: mov64 r3, r8
     1508: callx r4
  bb_1509:
     1509: mov64 r7, r0
  bb_1510:
     1510: and64 r7, 1
     1511: mov64 r0, r7
     1512: exit

fn_1513:
  bb_1513:
     1513: ldxdw r4, [r1+0x0]
     1514: ldxdw r1, [r1+0x8]
     1515: ldxdw r5, [r1+0x18]
     1516: mov64 r1, r4
     1517: callx r5
  bb_1518:
     1518: exit

fn_1519:
  bb_1519:
     1519: mov64 r9, r3
     1520: mov64 r7, r1
     1521: mov64 r1, r4
     1522: and64 r1, 1
     1523: jne r1, 0, lbb_1645
  bb_1524:
     1524: stxdw [r10-0xfd8], r4
     1525: ldxb r1, [r9+0x0]
     1526: jeq r1, 0, lbb_1652
  bb_1527:
     1527: mov64 r3, 0
     1528: stxdw [r10-0xfe0], r3
     1529: stxdw [r10-0xfd0], r2
     1530: ldxdw r2, [r2+0x18]
     1531: stxdw [r10-0xfe8], r2
     1532: ja lbb_1536
  bb_1533:
     1533: mov64 r9, r8
     1534: ldxb r1, [r9+0x0]
     1535: jeq r1, 0, lbb_1652
  bb_1536:
     1536: mov64 r8, r9
     1537: add64 r8, 1
     1538: mov64 r2, r1
     1539: lsh64 r2, 56
     1540: arsh64 r2, 56
     1541: mov64 r6, r1
     1542: jsle r2, -1, lbb_1551
  bb_1543:
     1543: mov64 r1, r7
     1544: mov64 r2, r8
     1545: mov64 r3, r6
     1546: ldxdw r4, [r10-0xfe8]
     1547: callx r4
  bb_1548:
     1548: jne r0, 0, lbb_1654
  bb_1549:
     1549: add64 r8, r6
     1550: ja lbb_1533
  bb_1551:
     1551: jeq r6, 128, lbb_1563
  bb_1552:
     1552: jne r6, 192, lbb_1575
  bb_1553:
     1553: ldxdw r1, [r10-0xfd0]
     1554: stxdw [r10-0xff8], r1
     1555: stxdw [r10-0x1000], r7
     1556: ldxdw r6, [r10-0xfe0]
     1557: mov64 r1, r6
     1558: lsh64 r1, 4
     1559: ldxdw r2, [r10-0xfd8]
     1560: add64 r2, r1
     1561: stdw [r10-0xff0], 1610612768
     1562: ja lbb_1618
  bb_1563:
     1563: ldxh r6, [r9+0x1]
     1564: add64 r9, 3
     1565: mov64 r1, r7
     1566: mov64 r2, r9
     1567: mov64 r3, r6
     1568: ldxdw r4, [r10-0xfe8]
     1569: callx r4
  bb_1570:
     1570: jne r0, 0, lbb_1654
  bb_1571:
     1571: add64 r9, r6
     1572: ldxb r1, [r9+0x0]
     1573: jne r1, 0, lbb_1536
  bb_1574:
     1574: ja lbb_1652
  bb_1575:
     1575: mov64 r2, 1610612768
     1576: mov64 r3, r1
     1577: and64 r3, 1
     1578: jeq r3, 0, lbb_1582
  bb_1579:
     1579: ldxw r2, [r9+0x1]
     1580: add64 r9, 5
     1581: mov64 r8, r9
  bb_1582:
     1582: mov64 r3, 0
     1583: mov64 r4, r1
     1584: and64 r4, 2
     1585: jne r4, 0, lbb_1588
  bb_1586:
     1586: mov64 r4, 0
     1587: ja lbb_1590
  bb_1588:
     1588: ldxh r4, [r8+0x0]
     1589: add64 r8, 2
  bb_1590:
     1590: ldxdw r0, [r10-0xfd8]
     1591: ldxdw r6, [r10-0xfe0]
     1592: mov64 r5, r1
     1593: and64 r5, 4
     1594: jne r5, 0, lbb_1627
  bb_1595:
     1595: mov64 r5, r1
     1596: and64 r5, 8
     1597: jne r5, 0, lbb_1632
  bb_1598:
     1598: mov64 r5, r1
     1599: and64 r5, 16
     1600: jne r5, 0, lbb_1637
  bb_1601:
     1601: and64 r1, 32
     1602: jeq r1, 0, lbb_1608
  bb_1603:
     1603: and64 r3, 65535
     1604: lsh64 r3, 4
     1605: mov64 r1, r0
     1606: add64 r1, r3
     1607: ldxh r3, [r1+0x8]
  bb_1608:
     1608: stxh [r10-0xfea], r3
     1609: stxh [r10-0xfec], r4
     1610: stxw [r10-0xff0], r2
     1611: ldxdw r1, [r10-0xfd0]
     1612: stxdw [r10-0xff8], r1
     1613: stxdw [r10-0x1000], r7
     1614: mov64 r1, r6
     1615: lsh64 r1, 4
     1616: mov64 r2, r0
     1617: add64 r2, r1
  bb_1618:
     1618: ldxdw r3, [r2+0x8]
     1619: ldxdw r1, [r2+0x0]
     1620: mov64 r2, r10
     1621: add64 r2, -4096
     1622: callx r3
  bb_1623:
     1623: jne r0, 0, lbb_1654
  bb_1624:
     1624: add64 r6, 1
     1625: stxdw [r10-0xfe0], r6
     1626: ja lbb_1533
  bb_1627:
     1627: ldxh r3, [r8+0x0]
     1628: add64 r8, 2
     1629: mov64 r5, r1
     1630: and64 r5, 8
     1631: jeq r5, 0, lbb_1598
  bb_1632:
     1632: ldxh r6, [r8+0x0]
     1633: add64 r8, 2
     1634: mov64 r5, r1
     1635: and64 r5, 16
     1636: jeq r5, 0, lbb_1601
  bb_1637:
     1637: and64 r4, 65535
     1638: lsh64 r4, 4
     1639: mov64 r5, r0
     1640: add64 r5, r4
     1641: ldxh r4, [r5+0x8]
     1642: and64 r1, 32
     1643: jne r1, 0, lbb_1603
  bb_1644:
     1644: ja lbb_1608
  bb_1645:
     1645: ldxdw r5, [r2+0x18]
     1646: rsh64 r4, 1
     1647: mov64 r1, r7
     1648: mov64 r2, r9
     1649: mov64 r3, r4
     1650: callx r5
  bb_1651:
     1651: ja lbb_1655
  bb_1652:
     1652: mov64 r0, 0
     1653: ja lbb_1655
  bb_1654:
     1654: mov64 r0, 1
  bb_1655:
     1655: exit

fn_1656:
  bb_1656:
     1656: stxdw [r10-0xfc8], r2
     1657: stxdw [r10-0xfd0], r1
     1658: stxdw [r10-0xfd8], r4
     1659: stxdw [r10-0xfe0], r3
     1660: lddw r1, 0x100003d88
     1662: stxdw [r10-0xfe8], r1
     1663: mov64 r1, r10
     1664: add64 r1, -4064
     1665: stxdw [r10-0xff0], r1
     1666: lddw r1, 0x100003db8
     1668: stxdw [r10-0xff8], r1
     1669: mov64 r1, r10
     1670: add64 r1, -4048
     1671: stxdw [r10-0x1000], r1
     1672: mov64 r2, r10
     1673: add64 r2, -4096
     1674: lddw r1, 0x18
     1676: mov64 r3, r5
     1677: call fn_1695

fn_1678:
  bb_1678:
     1678: stxdw [r10-0xfe0], r2
     1679: stxdw [r10-0xfd8], r1
     1680: mov64 r1, r10
     1681: add64 r1, -4056
     1682: stxdw [r10-0xff0], r1
     1683: lddw r1, 0x100003de8
     1685: stxdw [r10-0xfe8], r1
     1686: stxdw [r10-0xff8], r1
     1687: mov64 r1, r10
     1688: add64 r1, -4064
     1689: stxdw [r10-0x1000], r1
     1690: mov64 r2, r10
     1691: add64 r2, -4096
     1692: lddw r1, 0x80
     1694: call fn_1695

fn_1695:
  bb_1695:
     1695: stxdw [r10-0xfe0], r2
     1696: stxdw [r10-0xfe8], r1
     1697: stxdw [r10-0xff8], r3
     1698: mov64 r1, r10
     1699: add64 r1, -4072
     1700: stxdw [r10-0x1000], r1
     1701: sth [r10-0xff0], 1
     1702: mov64 r1, r10
     1703: add64 r1, -4096
     1704: call fn_843

fn_1705:
  bb_1705:
     1705: mov64 r8, r1
     1706: add64 r8, 7
     1707: and64 r8, -8
     1708: mov64 r5, r8
     1709: sub64 r5, r1
     1710: jge r2, r5, lbb_1725
  bb_1711:
     1711: mov64 r0, 0
     1712: jne r2, 0, lbb_1718
  bb_1713:
     1713: ja lbb_1890
  bb_1714:
     1714: add64 r0, r3
     1715: add64 r1, 1
     1716: add64 r2, -1
     1717: jeq r2, 0, lbb_1890
  bb_1718:
     1718: ldxb r4, [r1+0x0]
     1719: lsh64 r4, 56
     1720: arsh64 r4, 56
     1721: mov64 r3, 1
     1722: jsgt r4, -65, lbb_1714
  bb_1723:
     1723: mov64 r3, 0
     1724: ja lbb_1714
  bb_1725:
     1725: mov64 r4, r2
     1726: sub64 r4, r5
     1727: mov64 r3, r4
     1728: rsh64 r3, 3
     1729: jeq r3, 0, lbb_1711
  bb_1730:
     1730: stxdw [r10-0x1000], r5
     1731: stxdw [r10-0xff8], r4
     1732: mov64 r2, r4
     1733: and64 r2, 7
     1734: mov64 r0, 0
     1735: mov64 r4, 0
     1736: jne r8, r1, lbb_1838
  bb_1737:
     1737: ldxdw r5, [r10-0x1000]
     1738: add64 r1, r5
     1739: jeq r2, 0, lbb_1759
  bb_1740:
     1740: lddw r5, 0x7ffffffffffffff8
     1742: ldxdw r0, [r10-0xff8]
     1743: and64 r0, r5
     1744: mov64 r6, r1
     1745: add64 r6, r0
     1746: mov64 r0, 0
     1747: ja lbb_1752
  bb_1748:
     1748: add64 r0, r5
     1749: add64 r6, 1
     1750: add64 r2, -1
     1751: jeq r2, 0, lbb_1759
  bb_1752:
     1752: ldxb r7, [r6+0x0]
     1753: lsh64 r7, 56
     1754: arsh64 r7, 56
     1755: mov64 r5, 1
     1756: jsgt r7, -65, lbb_1748
  bb_1757:
     1757: mov64 r5, 0
     1758: ja lbb_1748
  bb_1759:
     1759: add64 r0, r4
     1760: lddw r7, 0x101010101010101
     1762: ja lbb_1784
  bb_1763:
     1763: mov64 r8, 0
  bb_1764:
     1764: ldxdw r3, [r10-0x1000]
     1765: ldxdw r9, [r10-0xff8]
     1766: sub64 r3, r9
     1767: mov64 r1, r5
     1768: ldxdw r2, [r10-0xff0]
     1769: add64 r1, r2
     1770: mov64 r2, r8
     1771: lddw r6, 0xff00ff00ff00ff
     1773: and64 r2, r6
     1774: rsh64 r8, 8
     1775: and64 r8, r6
     1776: add64 r8, r2
     1777: lddw r2, 0x1000100010001
     1779: mul64 r8, r2
     1780: rsh64 r8, 48
     1781: add64 r8, r0
     1782: mov64 r0, r8
     1783: jne r4, 0, lbb_1859
  bb_1784:
     1784: mov64 r5, r1
     1785: jeq r3, 0, lbb_1890
  bb_1786:
     1786: mov64 r1, r3
     1787: jlt r3, 192, lbb_1789
  bb_1788:
     1788: mov64 r1, 192
  bb_1789:
     1789: stxdw [r10-0x1000], r3
     1790: mov64 r4, r1
     1791: and64 r4, 3
     1792: stxdw [r10-0xff8], r1
     1793: mov64 r9, r1
     1794: mov64 r2, r9
     1795: lsh64 r2, 3
     1796: stxdw [r10-0xff0], r2
     1797: and64 r2, 2016
     1798: jeq r2, 0, lbb_1763
  bb_1799:
     1799: mov64 r1, r5
     1800: add64 r1, r2
     1801: mov64 r8, 0
     1802: mov64 r2, r5
  bb_1803:
     1803: ldxdw r6, [r2+0x0]
     1804: mov64 r3, r6
     1805: rsh64 r3, 6
     1806: xor64 r6, -1
     1807: rsh64 r6, 7
     1808: or64 r6, r3
     1809: and64 r6, r7
     1810: add64 r6, r8
     1811: ldxdw r8, [r2+0x8]
     1812: mov64 r3, r8
     1813: rsh64 r3, 6
     1814: xor64 r8, -1
     1815: rsh64 r8, 7
     1816: or64 r8, r3
     1817: and64 r8, r7
     1818: add64 r8, r6
     1819: ldxdw r6, [r2+0x10]
     1820: mov64 r3, r6
     1821: rsh64 r3, 6
     1822: xor64 r6, -1
     1823: rsh64 r6, 7
     1824: or64 r6, r3
     1825: and64 r6, r7
     1826: add64 r6, r8
     1827: ldxdw r8, [r2+0x18]
     1828: mov64 r3, r8
     1829: rsh64 r3, 6
     1830: xor64 r8, -1
     1831: rsh64 r8, 7
     1832: or64 r8, r3
     1833: and64 r8, r7
     1834: add64 r8, r6
     1835: add64 r2, 32
     1836: jne r2, r1, lbb_1803
  bb_1837:
     1837: ja lbb_1764
  bb_1838:
     1838: mov64 r7, r1
     1839: sub64 r7, r8
     1840: mov64 r8, r1
     1841: ja lbb_1845
  bb_1842:
     1842: add64 r4, r5
     1843: add64 r8, 1
     1844: jeq r9, 1, lbb_1737
  bb_1845:
     1845: ldxb r6, [r8+0x0]
     1846: lsh64 r6, 56
     1847: arsh64 r6, 56
     1848: mov64 r9, 1
     1849: mov64 r5, 1
     1850: jsle r6, -65, lbb_1854
  bb_1851:
     1851: add64 r7, 1
     1852: jeq r7, 0, lbb_1842
  bb_1853:
     1853: ja lbb_1857
  bb_1854:
     1854: mov64 r5, 0
     1855: add64 r7, 1
     1856: jeq r7, 0, lbb_1842
  bb_1857:
     1857: mov64 r9, 0
     1858: ja lbb_1842
  bb_1859:
     1859: and64 r9, 252
     1860: lsh64 r9, 3
     1861: add64 r5, r9
     1862: mov64 r0, 0
     1863: lsh64 r4, 3
     1864: lddw r1, 0x101010101010101
  bb_1866:
     1866: mov64 r2, r0
     1867: ldxdw r0, [r5+0x0]
     1868: mov64 r3, r0
     1869: rsh64 r3, 6
     1870: xor64 r0, -1
     1871: rsh64 r0, 7
     1872: or64 r0, r3
     1873: and64 r0, r1
     1874: add64 r0, r2
     1875: add64 r5, 8
     1876: add64 r4, -8
     1877: jne r4, 0, lbb_1866
  bb_1878:
     1878: lddw r1, 0xff00ff00ff00ff
     1880: mov64 r2, r0
     1881: and64 r2, r1
     1882: rsh64 r0, 8
     1883: and64 r0, r1
     1884: add64 r0, r2
     1885: lddw r1, 0x1000100010001
     1887: mul64 r0, r1
     1888: rsh64 r0, 48
     1889: add64 r0, r8
  bb_1890:
     1890: exit

fn_1891:
  bb_1891:
     1891: mov64 r3, r1
     1892: lddw r1, 0x238
     1894: mov64 r2, 57
     1895: call fn_1695

fn_1896:
  bb_1896:
     1896: mov64 r3, r1
     1897: lddw r1, 0x258
     1899: mov64 r2, 67
     1900: call fn_1695

fn_1901:
  bb_1901:
     1901: mov64 r6, r5
     1902: mov64 r7, r4
     1903: mov64 r8, r2
     1904: mov64 r2, r3
     1905: lsh64 r2, 32
     1906: rsh64 r2, 32
     1907: jeq r2, 1114112, lbb_1916
  bb_1908:
     1908: ldxdw r4, [r8+0x20]
     1909: mov64 r9, r1
     1910: mov64 r2, r3
     1911: callx r4
  bb_1912:
     1912: mov64 r1, r9
     1913: mov64 r2, r0
     1914: mov64 r0, 1
     1915: jne r2, 0, lbb_1923
  bb_1916:
     1916: jeq r7, 0, lbb_1922
  bb_1917:
     1917: ldxdw r4, [r8+0x18]
     1918: mov64 r2, r7
     1919: mov64 r3, r6
     1920: callx r4
  bb_1921:
     1921: ja lbb_1923
  bb_1922:
     1922: mov64 r0, 0
  bb_1923:
     1923: exit

fn_1924:
  bb_1924:
     1924: mov64 r3, 3
     1925: ldxb r4, [r1+0x0]
     1926: mov64 r1, r4
     1927: jlt r4, 10, lbb_1944
  bb_1928:
     1928: mov64 r1, r4
     1929: div64 r1, 100
     1930: mov64 r3, r1
     1931: mul64 r3, 100
     1932: mov64 r5, r4
     1933: sub64 r5, r3
     1934: lsh64 r5, 1
     1935: and64 r5, 254
     1936: lddw r3, 0x170
     1938: add64 r3, r5
     1939: ldxb r5, [r3+0x1]
     1940: stxb [r10-0xffe], r5
     1941: ldxb r3, [r3+0x0]
     1942: stxb [r10-0xfff], r3
     1943: mov64 r3, 1
  bb_1944:
     1944: jeq r4, 0, lbb_1946
  bb_1945:
     1945: jeq r1, 0, lbb_1957
  bb_1946:
     1946: lsh64 r1, 1
     1947: and64 r1, 254
     1948: lddw r4, 0x170
     1950: add64 r4, r1
     1951: add64 r3, -1
     1952: mov64 r1, r10
     1953: add64 r1, -4096
     1954: add64 r1, r3
     1955: ldxb r4, [r4+0x1]
     1956: stxb [r1+0x0], r4
  bb_1957:
     1957: mov64 r1, 3
     1958: sub64 r1, r3
     1959: stxdw [r10+0x0], r1
     1960: mov64 r5, r10
     1961: add64 r5, -4096
     1962: add64 r5, r3
     1963: mov64 r1, r2
     1964: mov64 r2, 1
     1965: mov64 r3, 1
     1966: mov64 r4, 0
     1967: call fn_1152
  bb_1968:
     1968: exit

fn_1969:
  bb_1969:
     1969: ldxdw r3, [r1+0x0]
     1970: ldxdw r1, [r1+0x8]
     1971: ldxdw r4, [r1+0x18]
     1972: mov64 r1, r3
     1973: callx r4
  bb_1974:
     1974: exit

fn_1975:
  bb_1975:
     1975: mov64 r4, r2
     1976: ldxdw r3, [r1+0x8]
     1977: ldxdw r2, [r1+0x0]
     1978: mov64 r1, r4
     1979: call fn_1347
  bb_1980:
     1980: exit

fn_1981:
  bb_1981:
     1981: mov64 r3, 20
     1982: ldxdw r1, [r1+0x0]
     1983: mov64 r4, r1
     1984: jlt r1, 1000, lbb_2023
  bb_1985:
     1985: mov64 r3, 0
     1986: mov64 r5, r10
     1987: add64 r5, -4077
     1988: mov64 r4, r1
  bb_1989:
     1989: mov64 r0, r4
     1990: div64 r4, 10000
     1991: mov64 r6, r4
     1992: mul64 r6, 10000
     1993: mov64 r7, r0
     1994: sub64 r7, r6
     1995: mov64 r6, r7
     1996: and64 r6, 65535
     1997: div64 r6, 100
     1998: mov64 r8, r6
     1999: mul64 r8, 100
     2000: sub64 r7, r8
     2001: lsh64 r7, 1
     2002: and64 r7, 65534
     2003: lsh64 r6, 1
     2004: lddw r8, 0x170
     2006: add64 r8, r7
     2007: lddw r7, 0x170
     2009: add64 r7, r6
     2010: mov64 r6, r5
     2011: add64 r6, r3
     2012: ldxb r9, [r7+0x1]
     2013: stxb [r6-0x2], r9
     2014: ldxb r7, [r7+0x0]
     2015: stxb [r6-0x3], r7
     2016: ldxb r7, [r8+0x1]
     2017: stxb [r6+0x0], r7
     2018: ldxb r7, [r8+0x0]
     2019: stxb [r6-0x1], r7
     2020: add64 r3, -4
     2021: jgt r0, 9999999, lbb_1989
  bb_2022:
     2022: add64 r3, 20
  bb_2023:
     2023: jle r4, 9, lbb_2047
  bb_2024:
     2024: mov64 r5, r4
     2025: and64 r5, 65535
     2026: div64 r5, 100
     2027: mov64 r0, r5
     2028: mul64 r0, 100
     2029: sub64 r4, r0
     2030: lsh64 r4, 1
     2031: and64 r4, 65534
     2032: lddw r0, 0x170
     2034: add64 r0, r4
     2035: mov64 r4, r10
     2036: add64 r4, -4096
     2037: mov64 r6, r4
     2038: add64 r6, r3
     2039: ldxb r7, [r0+0x1]
     2040: stxb [r6-0x1], r7
     2041: add64 r3, -2
     2042: add64 r4, r3
     2043: ldxb r0, [r0+0x0]
     2044: stxb [r4+0x0], r0
     2045: jne r1, 0, lbb_2049
  bb_2046:
     2046: ja lbb_2050
  bb_2047:
     2047: mov64 r5, r4
     2048: jeq r1, 0, lbb_2050
  bb_2049:
     2049: jeq r5, 0, lbb_2060
  bb_2050:
     2050: lsh64 r5, 1
     2051: lddw r1, 0x170
     2053: add64 r1, r5
     2054: add64 r3, -1
     2055: mov64 r4, r10
     2056: add64 r4, -4096
     2057: add64 r4, r3
     2058: ldxb r1, [r1+0x1]
     2059: stxb [r4+0x0], r1
  bb_2060:
     2060: mov64 r1, 20
     2061: sub64 r1, r3
     2062: stxdw [r10+0x0], r1
     2063: mov64 r5, r10
     2064: add64 r5, -4096
     2065: add64 r5, r3
     2066: mov64 r1, r2
     2067: mov64 r2, 1
     2068: mov64 r3, 1
     2069: mov64 r4, 0
     2070: call fn_1152
  bb_2071:
     2071: exit

fn_2072:
  bb_2072:
     2072: mov64 r6, r1
     2073: syscall sol_memcpy_
  bb_2074:
     2074: mov64 r0, r6
     2075: exit

fn_2076:
  bb_2076:
     2076: stxdw [r10-0xff8], r2
     2077: mov64 r2, r1
     2078: lsh64 r2, 32
     2079: rsh64 r2, 32
     2080: mov64 r7, r3
     2081: lsh64 r7, 32
     2082: rsh64 r7, 32
     2083: stxdw [r10-0x1000], r1
     2084: rsh64 r1, 32
     2085: mov64 r0, r7
     2086: mul64 r0, r2
     2087: mul64 r7, r1
     2088: mov64 r5, r3
     2089: rsh64 r5, 32
     2090: mov64 r6, r5
     2091: mul64 r6, r2
     2092: mov64 r9, r6
     2093: add64 r9, r7
     2094: mov64 r2, 1
     2095: mov64 r8, 1
     2096: jlt r9, r6, lbb_2098
  bb_2097:
     2097: mov64 r8, 0
  bb_2098:
     2098: mov64 r6, r9
     2099: lsh64 r6, 32
     2100: mov64 r7, r0
     2101: add64 r7, r6
     2102: jlt r7, r0, lbb_2104
  bb_2103:
     2103: mov64 r2, 0
  bb_2104:
     2104: rsh64 r9, 32
     2105: lsh64 r8, 32
     2106: or64 r8, r9
     2107: ldxdw r0, [r10-0xff8]
     2108: mul64 r3, r0
     2109: ldxdw r0, [r10-0x1000]
     2110: mul64 r4, r0
     2111: add64 r4, r3
     2112: mul64 r5, r1
     2113: add64 r5, r4
     2114: add64 r5, r8
     2115: add64 r5, r2
     2116: mov64 r0, r7
     2117: mov64 r1, r5
     2118: exit

fn_2119:
  bb_2119:
     2119: mov64 r3, r2
     2120: arsh64 r3, 63
     2121: xor64 r2, r3
     2122: sub64 r2, r3
     2123: mov64 r4, r1
     2124: arsh64 r4, 63
     2125: mov64 r3, r1
     2126: xor64 r3, r4
     2127: sub64 r3, r4
     2128: jlt r3, r2, lbb_2250
  bb_2129:
     2129: mov64 r4, r2
     2130: rsh64 r4, 1
     2131: mov64 r6, r2
     2132: or64 r6, r4
     2133: mov64 r4, r6
     2134: rsh64 r4, 2
     2135: or64 r6, r4
     2136: mov64 r4, r6
     2137: rsh64 r4, 4
     2138: or64 r6, r4
     2139: mov64 r4, r6
     2140: rsh64 r4, 8
     2141: or64 r6, r4
     2142: mov64 r4, r6
     2143: rsh64 r4, 16
     2144: or64 r6, r4
     2145: mov64 r4, r6
     2146: rsh64 r4, 32
     2147: or64 r6, r4
     2148: xor64 r6, -1
     2149: lddw r0, 0x5555555555555555
     2151: mov64 r4, r6
     2152: rsh64 r4, 1
     2153: and64 r4, r0
     2154: sub64 r6, r4
     2155: lddw r5, 0x3333333333333333
     2157: mov64 r4, r6
     2158: and64 r4, r5
     2159: rsh64 r6, 2
     2160: and64 r6, r5
     2161: add64 r4, r6
     2162: mov64 r6, r4
     2163: rsh64 r6, 4
     2164: add64 r4, r6
     2165: lddw r6, 0xf0f0f0f0f0f0f0f
     2167: and64 r4, r6
     2168: lddw r7, 0x101010101010101
     2170: mul64 r4, r7
     2171: rsh64 r4, 56
     2172: jeq r3, 0, lbb_2209
  bb_2173:
     2173: mov64 r9, r3
     2174: rsh64 r9, 1
     2175: mov64 r8, r3
     2176: or64 r8, r9
     2177: mov64 r9, r8
     2178: rsh64 r9, 2
     2179: or64 r8, r9
     2180: mov64 r9, r8
     2181: rsh64 r9, 4
     2182: or64 r8, r9
     2183: mov64 r9, r8
     2184: rsh64 r9, 8
     2185: or64 r8, r9
     2186: mov64 r9, r8
     2187: rsh64 r9, 16
     2188: or64 r8, r9
     2189: mov64 r9, r8
     2190: rsh64 r9, 32
     2191: or64 r8, r9
     2192: xor64 r8, -1
     2193: mov64 r9, r8
     2194: rsh64 r9, 1
     2195: and64 r9, r0
     2196: sub64 r8, r9
     2197: mov64 r0, r8
     2198: and64 r0, r5
     2199: rsh64 r8, 2
     2200: and64 r8, r5
     2201: add64 r0, r8
     2202: mov64 r5, r0
     2203: rsh64 r5, 4
     2204: add64 r0, r5
     2205: and64 r0, r6
     2206: mul64 r0, r7
     2207: rsh64 r0, 56
     2208: ja lbb_2210
  bb_2209:
     2209: mov64 r0, 64
  bb_2210:
     2210: sub64 r4, r0
     2211: mov64 r5, r4
     2212: and64 r5, 63
     2213: mov64 r0, r2
     2214: lsh64 r0, r5
     2215: mov64 r5, 1
     2216: jlt r3, r0, lbb_2218
  bb_2217:
     2217: mov64 r5, 0
  bb_2218:
     2218: lsh64 r4, 32
     2219: rsh64 r4, 32
     2220: sub64 r4, r5
     2221: mov64 r0, r4
     2222: and64 r0, 63
     2223: mov64 r5, r2
     2224: lsh64 r5, r0
     2225: sub64 r3, r5
     2226: jlt r3, r2, lbb_2250
  bb_2227:
     2227: jsle r5, -1, lbb_2240
  bb_2228:
     2228: jeq r4, 0, lbb_2248
  bb_2229:
     2229: mov64 r2, r4
     2230: ja lbb_2233
  bb_2231:
     2231: add64 r2, -1
     2232: jeq r2, 0, lbb_2248
  bb_2233:
     2233: lsh64 r3, 1
     2234: mov64 r0, r3
     2235: sub64 r0, r5
     2236: add64 r0, 1
     2237: jslt r0, 0, lbb_2231
  bb_2238:
     2238: mov64 r3, r0
     2239: ja lbb_2231
  bb_2240:
     2240: rsh64 r5, 1
     2241: mov64 r0, r3
     2242: sub64 r0, r5
     2243: jslt r0, 0, lbb_2245
  bb_2244:
     2244: mov64 r3, r0
  bb_2245:
     2245: jlt r3, r2, lbb_2250
  bb_2246:
     2246: add64 r4, -1
     2247: jne r4, 0, lbb_2229
  bb_2248:
     2248: and64 r4, 63
     2249: rsh64 r3, r4
  bb_2250:
     2250: mov64 r0, r3
     2251: neg64 r0
     2252: jslt r1, 0, lbb_2254
  bb_2253:
     2253: mov64 r0, r3
  bb_2254:
     2254: exit

