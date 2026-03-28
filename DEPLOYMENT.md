# Deployment Documentation for NEAR

This document provides comprehensive instructions for deploying to both NEAR testnet and mainnet.

## Prerequisites
Before deploying, ensure that you have the following:
- An active NEAR account.
- NEAR CLI installed on your local machine.
- The project codebase ready for deployment.

## Deployment to NEAR Testnet
1. **Set up NEAR CLI for Testnet**:
   - Run the command:
     ```
     near login --networkId=testnet
     ```
   - This will open a browser window for you to authenticate your NEAR account.

2. **Build the Project**:
   - Execute the following commands to build your project:
     ```bash
     npm install
     npm run build
     ```

3. **Deploy to Testnet**:
   - Use the following command to deploy:
     ```
     near deploy --accountId your-testnet-account.testnet --wasmFile path/to/your.wasm
     ```
   - Replace `your-testnet-account` with your actual testnet account name.

## Deployment to NEAR Mainnet
1. **Set up NEAR CLI for Mainnet**:
   - Run the command:
     ```
     near login --networkId=mainnet
     ```
   - Again, follow the authentication instructions in the browser window.

2. **Build the Project**:
   - As with the testnet, use the following commands to build your project:
     ```bash
     npm install
     npm run build
     ```
   
3. **Deploy to Mainnet**:
   - Use the following command to deploy:
     ```
     near deploy --accountId your-mainnet-account.mainnet --wasmFile path/to/your.wasm
     ```
   - Make sure to replace `your-mainnet-account` with your actual mainnet account name.

## Conclusion
Following these steps will ensure a successful deployment to both NEAR testnet and mainnet.