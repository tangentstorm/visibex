# visibex

bex is a rust crate for working with boolean expressions.
visibex is a rust http web server and vue frontend for bex.

see [vizibex.org](visibex.org) for a specification.

## setup

You need cargo for rust and npm for the js side.

```bash
cd bex-api
cargo build

cd ../vue-bex
npm install
```

## running

*backend:*

```bash
cd bex-api
cargo run
```

*frontend:*
```bash
cd vue-bex
npm run serve
```
