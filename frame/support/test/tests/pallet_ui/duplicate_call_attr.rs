#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::ModuleInterface;
	use frame_system::pallet_prelude::BlockNumberFor;
	use frame_support::pallet_prelude::StorageValueType;

	#[pallet::config]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	#[pallet::generate_store(trait Store)]
	pub struct Module<T>(core::marker::PhantomData<T>);

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface<BlockNumberFor<T>> for Module<T> {}

	#[pallet::call]
	impl<T: Trait> Module<T> {}

	#[pallet::storage]
	type Foo<T> = StorageValueType<_, u8>;

	#[pallet::call]
	impl<T: Trait> Module<T> {}
}

fn main() {
}
