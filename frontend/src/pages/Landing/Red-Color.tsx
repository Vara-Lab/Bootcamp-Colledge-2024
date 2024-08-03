import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { Button } from "@chakra-ui/react";
import { Sails } from "sails-js";
import { traffic_light_contract } from "./const";

function RedColor() {
  const alert = useAlert();
  const { accounts, account } = useAccount();
  const { api } = useApi();

  const signer = async () => {
    if (!accounts) {
      alert.error("Accounts is not ready");
      return;
    }

    const localaccount = account?.address;
    const isVisibleAccount = accounts.some(
      (visibleAccount) => visibleAccount.address === localaccount
    );

    if (isVisibleAccount) {
      if (!api) {
        alert.error("Api is not ready");
        return;
      }

      if (!account || !accounts) {
        alert.error("Account is not ready");
        return;
      }

      const sails = await Sails.new();
      sails.setApi(api);

      const { signer } = await web3FromSource(accounts[0].meta.source);

      sails.setProgramId(traffic_light_contract.programId);
      sails.parseIdl(traffic_light_contract.programIDL);

      try {
        alert.info("Will send a message");

        const transaction = await sails.services.TrafficLight.functions.Red();

        const allowOtherPanics = false;
        const increaseGasPercentage = 80;

        transaction.withAccount(account.decodedAddress, { signer });
        await transaction.calculateGas(allowOtherPanics ,increaseGasPercentage);

        const { blockHash, response } = await transaction.signAndSend();

        alert.success(`In block: ${blockHash}`);

        await response();

        alert.success("Message send!");
      } catch (e) {
        alert.error("Error while sending message");
      }
    } else {
      alert.error("Account not available to sign");
    }
  };

  return (
    <Button backgroundColor="red.300" onClick={signer}>
      {" "}
      Red
    </Button>
  );
}

export { RedColor };
