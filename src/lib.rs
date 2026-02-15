#![warn(missing_docs)]

//! Constant string with support for [Serde](https://crates.io/crates/serde) and [Utoipa](https://crates.io/crates/utoipa).
//!
//! # Example
//! ```
//! # extern crate serde;
//! # extern crate utoipa;
//! #
//! use constant_string::constant_string;
//! use serde::{Deserialize, Serialize};
//! use utoipa::ToSchema;
//!
//! constant_string!(NotFoundErrorCode, NOT_FOUND_ERROR_CODE, "notFound");
//!
//! #[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
//! #[serde(rename_all = "camelCase")]
//! struct NotFoundError {
//!     #[schema(inline)]
//!     code: NotFoundErrorCode,
//! }
//! ```
//!
//! # Features
//! - `serde` - Implement [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) and [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) traits from [`serde`](https://docs.rs/serde/latest/serde/).
//! - `utoipa` - Implement [`ToSchema`](https://docs.rs/utoipa/latest/utoipa/trait.ToSchema.html) trait from [`utoipa`](https://docs.rs/utoipa/latest/utoipa/).

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(all(feature = "serde", feature = "utoipa"))]
/// Implement a constant string.
///
/// # Example
/// ```
/// # use constant_string::constant_string;
/// #
/// constant_string!(NotFoundErrorCode, NOT_FOUND_ERROR_CODE, "notFound");
/// ```
#[macro_export]
macro_rules! constant_string {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::constant_string_base!($name, $code_name, $code);
        $crate::constant_string_serde!($name, $code_name, $code);
        $crate::constant_string_utoipa!($name, $code_name, $code);
    };
}

#[cfg(all(feature = "serde", not(feature = "utoipa")))]
/// Implement a constant string.
///
/// # Example
/// ```
/// # use constant_string::constant_string;
/// #
/// constant_string!(NotFoundErrorCode, NOT_FOUND_ERROR_CODE, "notFound");
/// ```
#[macro_export]
macro_rules! constant_string {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::constant_string_base!($name, $code_name, $code);
        $crate::constant_string_serde!($name, $code_name, $code);
    };
}

#[cfg(all(not(feature = "serde"), feature = "utoipa"))]
/// Implement a constant string.
///
/// # Example
/// ```
/// # use constant_string::constant_string;
/// #
/// constant_string!(NotFoundErrorCode, NOT_FOUND_ERROR_CODE, "notFound");
/// ```
#[macro_export]
macro_rules! constant_string {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::constant_string_base!($name, $code_name, $code);
        $crate::constant_string_utoipa!($name, $code_name, $code);
    };
}

#[cfg(all(not(feature = "serde"), not(feature = "utoipa")))]
/// Implement a constant string.
///
/// # Example
/// ```
/// # use constant_string::constant_string;
/// #
/// constant_string!(NotFoundErrorCode, NOT_FOUND_ERROR_CODE, "notFound");
/// ```
#[macro_export]
macro_rules! constant_string {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::constant_string_base!($name, $code_name, $code);
    };
}

/// Implement a constant string.
#[doc(hidden)]
#[macro_export]
macro_rules! constant_string_base {
    ($name:ident, $code_name:ident, $code:literal) => {
        #[doc = concat!("Constant for [`", stringify!($name), "`].")]
        const $code_name: &str = $code;

        #[doc = concat!("Constant string `", stringify!($code), "`.")]
        #[derive(Eq, PartialEq)]
        pub struct $name;

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                $code_name
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&**self, f)
            }
        }
    };
}

/// Implement [`serde`] traits for a constant string.
#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! constant_string_serde {
    ($name:ident, $code_name:ident, $code:literal) => {
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer
                    .deserialize_any($crate::serde::MustBeStrVisitor($code_name))
                    .map(|()| Self)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str($code_name)
            }
        }
    };
}

/// Implement [`utoipa`] traits for a constant string.
#[cfg(feature = "utoipa")]
#[doc(hidden)]
#[macro_export]
macro_rules! constant_string_utoipa {
    ($name:ident, $code_name:ident, $code:literal) => {
        impl ::utoipa::PartialSchema for $name {
            fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::schema::Schema> {
                ::utoipa::openapi::schema::ObjectBuilder::new()
                    .schema_type(::utoipa::openapi::schema::Type::String)
                    .enum_values(Some([$code_name]))
                    .build()
                    .into()
            }
        }

        impl ::utoipa::ToSchema for $name {}
    };
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    constant_string!(Constant, CONSTANT, "constant");

    #[test]
    fn constant() {
        assert_eq!(Constant.deref(), "constant");
        assert_eq!(Constant.to_string(), "constant".to_owned());
    }

    #[test]
    #[expect(clippy::default_constructed_unit_structs)]
    fn default() {
        assert_eq!(Constant::default().deref(), "constant");
        assert_eq!(Constant::default().to_string(), "constant".to_owned());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde() {
        assert_eq!(
            "\"constant\"",
            serde_json::to_string(&Constant).expect("serializable value")
        );
        assert_eq!(
            Constant,
            serde_json::from_str("\"constant\"").expect("deserializable value")
        );
    }

    #[cfg(feature = "utoipa")]
    #[test]
    fn utoipa() {
        use utoipa::{
            PartialSchema,
            openapi::{
                RefOr, Type,
                schema::{Object, Schema},
            },
        };

        assert_eq!(
            RefOr::T(Schema::Object(
                Object::builder()
                    .schema_type(Type::String)
                    .enum_values(Some(["constant"]))
                    .build()
            )),
            Constant::schema()
        )
    }
}
