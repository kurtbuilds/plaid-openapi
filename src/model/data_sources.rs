use serde::{Serialize, Deserialize};
/**A description of the source of data for a given product/data type.

`INSTITUTION`: The institution supports this product, and the data was provided by the institution.
`INSTITUTION_MASK`: The user manually provided the full account number, which was matched to the account mask provided by the institution. Only applicable to the `numbers` data type.
`USER`: The institution does not support this product, and the data was manually provided by the user.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataSources {
    #[serde(rename = "INSTITUTION")]
    Institution,
    #[serde(rename = "INSTITUTION_MASK")]
    InstitutionMask,
    #[serde(rename = "USER")]
    User,
}
