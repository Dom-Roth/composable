#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::traits::tokens::Balance;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_bet_to_account)]
    pub type BalanceToAccount<T> = StorageMap<
        _,
        T::AccountId,
        T::Balance,
        ValueQuery
    >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
        MintedNewSupply(T::AccountId),
        Transferred(T::AccountId, T::AccountId, T::Balance),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Write your extrinsics here!

        // Minting extrinsic
        #[pallet::weight(10_000 + T::DbWei)]
        pub(super) fn mint(
            origin: OriginFor<T>,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // Update storage variable: `BalanceToAccount` defined above.
            <BalanceToAccount<T>>::insert(&sender, amount);

            // Emit an event
            Self::deposit_event(Event::MintedNewSupply(sender));

            // Return a successful DispatchResultWithPostInfo.
            Ok(().into())
        }

        // Transfer extrinsic
        pub(super) fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            #[pallet::compact] amount: T::Balance
        ) -> DispatchResultWithPostInfo {
            // Variables
            let sender = ensure_signed(origin)?;
            let sender_balance = Self::get_balance(&sender);
            let receiver_balance = Self::get_balance(&to);

            // Checks & error handling.
            // Calculate new balances.
            let updated_from_balance = sender_balance.checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
            let updated_to_balance = receiver_balance.checked_sub(value).expect("Entire supply fits in u64, qed");

            // Write to storage
            <Balances<T>>::insert(&sender, updated_from_balance);
            <Balances<T>>::insert(&to, updated_to_balance);

            // Deposit event.
            Self::deposit_event(RawEvent::Transfer(sender, to, value));

            Ok(())

        }
    }
}
