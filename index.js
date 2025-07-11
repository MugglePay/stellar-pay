const StellarSdk = require('@stellar/stellar-sdk');

// === ENVIRONMENT CONFIG ===
const HORIZON_SERVER = "https://horizon.stellar.org";
const NETWORK_PASSPHRASE = StellarSdk.Networks.PUBLIC;

// === ACCOUNTS ===
const PAYMASTER_SECRET = "SCCLO63G65CB3ZCT6D2QPJU3G27QXJGRZFQZ3XTFAEMKCTFGNWIQIW5Z";
const BOB_ADDRESS = "GCVE5VUTIMCSUOHOIJVOEEBTM2SWNMLB4QKMM2GBPODZPE56TG3REXSN";

// === TOKENS ===
const USDC_ISSUER = "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN";
const USDC_CODE = "USDC";

// === SWAP CONFIG ===
const MAX_XLM_AMOUNT = "10"; // Maximum 10 XLM per swap for safety
const AMOUNT_IN = "1.4000000";  // 1.4 XLM (keeping 0.6 XLM for reserves)
const MIN_DEST_AMOUNT = "0.4118376"; // Minimum USDC to receive

const swap = async () => {
    try {
        // Safety check for amount
        if (parseFloat(AMOUNT_IN) > parseFloat(MAX_XLM_AMOUNT)) {
            throw new Error(`Amount exceeds maximum allowed (${MAX_XLM_AMOUNT} XLM)`);
        }

        console.log("Starting swap with params:", {
            amountIn: AMOUNT_IN,
            fromToken: "XLM",
            toToken: USDC_CODE,
            minDestAmount: MIN_DEST_AMOUNT,
            destinationAccount: BOB_ADDRESS
        });

        const server = new StellarSdk.Horizon.Server(HORIZON_SERVER, {
            allowHttp: true
        });

        const paymasterKeypair = StellarSdk.Keypair.fromSecret(PAYMASTER_SECRET);
        const paymasterPublicKey = paymasterKeypair.publicKey();
        console.log("Paymaster account:", paymasterPublicKey);
        console.log("Destination (Bob's) account:", BOB_ADDRESS);

        // Create asset objects
        const sourceAsset = StellarSdk.Asset.native(); // XLM
        const destinationAsset = new StellarSdk.Asset(USDC_CODE, USDC_ISSUER);

        // Function to build transaction with fresh sequence number
        const buildTransaction = async () => {
            const freshAccount = await server.loadAccount(paymasterPublicKey);

            // Get current market price to calculate expected amount
            try {
                const paths = await server
                    .strictSendPaths(sourceAsset, AMOUNT_IN, [destinationAsset])
                    .call();

                if (paths.records && paths.records.length > 0) {
                    const expectedAmount = paths.records[0].destination_amount;
                    console.log("Expected USDC amount:", expectedAmount);
                }
            } catch (pathError) {
                console.warn("Failed to fetch optimal path:", pathError);
            }

            const transaction = new StellarSdk.TransactionBuilder(freshAccount, {
                fee: (await server.fetchBaseFee()).toString(),
                networkPassphrase: NETWORK_PASSPHRASE
            })
                .addOperation(
                    StellarSdk.Operation.pathPaymentStrictSend({
                        sendAsset: sourceAsset,
                        sendAmount: AMOUNT_IN,
                        destination: BOB_ADDRESS, // Always send to Bob's address
                        destAsset: destinationAsset,
                        destMin: MIN_DEST_AMOUNT,
                        path: []
                    })
                )
                .setTimeout(30)
                .build();

            transaction.sign(paymasterKeypair);
            return transaction;
        };

        // Submit transaction with retry logic
        let retries = 5;
        let lastError;

        while (retries > 0) {
            try {
                console.log(`Attempt ${6 - retries} of 5`);
                const transaction = await buildTransaction();
                const result = await server.submitTransaction(transaction);
                console.log("txn ok");
                console.log("Transaction hash:", result.hash);
                return result;
            } catch (err) {
                lastError = err;
                const resultCodes = err?.response?.data?.extras?.result_codes;
                console.error("Transaction failed:", {
                    transaction: resultCodes?.transaction,
                    operations: resultCodes?.operations,
                    message: err.message
                });

                // Check if we should retry
                const shouldRetry =
                    resultCodes?.transaction === "tx_bad_seq" ||
                    resultCodes?.transaction === "tx_too_late";

                if (shouldRetry && retries > 1) {
                    console.log("Retrying with fresh sequence number...");
                    await new Promise(resolve => setTimeout(resolve, 2000));
                    retries--;
                    continue;
                }

                if (resultCodes?.operations?.includes("op_under_dest_min")) {
                    throw new Error("Insufficient output amount for swap. Try adjusting the minimum received amount.");
                }

                // If we're out of retries or it's not a retriable error
                if (retries === 1 || !shouldRetry) {
                    throw new Error(`Swap failed: ${resultCodes?.transaction || err.message}`);
                }

                retries--;
                await new Promise(resolve => setTimeout(resolve, 2000));
            }
        }

        throw lastError || new Error("Max retries exceeded");
    } catch (err) {
        console.error("Swap error:", {
            message: err.message,
            response: err.response?.data,
            resultCodes: err.response?.data?.extras?.result_codes
        });
        throw err;
    }
};

// Run the swap
swap().catch(console.error);
