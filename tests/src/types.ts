import { Address, Hex } from "viem";


export enum EtchedFn {
  NEW_CHAIN = 0,
  EVM_CALL = 1,
}

export type EtchedPayload = {
  fn: EtchedFn;
  input: String;
}