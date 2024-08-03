
import { Center, HStack, VStack, Button } from "@chakra-ui/react";
import { Link } from "react-router-dom";
import { GreenColor } from "./Green-Color";
import { RedColor } from "./Red-Color";
import { YellowColor } from "./Yellow-Color";
import { ReadState } from "./ReadState";


function Landing() {
  return (
    <Center>
    <HStack>
      <VStack>
        <GreenColor />
        <YellowColor />
        <RedColor />
      </VStack>
      <ReadState />
    </HStack>
  </Center>
  );
}

export { Landing };
