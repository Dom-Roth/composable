import { AssetId } from "@/defi/polkadot/types";
import create from "zustand";
import { ConstantProductPool, StableSwapPool } from "../pools/pools.types";

export interface AddLiquiditySlice {
  pool: StableSwapPool | ConstantProductPool | undefined;
  ui: {
    assetOne: AssetId | "none";
    assetTwo: AssetId | "none";
    assetOneAmount: string;
    assetTwoAmount: string;
  };
  findPoolManually: boolean;
}

export const useAddLiquiditySlice = create<AddLiquiditySlice>(() => ({
  pool: undefined,
  ui: {
    assetOne: "none",
    assetTwo: "none",
    assetOneAmount: "0",
    assetTwoAmount: "0",
  },
  findPoolManually: true,
}));

export const setPool = (
  pool: StableSwapPool | ConstantProductPool | undefined
) =>
  useAddLiquiditySlice.setState((state) => ({
    ...state,
    pool,
  }));

export const setSelection = (selections: Partial<AddLiquiditySlice["ui"]>) =>
  useAddLiquiditySlice.setState((state) => ({
    ...state,
    ui: {
      assetOne: selections.assetOne ? selections.assetOne : state.ui.assetOne,
      assetTwo: selections.assetTwo ? selections.assetTwo : state.ui.assetTwo,
      assetOneAmount: selections.assetOneAmount
        ? selections.assetOneAmount
        : state.ui.assetOneAmount,
      assetTwoAmount: selections.assetTwoAmount
        ? selections.assetTwoAmount
        : state.ui.assetTwoAmount,
    },
  }));

export const setManualPoolSearch = (searchManually: boolean) =>
  useAddLiquiditySlice.setState((state) => ({
    ...state,
    findPoolManually: searchManually,
  }));

export const resetAddLiquiditySlice = () =>
  useAddLiquiditySlice.setState((state) => ({
    pool: undefined,
    findPoolManually: true,
    ui: {
      assetOne: "none",
      assetTwo: "none",
      assetOneAmount: "0",
      assetTwoAmount: "0",
    },
  }));
