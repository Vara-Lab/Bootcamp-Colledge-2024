import { HexString } from "@gear-js/api";


interface ContractDataI {
  programId: HexString,
  programIDL: string
}

export const traffic_light_contract: ContractDataI = {
  programId: '0x3f7febcfd55129680744c2825a6bd2c470239fe5ad560525d70180a9fdb4841e',
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