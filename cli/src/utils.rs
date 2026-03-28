use crate::error::Error;
use crate::result::Result;
use Turkium_consensus_core::constants::SOMPI_PER_TURKIUM;
use std::fmt::Display;

pub fn try_parse_required_nonzero_Turkium_as_sompi_u64<S: ToString + Display>(Turkium_amount: Option<S>) -> Result<u64> {
    if let Some(Turkium_amount) = Turkium_amount {
        let sompi_amount = Turkium_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Turkium amount is not valid: '{Turkium_amount}'")))?
            * SOMPI_PER_TURKIUM as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Turkium amount is not valid: '{Turkium_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required Turkium amount must not be a zero: '{Turkium_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Turkium amount"))
    }
}

pub fn try_parse_required_Turkium_as_sompi_u64<S: ToString + Display>(Turkium_amount: Option<S>) -> Result<u64> {
    if let Some(Turkium_amount) = Turkium_amount {
        let sompi_amount = Turkium_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Turkium amount is not valid: '{Turkium_amount}'")))?
            * SOMPI_PER_TURKIUM as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Turkium amount is not valid: '{Turkium_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Turkium amount"))
    }
}

pub fn try_parse_optional_Turkium_as_sompi_i64<S: ToString + Display>(Turkium_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(Turkium_amount) = Turkium_amount {
        let sompi_amount = Turkium_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Kasapa amount is not valid: '{Turkium_amount}'")))?
            * SOMPI_PER_TURKIUM as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Turkium amount is not valid: '{Turkium_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
