use std::{error::Error as StdError, fmt};

use serde::Serialize;

use crate::{
    method::Method,
    request::Request,
    types::{EncryptedPassportElementKind, Integer},
};

#[cfg(test)]
mod tests;

/// Error in the Telegram Passport element which was submitted that should be resolved by the user
#[derive(Clone, Debug, Serialize)]
pub struct PassportElementError {
    #[serde(flatten)]
    kind: PassportElementErrorKind,
}

impl PassportElementError {
    /// Represents an issue in one of the data fields that was provided by the user
    /// The error is considered resolved when the field's value changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the error,
    ///          one of “personal_details”, “passport”, “driver_license”,
    ///          “identity_card”, “internal_passport”, “address”
    /// * field_name - Name of the data field which has the error
    /// * data_hash - Base64-encoded data hash
    /// * message - Error message
    pub fn data_field<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        field_name: S,
        data_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            Address | DriverLicense | IdentityCard | InternalPassport | Passport | PersonalDetails => {
                Ok(PassportElementError {
                    kind: PassportElementErrorKind::DataField {
                        kind,
                        field_name: field_name.into(),
                        data_hash: data_hash.into(),
                        message: message.into(),
                    },
                })
            }
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the front side of a document
    /// The error is considered resolved when the file
    /// with the front side of the document changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    /// * file_hash - Base64-encoded hash of the file with the front side of the document
    /// * message -  Error message
    pub fn front_side<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            DriverLicense | IdentityCard | InternalPassport | Passport => Ok(PassportElementError {
                kind: PassportElementErrorKind::FrontSide {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the reverse side of a document
    /// The error is considered resolved when the
    /// file with reverse side of the document changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “driver_license”, “identity_card”
    /// * file_hash - Base64-encoded hash of the file with the reverse side of the document
    /// * message - Error message
    pub fn reverse_side<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            DriverLicense | IdentityCard => Ok(PassportElementError {
                kind: PassportElementErrorKind::ReverseSide {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the selfie with a document
    /// The error is considered resolved when the file with the selfie changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    pub fn selfie<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            DriverLicense | IdentityCard | InternalPassport | Passport => Ok(PassportElementError {
                kind: PassportElementErrorKind::Selfie {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }
    /// Represents an issue with a document scan
    /// The error is considered resolved when
    /// the file with the document scan changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “utility_bill”, “bank_statement”, “rental_agreement”,
    ///          “passport_registration”, “temporary_registration”
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    pub fn file<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement | PassportRegistration | RentalAgreement | TemporaryRegistration | UtilityBill => {
                Ok(PassportElementError {
                    kind: PassportElementErrorKind::File {
                        kind,
                        file_hash: file_hash.into(),
                        message: message.into(),
                    },
                })
            }
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with a list of scans
    /// The error is considered resolved when
    /// the list of files containing the scans changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue, one of
    ///          “utility_bill”, “bank_statement”, “rental_agreement”,
    ///          “passport_registration”, “temporary_registration”
    /// * file_hashes - List of base64-encoded file hashes
    /// * message - Error message
    pub fn files<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement | PassportRegistration | RentalAgreement | TemporaryRegistration | UtilityBill => {
                Ok(PassportElementError {
                    kind: PassportElementErrorKind::Files {
                        kind,
                        file_hashes,
                        message: message.into(),
                    },
                })
            }
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with one of the files that constitute
    /// the translation of a document
    /// The error is considered resolved when the file changes
    ///
    /// # Arguments
    ///
    /// * kind - Type of element of the user's Telegram Passport which has the issue,
    ///          one of “passport”, “driver_license”, “identity_card”,
    ///          “internal_passport”, “utility_bill”, “bank_statement”,
    ///          “rental_agreement”, “passport_registration”, “temporary_registration”
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    pub fn translation_file<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement
            | DriverLicense
            | IdentityCard
            | InternalPassport
            | Passport
            | PassportRegistration
            | RentalAgreement
            | TemporaryRegistration
            | UtilityBill => Ok(PassportElementError {
                kind: PassportElementErrorKind::TranslationFile {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the translated version of a document
    /// The error is considered resolved when a file
    /// with the document translation change
    ///
    /// # Arguments
    ///
    /// * kind - Type of element of the user's Telegram Passport which has the issue, one of
    ///          “passport”, “driver_license”, “identity_card”,
    ///          “internal_passport”, “utility_bill”, “bank_statement”,
    ///          “rental_agreement”, “passport_registration”, “temporary_registration”
    /// * file_hashes - List of base64-encoded file hashes
    /// * message - Error message
    pub fn translation_files<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement
            | DriverLicense
            | IdentityCard
            | InternalPassport
            | Passport
            | PassportRegistration
            | RentalAgreement
            | TemporaryRegistration
            | UtilityBill => Ok(PassportElementError {
                kind: PassportElementErrorKind::TranslationFiles {
                    kind,
                    file_hashes,
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue in an unspecified place
    /// The error is considered resolved when new data is added
    ///
    /// # Arguments
    ///
    /// * kind - Type of element of the user's Telegram Passport which has the issue
    /// * element_hash - Base64-encoded element hash
    /// * message - Error message
    pub fn unspecified<S: Into<String>>(kind: EncryptedPassportElementKind, element_hash: S, message: S) -> Self {
        PassportElementError {
            kind: PassportElementErrorKind::Unspecified {
                kind,
                element_hash: element_hash.into(),
                message: message.into(),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "source")]
enum PassportElementErrorKind {
    #[serde(rename = "data")]
    DataField {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        field_name: String,
        data_hash: String,
        message: String,
    },
    #[serde(rename = "front_side")]
    FrontSide {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "reverse_side")]
    ReverseSide {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "selfie")]
    Selfie {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "file")]
    File {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "files")]
    Files {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "translation_file")]
    TranslationFile {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "translation_files")]
    TranslationFiles {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "unspecified")]
    Unspecified {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        element_hash: String,
        message: String,
    },
}

/// Unexpected encrypted passport element kind
#[derive(Clone, Debug)]
pub struct UnexpectedEncryptedPassportElementKind(EncryptedPassportElementKind);

impl StdError for UnexpectedEncryptedPassportElementKind {}

impl fmt::Display for UnexpectedEncryptedPassportElementKind {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "unexpected element kind: {:?}", self.0)
    }
}

/// Informs a user that some of the Telegram Passport elements they provided contains errors
///
/// The user will not be able to re-submit their Passport to you until the errors are fixed
/// (the contents of the field for which you returned the error must change)
///
/// Use this if the data submitted by the user doesn't satisfy the standards
/// your service requires for any reason
///
/// For example, if a birthday date seems invalid, a submitted document is blurry,
/// a scan shows evidence of tampering, etc.
///
/// Supply some details in the error message to make sure the user knows how to correct the issues
#[derive(Clone, Debug, Serialize)]
pub struct SetPassportDataErrors {
    user_id: Integer,
    errors: Vec<PassportElementError>,
}

impl SetPassportDataErrors {
    /// Creates a new SetPassportDataErrors
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier
    /// * errors - Array describing the errors
    pub fn new(user_id: Integer, errors: Vec<PassportElementError>) -> Self {
        SetPassportDataErrors { user_id, errors }
    }
}

impl Method for SetPassportDataErrors {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("setPassportDataErrors", self)
    }
}
