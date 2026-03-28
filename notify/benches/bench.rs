use Turkium_addresses::{Address, Prefix};
use Turkium_math::Uint256;
use Turkium_notify::{address::tracker::Indexes, subscription::context::SubscriptionContext};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn create_addresses(count: usize) -> Vec<Address> {
    (0..count)
        .map(|i| Address::new(Prefix::Mainnet, Turkium_addresses::Version::PubKey, &Uint256::from_u64(i as u64).to_le_bytes()))
        .collect()
}

fn create_and_fill_context(addresses: Vec<Address>) -> SubscriptionContext {
    let mut indexes = Indexes::new(vec![]);
    let context = SubscriptionContext::with_options(Some(ADDRESS_COUNT));
    let _ = context.address_tracker.register(&mut indexes, addresses);
    context
}

const ADDRESS_COUNT: usize = 1_000_000;

pub fn bench_subscription_context(c: &mut Criterion) {
    c.bench_function("create_and_fill_context", |b| {
        let addresses = create_addresses(ADDRESS_COUNT);
        b.iter(|| black_box(create_and_fill_context(addresses.clone())))
    });
}

// `cargo bench --package Turkium-notify --bench bench`
criterion_group!(benches, bench_subscription_context);
criterion_main!(benches);
