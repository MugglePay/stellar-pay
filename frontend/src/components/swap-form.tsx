import { useStellar } from "@/providers/stellar.provider";
import { Box, Text, Stack, Input } from "@chakra-ui/react";
import { Alert } from "./ui/alert";
import { Field } from "./ui/field";

import {
  NumberInputField,
  NumberInputRoot,
} from "@/components/ui/number-input";

import { useState } from "react";
import { Button } from "./ui/button";
import { toaster } from "./ui/toaster";
import { FREIGHTER_ID } from "@creit.tech/stellar-wallets-kit";
import {
  Account,
  Asset,
  BASE_FEE,
  nativeToScVal,
  Operation,
  TransactionBuilder,
} from "@stellar/stellar-sdk";
import { addressToScVal, i128ToScVal } from "@/helpers/utils";
import axios from "axios";
import { get } from "lodash";

function SwapForm() {
  const { kit, account, publicKey, server, wallet, env } = useStellar();
  const [amount, setAmount] = useState<string>("0");
  const [recipient, setRecipient] = useState<string>();
  const [loading, setLoading] = useState<boolean>(false);

  const { contractAddress, nativeToken, outputToken, passphrase, rpcUrl } = env;

  async function getExpirationLedger(ledgerOffset = 100) {
    try {
      // Get latest ledger
      const ledger = await server?.ledgers().order("desc").limit(1).call();

      const currentLedger = ledger?.records[0].sequence;
      const expirationLedger = (currentLedger ?? 0) + ledgerOffset;

      return expirationLedger;
    } catch (error) {
      throw error;
    }
  }

  const doSwap = async (amount: number) => {
    try {
      if (account && publicKey) {
        const accountDetails = await server?.loadAccount(publicKey);
        const expirationLedger = await getExpirationLedger();
        const args = [
          // Customer Address
          addressToScVal(publicKey),
          // Merchant Address
          addressToScVal(publicKey),
          // Convert amount to i128 ScVal with 7 decimal places (10,000,000 multiplier)
          i128ToScVal(BigInt(Math.floor(amount * 10_000_000))),
          // Input token address
          addressToScVal(nativeToken.address),
          // Output token address
          addressToScVal(outputToken.address),
          nativeToScVal(expirationLedger, { type: "u32" }),
        ];
        const source = await new Account(
          publicKey,
          String(accountDetails?.sequence ?? 0)
        );

        const transaction = new TransactionBuilder(source, {
          fee: BASE_FEE,
          networkPassphrase: passphrase,
        })
          .addOperation(
            Operation.invokeContractFunction({
              contract: contractAddress,
              function: "swap",
              args: args,
            })
          )
          .setTimeout(30)
          .setNetworkPassphrase(passphrase)
          .build();

        // Specific handling for Freighter wallet
        const signedTx = await kit?.signTransaction(transaction.toXDR(), {
          address: publicKey,
          networkPassphrase: passphrase,
        });

        if (signedTx && signedTx.signedTxXdr) {
          const submittedTx = await server?.submitTransaction(
            TransactionBuilder.fromXDR(signedTx.signedTxXdr, passphrase)
          );

          return submittedTx;
        }
      }

      throw new Error("Failed to sign transaction");
    } catch (error: any) {
      throw error;
    }
  };

  const doTransfer = async (amount: number) => {
    try {
      if (account && publicKey) {
        const accountDetails = await server?.loadAccount(publicKey);
        const source = await new Account(
          publicKey,
          String(accountDetails?.sequence ?? 0)
        );
        const asset = new Asset(outputToken.code, outputToken.issuer);

        const opt = Operation.payment({
          destination: recipient ?? "",
          asset,
          amount: String(amount),
        });

        const transaction = new TransactionBuilder(source, {
          fee: BASE_FEE,
          networkPassphrase: passphrase,
        })
          .addOperation(opt)
          .setTimeout(180)
          .build();

        const xdr = transaction.toXDR();
        const signTrans = await kit?.signTransaction(xdr, {
          address: publicKey,
          networkPassphrase: passphrase,
        });

        if (signTrans?.signedTxXdr) {
          const submittedTx = await server?.submitTransaction(
            TransactionBuilder.fromXDR(signTrans.signedTxXdr, passphrase)
          );

          return submittedTx;
        }

        throw new Error("Failed to sign transaction");
      }
    } catch (error: any) {
      throw error;
    }
  };

  const getPaymentAmount = async (txHash: string) => {
    const req = await axios.get(`${rpcUrl}/transactions/${txHash}/payments`, {
      params: {
        limit: 1,
        order: "desc",
      },
      maxBodyLength: Infinity,
      headers: {
        "Content-Type": "application/json",
      },
    });

    const records = get(req, "data._embedded.records", []);

    if (records.length > 0) {
      const record = records[0];
      const asset = record.asset_balance_changes.filter(
        (change: any) =>
          change.asset_code === outputToken.code &&
          change.asset_type === "credit_alphanum4"
      );

      if (asset.length > 0) {
        const assetChange = asset[0];
        const amount = assetChange.amount;

        return amount;
      }
    }

    return null;
  };

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setLoading(true);

    try {
      const amountInXlm = Number(amount);
      if (amountInXlm <= 0) throw new Error("Amount must be greater than 0");

      if (account && publicKey && kit) {
        await kit?.setWallet(String(wallet?.id ?? FREIGHTER_ID));

        const swapTx = await doSwap(amountInXlm);

        if (swapTx) {
          const swappedAmount = await getPaymentAmount(swapTx?.hash);

          if (swappedAmount !== null) {
            // Do transfer transaction using USDC amount
            await doTransfer(swappedAmount);

            setLoading(false);
            toaster.create({
              title: "Swap and Transfer Success",
              description: "Tokens transferred successfully",
              type: "success",
            });
          }
        } else {
          throw new Error("Failed to swap tokens");
        }
      }
    } catch (error: any) {
      setLoading(false);
      toaster.create({
        title: "Swap Error",
        description: error.message ?? "Failed to swap tokens",
        type: "error",
      });
    }
  };

  return (
    <Box display="flex" flexDir="column" gap={2}>
      <Alert status="info" width="full" title="Connected Account">
        <Text
          fontSize="sm"
          style={{ wordBreak: "break-all", textDecoration: "underline" }}
        >
          {publicKey}
        </Text>
      </Alert>

      <form onSubmit={onSubmit}>
        <Stack gap={2}>
          <Field label="Enter Recipient Address" w="full">
            <Input
              value={recipient}
              onChange={(e) => setRecipient(e.target.value)}
            />
          </Field>
          <Field label="Enter XLM Amount" w="full">
            <NumberInputRoot
              w="full"
              defaultValue={amount}
              step={0.01}
              min={0.01}
              onValueChange={(e) => setAmount(e.value)}
            >
              <NumberInputField />
            </NumberInputRoot>
          </Field>
          <Button
            type="submit"
            colorPalette="green"
            disabled={!recipient || !amount}
            loading={loading}
          >
            Transfer
          </Button>
        </Stack>
      </form>
    </Box>
  );
}

export default SwapForm;
