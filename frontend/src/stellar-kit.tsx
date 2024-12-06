import { Box, Card } from "@chakra-ui/react";
import { useStellar } from "@/providers/stellar.provider";
import ConnectButton from "@/components/connect-button";
import SwapForm from "@/components/swap-form";
import { Alert } from "@/components/ui/alert";

function StellarKit() {
  const { publicKey, env } = useStellar();

  return (
    <Box
      minH="100vh"
      display="flex"
      flexDir="column"
      gap={4}
      justifyContent="center"
      alignItems="center"
      maxW="md"
      mx="auto"
    >
      <Alert status="info" width="full" title="Current Network">
        You are connected to the{" "}
        <strong>{env?.testMode ? "Testnet" : "Mainnet"}</strong>
      </Alert>
      <Card.Root w="full">
        <Card.Header>
          <Card.Title>Stellar Token Swap (XLM to USDC)</Card.Title>
          <Card.Description>
            Connect your Stellar wallet to start swapping tokens
          </Card.Description>
        </Card.Header>
        <Card.Body display="flex" flexDir="column" gap={4}>
          {publicKey && <SwapForm />}
          <ConnectButton />
        </Card.Body>
      </Card.Root>
    </Box>
  );
}

export default StellarKit;
