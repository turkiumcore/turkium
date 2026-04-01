use turkium_consensus_core::constants::*;
use turkium_consensus_core::network::NetworkType;
use separator::{Separatable, separated_float, separated_int, separated_uint_with_output};

#[inline]
pub fn sompi_to_turkium(sompi: u64) -> f64 {
    sompi as f64 / SOMPI_PER_TURKIUM as f64
}

#[inline]
pub fn turkium_to_sompi(turkium: f64) -> u64 {
    (turkium * SOMPI_PER_TURKIUM as f64) as u64
}

#[inline]
pub fn sompi_to_turkium_string(sompi: u64) -> String {
    sompi_to_turkium(sompi).separated_string()
}

#[inline]
pub fn sompi_to_turkium_string_with_trailing_zeroes(sompi: u64) -> String {
    separated_float!(format!("{:.8}", sompi_to_turkium(sompi)))
}

pub fn turkium_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "TUR",
        NetworkType::Testnet => "TTUR",
        NetworkType::Simnet => "STUR",
        NetworkType::Devnet => "DTURK",
    }
}

#[inline]
pub fn sompi_to_turkium_string_with_suffix(sompi: u64, network_type: &NetworkType) -> String {
    let turk = sompi_to_turkium_string(sompi);
    let suffix = turkium_suffix(network_type);
    format!("{turk} {suffix}")
}
