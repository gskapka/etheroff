# :heart_eyes_cat: Etheroff

A simple CLI for offline signing of ethereum transactions.

&nbsp;

***

&nbsp;

## :page_with_curl: Build Instructions

**1)** Clone the repo:

__`❍ git clone https://github.com/gskapka/etheroff.git`__

**2)** Enter app dir and build it:

__`❍ cd etheroff && cargo +nightly b --release`__

**3)** You'll find the __`etheroff`__ binary in:

__`❍ cd ./target/release`__

&nbsp;

***

&nbsp;

## :wrench: Usage Instructions


:exclamation: __NOTE:__ This tool requires you have a __[gpg encrypted](https://www.howtogeek.com/427982/how-to-encrypt-and-decrypt-files-with-gpg-on-linux/)__ ethereum private key in __hex format__. There is a sample private key in the __`./example`__ directory which is symmetrically encrypted with the password __`etheroff`__ for you to use to experiment with the tool!

To see the full usage instructions, run the binary with the help flag (__`etheroff --help`__):

```
❍ Etheroff ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for offline signing of ethereum transactions.

❍ Usage ❍

Usage:  etheroff [--help]
        etheroff version
        etheroff [--keyfile=<path>]
        etheroff signTransaction <to> <value> <nonce> [--gasLimit=<uint>] [--gasPrice=<uint>] [--data=<bytes>] [--chainId=<uint>] [--keyfile=<path>] [--logLevel=<string>]

Commands:

        <no-command>           ❍ Run the interactive Q&A version of this tool.
        version                ❍ Show version info.
        signTransaction        ❍ Sign an ethereum transaction.
        <to>                   ❍ Ethereum address to send the transaction to.
        <nonce>                ❍ The nonce to be used for the ethereum transaction.
        <value>                ❍ How much ether to send (in Wei!)

Options:

        --help                 ❍ Show this message.
        --keyfile=<path>       ❍ Path to GPG-encrypted ETH private key in hex format. [default: ./pk.gpg]
        --chainId=<uint>       ❍ The integer used to identify which chain to sign a transaction for.  [default: 1]
        --data=<bytes>         ❍ Any calldata required for the transaction, in hex format. [default: 0x]
        --gasLimit=<uint>      ❍ The gas limit for the ETH transaction. [default: 21000]
        --gasPrice=<uint>      ❍ The gas price for the ETH transaction. [default: 20000000000]
        --logLevel=<level>     ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`,
                                 `trace` or `error`. [default: none]
```

&nbsp;

***

&nbsp;

## :clap: Interactive Example:

Once you've built the tool, if you run the binary with NO commands (__`etheroff`__) you'll start the interactive version of the tool which walks you though each required part of an ethereum transaction, step-by-step:

```
❍ Etheroff - An ethereum offline transaction signer!
❍ Decrypting ethereum private key...
❍ Please enter the ethereum address to send to:
0xfEDFe2616EB3661CB8FEd2782F5F0cC91D59DCaC
✔ ETH address: 0xfedf…dcac
❍ Please enter the amount of ethereum to send (in ETH):
1.23
✔ Amount: 1.23 Ξ
❍ Please enter the nonce for the transaction:
1337
✔ Nonce: 1337
❍ Please enter the gas limit for the transaction:
21000
✔ Gas limit: 21000
❍ Please enter the gas price for the transaction (in GWEI):
60
✔ Gas price: 60 GWei
❍ Please select which network to use:
1 - Mainnet
2 - Kovan
3 - Goerli
4 - Ropsten
5 - Rinkeby
1
✔ Mainnet
❍ Please enter any calldata you want included (or leave blank for none!):

✔ ETH calldata: 0x

❍ Transaction details:
To:         0xfedf…dcac
Amount:     1.23 Ξ (1230000000000000000 Wei)
Nonce:      1337
Gas limit:  21000
Gas price:  60 GWei (60000000000 Wei)
Network:    Mainnet
Calldata:   0x
Total cost: 1 Ξ (1231260000000000000 Wei)

❍ Sign transaction? (y/n)
y
f86e820539850df847580082520894fedfe2616eb3661cb8fed2782f5f0cc91d59dcac881111d67bb1bb00008026a09d76015c4eda9937b84c0e02045bbeedf2498c24e8a1f4a04e8c9e5208de5078a04a97a0e04a20161c32b7e99b1e5a9af309e95a2988edeca6b39401d2435949e2
```

&nbsp;

***

&nbsp;

## :computer: Programmatic Example

You'll find a sample script in the __`./example/`__ directory. Run it to see the tool in action:

```
11:18:23 [ INFO] ✔ Logger initialized successfully!
11:18:23 [ INFO] ✔ Signing transaction...
11:18:23 [ INFO] ✔ Maybe getting ETH private key & adding to state...
11:18:23 [ INFO] ✔ Checking ETH private keyfile exists...
11:18:23 [ INFO] ✔ Key file found @ ./example-private-key.gpg!
11:18:23 [ INFO] ✔ Decrypting private key...
11:18:26 [ INFO] ✔ Keyfile decrypted!
11:18:26 [ INFO] ✔ Creating Eth private key from hex...
11:18:26 [ INFO] ✔ ETH address: 0xd4f003e122e94982164f3d63ff68467e5c676e9e
11:18:26 [ INFO] ✔ Signing ETH transaction...
11:18:26 [ INFO] ✔ To: 0xfedf…dcac
11:18:26 [ INFO] ✔ On Ropsten Testnet
11:18:26 [ INFO] ✔ Using nonce: 1337
11:18:26 [ INFO] ✔ For amount: 1 Wei
11:18:26 [ INFO] ✔ With gas limit of: 30,000
11:18:26 [ INFO] ✔ And a gas price of: 60,000,000,000 Wei
11:18:26 [ INFO] ✔ With calldata: 0x0decaf
11:18:26 [ INFO] ✔ For a total price of : 0.001800000000000001 ETH
f869820539850df847580082753094fedfe2616eb3661cb8fed2782f5f0cc91d59dcac01830decaf2aa0c8a9434fa0775488d27f5395bcd4e1180b0b67800d58bc3a69f1e9071e45d3eba06084d3d8e723188c441403e0ecd6d7cd894346730a28d557eda393fc2eb0fda3
```

:exclamation: __NOTE:__ The above example has logging turned on. The tool will _default_ to no logging and so _only_ the final signed transaction will be outputted to stdout. Perfect for programmatic use-cases.
