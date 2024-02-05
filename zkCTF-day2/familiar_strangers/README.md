# Familiar Strangers
We need to submit two integer to pass the challenge.

According to the circuit, the core of the problem is `LessThan()` template in the `comparators.circom`.
```js
template LessThan(n) {
    assert(n <= 252);
    signal input in[2];
    signal output out;

    component n2b = Num2Bits(n+1);

    n2b.in <== in[0]+ (1<<n) - in[1];

    out <== 1-n2b.out[n];
}
```
It requires a parameter `n` representing the number of bits, then use the highest bit of $a+(1\ll n)-b$ to determine whether $a\< b$ or $a\geq b$.(notice that the `Num2Bits` template will check the input after transform the number to bit array, so the `n2b.in` can't be larger than $1\ll n+1$)

In Level1, we have:

$$
n2b(in + (1\ll 201) - 6026017665971213533282357846279359759458261226685473132380160)[201] = 0
$$

$$
n2b((1\ll 201) - 401734511064747568885490523085290650630550748445698208825344 - x)[201] = 0
$$

more obviously:

$$
n2b(in - 2812141577453232982198433661597034554413855239119887461777408)[201] = 0
$$

$$
n2b(2812141577453232982198433661597034554413855239119887461777408 - in)[201] = 0
$$

So the answer to the level1 is $2812141577453232982198433661597034554413855239119887461777408$.

In Level2, we have:

$$
n2b(in - 5897488333439202455083409550285544209858125342430750230241414742016)[241] = 0
$$

$$
n2b(5897488333439202455083409550285544209858125342430750230241414742016 - in)[251] = 0
$$

So we take it for granted that the answer is $5897488333439202455083409550285544209858125342430750230241414742016$.

But it will tell you that your answer is incorrect, the web server limits the input length of level2:
```js
// Level 2
if (inputs.l2.length <= 70) {
    res.json({ success: false });
    return;
}
```
It seems impossible to pass using a number longer than 70, but it should be noted that all operations on the circuit are on finite fields, so we only need to superpose the modulus to make it reach the specified length. The answer of level2 is $21888242877736763555685608200340684638833908610274159686128954416817223237633$.
