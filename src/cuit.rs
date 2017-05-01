/* Validates and explains a cuit/cuil */
#[derive(Debug, PartialEq)]
pub struct Cuit<'a> {
    id: &'a str,
    kind: &'a str,
    person_id: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum CuitError { Format, Checksum, Kind }

impl<'a> Cuit<'a> {
    pub fn new(id: &str) -> Result<Cuit, CuitError> {
        if !Self::is_format_valid(id) {
            return Err(CuitError::Format)
        }
        if !Self::is_kind_valid(&id[0..2]) {
            return Err(CuitError::Kind)
        }
        if !Self::is_checksum_valid(&id.clone(), vec![5,4,3,2,7,6,5,4,3,2]) {
            return Err(CuitError::Checksum)
        }
        Ok(Cuit{
            id: id.clone(),
            kind: &id[0..2],
            person_id: &id[2..10],
        })
    }
    
    fn is_format_valid(cuit: &str) -> bool {
        cuit.chars().count() == 11 && cuit.chars().all(|c| c.is_numeric())
    }

    fn is_kind_valid(kind: &str) -> bool {
        let kind = kind.parse().unwrap();
        vec![20, 23, 24, 27, 30, 33, 34].into_iter()
            .find(|&x| x == kind).is_some()
    }

    fn is_checksum_valid(payload: &str, mults: Vec<u32>) -> bool {
        let (values, checksum) = payload.split_at(payload.len() - 1);
        let digits : Vec<u32> = values.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let sum : u32 = digits.iter()
            .zip(mults.iter())
            .map(|(a,b)| a * b)
            .sum();

        let diff = 11 - (sum % 11);
        let expected = match diff {
            10 => 9,
            11 => 0,
            _ => diff
        };
        
        expected == checksum.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{Cuit, CuitError};
    
    #[test]
    fn it_validates_and_explains() {
        let c = Cuit::new("20319274228").unwrap();
        assert_eq!(c.id, "20319274228");
        assert_eq!(c.kind, "20");
        assert_eq!(c.person_id, "31927422");
    }

    #[test]
    fn it_validates_format() {
        for cuit in [
            "2031927422",      // too short
            "203192742222",    // too long
            "hello274228"      // non digit chars
        ].iter() {
            assert_eq!(Cuit::new(cuit), Err(CuitError::Format));
        }
    }

    #[test]
    fn it_validates_checksum() {
        assert_eq!(Cuit::new("20319274229"), Err(CuitError::Checksum));
    }

    #[test]
    fn it_validates_kind() {
        assert_eq!(Cuit::new("60319274229"), Err(CuitError::Kind));
    }
}
