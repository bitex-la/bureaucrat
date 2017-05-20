/* Validates and explains a Cbu and all it's parts */
use errors::*;
use common;

#[derive(Debug, PartialEq)]
pub struct Cbu {
    pub id: String,
    pub bank_name: String,
    pub bank: String,
    pub branch: String,
    pub account: String,
}

impl Cbu {
    pub fn new(id: String) -> Result<Cbu> {
        if !common::is_all_numeric(&id, 22) {
            bail!(ErrorKind::InvalidCbuFormat)
        }
        if !Self::is_checksum_valid(&id[0..8], vec![7,1,3,9,7,1,3]) {
            bail!(ErrorKind::InvalidCbuChecksum)
        }
        if !Self::is_checksum_valid(&id[8..22], vec![3,9,7,1,3,9,7,1,3,9,7,1,3]){
            bail!(ErrorKind::InvalidCbuChecksum)
        }

        Ok(Cbu{
            id: id.clone(),
            bank_name: Self::bank_name_lookup(&id[0..3]),
            bank: id[0..3].to_string(),
            branch: id[3..7].to_string(),
            account: id[8..22].to_string(),
        })
    }
    
    fn is_checksum_valid(payload: &str, mults: Vec<u32>) -> bool {
        let (values, checksum) = payload.split_at(payload.len() - 1);
        let sum = common::checksum(values, mults);
        let expected = (10 - (sum % 10)) % 10;
        expected == checksum.parse::<u32>().unwrap()
    }
    
    pub fn bank_name_lookup(bank_code: &str) -> String {
        match bank_code.parse::<i32>().unwrap() {
            5 =>   "The Royal Bank of Scotland N.V.",
            7 =>   "Banco de Galicia y Buenos Aires S.A.",
            11 =>  "Banco de la Nación Argentina",
            14 =>  "Banco de la Provincia de Buenos Aires",
            15 =>  "Industrial and Commercial Bank of China (Argentina) S.A.",
            16 =>  "Citibank N.A.",
            17 =>  "BBVA Banco Francés S.A.",
            18 =>  "The Bank of Tokyo-Mitsubishi UFJ, LTD.",
            20 =>  "Banco de la Provincia de Córdoba S.A.",
            27 =>  "Banco Supervielle S.A.",
            29 =>  "Banco de la Ciudad de Buenos Aires",
            30 =>  "Central de la República Argentina",
            34 =>  "Banco Patagonia S.A.",
            44 =>  "Banco Hipotecario S.A.",
            45 =>  "Banco de San Juan S.A.",
            46 =>  "Banco do Brasil S.A.",
            60 =>  "Banco de Tucumán S.A.",
            65 =>  "Banco Municipal de Rosario",
            72 =>  "Banco Santander Rio S.A.",
            83 =>  "Banco del Chubut S.A.",
            86 =>  "Banco de Santa Cruz S.A.",
            93 =>  "Banco de la Pampa Sociedad de Economía Mixta",
            94 =>  "Banco de Corrientes S.A.",
            97 =>  "Banco Provincia del Neuquén S.A.",
            147 => "Banco Interfinanzas S.A.",
            150 => "HSBC Bank Argentina S.A.",
            165 => "JP Morgan Chase Bank NA (Sucursal Buenos Aires)",
            191 => "Banco Credicoop Cooperativo Limitado",
            198 => "Banco de Valores S.A.",
            247 => "Banco Roela S.A.",
            254 => "Banco Mariva S.A.",
            259 => "Banco Itaú Argentina S.A.",
            262 => "Bank of America National Association",
            266 => "BNP Paribas",
            268 => "Banco Provincia de Tierra del Fuego",
            269 => "Banco de la República Oriental del Uruguay",
            277 => "Banco Saenz S.A.",
            281 => "Banco Meridian S.A.",
            285 => "Banco Macro S.A.",
            295 => "American Express Bank LTD. S.A.",
            299 => "Banco Comafi S.A.",
            300 => "Banco de Inversión y Comercio Exterior S.A.",
            301 => "Banco Piano S.A.",
            303 => "Banco Finansur S.A.",
            305 => "Banco Julio S.A.",
            309 => "Nuevo Banco de la Rioja S.A.",
            310 => "Banco del Sol S.A.",
            311 => "Nuevo Banco del Chaco S.A.",
            312 => "BANCO VOII S.A.",
            315 => "Banco de Formosa S.A.",
            319 => "Banco CMF S.A.",
            321 => "Banco de Santiago del Estero S.A.",
            322 => "Banco Industrial S.A.",
            325 => "Deutsche Bank S.A.",
            330 => "Nuevo Banco de Santa Fe S.A.",
            331 => "Banco Cetelem Argentina S.A.",
            332 => "Banco de Servicios Financieros S.A.",
            336 => "Banco Bradesco Argentina S.A.",
            338 => "Banco de Servicios y Transacciones S.A.",
            339 => "RCI Banque S.A.",
            340 => "BACS Banco de Crédito y Securitización S.A.",
            341 => "Más Ventas S.A.",
            386 => "Nuevo Banco de Entre Ríos S.A.",
            389 => "Banco Columbia S.A.",
            405 => "Ford Credit CIA. Finan. S.A.",
            406 => "Metrópolis Compañía Financiera S.A.",
            408 => "Compañía Financiera Argentina S.A.",
            413 => "Montemar CIA. Finan. S.A.",
            415 => "Multifinanzas CIA. Finan. S.A.",
            426 => "Banco Bica S.A.",
            428 => "Caja de Crédito Coop. La Capital del Plata LTDA.",
            431 => "Banco Coinag S.A.",
            432 => "Banco de Comercio S.A.",
            434 => "Caja de Crédito Cuenca Coop. LTDA.",
            437 => "Volkswagen Credit Compañía Financiera S.A.",
            438 => "Cordial Compañía Financiera S.A.",
            440 => "FCA Compañía Financiera S.A.",
            441 => "GPAT Compañía Financiera S.A.",
            442 => "Mercedes-Benz Compañía Financiera Argentina S.A.",
            443 => "Rombo CIA. Finan. S.A.",
            444 => "John Deere Credit CIA. Finan. S.A.",
            445 => "PSA Finance Argentina CIA. Finan. S.A.",
            446 => "Toyota Compañía Financiera de Argentina S.A.",
            448 => "Finandino Compañía Financiera S.A.",
            992 => "Provincanje Sociedad Anónima",
            993 => "AFIP Seti DJ",
            _ => "",
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use errors::*;
    use super::Cbu;
    
    #[test]
    fn it_validates_and_explains() {
        let c = Cbu::new("0170035040000002373188".to_string()).unwrap();
        assert_eq!(c.id, "0170035040000002373188".to_string());
        assert_eq!(c.bank_name, "BBVA Banco Francés S.A.");
        assert_eq!(c.bank, "017");
        assert_eq!(c.branch, "0035");
        assert_eq!(c.account, "40000002373188");
    }

    #[test]
    fn it_validates_format() {
        for cbu in [
            "01700350400000023731",        // too short
            "017003504000000237318811",    // too long
            "hello35040000002373188"       // non digit chars
        ].iter() {
			assert_error!(ErrorKind::InvalidCbuFormat,
                Cbu::new(cbu.to_string()));
        }
    }

    #[test]
    fn it_validates_bank_checksum() {
        assert_error!(ErrorKind::InvalidCbuChecksum,
			Cbu::new("0170135040000002373188".to_string()));
    }

    #[test]
    fn it_validates_account_checksum() {
		assert_error!(ErrorKind::InvalidCbuChecksum,
        	Cbu::new("0170035040000003373188".to_string()));
    }
}
