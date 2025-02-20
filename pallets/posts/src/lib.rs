#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        dispatch::DispatchResult,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: ReservableCurrency<Self::AccountId>;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type MaxPostLength: Get<u32>;
        type MinStake: Get<BalanceOf<Self>>;
    }

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PostCreated { account: T::AccountId, post_id: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        PostTooLong,
    }

    #[pallet::storage]
    #[pallet::getter(fn next_post_id)]
    pub type NextPostId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn posts)]
    pub type Posts<T: Config> = StorageMap<_, Blake2_128Concat, u64, (T::AccountId, Vec<u8>, BalanceOf<T>)>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn create_post(origin: OriginFor<T>, content: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(content.len() <= T::MaxPostLength::get() as usize, Error::<T>::PostTooLong);

            let stake_amount = T::MinStake::get();
            T::Currency::reserve(&sender, stake_amount)?;

            // Get the next available post ID
            let post_id = NextPostId::<T>::get();
            NextPostId::<T>::put(post_id + 1); // Increment for the next post

            // Store the post
            <Posts<T>>::insert(post_id, (sender.clone(), content, stake_amount));

            // Emit event
            Self::deposit_event(Event::PostCreated { account: sender, post_id });

            Ok(())
        }
    }
}
