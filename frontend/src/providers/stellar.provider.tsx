import { toaster } from "@/components/ui/toaster";
import { CONFIG } from "@/config";
import {
  allowAllModules,
  FREIGHTER_ID,
  ISupportedWallet,
  StellarWalletsKit,
} from "@creit.tech/stellar-wallets-kit";
import { Horizon } from "@stellar/stellar-sdk";
import type { ReactNode } from "react";
import { createContext, useContext, useEffect, useState } from "react";

type StellarProvider = {
  children: ReactNode;
};

type StellarContextProviderValue = {
  kit: StellarWalletsKit | undefined;
  server: Horizon.Server | undefined;
  publicKey: string | undefined;
  setPublicKey: (publicKey: string | undefined) => void;
  account: Horizon.AccountResponse | undefined;
  wallet: ISupportedWallet | undefined;
  setWallet: (wallet: ISupportedWallet | undefined) => void;
  env: any;
};

const initialContext = {
  kit: undefined,
  server: undefined,
  publicKey: undefined,
  setPublicKey: () => {},
  account: undefined,
  wallet: undefined,
  setWallet: () => {},
  env: undefined,
};

export const StellarContext =
  createContext<StellarContextProviderValue>(initialContext);

export const StellarProvider = ({ children }: StellarProvider) => {
  const [publicKey, setPublicKey] = useState<string | undefined>(undefined);
  const [accountInfo, setAccountInfo] = useState<
    Horizon.AccountResponse | undefined
  >(undefined);
  const [wallet, setWallet] = useState<ISupportedWallet | undefined>(undefined);

  // Set the environment in here
  const env = CONFIG.mainnet;

  const kit = new StellarWalletsKit({
    network: env.passphrase as any,
    selectedWalletId: FREIGHTER_ID,
    modules: allowAllModules(),
  });

  const server = new Horizon.Server(env.rpcUrl);

  const getAccountInfo = async (publicKey: string) => {
    try {
      const data = await server.loadAccount(publicKey);

      setAccountInfo(data);
    } catch (error: any) {
      toaster.create({
        title: "Error",
        description: error.message,
        type: "error",
      });
    }
  };

  useEffect(() => {
    if (publicKey) {
      getAccountInfo(publicKey);
    }
  }, [publicKey]);

  return (
    <StellarContext.Provider
      value={{
        kit,
        publicKey,
        setPublicKey,
        server,
        account: accountInfo,
        wallet,
        setWallet,
        env,
      }}
    >
      {children}
    </StellarContext.Provider>
  );
};

export const useStellar = () => useContext(StellarContext);
