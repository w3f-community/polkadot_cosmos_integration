use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use pallet_cosmos_abci::{crypto, Call, Module, Trait, KEY_TYPE};
use sp_core::{crypto::KeyTypeId, H256};
use sp_runtime::{
    generic,
    testing::TestXt,
    traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
    MultiSignature, Perbill,
};
use sp_std::boxed::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 100;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl frame_system::Trait for Test {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = generic::Header<u64, BlakeTwo256>;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
}

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}

impl Trait for Test {
    type AuthorityId = crypto::ABCIAuthId;
    type Call = Call<Test>;
}

pub type Extrinsic = TestXt<Call<Test>, ()>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Signature = MultiSignature;

impl frame_system::offchain::SigningTypes for Test {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call<Test>: From<LocalCall>,
{
    type OverarchingCall = Call<Test>;
    type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
    Call<Test>: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call<Test>,
        _public: <Signature as Verify>::Signer,
        _account: AccountId,
        nonce: u64,
    ) -> Option<(Call<Test>, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
        Some((call, (nonce, ())))
    }
}

pub type AbciModule = Module<Test>;

#[test]
fn should_match_key_type() {
    println!("{:?}", KEY_TYPE);
    assert_eq!(KEY_TYPE, KeyTypeId(*b"abci"));
}

#[test]
fn should_begin_block_on_initialize() {
    use testcontainers::*;
    let docker = clients::Cli::default();
    let cosmos = images::generic::GenericImage::new("andoriasoft/cosmos-node:latest")
        .with_args(vec![
            "start".to_owned(),
            "--with-tendermint=false".to_owned(),
            "--transport=grpc".to_owned(),
        ])
        .with_wait_for(images::generic::WaitFor::message_on_stdout("starting ABCI"));
    let node = docker.run(cosmos);

    let url = format!(
        "tcp://localhost:{}",
        node.get_host_port(26658).unwrap_or(26658)
    );
    pallet_abci::set_abci_instance(Box::new(
        pallet_abci::grpc::AbciinterfaceGrpc::connect(&url)
            .map_err(|_| "failed to connect")
            .unwrap(),
    ))
    .map_err(|_| "failed to set abci instance")
    .unwrap();

    let mut client = pallet_abci::get_abci_instance().unwrap();

    let genesis = pallet_abci::utils::parse_cosmos_genesis_file(pallet_abci::TEST_GENESIS).unwrap();
    let result = client.init_chain(
        genesis.time_seconds,
        genesis.time_nanos,
        &genesis.chain_id,
        genesis.pub_key_types,
        genesis.max_bytes,
        genesis.max_gas,
        genesis.max_age_num_blocks,
        genesis.max_age_duration,
        genesis.app_state_bytes,
    );
    assert!(result.is_ok(), "should successfully call init chain");

    // FIXME: Doesn't work after begin_block call
    // assert_eq!(AbciModule::call_on_initialize(1), true);
    // assert_eq!(AbciModule::call_on_finalize(1), true);

    // let data = [0u8, 24];
    // let hash = Hasher::hash(&data);
    // AbciModule::deliver_tx(Origin::signed(AccountId32::from(Into::<[u8; 32]>::into(hash))), vec![]);
    // todo
}