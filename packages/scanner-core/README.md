# massa-sc-scanner-core

This package contains the core functionality for the Massa smart contract bytecode scanner.

## Introduction

Massa smart contracts are written in WebAssembly (WASM) and are executed in a sandboxed environment. This package provides the core functionality for scanning the WASM bytecode of a smart contract to extract information about it.

This package is written in Rust and then transformed to TypeScript using `wasm-pack`.
While we do release this package on its own, it is primarily meant to be used as a dependency for the `@quartz-technology/massa-sc-scanner-sdk` npm package.

## Installation

To install this package, you can use npm:

```bash
npm install @quartz-technology/massa-sc-scanner-core
```

## Usage

Below is an example of how to use this package:

```typescript
import fs from 'fs';
import { Scanner } from '@quartz-technology/massa-sc-scanner-core';

// Here, index.wasm is the compiled Massa smart contract.
const input = fs.readFileSync('index.wasm');
const scanner = new wasm.Scanner(input);

// Extract the smart contract information.
const exported_function_names = scanner.exported_function_names();
const host_functions = scanner.host_functions();

// Log the extracted information!
console.log(exported_function_names);
console.log(host_functions);
```

## Features

- Extracts the names of the smart contract's exported functions.
- Extracts the names of the host functions used in the smart contract.

## Contributing

We welcome contributions from everyone. Please refer to the [contributing guidelines](../../CONTRIBUTING.md) for more information.

## Acknowledgements

We would like to thank the following entities for their amazing work. Without them, this project would not be possible.

- [Bytecode Alliance](https://github.com/bytecodealliance)
- [Massa](https://massa.net/)

## Authors

This project is maintained by the 📡 at [Quartz](https://quartz.technology).

It is being developed with the help of [0xtekgrinder](https://github.com/0xtekgrinder).
