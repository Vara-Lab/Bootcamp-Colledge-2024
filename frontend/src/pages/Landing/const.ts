import { HexString } from "@gear-js/api";

// const ADDRESS = {
//   NODE: process.env.REACT_APP_NODE_ADDRESS as string,
// };

// const LOCAL_STORAGE = {
//   ACCOUNT: 'account',
// };

// export { ADDRESS, LOCAL_STORAGE };



interface ContractDataI {
  programId: HexString,
  programIDL: string
}

export const traffic_light_contract: ContractDataI = {
  programId: '0x670f071608b2075f21fe1aa60dfe124c1921315736ccc6f06b8937e4bc909b9c',
  programIDL: `
    type TrafficLightEvent = enum {
      Green,
      Yellow,
      Red,
    };

    type IoTrafficLightState = struct {
      current_light: str,
      all_users: vec struct { actor_id, str },
    };

    constructor {
      New : ();
    };

    service TrafficLight {
      Green : () -> TrafficLightEvent;
      Red : () -> TrafficLightEvent;
      Yellow : () -> TrafficLightEvent;
      query TrafficLightState : () -> IoTrafficLightState;
    };
  `
}