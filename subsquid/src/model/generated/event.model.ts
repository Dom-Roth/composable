import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import * as marshal from "./marshal"
import {EventType} from "./_eventType"
import {Activity} from "./activity.model"

@Entity_()
export class Event {
    constructor(props?: Partial<Event>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    /**
     * ID of account that executed the extrinsic
     */
    @Index_()
    @Column_("text", {nullable: true})
    accountId!: string | undefined | null

    /**
     * Type of transaction
     */
    @Column_("varchar", {length: 43, nullable: false})
    eventType!: EventType

    /**
     * Block in which transaction was registered
     */
    @Index_()
    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    blockNumber!: bigint

    /**
     * Timestamp of the block in which this transaction was registered
     */
    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @OneToMany_(() => Activity, e => e.event)
    activities!: Activity[]

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    @Column_("text", {nullable: true})
    txHash!: string | undefined | null
}
