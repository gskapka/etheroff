# :heart_eyes_cat: Etheroff

A simple CLI for offline signing of ethereum transactions.

&nbsp;

***

&nbsp;

## :page_with_curl: Usage:

**1)** Clone the repo:

__`❍ git clone https://github.com/gskapka/etheroff.git`__

**2)** Enter app dir and build it:

__`❍ cd etheroff && cargo b --release`__

**3)** You'll find the __`etheroff`__binary in:

__`❍ cd ./target/release`__

**4)** To use the binary itself see the following:

```
❍ Etheroff ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for offline signing of ethereum transactions.

❍ Usage ❍

Usage:  etheroff [--help]
        etheroff version
        etheroff signTransaction <to> <value> <nonce> [--gasLimit=<uint>] [--gasPrice=<uint>] [--data=<bytes>] [--chainId=<uint>] [--keyfile=<path>] [--logLevel=<string>]

Commands:

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

## :clap: Example

You'll find a sample script in the __`./example/`__ directory. Run it to see the tool in action. The sample private key is symmetrically encrypted with the password __`etheroff`__. Output of example:

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
