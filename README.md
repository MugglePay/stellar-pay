# XLM to USDC Swap Script

This script allows you to swap XLM (Stellar's native token) to USDC on the Stellar network.

## Prerequisites

- Node.js installed
- A Stellar account with:
  - Sufficient XLM balance (minimum 1 XLM base reserve + amount you want to swap)
  - USDC trustline established
  - The account's secret key

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Create a `.env` file in the root directory:
   ```bash
   cp .env.example .env
   ```

3. Add your Stellar account's secret key to the `.env` file:
   ```
   PAYMASTER_SECRET=YOUR_STELLAR_SECRET_KEY
   ```
   ⚠️ Never commit your secret key to git!

## Funding Your Account

Before running the swap, ensure your account has:
1. Minimum 1 XLM for base reserve
2. Additional XLM for the amount you want to swap
3. USDC trustline established

To check your account balance and establish trustline:
1. Visit [Stellar Laboratory](https://laboratory.stellar.org/#explorer?resource=accounts)
2. Enter your public key
3. To establish USDC trustline, create a transaction with "Change Trust" operation for:
   - Asset Code: USDC
   - Issuer: GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN

To fund your account:
1. Transfer XLM to your account address
2. Wait for the transaction to confirm

## Configuration

Edit these values in `index.js`:
- `AMOUNT_IN`: Amount of XLM to swap (default: 1.4 XLM)
- `MIN_DEST_AMOUNT`: Minimum USDC to receive
- `MAX_XLM_AMOUNT`: Safety limit for maximum XLM to swap

## Usage

Run the script:
```bash
node index.js
```

The script will:
1. Check if the swap amount is within safety limits
2. Fetch current market price
3. Execute the swap
4. Retry up to 5 times if needed
5. Output the transaction hash on success

## Error Handling

Common errors:
- `op_underfunded`: Your account lacks sufficient XLM. Add more funds.
- `op_under_dest_min`: Price slippage too high. Adjust `MIN_DEST_AMOUNT`.
- `tx_failed`: Generic failure. Check account setup and balances. 