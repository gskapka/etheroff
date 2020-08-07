pub const USAGE_INFO: &str = "
❍ Etheroff ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for offline signging of ethereum transactions.

❍ Usage ❍

Usage:  etheroff [--help]
        etheroff version
        etheroff signTransaction <to> <value> <nonce> [--gasLimit=<uint>] [--gasPrice=<uint>] [--chainId=<uint>] [--keyfile=<path>] [--logLevel=<string>]

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
        --gasLimit=<uint>      ❍ The gas limit for the ETH transaction. [default: 21000]
        --gasPrice=<uint>      ❍ The gas price for the ETH transaction. [default: 20000000000]
        --logLevel=<level>     ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`,
                                 `trace` or `error`. [default: none]

";
