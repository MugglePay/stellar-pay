import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { Provider } from "./components/ui/provider.tsx";
import { Toaster } from "./components/ui/toaster.tsx";
import { StellarProvider } from "./providers/stellar.provider.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Provider>
      <StellarProvider>
        <Toaster />
        <App />
      </StellarProvider>
    </Provider>
  </StrictMode>
);
