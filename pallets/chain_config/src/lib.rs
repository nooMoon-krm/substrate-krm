#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_system::ensure_root;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, StorageVersion},
    };
    use frame_system::pallet_prelude::*;
    
    pub type BalanceOf<T> = 
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: ReservableCurrency<Self::AccountId>;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn post_stake)]
    pub type PostStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn comment_stake)]
    pub type CommentStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn min_upvote_cost)]
    pub type MinUpvoteCost<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn min_downvote_cost)]
    pub type MinDownvoteCost<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pos_node_tax)]
    pub type PoSNodeTax<T> = StorageValue<_, u8, ValueQuery>; // % tax

    #[pallet::storage]
    #[pallet::getter(fn max_daily_posts)]
    pub type MaxDailyPosts<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn chain_cover_photo)]
    pub type ChainCoverPhoto<T> = StorageValue<_, Vec<u8>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn chain_description)]
    pub type ChainDescription<T> = StorageValue<_, Vec<u8>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub post_stake: u64,
        pub comment_stake: u64,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                post_stake: 1_000,
                comment_stake: 500,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            PostStake::<T>::put(self.post_stake.into());
            CommentStake::<T>::put(self.comment_stake.into());
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn update_post_stake(origin: OriginFor<T>, new_stake: BalanceOf<T>) -> DispatchResult {
            ensure_root(origin)?; // Ensures only root (governance/admin) can call this
            PostStake::<T>::put(new_stake);
            Self::deposit_event(Event::PostStakeUpdated(new_stake));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn update_comment_stake(origin: OriginFor<T>, new_stake: BalanceOf<T>) -> DispatchResult {
            ensure_root(origin)?; // Ensures only root can call this
            CommentStake::<T>::put(new_stake);
            Self::deposit_event(Event::CommentStakeUpdated(new_stake));
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PostStakeUpdated(BalanceOf<T>),
        CommentStakeUpdated(BalanceOf<T>),
    }
}
