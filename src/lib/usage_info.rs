pub const USAGE_INFO: &str = "
❍ Etheroff ❍

    Copyright: Greg Kapka 2020
    Questions: greg@kapka.co.uk

❍ Info ❍

A simple CLI for offline signging of ethereum transactions.

❍ Usage ❍

Usage:  etheroff [--help]
        etheroff version
        etheroff signTransaction [--chainId=<uint>] [--keyfile=<path>] [--logLevel=<string>]

Commands:

        version                ❍ Show version info.
        signTransaction        ❍ Sign an ethereum transaction.

Options:

        --help                 ❍ Show this message.
        --keyfile=<path>       ❍ Path to GPG-encrypted ETH private key in hex format. [default: ./pk.gpg]
        --chainId=<uint>       ❍ The integer used to identify which chain to sign a transaction for.  [default: 1]
        --logLevel=<level>     ❍ Define the level of logging in the tool's output as one of: `none`, `info`, `debug`,
                                 `trace` or `error`. [default: none]

";
