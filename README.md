# 🧮 Risk Alfred Workflow

A simple [Alfred Workflow](https://www.alfredapp.com/workflows/) to calculate required tokens to only "risk" a set dollar amount.

> Big thank you to [@rossmacarthur](https://github.com/rossmacarthur) for creating [powerpack ⚡️](https://github.com/rossmacarthur/powerpack) 👏🏽

Arguments:

1. Entry Price
2. Stop-Loss Price
3. Dollar Risk

## 📦 Installation

### Pre-packaged

Grab the latest release from
[the releases page](https://github.com/derekahn/risk.alfredworkflow/releases).

Because the release contains an executable binary later versions of macOS will mark it as untrusted.
You can run the following to explicitly trust the release before installing to Alfred.

```bash
xattr -c ~/Downloads/risk-*-x86_64-apple-darwin.alfredworkflow
```

### Building from source

This workflow is written in Rust, so to install it from source you will first
need to install Rust and Cargo using [rustup](https://rustup.rs/). Then install
[powerpack](https://github.com/rossmacarthur/powerpack). Then you can run the
following to build an `.alfredworkflow` file.

```bash
git clone https://github.com/derekahn/risk.alfredworkflow.git

cd risk.alfredworkflow

powerpack package
```

The release will be available at `target/workflow/risk.alfredworkflow`.

## 🪪 License

This project is distributed under the terms of the MIT license.
