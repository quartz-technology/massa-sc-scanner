# massa-sc-scanner-sdk

This package contains the implementation of the Massa smart contract bytecode scanner SDK.

## Introduction

The Massa smart contract bytecode scanner SDK is a tool that allows you to extract information from smart contract bytecode.
It is currently in development and more features will be added in the future.

## Installation

To install this package, you can use npm:

```bash
npm install @quartz-technology/massa-sc-scanner-sdk
```

## Usage

Below is an example of how to use this package:

```typescript
import { ScannerSDK } from "@quartz-technology/massa-sc-scanner-sdk";

const sdk = new ScannerSDK("https://buildnet.massa.net");
(async () => {
    const res = await sdk.scanSmartContract("AS1sKBEGsqtm8vQhQzi7KJ4YhyaKTSkhJrLkRc7mQtPqme3VcFHm");

    console.log(res);
})();
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
