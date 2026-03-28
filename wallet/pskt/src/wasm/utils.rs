use Turkium_consensus_core::constants::*;
use Turkium_consensus_core::network::NetworkType;
use separator::{Separatable, separated_float, separated_int, separated_uint_with_output};

#[inline]
pub fn sompi_to_Turkium(sompi: u64) -> f64 {
    sompi as f64 / SOMPI_PER_TURKIUM as f64
}

#[inline]
pub fn Turkium_to_sompi(Turkium: f64) -> u64 {
    (Turkium * SOMPI_PER_TURKIUM as f64) as u64
}

#[inline]
pub fn sompi_to_Turkium_string(sompi: u64) -> String {
    sompi_to_Turkium(sompi).separated_string()
}

#[inline]
pub fn sompi_to_Turkium_string_with_trailing_zeroes(sompi: u64) -> String {
    separated_float!(format!("{:.8}", sompi_to_Turkium(sompi)))
}

pub fn Turkium_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "TURK",
        NetworkType::Testnet => "TTURK",
        NetworkType::Simnet => "STURK",
        NetworkType::Devnet => "DTURK",
    }
}

#[inline]
pub fn sompi_to_Turkium_string_with_suffix(sompi: u64, network_type: &NetworkType) -> String {
    let turk = sompi_to_Turkium_string(sompi);
    let suffix = Turkium_suffix(network_type);
    format!("{turk} {suffix}")
}
