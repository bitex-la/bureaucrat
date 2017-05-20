/* Validates and explains a cuit/cuil */
use errors::*;
use common;

#[derive(Debug, PartialEq)]
pub struct Cuit {
    pub id: String,
    pub kind: String,
    pub person_id: String,
}

impl Cuit{
    pub fn new(id: String) -> Result<Cuit> {
        if !common::is_all_numeric(&id, 11) {
            bail!(ErrorKind::InvalidCuitFormat)
        }
        if !Self::is_kind_valid(&id[0..2]) {
            bail!(ErrorKind::InvalidCuitKind)
        }
        if !Self::is_checksum_valid(&id, vec![5,4,3,2,7,6,5,4,3,2]) {
            bail!(ErrorKind::InvalidCuitChecksum)
        }
        Ok(Cuit{
            id: id.clone(),
            kind: id[0..2].to_string(),
            person_id: id[2..10].to_string(),
        })
    }
    
    fn is_kind_valid(kind: &str) -> bool {
        let kind = kind.parse::<u32>().unwrap();
        vec![20, 23, 24, 27, 30, 33, 34].into_iter()
            .find(|&x| x == kind).is_some()
    }

    fn is_checksum_valid(payload: &str, mults: Vec<u32>) -> bool {
        let (values, checksum) = payload.split_at(payload.len() - 1);
        let sum = common::checksum(values, mults);
        let diff = 11 - (sum % 11);
        let expected = match diff {
            10 => 9,
            11 => 0,
            _ => diff
        };
        
        expected == checksum.parse::<u32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use errors::*;
    use super::Cuit;
    
    #[test]
    fn it_validates_and_explains() {
        let c = Cuit::new("20319274228".to_string()).unwrap();
        assert_eq!(c.id, "20319274228".to_string());
        assert_eq!(c.kind, "20".to_string());
        assert_eq!(c.person_id, "31927422".to_string());
    }

    #[test]
    fn it_validates_format() {
        for cuit in [
            "2031927422",      // too short
            "203192742222",    // too long
            "hello274228"      // non digit chars
        ].iter() {
            assert_error!(ErrorKind::InvalidCuitFormat,
                Cuit::new(cuit.to_string()))
        }
    }

    #[test]
    fn it_validates_checksum() {
        assert_error!(ErrorKind::InvalidCuitChecksum,
            Cuit::new("20319274229".to_string()))
    }

    #[test]
    fn it_validates_kind() {
        assert_error!(ErrorKind::InvalidCuitKind,
            Cuit::new("60319274229".to_string()))
    }
}
