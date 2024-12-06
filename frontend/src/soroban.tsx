import { Box, Card } from "@chakra-ui/react";

import { useSorobanReact } from "@soroban-react/core";
import { Alert } from "./components/ui/alert";
import SorobanConnectButton from "@/components/soroban/connect-button";
import SorobanSwapForm from "@/components/soroban/swap-form";

function SorobanKit() {
  const { address, activeChain } = useSorobanReact();
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
        You are connected to the <strong>{activeChain?.name}</strong>
      </Alert>
      <Card.Root w="full">
        <Card.Header>
          <Card.Title>Stellar Token Swap (XLM to USDC)</Card.Title>
          <Card.Description>
            Connect your Stellar wallet to start swapping tokens
          </Card.Description>
        </Card.Header>
        <Card.Body display="flex" flexDir="column" gap={4}>
          {address && <SorobanSwapForm />}
          <SorobanConnectButton />
        </Card.Body>
      </Card.Root>
    </Box>
  );
}

export default SorobanKit;
