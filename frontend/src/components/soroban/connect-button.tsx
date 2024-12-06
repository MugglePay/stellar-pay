import { useSorobanReact } from "@soroban-react/core";
import { Button } from "@/components/ui/button";
import {
  MenuRoot,
  MenuTrigger,
  MenuContent,
  MenuItem,
} from "@/components/ui/menu";

const SorobanConnectButton = () => {
  const { connectors, setActiveConnectorAndConnect, address, disconnect } =
    useSorobanReact();

  if (address) {
    return (
      <Button
        variant="outline"
        size="sm"
        onClick={disconnect}
        colorPalette="red"
      >
        Disconnect
      </Button>
    );
  }

  return (
    <MenuRoot>
      <MenuTrigger asChild>
        <Button variant="outline" size="sm">
          Connect Wallet
        </Button>
      </MenuTrigger>
      <MenuContent>
        {connectors.map((connector) => (
          <MenuItem
            value={connector.id}
            onClick={() => {
              if (setActiveConnectorAndConnect) {
                setActiveConnectorAndConnect(connector);
              }
            }}
          >
            {connector.name}
          </MenuItem>
        ))}
      </MenuContent>
    </MenuRoot>
  );
};

export default SorobanConnectButton;
