[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)]( https://github.com/Vara-Lab/Bootcamp-Colledge-2024.git)


# Colledge Bootcamp 2024

Welcome to the Colledge Bootcamp 2024. In this bootcamp, we will explore frontend development and the deployment of smart contracts using Sails and the Vara network. This repository contains all the resources and guides needed to complete the bootcamp.

# Frontend

In this section, you will learn how to build modern frontend applications. Follow the steps below to get started.

### Step 1: Navigate to the Frontend Directory

```bash
cd frontend
```

### Step 2: Install Dependencies

Make sure you have Node.js and npm installed. Then, run the following command to install the project dependencies:

```bash
yarn install
```

### Step 3: Start the Development Server

To start the development server and see your project in action, run:

```bash
yarn start
```


# Sails Contracts on Vara Network

## Deploy the Contract on the IDEA Platform and Interact with Your Contract

## Step 1: Open Contract on Gitpod

<p align="center">
  <a href="https://gitpod.io/#https://github.com/Vara-Lab/Bootcamp-Colledge-2024.git" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

## Step 2: Compile and Deploy the Smart Contract

### Compile the smart contract by running the following command:

```bash
cargo build --release
```

Once the compilation is complete, locate the `*.opt.wasm` file in the `target/wasm32-unknown-unknown/release` directory.

## Step 3: Interact with Your Contract on Vara Network

1. Access [Gear IDE](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.
2. Connect your Substrate wallet to Gear IDE.
3. Upload the `*.opt.wasm` and `metadata.txt` files by clicking the "Upload Program" button.

Thank you for participating in the Colledge Bootcamp 2024! If you have any questions or need assistance, feel free to contact us.