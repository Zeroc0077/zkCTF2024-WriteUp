# Mixer
The contract uses zk-merkle-tree to implement anonymous deposit and withdrawal logic similar to that of a bank. Our aim is to empty the entire bank.

Unfortunately, we don’t have any information for a zero-knowledge proof.

When it is deployed, it will provide you with seven transactions involving deposits and withdrawals:
```js
0xb23e9e64c9f65cd60d26cb38a046ce319a97af647576a6117537c05c045edaf8
0xd5ea7a8587df87245184254ae4a251d7dc25e4d008223cd2feb8e5a137b99ba8
0x2d31cf0dac31047df8288f93455a548ba648e200269a5d67461e81c24d068a44
0x240d60841a5df3c5462387e79fd359c451a3d5e21e6a9eeb6f4aa40efa4aad76
0x191b03720da867c7d3e754d450bfba6a4588cfa31e8462b2635d17bc4eb2f9c2
0x0c33045c695900762fd42c3ed396966c82f354b3adc688b7bff8feddfd5726cf
0x1061d70419683db5cbbbf81521982edd8b29dbaebab92c59e7ae4b5c2e277f50
```
We can get they calldata:
```js
1:
Possible methods:
- deposit(uint256)
------------
[000]: 041d6792ba5bd8f4d1472b2079cfd6cca661d012b82ca9e786c3e01ba7ab0105
2:
Possible methods:
- deposit(uint256)
------------
[000]: 1d05021d3ea974a2b57c3266b6680c93a60742932403331d6c1e5d513ce559af
3:
Possible methods:
- deposit(uint256)
------------
[000]: 064c9f243e59fafa66f06496caeebcb5e915d775f4cbc5e286911e66dcf6c327
4:
Possible methods:
- deposit(uint256)
------------
[000]: 08cb4a8fe0dd01727e8635c8669181e6704070a9e24ee814b92a6ec8a0dcfc20
5:
Possible methods:
- deposit(uint256)
------------
[000]: 2289219bd505eb1a244632d2631d63211252c03a71f9ae55b9d3c84223300e62
6:
Possible methods:
- withdraw(uint256[2],uint256[2][2],uint256[2],uint256,uint256,address)
------------
[000]: 00742354aa0fd4964945660fff0879277e292e9bc9e59843a46afc488c0f9a60
[020]: 0efa711c8768aa5498a0612f45e721aa9815780d8e89907cb04a2ed3b753a3ee
[040]: 131a8513b1141080b0558e29e0d7db14111675b3ae8992c196b84746f22e9218
[060]: 1fe66c3ea92a489864aa2d3dcdaef4676514ad75f23118bedd3a529f0ea2e8d2
[080]: 01013e63989a110b4aaf05760ad34ee26fd228ad8bae950e873c822cf0bf4f0c
[0a0]: 299cbf23aa74d960072b7720d385a4d4e5f0cf9459cf5acd684da0f508c046ed
[0c0]: 04773ef45b8829f5ac9f9ffd63eed5de39efc76985f9adde12580b4998b3b4cf
[0e0]: 2cd51172777fcc7907f058e86aa05f0e2ba79d04cb76eb6f6aadebab03907804
[100]: 043aac51b20bb647b3c82dae6be1eb31d857676454b30937b6a3415069355808
[120]: 0aff5e01b308b2bb69783573d9204d0aa434e816c73a4fe0abf7936dfd13d925
[140]: 000000000000000000000000bd10a9fc2a4d1a62ba8228f8d4940cd76a981080
7:
Possible methods:
- withdraw(uint256[2],uint256[2][2],uint256[2],uint256,uint256,address)
------------
[000]: 1a740c1a7c95fc4ee00a1c61b65ff299223fc4dc4150141b389a1206db3b7c9d
[020]: 26f54bd5d707f812bafc558322ff7547d266bfea12060bd7aae0f54e3581c154
[040]: 244d2c57ea65483a03802c0f3662ca58147093f6a68ee0fd6164f933138ac2da
[060]: 1a0c94d8474a86ede06846f8436fb1d39acf3b900f599b7e1c8b8a2fb0981570
[080]: 25f8b8260c4fce5370d0c5d47f65d574172ffe1c2a2b7b79edfa7a54b1d03182
[0a0]: 03c0ad185e242198be1223cb9afc6db97b265cfa31373511e8624b38e8c5954f
[0c0]: 2227233ebe9423faf7e8e4516eb7380e46d34e1b41f25adf2bc84e19782e9e13
[0e0]: 237f92fbaec959215818dfad6f8a20c7c195c7da306de39b29bc35401bf93423
[100]: 07db15df780cb0cb68d539e60a5d971957fae2f98ad209226705457445db580b
[120]: 0aff5e01b308b2bb69783573d9204d0aa434e816c73a4fe0abf7936dfd13d925
[140]: 0000000000000000000000001f60ae0be12a98d879e40330b8028a809afe3a95
```
Obviously, we need to focus on the last two transactions, the signature of `withdraw()`:
```solidity
function withdraw(
    uint256[2] calldata pA,
    uint256[2][2] calldata pB,
    uint256[2] calldata pC,
    uint256 nullifier,
    uint256 root,
    address recipient
) external
```
Staring at the value of nullifier, I remembered the article I had read about the [double-spend vulnerability](https://secbit.io/blog/2019/07/29/the-input-aliasing-bug-caused-by-a-contract-library-of-zksnarks/).

So the problem can be solved by reusing the `nullifier` plus the modulus.