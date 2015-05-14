#![allow(non_camel_case_types)]
#![allow(unused_imports)]

extern crate libc;
#[macro_use]
extern crate enum_primitive;

use libc::{c_int, c_char, c_uint, size_t, c_uchar, c_void, c_ushort};

use enum_primitive::FromPrimitive;

pub type gpg_error_t = c_uint;

enum_from_primitive! {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub enum gpg_err_source_t {
        GPG_ERR_SOURCE_UNKNOWN = 0,
        GPG_ERR_SOURCE_GCRYPT = 1,
        GPG_ERR_SOURCE_GPG = 2,
        GPG_ERR_SOURCE_GPGSM = 3,
        GPG_ERR_SOURCE_GPGAGENT = 4,
        GPG_ERR_SOURCE_PINENTRY = 5,
        GPG_ERR_SOURCE_SCD = 6,
        GPG_ERR_SOURCE_GPGME = 7,
        GPG_ERR_SOURCE_KEYBOX = 8,
        GPG_ERR_SOURCE_KSBA = 9,
        GPG_ERR_SOURCE_DIRMNGR = 10,
        GPG_ERR_SOURCE_GSTI = 11,
        GPG_ERR_SOURCE_GPA = 12,
        GPG_ERR_SOURCE_KLEO = 13,
        GPG_ERR_SOURCE_G13 = 14,
        GPG_ERR_SOURCE_ASSUAN = 15,
        GPG_ERR_SOURCE_TLS = 17,
        GPG_ERR_SOURCE_ANY = 31,
        GPG_ERR_SOURCE_USER_1 = 32,
        GPG_ERR_SOURCE_USER_2 = 33,
        GPG_ERR_SOURCE_USER_3 = 34,
        GPG_ERR_SOURCE_USER_4 = 35,

        GPG_ERR_SOURCE_DIM = 128
    }
}
pub use gpg_err_source_t::*;

pub const GPG_ERR_SYSTEM_ERROR: isize = 1 << 15;

enum_from_primitive! {
    #[repr(C)]
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum gpg_err_code_t {
        GPG_ERR_NO_ERROR = 0,
        GPG_ERR_GENERAL = 1,
        GPG_ERR_UNKNOWN_PACKET = 2,
        GPG_ERR_UNKNOWN_VERSION = 3,
        GPG_ERR_PUBKEY_ALGO = 4,
        GPG_ERR_DIGEST_ALGO = 5,
        GPG_ERR_BAD_PUBKEY = 6,
        GPG_ERR_BAD_SECKEY = 7,
        GPG_ERR_BAD_SIGNATURE = 8,
        GPG_ERR_NO_PUBKEY = 9,
        GPG_ERR_CHECKSUM = 10,
        GPG_ERR_BAD_PASSPHRASE = 11,
        GPG_ERR_CIPHER_ALGO = 12,
        GPG_ERR_KEYRING_OPEN = 13,
        GPG_ERR_INV_PACKET = 14,
        GPG_ERR_INV_ARMOR = 15,
        GPG_ERR_NO_USER_ID = 16,
        GPG_ERR_NO_SECKEY = 17,
        GPG_ERR_WRONG_SECKEY = 18,
        GPG_ERR_BAD_KEY = 19,
        GPG_ERR_COMPR_ALGO = 20,
        GPG_ERR_NO_PRIME = 21,
        GPG_ERR_NO_ENCODING_METHOD = 22,
        GPG_ERR_NO_ENCRYPTION_SCHEME = 23,
        GPG_ERR_NO_SIGNATURE_SCHEME = 24,
        GPG_ERR_INV_ATTR = 25,
        GPG_ERR_NO_VALUE = 26,
        GPG_ERR_NOT_FOUND = 27,
        GPG_ERR_VALUE_NOT_FOUND = 28,
        GPG_ERR_SYNTAX = 29,
        GPG_ERR_BAD_MPI = 30,
        GPG_ERR_INV_PASSPHRASE = 31,
        GPG_ERR_SIG_CLASS = 32,
        GPG_ERR_RESOURCE_LIMIT = 33,
        GPG_ERR_INV_KEYRING = 34,
        GPG_ERR_TRUSTDB = 35,
        GPG_ERR_BAD_CERT = 36,
        GPG_ERR_INV_USER_ID = 37,
        GPG_ERR_UNEXPECTED = 38,
        GPG_ERR_TIME_CONFLICT = 39,
        GPG_ERR_KEYSERVER = 40,
        GPG_ERR_WRONG_PUBKEY_ALGO = 41,
        GPG_ERR_TRIBUTE_TO_D_A = 42,
        GPG_ERR_WEAK_KEY = 43,
        GPG_ERR_INV_KEYLEN = 44,
        GPG_ERR_INV_ARG = 45,
        GPG_ERR_BAD_URI = 46,
        GPG_ERR_INV_URI = 47,
        GPG_ERR_NETWORK = 48,
        GPG_ERR_UNKNOWN_HOST = 49,
        GPG_ERR_SELFTEST_FAILED = 50,
        GPG_ERR_NOT_ENCRYPTED = 51,
        GPG_ERR_NOT_PROCESSED = 52,
        GPG_ERR_UNUSABLE_PUBKEY = 53,
        GPG_ERR_UNUSABLE_SECKEY = 54,
        GPG_ERR_INV_VALUE = 55,
        GPG_ERR_BAD_CERT_CHAIN = 56,
        GPG_ERR_MISSING_CERT = 57,
        GPG_ERR_NO_DATA = 58,
        GPG_ERR_BUG = 59,
        GPG_ERR_NOT_SUPPORTED = 60,
        GPG_ERR_INV_OP = 61,
        GPG_ERR_TIMEOUT = 62,
        GPG_ERR_INTERNAL = 63,
        GPG_ERR_EOF_GCRYPT = 64,
        GPG_ERR_INV_OBJ = 65,
        GPG_ERR_TOO_SHORT = 66,
        GPG_ERR_TOO_LARGE = 67,
        GPG_ERR_NO_OBJ = 68,
        GPG_ERR_NOT_IMPLEMENTED = 69,
        GPG_ERR_CONFLICT = 70,
        GPG_ERR_INV_CIPHER_MODE = 71,
        GPG_ERR_INV_FLAG = 72,
        GPG_ERR_INV_HANDLE = 73,
        GPG_ERR_TRUNCATED = 74,
        GPG_ERR_INCOMPLETE_LINE = 75,
        GPG_ERR_INV_RESPONSE = 76,
        GPG_ERR_NO_AGENT = 77,
        GPG_ERR_AGENT = 78,
        GPG_ERR_INV_DATA = 79,
        GPG_ERR_ASSUAN_SERVER_FAULT = 80,
        GPG_ERR_ASSUAN = 81,
        GPG_ERR_INV_SESSION_KEY = 82,
        GPG_ERR_INV_SEXP = 83,
        GPG_ERR_UNSUPPORTED_ALGORITHM = 84,
        GPG_ERR_NO_PIN_ENTRY = 85,
        GPG_ERR_PIN_ENTRY = 86,
        GPG_ERR_BAD_PIN = 87,
        GPG_ERR_INV_NAME = 88,
        GPG_ERR_BAD_DATA = 89,
        GPG_ERR_INV_PARAMETER = 90,
        GPG_ERR_WRONG_CARD = 91,
        GPG_ERR_NO_DIRMNGR = 92,
        GPG_ERR_DIRMNGR = 93,
        GPG_ERR_CERT_REVOKED = 94,
        GPG_ERR_NO_CRL_KNOWN = 95,
        GPG_ERR_CRL_TOO_OLD = 96,
        GPG_ERR_LINE_TOO_LONG = 97,
        GPG_ERR_NOT_TRUSTED = 98,
        GPG_ERR_CANCELED = 99,
        GPG_ERR_BAD_CA_CERT = 100,
        GPG_ERR_CERT_EXPIRED = 101,
        GPG_ERR_CERT_TOO_YOUNG = 102,
        GPG_ERR_UNSUPPORTED_CERT = 103,
        GPG_ERR_UNKNOWN_SEXP = 104,
        GPG_ERR_UNSUPPORTED_PROTECTION = 105,
        GPG_ERR_CORRUPTED_PROTECTION = 106,
        GPG_ERR_AMBIGUOUS_NAME = 107,
        GPG_ERR_CARD = 108,
        GPG_ERR_CARD_RESET = 109,
        GPG_ERR_CARD_REMOVED = 110,
        GPG_ERR_INV_CARD = 111,
        GPG_ERR_CARD_NOT_PRESENT = 112,
        GPG_ERR_NO_PKCS15_APP = 113,
        GPG_ERR_NOT_CONFIRMED = 114,
        GPG_ERR_CONFIGURATION = 115,
        GPG_ERR_NO_POLICY_MATCH = 116,
        GPG_ERR_INV_INDEX = 117,
        GPG_ERR_INV_ID = 118,
        GPG_ERR_NO_SCDAEMON = 119,
        GPG_ERR_SCDAEMON = 120,
        GPG_ERR_UNSUPPORTED_PROTOCOL = 121,
        GPG_ERR_BAD_PIN_METHOD = 122,
        GPG_ERR_CARD_NOT_INITIALIZED = 123,
        GPG_ERR_UNSUPPORTED_OPERATION = 124,
        GPG_ERR_WRONG_KEY_USAGE = 125,
        GPG_ERR_NOTHING_FOUND = 126,
        GPG_ERR_WRONG_BLOB_TYPE = 127,
        GPG_ERR_MISSING_VALUE = 128,
        GPG_ERR_HARDWARE = 129,
        GPG_ERR_PIN_BLOCKED = 130,
        GPG_ERR_USE_CONDITIONS = 131,
        GPG_ERR_PIN_NOT_SYNCED = 132,
        GPG_ERR_INV_CRL = 133,
        GPG_ERR_BAD_BER = 134,
        GPG_ERR_INV_BER = 135,
        GPG_ERR_ELEMENT_NOT_FOUND = 136,
        GPG_ERR_IDENTIFIER_NOT_FOUND = 137,
        GPG_ERR_INV_TAG = 138,
        GPG_ERR_INV_LENGTH = 139,
        GPG_ERR_INV_KEYINFO = 140,
        GPG_ERR_UNEXPECTED_TAG = 141,
        GPG_ERR_NOT_DER_ENCODED = 142,
        GPG_ERR_NO_CMS_OBJ = 143,
        GPG_ERR_INV_CMS_OBJ = 144,
        GPG_ERR_UNKNOWN_CMS_OBJ = 145,
        GPG_ERR_UNSUPPORTED_CMS_OBJ = 146,
        GPG_ERR_UNSUPPORTED_ENCODING = 147,
        GPG_ERR_UNSUPPORTED_CMS_VERSION = 148,
        GPG_ERR_UNKNOWN_ALGORITHM = 149,
        GPG_ERR_INV_ENGINE = 150,
        GPG_ERR_PUBKEY_NOT_TRUSTED = 151,
        GPG_ERR_DECRYPT_FAILED = 152,
        GPG_ERR_KEY_EXPIRED = 153,
        GPG_ERR_SIG_EXPIRED = 154,
        GPG_ERR_ENCODING_PROBLEM = 155,
        GPG_ERR_INV_STATE = 156,
        GPG_ERR_DUP_VALUE = 157,
        GPG_ERR_MISSING_ACTION = 158,
        GPG_ERR_MODULE_NOT_FOUND = 159,
        GPG_ERR_INV_OID_STRING = 160,
        GPG_ERR_INV_TIME = 161,
        GPG_ERR_INV_CRL_OBJ = 162,
        GPG_ERR_UNSUPPORTED_CRL_VERSION = 163,
        GPG_ERR_INV_CERT_OBJ = 164,
        GPG_ERR_UNKNOWN_NAME = 165,
        GPG_ERR_LOCALE_PROBLEM = 166,
        GPG_ERR_NOT_LOCKED = 167,
        GPG_ERR_PROTOCOL_VIOLATION = 168,
        GPG_ERR_INV_MAC = 169,
        GPG_ERR_INV_REQUEST = 170,
        GPG_ERR_UNKNOWN_EXTN = 171,
        GPG_ERR_UNKNOWN_CRIT_EXTN = 172,
        GPG_ERR_LOCKED = 173,
        GPG_ERR_UNKNOWN_OPTION = 174,
        GPG_ERR_UNKNOWN_COMMAND = 175,
        GPG_ERR_NOT_OPERATIONAL = 176,
        GPG_ERR_NO_PASSPHRASE = 177,
        GPG_ERR_NO_PIN = 178,
        GPG_ERR_NOT_ENABLED = 179,
        GPG_ERR_NO_ENGINE = 180,
        GPG_ERR_MISSING_KEY = 181,
        GPG_ERR_TOO_MANY = 182,
        GPG_ERR_LIMIT_REACHED = 183,
        GPG_ERR_NOT_INITIALIZED = 184,
        GPG_ERR_MISSING_ISSUER_CERT = 185,
        GPG_ERR_NO_KEYSERVER = 186,
        GPG_ERR_INV_CURVE = 187,
        GPG_ERR_UNKNOWN_CURVE = 188,
        GPG_ERR_DUP_KEY = 189,
        GPG_ERR_AMBIGUOUS = 190,
        GPG_ERR_NO_CRYPT_CTX = 191,
        GPG_ERR_WRONG_CRYPT_CTX = 192,
        GPG_ERR_BAD_CRYPT_CTX = 193,
        GPG_ERR_CRYPT_CTX_CONFLICT = 194,
        GPG_ERR_BROKEN_PUBKEY = 195,
        GPG_ERR_BROKEN_SECKEY = 196,
        GPG_ERR_MAC_ALGO = 197,
        GPG_ERR_FULLY_CANCELED = 198,
        GPG_ERR_UNFINISHED = 199,
        GPG_ERR_BUFFER_TOO_SHORT = 200,
        GPG_ERR_SEXP_INV_LEN_SPEC = 201,
        GPG_ERR_SEXP_STRING_TOO_LONG = 202,
        GPG_ERR_SEXP_UNMATCHED_PAREN = 203,
        GPG_ERR_SEXP_NOT_CANONICAL = 204,
        GPG_ERR_SEXP_BAD_CHARACTER = 205,
        GPG_ERR_SEXP_BAD_QUOTATION = 206,
        GPG_ERR_SEXP_ZERO_PREFIX = 207,
        GPG_ERR_SEXP_NESTED_DH = 208,
        GPG_ERR_SEXP_UNMATCHED_DH = 209,
        GPG_ERR_SEXP_UNEXPECTED_PUNC = 210,
        GPG_ERR_SEXP_BAD_HEX_CHAR = 211,
        GPG_ERR_SEXP_ODD_HEX_NUMBERS = 212,
        GPG_ERR_SEXP_BAD_OCT_CHAR = 213,
        GPG_ERR_NO_CERT_CHAIN = 226,
        GPG_ERR_CERT_TOO_LARGE = 227,
        GPG_ERR_INV_RECORD = 228,
        GPG_ERR_BAD_MAC = 229,
        GPG_ERR_UNEXPECTED_MSG = 230,
        GPG_ERR_COMPR_FAILED = 231,
        GPG_ERR_WOULD_WRAP = 232,
        GPG_ERR_FATAL_ALERT = 233,
        GPG_ERR_NO_CIPHER = 234,
        GPG_ERR_MISSING_CLIENT_CERT = 235,
        GPG_ERR_CLOSE_NOTIFY = 236,
        GPG_ERR_TICKET_EXPIRED = 237,
        GPG_ERR_BAD_TICKET = 238,
        GPG_ERR_UNKNOWN_IDENTITY = 239,
        GPG_ERR_BAD_HS_CERT = 240,
        GPG_ERR_BAD_HS_CERT_REQ = 241,
        GPG_ERR_BAD_HS_CERT_VER = 242,
        GPG_ERR_BAD_HS_CHANGE_CIPHER = 243,
        GPG_ERR_BAD_HS_CLIENT_HELLO = 244,
        GPG_ERR_BAD_HS_SERVER_HELLO = 245,
        GPG_ERR_BAD_HS_SERVER_HELLO_DONE = 246,
        GPG_ERR_BAD_HS_FINISHED = 247,
        GPG_ERR_BAD_HS_SERVER_KEX = 248,
        GPG_ERR_BAD_HS_CLIENT_KEX = 249,
        GPG_ERR_BOGUS_STRING = 250,
        GPG_ERR_KEY_DISABLED = 252,
        GPG_ERR_KEY_ON_CARD = 253,
        GPG_ERR_INV_LOCK_OBJ = 254,
        GPG_ERR_ASS_GENERAL = 257,
        GPG_ERR_ASS_ACCEPT_FAILED = 258,
        GPG_ERR_ASS_CONNECT_FAILED = 259,
        GPG_ERR_ASS_INV_RESPONSE = 260,
        GPG_ERR_ASS_INV_VALUE = 261,
        GPG_ERR_ASS_INCOMPLETE_LINE = 262,
        GPG_ERR_ASS_LINE_TOO_LONG = 263,
        GPG_ERR_ASS_NESTED_COMMANDS = 264,
        GPG_ERR_ASS_NO_DATA_CB = 265,
        GPG_ERR_ASS_NO_INQUIRE_CB = 266,
        GPG_ERR_ASS_NOT_A_SERVER = 267,
        GPG_ERR_ASS_NOT_A_CLIENT = 268,
        GPG_ERR_ASS_SERVER_START = 269,
        GPG_ERR_ASS_READ_ERROR = 270,
        GPG_ERR_ASS_WRITE_ERROR = 271,
        GPG_ERR_ASS_TOO_MUCH_DATA = 273,
        GPG_ERR_ASS_UNEXPECTED_CMD = 274,
        GPG_ERR_ASS_UNKNOWN_CMD = 275,
        GPG_ERR_ASS_SYNTAX = 276,
        GPG_ERR_ASS_CANCELED = 277,
        GPG_ERR_ASS_NO_INPUT = 278,
        GPG_ERR_ASS_NO_OUTPUT = 279,
        GPG_ERR_ASS_PARAMETER = 280,
        GPG_ERR_ASS_UNKNOWN_INQUIRE = 281,
        GPG_ERR_USER_1 = 1024,
        GPG_ERR_USER_2 = 1025,
        GPG_ERR_USER_3 = 1026,
        GPG_ERR_USER_4 = 1027,
        GPG_ERR_USER_5 = 1028,
        GPG_ERR_USER_6 = 1029,
        GPG_ERR_USER_7 = 1030,
        GPG_ERR_USER_8 = 1031,
        GPG_ERR_USER_9 = 1032,
        GPG_ERR_USER_10 = 1033,
        GPG_ERR_USER_11 = 1034,
        GPG_ERR_USER_12 = 1035,
        GPG_ERR_USER_13 = 1036,
        GPG_ERR_USER_14 = 1037,
        GPG_ERR_USER_15 = 1038,
        GPG_ERR_USER_16 = 1039,
        GPG_ERR_MISSING_ERRNO = 16381,
        GPG_ERR_UNKNOWN_ERRNO = 16382,
        GPG_ERR_EOF = 16383,

        GPG_ERR_E2BIG = GPG_ERR_SYSTEM_ERROR | 0,
        GPG_ERR_EACCES = GPG_ERR_SYSTEM_ERROR | 1,
        GPG_ERR_EADDRINUSE = GPG_ERR_SYSTEM_ERROR | 2,
        GPG_ERR_EADDRNOTAVAIL = GPG_ERR_SYSTEM_ERROR | 3,
        GPG_ERR_EADV = GPG_ERR_SYSTEM_ERROR | 4,
        GPG_ERR_EAFNOSUPPORT = GPG_ERR_SYSTEM_ERROR | 5,
        GPG_ERR_EAGAIN = GPG_ERR_SYSTEM_ERROR | 6,
        GPG_ERR_EALREADY = GPG_ERR_SYSTEM_ERROR | 7,
        GPG_ERR_EAUTH = GPG_ERR_SYSTEM_ERROR | 8,
        GPG_ERR_EBACKGROUND = GPG_ERR_SYSTEM_ERROR | 9,
        GPG_ERR_EBADE = GPG_ERR_SYSTEM_ERROR | 10,
        GPG_ERR_EBADF = GPG_ERR_SYSTEM_ERROR | 11,
        GPG_ERR_EBADFD = GPG_ERR_SYSTEM_ERROR | 12,
        GPG_ERR_EBADMSG = GPG_ERR_SYSTEM_ERROR | 13,
        GPG_ERR_EBADR = GPG_ERR_SYSTEM_ERROR | 14,
        GPG_ERR_EBADRPC = GPG_ERR_SYSTEM_ERROR | 15,
        GPG_ERR_EBADRQC = GPG_ERR_SYSTEM_ERROR | 16,
        GPG_ERR_EBADSLT = GPG_ERR_SYSTEM_ERROR | 17,
        GPG_ERR_EBFONT = GPG_ERR_SYSTEM_ERROR | 18,
        GPG_ERR_EBUSY = GPG_ERR_SYSTEM_ERROR | 19,
        GPG_ERR_ECANCELED = GPG_ERR_SYSTEM_ERROR | 20,
        GPG_ERR_ECHILD = GPG_ERR_SYSTEM_ERROR | 21,
        GPG_ERR_ECHRNG = GPG_ERR_SYSTEM_ERROR | 22,
        GPG_ERR_ECOMM = GPG_ERR_SYSTEM_ERROR | 23,
        GPG_ERR_ECONNABORTED = GPG_ERR_SYSTEM_ERROR | 24,
        GPG_ERR_ECONNREFUSED = GPG_ERR_SYSTEM_ERROR | 25,
        GPG_ERR_ECONNRESET = GPG_ERR_SYSTEM_ERROR | 26,
        GPG_ERR_ED = GPG_ERR_SYSTEM_ERROR | 27,
        GPG_ERR_EDEADLK = GPG_ERR_SYSTEM_ERROR | 28,
        GPG_ERR_EDEADLOCK = GPG_ERR_SYSTEM_ERROR | 29,
        GPG_ERR_EDESTADDRREQ = GPG_ERR_SYSTEM_ERROR | 30,
        GPG_ERR_EDIED = GPG_ERR_SYSTEM_ERROR | 31,
        GPG_ERR_EDOM = GPG_ERR_SYSTEM_ERROR | 32,
        GPG_ERR_EDOTDOT = GPG_ERR_SYSTEM_ERROR | 33,
        GPG_ERR_EDQUOT = GPG_ERR_SYSTEM_ERROR | 34,
        GPG_ERR_EEXIST = GPG_ERR_SYSTEM_ERROR | 35,
        GPG_ERR_EFAULT = GPG_ERR_SYSTEM_ERROR | 36,
        GPG_ERR_EFBIG = GPG_ERR_SYSTEM_ERROR | 37,
        GPG_ERR_EFTYPE = GPG_ERR_SYSTEM_ERROR | 38,
        GPG_ERR_EGRATUITOUS = GPG_ERR_SYSTEM_ERROR | 39,
        GPG_ERR_EGREGIOUS = GPG_ERR_SYSTEM_ERROR | 40,
        GPG_ERR_EHOSTDOWN = GPG_ERR_SYSTEM_ERROR | 41,
        GPG_ERR_EHOSTUNREACH = GPG_ERR_SYSTEM_ERROR | 42,
        GPG_ERR_EIDRM = GPG_ERR_SYSTEM_ERROR | 43,
        GPG_ERR_EIEIO = GPG_ERR_SYSTEM_ERROR | 44,
        GPG_ERR_EILSEQ = GPG_ERR_SYSTEM_ERROR | 45,
        GPG_ERR_EINPROGRESS = GPG_ERR_SYSTEM_ERROR | 46,
        GPG_ERR_EINTR = GPG_ERR_SYSTEM_ERROR | 47,
        GPG_ERR_EINVAL = GPG_ERR_SYSTEM_ERROR | 48,
        GPG_ERR_EIO = GPG_ERR_SYSTEM_ERROR | 49,
        GPG_ERR_EISCONN = GPG_ERR_SYSTEM_ERROR | 50,
        GPG_ERR_EISDIR = GPG_ERR_SYSTEM_ERROR | 51,
        GPG_ERR_EISNAM = GPG_ERR_SYSTEM_ERROR | 52,
        GPG_ERR_EL2HLT = GPG_ERR_SYSTEM_ERROR | 53,
        GPG_ERR_EL2NSYNC = GPG_ERR_SYSTEM_ERROR | 54,
        GPG_ERR_EL3HLT = GPG_ERR_SYSTEM_ERROR | 55,
        GPG_ERR_EL3RST = GPG_ERR_SYSTEM_ERROR | 56,
        GPG_ERR_ELIBACC = GPG_ERR_SYSTEM_ERROR | 57,
        GPG_ERR_ELIBBAD = GPG_ERR_SYSTEM_ERROR | 58,
        GPG_ERR_ELIBEXEC = GPG_ERR_SYSTEM_ERROR | 59,
        GPG_ERR_ELIBMAX = GPG_ERR_SYSTEM_ERROR | 60,
        GPG_ERR_ELIBSCN = GPG_ERR_SYSTEM_ERROR | 61,
        GPG_ERR_ELNRNG = GPG_ERR_SYSTEM_ERROR | 62,
        GPG_ERR_ELOOP = GPG_ERR_SYSTEM_ERROR | 63,
        GPG_ERR_EMEDIUMTYPE = GPG_ERR_SYSTEM_ERROR | 64,
        GPG_ERR_EMFILE = GPG_ERR_SYSTEM_ERROR | 65,
        GPG_ERR_EMLINK = GPG_ERR_SYSTEM_ERROR | 66,
        GPG_ERR_EMSGSIZE = GPG_ERR_SYSTEM_ERROR | 67,
        GPG_ERR_EMULTIHOP = GPG_ERR_SYSTEM_ERROR | 68,
        GPG_ERR_ENAMETOOLONG = GPG_ERR_SYSTEM_ERROR | 69,
        GPG_ERR_ENAVAIL = GPG_ERR_SYSTEM_ERROR | 70,
        GPG_ERR_ENEEDAUTH = GPG_ERR_SYSTEM_ERROR | 71,
        GPG_ERR_ENETDOWN = GPG_ERR_SYSTEM_ERROR | 72,
        GPG_ERR_ENETRESET = GPG_ERR_SYSTEM_ERROR | 73,
        GPG_ERR_ENETUNREACH = GPG_ERR_SYSTEM_ERROR | 74,
        GPG_ERR_ENFILE = GPG_ERR_SYSTEM_ERROR | 75,
        GPG_ERR_ENOANO = GPG_ERR_SYSTEM_ERROR | 76,
        GPG_ERR_ENOBUFS = GPG_ERR_SYSTEM_ERROR | 77,
        GPG_ERR_ENOCSI = GPG_ERR_SYSTEM_ERROR | 78,
        GPG_ERR_ENODATA = GPG_ERR_SYSTEM_ERROR | 79,
        GPG_ERR_ENODEV = GPG_ERR_SYSTEM_ERROR | 80,
        GPG_ERR_ENOENT = GPG_ERR_SYSTEM_ERROR | 81,
        GPG_ERR_ENOEXEC = GPG_ERR_SYSTEM_ERROR | 82,
        GPG_ERR_ENOLCK = GPG_ERR_SYSTEM_ERROR | 83,
        GPG_ERR_ENOLINK = GPG_ERR_SYSTEM_ERROR | 84,
        GPG_ERR_ENOMEDIUM = GPG_ERR_SYSTEM_ERROR | 85,
        GPG_ERR_ENOMEM = GPG_ERR_SYSTEM_ERROR | 86,
        GPG_ERR_ENOMSG = GPG_ERR_SYSTEM_ERROR | 87,
        GPG_ERR_ENONET = GPG_ERR_SYSTEM_ERROR | 88,
        GPG_ERR_ENOPKG = GPG_ERR_SYSTEM_ERROR | 89,
        GPG_ERR_ENOPROTOOPT = GPG_ERR_SYSTEM_ERROR | 90,
        GPG_ERR_ENOSPC = GPG_ERR_SYSTEM_ERROR | 91,
        GPG_ERR_ENOSR = GPG_ERR_SYSTEM_ERROR | 92,
        GPG_ERR_ENOSTR = GPG_ERR_SYSTEM_ERROR | 93,
        GPG_ERR_ENOSYS = GPG_ERR_SYSTEM_ERROR | 94,
        GPG_ERR_ENOTBLK = GPG_ERR_SYSTEM_ERROR | 95,
        GPG_ERR_ENOTCONN = GPG_ERR_SYSTEM_ERROR | 96,
        GPG_ERR_ENOTDIR = GPG_ERR_SYSTEM_ERROR | 97,
        GPG_ERR_ENOTEMPTY = GPG_ERR_SYSTEM_ERROR | 98,
        GPG_ERR_ENOTNAM = GPG_ERR_SYSTEM_ERROR | 99,
        GPG_ERR_ENOTSOCK = GPG_ERR_SYSTEM_ERROR | 100,
        GPG_ERR_ENOTSUP = GPG_ERR_SYSTEM_ERROR | 101,
        GPG_ERR_ENOTTY = GPG_ERR_SYSTEM_ERROR | 102,
        GPG_ERR_ENOTUNIQ = GPG_ERR_SYSTEM_ERROR | 103,
        GPG_ERR_ENXIO = GPG_ERR_SYSTEM_ERROR | 104,
        GPG_ERR_EOPNOTSUPP = GPG_ERR_SYSTEM_ERROR | 105,
        GPG_ERR_EOVERFLOW = GPG_ERR_SYSTEM_ERROR | 106,
        GPG_ERR_EPERM = GPG_ERR_SYSTEM_ERROR | 107,
        GPG_ERR_EPFNOSUPPORT = GPG_ERR_SYSTEM_ERROR | 108,
        GPG_ERR_EPIPE = GPG_ERR_SYSTEM_ERROR | 109,
        GPG_ERR_EPROCLIM = GPG_ERR_SYSTEM_ERROR | 110,
        GPG_ERR_EPROCUNAVAIL = GPG_ERR_SYSTEM_ERROR | 111,
        GPG_ERR_EPROGMISMATCH = GPG_ERR_SYSTEM_ERROR | 112,
        GPG_ERR_EPROGUNAVAIL = GPG_ERR_SYSTEM_ERROR | 113,
        GPG_ERR_EPROTO = GPG_ERR_SYSTEM_ERROR | 114,
        GPG_ERR_EPROTONOSUPPORT = GPG_ERR_SYSTEM_ERROR | 115,
        GPG_ERR_EPROTOTYPE = GPG_ERR_SYSTEM_ERROR | 116,
        GPG_ERR_ERANGE = GPG_ERR_SYSTEM_ERROR | 117,
        GPG_ERR_EREMCHG = GPG_ERR_SYSTEM_ERROR | 118,
        GPG_ERR_EREMOTE = GPG_ERR_SYSTEM_ERROR | 119,
        GPG_ERR_EREMOTEIO = GPG_ERR_SYSTEM_ERROR | 120,
        GPG_ERR_ERESTART = GPG_ERR_SYSTEM_ERROR | 121,
        GPG_ERR_EROFS = GPG_ERR_SYSTEM_ERROR | 122,
        GPG_ERR_ERPCMISMATCH = GPG_ERR_SYSTEM_ERROR | 123,
        GPG_ERR_ESHUTDOWN = GPG_ERR_SYSTEM_ERROR | 124,
        GPG_ERR_ESOCKTNOSUPPORT = GPG_ERR_SYSTEM_ERROR | 125,
        GPG_ERR_ESPIPE = GPG_ERR_SYSTEM_ERROR | 126,
        GPG_ERR_ESRCH = GPG_ERR_SYSTEM_ERROR | 127,
        GPG_ERR_ESRMNT = GPG_ERR_SYSTEM_ERROR | 128,
        GPG_ERR_ESTALE = GPG_ERR_SYSTEM_ERROR | 129,
        GPG_ERR_ESTRPIPE = GPG_ERR_SYSTEM_ERROR | 130,
        GPG_ERR_ETIME = GPG_ERR_SYSTEM_ERROR | 131,
        GPG_ERR_ETIMEDOUT = GPG_ERR_SYSTEM_ERROR | 132,
        GPG_ERR_ETOOMANYREFS = GPG_ERR_SYSTEM_ERROR | 133,
        GPG_ERR_ETXTBSY = GPG_ERR_SYSTEM_ERROR | 134,
        GPG_ERR_EUCLEAN = GPG_ERR_SYSTEM_ERROR | 135,
        GPG_ERR_EUNATCH = GPG_ERR_SYSTEM_ERROR | 136,
        GPG_ERR_EUSERS = GPG_ERR_SYSTEM_ERROR | 137,
        GPG_ERR_EWOULDBLOCK = GPG_ERR_SYSTEM_ERROR | 138,
        GPG_ERR_EXDEV = GPG_ERR_SYSTEM_ERROR | 139,
        GPG_ERR_EXFULL = GPG_ERR_SYSTEM_ERROR | 140,

        GPG_ERR_CODE_DIM = 65536
    }
}
pub use gpg_err_code_t::*;

pub const GPG_ERR_CODE_MASK: gpg_error_t = (GPG_ERR_CODE_DIM as gpg_error_t) - 1;

pub const GPG_ERR_SOURCE_MASK: gpg_error_t = (GPG_ERR_SOURCE_DIM as gpg_error_t) - 1;
pub const GPG_ERR_SOURCE_SHIFT: gpg_error_t = 24;

pub fn gpg_err_make(source: gpg_err_source_t, code: gpg_err_code_t) -> gpg_error_t {
    if code == GPG_ERR_NO_ERROR {
        GPG_ERR_NO_ERROR as gpg_error_t
    } else {
        (((source as gpg_error_t) & GPG_ERR_SOURCE_MASK) << GPG_ERR_SOURCE_SHIFT) |
            ((code as gpg_error_t) & GPG_ERR_CODE_MASK)
    }
}

pub fn gpg_err_code(err: gpg_error_t) -> gpg_err_code_t {
    gpg_err_code_t::from_u32(err & GPG_ERR_CODE_MASK).unwrap()
}

pub fn gpg_err_source(err: gpg_error_t) -> gpg_err_source_t {
    gpg_err_source_t::from_u32((err >> GPG_ERR_SOURCE_SHIFT) & GPG_ERR_SOURCE_MASK).unwrap()
}

#[link(name = "gpg-error")]
extern {
    pub fn gpg_err_init() -> gpg_error_t;
    pub fn gpg_err_deinit(mode: c_int);

    pub fn gpg_strerror(err: gpg_error_t) -> *const c_char;
    pub fn gpg_strerror_r(err: gpg_error_t, buf: *mut c_char, buflen: size_t) -> c_int;

    pub fn gpg_strsource(err: gpg_error_t) -> *const c_char;

    pub fn gpg_err_code_to_errno(code: gpg_err_code_t) -> c_int;
    pub fn gpg_err_code_from_syserror() -> gpg_err_code_t;

    pub fn gpg_err_set_errno(err: c_int);

    pub fn gpg_error_check_version(req_version: *const c_char) -> *const c_char;
}
