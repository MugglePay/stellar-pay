import { Box, Input, Stack, Text } from "@chakra-ui/react";
import { useSorobanReact } from "@soroban-react/core";
import { useEffect, useState } from "react";
import { Alert } from "@/components/ui/alert";
import { Field } from "@/components/ui/field";
import { toaster } from "@/components/ui/toaster";
import {
  NumberInputField,
  NumberInputRoot,
} from "@/components/ui/number-input";
import { Button } from "@/components/ui/button";
import { CONFIG } from "@/config";
import {
  contractInvoke,
  contractTransaction,
  useSendTransaction,
} from "@soroban-react/contracts";
import {
  Account,
  Contract,
  Horizon,
  nativeToScVal,
  xdr,
} from "@stellar/stellar-sdk";
import {
  addressToScVal,
  decodei128ScVal,
  i128ToScVal,
  scvalToBigNumber,
} from "@/helpers/utils";
import { scValToJs } from "@/helpers/convert";

const SorobanSwapForm = () => {
  const sorobanContext = useSorobanReact();
  const { sendTransaction } = useSendTransaction();
  const { address, activeChain } = sorobanContext;

  const [amount, setAmount] = useState<string>("0");
  const [recipient, setRecipient] = useState<string>();
  const [loading, setLoading] = useState<boolean>(false);
  const [env, setEnv] = useState<any>(
    CONFIG[activeChain?.id as keyof typeof CONFIG]
  );

  const server = new Horizon.Server(env.rpcUrl);

  async function getExpirationLedger(ledgerOffset = 60) {
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

  const getExpectedAmount = async (amount: number) => {
    try {
      const contract = new Contract(env.contractAddress);
      const result = await contractInvoke({
        contractAddress: contract.contractId(),
        method: "get_expected_amount",
        args: [
          i128ToScVal(BigInt(Math.floor(amount * 10_000_000))),
          addressToScVal(env.nativeToken.address),
          addressToScVal(env.outputToken.address),
        ],
        sorobanContext,
      });

      const val = scValToJs(result as xdr.ScVal);

      return val;
    } catch (error: any) {
      throw new Error(error.message ?? "Failed to get expected amount");
    }
  };

  const doSwap = async (amount: number) => {
    try {
      if (address) {
        const accountDetails = await server?.loadAccount(address);
        const source = await new Account(
          address,
          String(accountDetails?.sequence ?? 0)
        );
        const expirationLedger = await getExpirationLedger();

        const args = [
          // Customer Address
          addressToScVal(address),
          // Merchant Address
          addressToScVal(address),
          // Convert amount to i128 ScVal with 7 decimal places (10,000,000 multiplier)
          i128ToScVal(BigInt(Math.floor(amount * 10_000_000))),
          // Input token address
          addressToScVal(env.nativeToken.address),
          // Output token address
          addressToScVal(env.outputToken.address),
          nativeToScVal(expirationLedger, { type: "u32" }),
        ];

        const contract = new Contract(env.contractAddress);
        const tx = await contractTransaction({
          networkPassphrase: env.passphrase,
          source,
          contractAddress: contract.contractId(),
          method: "swap",
          args,
        });

        const result = await sendTransaction(tx, { sorobanContext });
        console.log(result);
      }
    } catch (error: any) {
      throw new Error(error.message ?? "Failed to do swap");
    }
  };

  const onSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setLoading(true);

    try {
      const amountInXlm = Number(amount);
      if (amountInXlm <= 0) throw new Error("Amount must be greater than 0");

      if (address) {
        // const expectedAmount = await getExpectedAmount(amountInXlm);
        const swappedAmount = await doSwap(amountInXlm);
        console.log(swappedAmount);
      }
    } catch (error: any) {
      setLoading(false);
      toaster.create({
        title: "Swap Error",
        description: error.message ?? "Failed to swap tokens",
        type: "error",
      });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (activeChain) {
      setEnv(CONFIG[activeChain.id as keyof typeof CONFIG]);
    }
  }, [activeChain]);

  return (
    <Box display="flex" flexDir="column" gap={2}>
      <Alert status="info" width="full" title="Connected Account">
        <Text
          fontSize="sm"
          style={{ wordBreak: "break-all", textDecoration: "underline" }}
        >
          {address}
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
};

export default SorobanSwapForm;
