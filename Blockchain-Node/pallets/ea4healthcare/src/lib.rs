#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use frame_support::{pallet_prelude::*,
		traits::{Randomness},
		sp_runtime::traits::{Hash, AccountIdConversion},
		inherent::Vec};
	use frame_system::pallet_prelude::*;
	 #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type HealthRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

	pub type VecString = Vec<u8>;
	pub type BigNumberType = u64;
	pub type SmallNumberType = u8;
	pub type Bool = bool;

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum RateScale {
		Zero, One, Two, Three, Four, Five,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum MaritalStatus {
		Single,
		Married,
		Divorced
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Attrition {
		Yes,
		No,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum BussinessTravel {
		TravelRarely,
		TravelFrequently,
		NonTravel,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum DepartmentName {
		Cardiology,
		Maternity,
		Neurology,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum FieldName {
		Medical,
		TechnicalDegree,
		Marketing,
		LifeSciences,
		Other,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum JobRole {
		Nurse,
		Administrative,
		Therapist,
		Other
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct AccountInfo<T: Config> {
		pub owner: T::AccountId,
		pub hashid : T::Hash,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct IndividualInfo {
		pub employeeid: [SmallNumberType; 7],
		pub age : SmallNumberType,
		pub gender: Gender,
		pub education: RateScale,
		pub education_field: FieldName,
		pub over18: Bool,
		pub department: DepartmentName,
		pub marital_status: MaritalStatus,
		pub distance_from_home: SmallNumberType,
		pub no_companies_worked: SmallNumberType,
		pub monthly_income: BigNumberType,
		pub percent_salary_hike: SmallNumberType,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct WorkingInfo {
		pub job_involvement: SmallNumberType,
		pub job_level: RateScale,
		pub job_role: JobRole,
		pub hourly_rate: SmallNumberType,
		pub daily_rate: BigNumberType,
		pub monthly_rate: BigNumberType,
		pub standard_hours: SmallNumberType,
		pub overtime: Bool,
		pub bussiness_travel: BussinessTravel,
		pub training_times_lastyear: SmallNumberType,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct WorkingYearInfo {
		pub total_working_years: SmallNumberType,
		pub year_atcompany: SmallNumberType,
		pub year_incurrent_role: SmallNumberType,
		pub year_inlast_promotion: SmallNumberType,
		pub year_withcurrent_manager: SmallNumberType,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct RatingInfo {
		pub attrition: Attrition,
		pub environment_satisfaction:RateScale,
		pub job_satisfaction: RateScale,
		pub performance_rating: RateScale,
		pub relationship_satisfaction: RateScale,
		pub shift: RateScale,
		pub work_life_balance: RateScale,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct PatientInfo<T: Config> {
		pub account: AccountInfo<T>,
		pub individual_info: IndividualInfo,
		pub working_info: WorkingInfo,
		pub working_year_info: WorkingYearInfo,
		pub rating: RatingInfo,
	}


	#[pallet::storage]
	#[pallet::getter(fn employee_count)]
	pub(super) type EmployeeCount<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_info_storage)]
	pub(super) type AccountInfoStorage<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, T::Hash, 
		AccountInfo<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn individual_info_storage)]
	pub(super) type IndividualInfoStorage<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, T::Hash, 
		IndividualInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn working_info_storage)]
	pub(super) type WorkingInfoStorage<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, T::Hash, 
		WorkingInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn working_year_info_storage)]
	pub(super) type WorkingYearStorage<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, T::Hash, 
		WorkingYearInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn rating_info_storage)]
	pub(super) type RatingInfoStorage<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, T::Hash, 
		RatingInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn patient_info_storage)]
	pub(super) type PatientInfoStorage<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, T::Hash, 
		PatientInfo<T>,
	>;


	#[pallet::error]
	pub enum Error<T> {
		EmployIDnotCorrect,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddAccountInfo(T::AccountId, T::Hash, T::BlockNumber),
		AddIndividualInfo(T::AccountId, T::Hash),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn add_account_info(origin: OriginFor<T>, item: [SmallNumberType; 7]) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let hash = Self::generate_employeeid_hash(item);
			let blocknumber = <frame_system::Pallet<T>>::block_number();

			let account_info = AccountInfo::<T>{
																owner: sender.clone(),
																hashid: hash.clone()
															};
			<AccountInfoStorage<T>>::insert(sender.clone(), hash.clone(), account_info);
			Self::deposit_event(Event::AddAccountInfo(sender, hash, blocknumber));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn add_individual_info(origin: OriginFor<T>, item: [SmallNumberType; 7]) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let hash = Self::generate_employeeid_hash(item);
			let blocknumber = <frame_system::Pallet<T>>::block_number();

			let account_info = AccountInfo::<T>{
																owner: sender.clone(),
																hashid: hash.clone()
															};
			<AccountInfoStorage<T>>::insert(sender.clone(), hash.clone(), account_info);
			Self::deposit_event(Event::AddAccountInfo(sender, hash, blocknumber));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn generate_employeeid_hash<V> (item: V) -> T::Hash 
		where
        V: Encode, 
		{
			let tmp = (
				T::HealthRandomness::random(&b"Generate_EmployeeID_Hash"[..]).0,
				item,
				// <frame_system::Pallet<T>>::block_number(),
			);
			let hash = T::Hashing::hash_of(&tmp.encode());
			hash
		}
	}

	
}