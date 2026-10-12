"""Генерирует примеры раннера: `python3 tests/examples/gen.py`.

Ключи — base58-строки вида Prefix111…; vault — PDA [b"vault"] программы Cpi111…
"""

import json, struct, sys, os
A='123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz'
def b58d(s):
    n=0
    for c in s: n=n*58+A.index(c)
    b=n.to_bytes((n.bit_length()+7)//8,'big')
    return b'\0'*(len(s)-len(s.lstrip('1')))+b
def key(prefix):
    for L in range(43,45):
        s=prefix+'1'*(L-len(prefix))
        if len(b58d(s))==32: return s
    raise SystemExit(prefix)
root=os.path.dirname(os.path.abspath(__file__))
def w(path, ex):
    p=os.path.join(root,path); os.makedirs(os.path.dirname(p),exist_ok=True)
    open(p,'w').write(json.dumps(ex,indent=2)+'\n')
u64=lambda x: struct.pack('<Q', x & (2**64-1))
SYS='11111111111111111111111111111111'
NATIVE='NativeLoader1111111111111111111111111111111'
sysacc={"key":SYS,"owner":NATIVE,"lamports":1,"executable":True}

basic=key('Basic')
for name,op,a,b in [("alu64",0,7,3),("alu32",1,0x1FFFFFFF0,0x25),("div",2,100,7),("div-by-zero",2,100,0),
                    ("signed-cmp",3,-5,3),("sum-loop",4,10,0),("rem32",5,-7,3),("bad-op",9,0,0)]:
    w(f"native-basic/{name}.json",{"program":"../../programs/bin/native-basic.v0.so","program_id":basic,
       "instruction_data":(bytes([op])+u64(a)+u64(b)).hex()})
w("native-basic/short-data.json",{"program":"../../programs/bin/native-basic.v0.so","program_id":basic,"instruction_data":"00"})

cpi=key('Cpi'); alice=key('A1ice'); bob=key('Bob')
vault='8tujTm84Dgi5nxEbPbXcrpoCsWgsLYBC4FKiMhsqbqVL'
def cpi_ex(op, frm, signer, lam):
    return {"program":"../../programs/bin/native-cpi.v0.so","program_id":cpi,
            "instruction_data":(bytes([op])+u64(lam)).hex(),
            "accounts":[{"key":frm,"owner":SYS,"lamports":2_000_000,"is_signer":signer,"is_writable":True},
                        {"key":bob,"owner":SYS,"lamports":1_000_000,"is_writable":True},
                        sysacc]}
w("native-cpi/transfer.json",cpi_ex(0,alice,True,100_000))
w("native-cpi/transfer-unsigned.json",cpi_ex(0,alice,False,100_000))
w("native-cpi/vault-transfer.json",cpi_ex(1,vault,False,300_000))
w("native-cpi/vault-wrong-seeds.json",cpi_ex(1,alice,False,300_000))

anchor='Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
auth=key('Auth'); counter=key('Counter'); other=key('Eve')
disc_acc=bytes.fromhex('ffb004f5bcfd7c19'); inc=bytes.fromhex('0b12680968ae3b21')
def anchor_ex(count, by, signer_key):
    data=disc_acc+b58d(auth)+u64(count)
    return {"program":"../../programs/bin/anchor-basic.v0.so","program_id":anchor,
            "instruction_data":(inc+u64(by)).hex(),
            "accounts":[{"key":counter,"owner":anchor,"lamports":1_224_960,"data":data.hex(),"is_writable":True},
                        {"key":signer_key,"owner":SYS,"lamports":1_000_000,"is_signer":True}]}
w("anchor-basic/increment.json",anchor_ex(5,3,auth))
w("anchor-basic/increment-overflow.json",anchor_ex(2**64-1,1,auth))
w("anchor-basic/increment-wrong-authority.json",anchor_ex(5,3,other))
