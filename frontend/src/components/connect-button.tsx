import { useStellar } from "@/providers/stellar.provider";
import { Button } from "./ui/button";
import { ISupportedWallet } from "@creit.tech/stellar-wallets-kit";
import { toaster } from "./ui/toaster";
import { useState } from "react";

function ConnectButton() {
  const { kit, setPublicKey, publicKey, setWallet } = useStellar();
  const [isConnecting, setIsConnecting] = useState(false);

  const onConnect = async () => {
    if (!kit) {
      toaster.create({
        title: "Wallet Error",
        description: "Stellar wallet kit is not initialized",
        type: "error",
      });
      return;
    }

    try {
      setIsConnecting(true);

      await kit.openModal({
        onWalletSelected: async (wallet: ISupportedWallet) => {
          try {
            await kit.setWallet(wallet.id);
            const { address } = await kit.getAddress();

            setPublicKey(address);
            setWallet(wallet);

            toaster.create({
              title: "Wallet Connected",
              description: `Connected with ${wallet.name}`,
              type: "success",
            });
          } catch (walletError: any) {
            toaster.create({
              title: "Wallet Connection Error",
              description: walletError.message ?? "Failed to connect wallet",
              type: "error",
            });
          }
        },
      });
    } catch (error: any) {
      toaster.create({
        title: "Connection Error",
        description: error.message ?? "Failed to connect to Stellar",
        type: "error",
      });
    } finally {
      setIsConnecting(false);
    }
  };

  const onDisconnect = async () => {
    try {
      await kit?.disconnect();
      setPublicKey(undefined);

      toaster.create({
        title: "Wallet Disconnected",
        description: "Successfully disconnected from wallet",
        type: "info",
      });
    } catch (error: any) {
      toaster.create({
        title: "Disconnect Error",
        description: error.message ?? "Failed to disconnect wallet",
        type: "error",
      });
    }
  };

  return (
    <Button
      onClick={() => (publicKey ? onDisconnect() : onConnect())}
      colorPalette={publicKey ? "red" : "gray"}
      loading={isConnecting}
      disabled={isConnecting}
    >
      {publicKey ? "Disconnect" : "Connect to Wallet"}
    </Button>
  );
}

export default ConnectButton;
