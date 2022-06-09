import { makeClient, subsquidClient } from "@/subsquid";

export const queryPoolTransactionsByType = (
  poolId: number,
  transactionType: "SWAP" | "ADD_LIQUIDITY" | "CREATE_POOL" | "REMOVE_LIQUIDITY",
  limit: number = 50
) => subsquidClient.query(`query queryPoolTransactionsByType {
  pabloTransactions(limit: ${limit}, orderBy: receivedTimestamp_DESC, where: {
    transactionType_eq: ${transactionType},
    pool: {poolId_eq: ${poolId.toString()}}
  }) {
    id
    spotPrice
    baseAssetId
    baseAssetAmount
    quoteAssetAmount
    quoteAssetId
    receivedTimestamp
    who
  }
}`).toPromise();

export const liquidityTransactionsByAddressAndPool = (
  who: string,
  poolId: number
) => makeClient().query(`query queryAddOrRemoveLiquidityTransactionsByUserAddress {
  pabloTransactions(
    orderBy: receivedTimestamp_ASC,where: {
        who_eq: "${who}",
				transactionType_in: [ADD_LIQUIDITY,REMOVE_LIQUIDITY],
        pool: {
          poolId_eq: ${poolId}
        }
  }) {
    baseAssetId
    baseAssetAmount
    quoteAssetAmount
    quoteAssetId
    receivedTimestamp
    transactionType
    who
    pool {
      poolId
    }
  }
}`).toPromise();


export const queryPabloPoolById = (poolId: number) => makeClient().query(`query queryPabloPoolById {
  pabloPools(orderBy: calculatedTimestamp_DESC, where: {poolId_eq: ${poolId}}) {
    totalLiquidity
    totalVolume
    transactionCount
    totalFees
    calculatedTimestamp
    quoteAssetId
    poolId
  }
}
`).toPromise()
