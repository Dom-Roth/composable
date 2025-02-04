import {
  BatchBlock,
  BatchContext,
  BatchProcessorCallItem,
  BatchProcessorEventItem,
  BatchProcessorItem,
  SubstrateBatchProcessor
} from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { archive, chain, firstBlock } from "./config";

console.log(`Chain ${chain()}`);
console.log(`Archive ${archive()}`);

export const processor = new SubstrateBatchProcessor()
  .setDataSource({
    chain: chain(),
    archive: archive()
  })
  .setBlockRange({
    from: firstBlock()
  })
  .addEvent("Pablo.PoolCreated", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Pablo.LiquidityAdded", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Pablo.LiquidityRemoved", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Pablo.Swapped", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Balances.Transfer", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Balances.Withdraw", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Balances.Deposit", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("BondedFinance.NewOffer", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("BondedFinance.NewBond", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("BondedFinance.OfferCancelled", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Vesting.VestingScheduleAdded", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addEvent("Vesting.Claimed", {
    data: { event: { extrinsic: { signature: true, hash: true }, args: true } }
  } as const)
  .addCall("Pablo.add_liquidity", {
    data: { call: { error: true, args: true }, extrinsic: { signature: true } }
  })
  .addCall("Pablo.remove_liquidity", {
    data: { call: { error: true, args: true }, extrinsic: { signature: true } }
  })
  .addCall("Pablo.swap", {
    data: { call: { error: true, args: true }, extrinsic: { signature: true } }
  });

export type Item = BatchProcessorItem<typeof processor>;
export type EventItem = BatchProcessorEventItem<typeof processor>;
export type CallItem = BatchProcessorCallItem<typeof processor>;
export type Context = BatchContext<Store, Item>;
export type Block = BatchBlock<Item>;
