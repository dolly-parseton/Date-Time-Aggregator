use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Timespan {
    value: u8,
    unit: Unit,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Unit {
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnitVisitor;

        impl<'de> Visitor<'de> for UnitVisitor {
            type Value = Unit;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid timespan unit")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "M" => Ok(Unit::Month),
                    "w" => Ok(Unit::Week),
                    "d" => Ok(Unit::Day),
                    "h" => Ok(Unit::Hour),
                    "m" => Ok(Unit::Minute),
                    "s" => Ok(Unit::Second),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["M", "w", "d", "h", "m", "s"],
                    )),
                }
            }
        }

        //
        deserializer.deserialize_str(UnitVisitor)
    }
}

//

#[cfg(test)]
mod tests {
    use super::{Timespan, Unit};

    extern crate serde;

    #[test]
    fn unit_de() {
        let valid_values: Vec<&str> = vec!["M", "w", "d", "h", "m", "s"];
        let expected_results: Vec<Unit> = vec![
            Unit::Month,
            Unit::Week,
            Unit::Day,
            Unit::Hour,
            Unit::Minute,
            Unit::Second,
        ];
        for (i, value) in valid_values.iter().enumerate() {
            let result = serde_json::from_str::<Unit>(value).unwrap();
            assert_eq!(result, expected_results[i]);
        }
    }
}
