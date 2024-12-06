import { SorobanReactProvider } from "@soroban-react/core";
import { StellarProvider } from "./providers/stellar.provider";
import StellarKit from "./stellar-kit";
import SorobanKit from "./soroban";
import { mainnet, testnet } from "@soroban-react/chains";
import { freighter } from "@soroban-react/freighter";
import { xbull } from "@soroban-react/xbull";
import { hana } from "@soroban-react/hana";
import { lobstr } from "@soroban-react/lobstr";

function App({ mode }: { mode: "kit" | "soroban" }) {
  return (
    <>
      {mode === "kit" && (
        <StellarProvider>
          <StellarKit />
        </StellarProvider>
      )}
      {mode === "soroban" && (
        <SorobanReactProvider
          autoconnect
          chains={[mainnet, testnet]}
          activeChain={mainnet}
          connectors={[freighter(), xbull(), hana(), lobstr()]}
        >
          <SorobanKit />
        </SorobanReactProvider>
      )}
    </>
  );
}

export default App;
