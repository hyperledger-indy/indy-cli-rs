## CLI for Indy Ledger

This is command line interface for Indy, which provides a distributed-ledger-based
foundation for self-sovereign identity. 

It provides the commands to:
* Manage wallets
* Manage pool configurations
* Manage DIDs
* Sending transactions to distributed ledger

### Table of contents
* [Installation](#installation)
* [Execution modes](#execution-modes)
* [Getting help](#getting-help)
* [Options](#options)
* [Compatibility with old Indy-CLI](#compatibility-with-old-indy-cli)
* [Migration of a wallet created by old Indy-CLI](#migration-of-a-wallet-created-by-old-indy-cli)
* [Troubleshooting](#troubleshooting)

### Installation
1. Go to the [Github Releases](https://github.com/hyperledger-indy/indy-cli-rs/releases) page.
1. Download archive with the last version of CLI depending on your system platform.
1. Unzip archive to the directory where you want to save CLI.
1. After unzip you will be able to execute binary file.

### Execution modes
CLI supports 2 execution modes:
* Interactive:
  * In this mode CLI reads commands from terminal interactively. 
  * To start this mode just run `indy-cli-rs` without params.
* Batch:
  * In this mode all commands will be read from a text file or pipe and executed in series. 
  * To start this mode run `indy-cli-rs <path-to-text-file>`. 
  * Batch mode supports the same commands as interactive mode. 
  * Note that by default if some command finishes with an error batch execution will be interrupted. 
    * To prevent this start command with `-`.
    * For example, `-wallet create test`. In this case the result of this command will be ignored. 
  * To make a comment in the batch script start the line with the `#` symbol.

### Getting help
* The most simple way is just start cli by `indy-cli-rs` command and put `help` command. 
* Also, you can refer to [CLI design document](docs/README.md) containing the list of commands and architecture overview.

### Options
* -h and --help - Print usage.
* --logger-config - Init logger according to a config file (default no logger initialized).
* --config - Define config file for CLI initialization. A config file can contain the following fields:
    * loggerConfig - path to a logger config file (is equal to usage of "--logger-config" option).
    * taaAcceptanceMechanism - transaction author agreement acceptance mechanism to be used when sending write transactions to the Ledger.
* --plugins - **DEPRECATED** Load plugins in Libindy (usage: <lib-1-name>:<init-func-1-name>,...,<lib-n-name>:<init-func-n-name>).

### Compatibility with old Indy-CLI.
* The names and parameters for all commands are preserved compared to the **old** Indy-CLI.
* Payment related commands and functionality are **not** included into **this** CLI.
* Pool Ledger created by the **old** Indy-CLI **can** be also opened using **this** CLI.
* Wallet created by the **old** Indy-CLI **cannot** be opened using **this** CLI due to different storage format.
* Wallet backup created by the **old** Indy-CLI **can** be imported using **this** CLI.

### Migration of a wallet created by old Indy-CLI
1. Run old CLI and create wallet backup
```
indy-cli> wallet open wallet_to_export key
indy-cli> wallet export export_path=/Users/home/backup export_key
indy-cli> wallet close
```
2. Run new CLI and import wallet
```
indy-cli-rs> wallet import wallet_imported key export_path=/Users/home/backup export_key
```

### Troubleshooting
CLI depends on `term` rust library that has a system dependency on terminfo database.
That is why CLI Debian package additionally installs `libncursesw5-dev` library.
More about it read [here](https://crates.io/crates/term) at `Packaging and Distributing` section.