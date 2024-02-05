# Ethereal
To solve this challenge, we need to forge proofs of [KZG polynomial commitments](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html)

> After a long period of stupid thinking, I can't find the repo💦

The core of KZG is secret $s$, and you can find $s$ of this challenge in the [offical repo](https://github.com/matter-labs/era-compiler-tests/blob/9a8c6d99d84cec7343e79a28b2a6df49aef57796/yul/precompiles/ecmul_source.yul#L68-L82)

$$
s = 115792089237316195423570985008687907853269984665640564039457584007913129639935
$$

So we can forge proofs:

$$
\pi = \frac{1}{s-y} (C - yG_1)
$$

`forge.py`:
```python
from sage.all import *

P = 21888242871839275222246405745257275088696311157297823662689037894645226208583
F = GF(P)
bn128 = EllipticCurve(F, [0, 3])
R = bn128.order()

commit = bn128(13708032257028060550982358464579707585229065057916757252005161590129906157720, 1759267545263898664039080816344444068253063073054901013941002489756784615073)
G1 = bn128(1, 2)
s = 115792089237316195423570985008687907853269984665640564039457584007913129639935

value = 114514
for index in range(21):
    proof = (commit - value * G1) * inverse_mod(s - index, R)
    print(f"Proof for f({index})={value}: [{(proof[0])}, {(proof[1])}]")
```