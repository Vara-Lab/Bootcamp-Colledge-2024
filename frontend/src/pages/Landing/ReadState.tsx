import { ProgramMetadata } from "@gear-js/api";
import { useState } from "react";
import { useApi, useAlert, useAccount } from "@gear-js/react-hooks";
import { Button, Card, Center, Heading, VStack, Text } from "@chakra-ui/react";
import { Sails, TransactionBuilder } from "sails-js";
import { traffic_light_contract
} from "./const";

function ReadState() {
  const { api } = useApi();
  const { account } = useAccount();

  const alert = useAlert();

  const [fullState, setFullState] = useState<any | undefined>(0);


  const color = (fullState.current_light) ?? "Black";

  const getState = async () => {
    if (!api) {
      alert.error('Api is not ready');
      return;
    }

    if (!account) {
      alert.error('Account is not ready');
      return;
    }

    const sails = await Sails.new();
    sails.setApi(api);
    sails.setProgramId(traffic_light_contract.programId);
    sails.parseIdl(traffic_light_contract.programIDL);
    const queryResponse = await sails  
      .services
      .TrafficLight
      .queries
      .TrafficLightState(account.decodedAddress);

    console.log(queryResponse);
    setFullState(queryResponse);
  };

  getState();

  return (
    <Card>
      <Center>
        <VStack>
          <Heading>Traffic-light</Heading>
          <Button
            borderRadius="50px"
            w="400px"
            h="400px"
            backgroundColor={color ?? "black"}
          >
            Light
          </Button>

          <Heading>State Contract</Heading>
          <Text>{JSON.stringify(fullState.current_light)}</Text>
        </VStack>
      </Center>
    </Card>
  );
}

export { ReadState };