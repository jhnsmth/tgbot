use serde::{Deserialize, Serialize};

use crate::types::Integer;

#[cfg(test)]
mod tests;

/// File uploaded to Telegram Passport
///
/// Currently all Telegram Passport files are in JPEG
/// format when decrypted and don't exceed 10MB
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size
    pub file_size: Integer,
    /// Unix time when the file was uploaded
    pub file_date: Integer,
}

/// Data required for decrypting and authenticating EncryptedPassportElement
///
/// See the Telegram Passport Documentation for a complete description
/// of the data decryption and authentication processes
#[derive(Clone, Debug, Deserialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data
    /// with unique user's payload,
    /// data hashes and secrets required
    /// for EncryptedPassportElement decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted
    /// with the bot's public RSA key,
    /// required for data decryption
    pub secret: String,
}

/// Address
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementAddress {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Bank statement
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementBankStatement {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Driver license
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementDriverLicense {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the reverse side of the document,
    /// provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// E-Mail
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementEmail {
    /// User's verified email address
    pub email: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Identity card
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementIdentityCard {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the reverse side of the document,
    /// provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Internal passport
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementInternalPassport {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Passport
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementPassport {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Passport registration
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementPassportRegistration {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Personal details
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementPersonalDetails {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Phone number
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementPhoneNumber {
    /// User's verified phone number
    pub phone_number: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Rental agreement
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementRentalAgreement {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Temporary registration
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementTemporaryRegistration {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Utility bill
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct EncryptedPassportElementUtilityBill {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Information about documents or other Telegram Passport elements shared with the bot by the user
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[allow(clippy::large_enum_variant)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum EncryptedPassportElement {
    /// Address
    Address(EncryptedPassportElementAddress),
    /// Bank statement
    BankStatement(EncryptedPassportElementBankStatement),
    /// Driver license
    DriverLicense(EncryptedPassportElementDriverLicense),
    /// E-Mail
    Email(EncryptedPassportElementEmail),
    /// Identity card
    IdentityCard(EncryptedPassportElementIdentityCard),
    /// Internal passport
    InternalPassport(EncryptedPassportElementInternalPassport),
    /// Passport
    Passport(EncryptedPassportElementPassport),
    /// Passport registration
    PassportRegistration(EncryptedPassportElementPassportRegistration),
    /// Personal details
    PersonalDetails(EncryptedPassportElementPersonalDetails),
    /// Phone number
    PhoneNumber(EncryptedPassportElementPhoneNumber),
    /// Rental agreement
    RentalAgreement(EncryptedPassportElementRentalAgreement),
    /// Temporary registration
    TemporaryRegistration(EncryptedPassportElementTemporaryRegistration),
    /// Utility bill
    UtilityBill(EncryptedPassportElementUtilityBill),
}

/// Type of encrypted passport element
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementKind {
    /// Address
    Address,
    /// Bank statement
    BankStatement,
    /// Driver license
    DriverLicense,
    /// E-Mail
    Email,
    /// Identity card
    IdentityCard,
    /// Internal passport
    InternalPassport,
    /// Passport
    Passport,
    /// Passport registration
    PassportRegistration,
    /// Personal details
    PersonalDetails,
    /// Phone number
    PhoneNumber,
    /// Rental agreement
    RentalAgreement,
    /// Temporary registration
    TemporaryRegistration,
    /// Utility bill
    UtilityBill,
}

/// Telegram Passport data shared with the bot by the user
#[derive(Clone, Debug, Deserialize)]
pub struct PassportData {
    /// Array with information about documents
    /// and other Telegram Passport elements
    /// that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}
