#![allow(unused_qualifications)]

use validator::Validate;

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddPartitionSpecUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "spec")]
    pub spec: models::PartitionSpec,

}


impl AddPartitionSpecUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, spec: models::PartitionSpec, ) -> AddPartitionSpecUpdate {
        AddPartitionSpecUpdate {
            action,
            spec,
        }
    }
}

/// Converts the AddPartitionSpecUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddPartitionSpecUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

            // Skipping spec in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddPartitionSpecUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddPartitionSpecUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub spec: Vec<models::PartitionSpec>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddPartitionSpecUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spec" => intermediate_rep.spec.push(<models::PartitionSpec as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddPartitionSpecUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddPartitionSpecUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in AddPartitionSpecUpdate".to_string())?,
            spec: intermediate_rep.spec.into_iter().next().ok_or_else(|| "spec missing in AddPartitionSpecUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddPartitionSpecUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddPartitionSpecUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddPartitionSpecUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddPartitionSpecUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddPartitionSpecUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddPartitionSpecUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddPartitionSpecUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddSchemaUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "schema")]
    pub schema: models::Schema,

    /// The highest assigned column ID for the table. This is used to ensure columns are always assigned an unused ID when evolving schemas. When omitted, it will be computed on the server side.
    #[serde(rename = "last-column-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_column_id: Option<i32>,

}


impl AddSchemaUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, schema: models::Schema, ) -> AddSchemaUpdate {
        AddSchemaUpdate {
            action,
            schema,
            last_column_id: None,
        }
    }
}

/// Converts the AddSchemaUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddSchemaUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

            // Skipping schema in query parameter serialization


            self.last_column_id.as_ref().map(|last_column_id| {
                [
                    "last-column-id".to_string(),
                    last_column_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddSchemaUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddSchemaUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub schema: Vec<models::Schema>,
            pub last_column_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddSchemaUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema" => intermediate_rep.schema.push(<models::Schema as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last-column-id" => intermediate_rep.last_column_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddSchemaUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddSchemaUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in AddSchemaUpdate".to_string())?,
            schema: intermediate_rep.schema.into_iter().next().ok_or_else(|| "schema missing in AddSchemaUpdate".to_string())?,
            last_column_id: intermediate_rep.last_column_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddSchemaUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddSchemaUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddSchemaUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddSchemaUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddSchemaUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddSchemaUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddSchemaUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddSnapshotUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "snapshot")]
    pub snapshot: models::Snapshot,

}


impl AddSnapshotUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, snapshot: models::Snapshot, ) -> AddSnapshotUpdate {
        AddSnapshotUpdate {
            action,
            snapshot,
        }
    }
}

/// Converts the AddSnapshotUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddSnapshotUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

            // Skipping snapshot in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddSnapshotUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddSnapshotUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub snapshot: Vec<models::Snapshot>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddSnapshotUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot" => intermediate_rep.snapshot.push(<models::Snapshot as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddSnapshotUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddSnapshotUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in AddSnapshotUpdate".to_string())?,
            snapshot: intermediate_rep.snapshot.into_iter().next().ok_or_else(|| "snapshot missing in AddSnapshotUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddSnapshotUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddSnapshotUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddSnapshotUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddSnapshotUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddSnapshotUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddSnapshotUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddSnapshotUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddSortOrderUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "sort-order")]
    pub sort_order: models::SortOrder,

}


impl AddSortOrderUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, sort_order: models::SortOrder, ) -> AddSortOrderUpdate {
        AddSortOrderUpdate {
            action,
            sort_order,
        }
    }
}

/// Converts the AddSortOrderUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddSortOrderUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

            // Skipping sort-order in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddSortOrderUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddSortOrderUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub sort_order: Vec<models::SortOrder>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddSortOrderUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sort-order" => intermediate_rep.sort_order.push(<models::SortOrder as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddSortOrderUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddSortOrderUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in AddSortOrderUpdate".to_string())?,
            sort_order: intermediate_rep.sort_order.into_iter().next().ok_or_else(|| "sort-order missing in AddSortOrderUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddSortOrderUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddSortOrderUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddSortOrderUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddSortOrderUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddSortOrderUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddSortOrderUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddSortOrderUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddViewVersionUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "view-version")]
    pub view_version: models::ViewVersion,

}


impl AddViewVersionUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, view_version: models::ViewVersion, ) -> AddViewVersionUpdate {
        AddViewVersionUpdate {
            action,
            view_version,
        }
    }
}

/// Converts the AddViewVersionUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddViewVersionUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

            // Skipping view-version in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddViewVersionUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddViewVersionUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub view_version: Vec<models::ViewVersion>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddViewVersionUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "view-version" => intermediate_rep.view_version.push(<models::ViewVersion as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddViewVersionUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddViewVersionUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in AddViewVersionUpdate".to_string())?,
            view_version: intermediate_rep.view_version.into_iter().next().ok_or_else(|| "view-version missing in AddViewVersionUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddViewVersionUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddViewVersionUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddViewVersionUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddViewVersionUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddViewVersionUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddViewVersionUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddViewVersionUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AndOrExpression {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "left")]
    pub left: models::Expression,

    #[serde(rename = "right")]
    pub right: models::Expression,

}


impl AndOrExpression {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, left: models::Expression, right: models::Expression, ) -> AndOrExpression {
        AndOrExpression {
            r#type,
            left,
            right,
        }
    }
}

/// Converts the AndOrExpression value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AndOrExpression {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping left in query parameter serialization

            // Skipping right in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AndOrExpression value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AndOrExpression {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub left: Vec<models::Expression>,
            pub right: Vec<models::Expression>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AndOrExpression".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "left" => intermediate_rep.left.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "right" => intermediate_rep.right.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AndOrExpression".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AndOrExpression {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AndOrExpression".to_string())?,
            left: intermediate_rep.left.into_iter().next().ok_or_else(|| "left missing in AndOrExpression".to_string())?,
            right: intermediate_rep.right.into_iter().next().ok_or_else(|| "right missing in AndOrExpression".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AndOrExpression> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AndOrExpression>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AndOrExpression>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AndOrExpression - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AndOrExpression> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AndOrExpression as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AndOrExpression - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table must not already exist; used for create transactions
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertCreate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

}


impl AssertCreate {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, ) -> AssertCreate {
        AssertCreate {
            r#type,
        }
    }
}

/// Converts the AssertCreate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertCreate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertCreate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertCreate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertCreate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertCreate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertCreate {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertCreate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertCreate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertCreate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertCreate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertCreate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertCreate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertCreate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertCreate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table's current schema id must match the requirement's `current-schema-id`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertCurrentSchemaId {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "current-schema-id")]
    pub current_schema_id: i32,

}


impl AssertCurrentSchemaId {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, current_schema_id: i32, ) -> AssertCurrentSchemaId {
        AssertCurrentSchemaId {
            r#type,
            current_schema_id,
        }
    }
}

/// Converts the AssertCurrentSchemaId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertCurrentSchemaId {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("current-schema-id".to_string()),
            Some(self.current_schema_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertCurrentSchemaId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertCurrentSchemaId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub current_schema_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertCurrentSchemaId".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "current-schema-id" => intermediate_rep.current_schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertCurrentSchemaId".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertCurrentSchemaId {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertCurrentSchemaId".to_string())?,
            current_schema_id: intermediate_rep.current_schema_id.into_iter().next().ok_or_else(|| "current-schema-id missing in AssertCurrentSchemaId".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertCurrentSchemaId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertCurrentSchemaId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertCurrentSchemaId>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertCurrentSchemaId - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertCurrentSchemaId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertCurrentSchemaId as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertCurrentSchemaId - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table's default sort order id must match the requirement's `default-sort-order-id`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertDefaultSortOrderId {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "default-sort-order-id")]
    pub default_sort_order_id: i32,

}


impl AssertDefaultSortOrderId {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, default_sort_order_id: i32, ) -> AssertDefaultSortOrderId {
        AssertDefaultSortOrderId {
            r#type,
            default_sort_order_id,
        }
    }
}

/// Converts the AssertDefaultSortOrderId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertDefaultSortOrderId {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("default-sort-order-id".to_string()),
            Some(self.default_sort_order_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertDefaultSortOrderId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertDefaultSortOrderId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub default_sort_order_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertDefaultSortOrderId".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "default-sort-order-id" => intermediate_rep.default_sort_order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertDefaultSortOrderId".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertDefaultSortOrderId {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertDefaultSortOrderId".to_string())?,
            default_sort_order_id: intermediate_rep.default_sort_order_id.into_iter().next().ok_or_else(|| "default-sort-order-id missing in AssertDefaultSortOrderId".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertDefaultSortOrderId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertDefaultSortOrderId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertDefaultSortOrderId>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertDefaultSortOrderId - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertDefaultSortOrderId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertDefaultSortOrderId as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertDefaultSortOrderId - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table's default spec id must match the requirement's `default-spec-id`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertDefaultSpecId {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "default-spec-id")]
    pub default_spec_id: i32,

}


impl AssertDefaultSpecId {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, default_spec_id: i32, ) -> AssertDefaultSpecId {
        AssertDefaultSpecId {
            r#type,
            default_spec_id,
        }
    }
}

/// Converts the AssertDefaultSpecId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertDefaultSpecId {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("default-spec-id".to_string()),
            Some(self.default_spec_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertDefaultSpecId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertDefaultSpecId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub default_spec_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertDefaultSpecId".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "default-spec-id" => intermediate_rep.default_spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertDefaultSpecId".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertDefaultSpecId {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertDefaultSpecId".to_string())?,
            default_spec_id: intermediate_rep.default_spec_id.into_iter().next().ok_or_else(|| "default-spec-id missing in AssertDefaultSpecId".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertDefaultSpecId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertDefaultSpecId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertDefaultSpecId>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertDefaultSpecId - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertDefaultSpecId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertDefaultSpecId as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertDefaultSpecId - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table's last assigned column id must match the requirement's `last-assigned-field-id`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertLastAssignedFieldId {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "last-assigned-field-id")]
    pub last_assigned_field_id: i32,

}


impl AssertLastAssignedFieldId {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, last_assigned_field_id: i32, ) -> AssertLastAssignedFieldId {
        AssertLastAssignedFieldId {
            r#type,
            last_assigned_field_id,
        }
    }
}

/// Converts the AssertLastAssignedFieldId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertLastAssignedFieldId {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("last-assigned-field-id".to_string()),
            Some(self.last_assigned_field_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertLastAssignedFieldId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertLastAssignedFieldId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub last_assigned_field_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertLastAssignedFieldId".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last-assigned-field-id" => intermediate_rep.last_assigned_field_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertLastAssignedFieldId".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertLastAssignedFieldId {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertLastAssignedFieldId".to_string())?,
            last_assigned_field_id: intermediate_rep.last_assigned_field_id.into_iter().next().ok_or_else(|| "last-assigned-field-id missing in AssertLastAssignedFieldId".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertLastAssignedFieldId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertLastAssignedFieldId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertLastAssignedFieldId>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertLastAssignedFieldId - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertLastAssignedFieldId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertLastAssignedFieldId as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertLastAssignedFieldId - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table's last assigned partition id must match the requirement's `last-assigned-partition-id`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertLastAssignedPartitionId {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "last-assigned-partition-id")]
    pub last_assigned_partition_id: i32,

}


impl AssertLastAssignedPartitionId {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, last_assigned_partition_id: i32, ) -> AssertLastAssignedPartitionId {
        AssertLastAssignedPartitionId {
            r#type,
            last_assigned_partition_id,
        }
    }
}

/// Converts the AssertLastAssignedPartitionId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertLastAssignedPartitionId {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("last-assigned-partition-id".to_string()),
            Some(self.last_assigned_partition_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertLastAssignedPartitionId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertLastAssignedPartitionId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub last_assigned_partition_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertLastAssignedPartitionId".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last-assigned-partition-id" => intermediate_rep.last_assigned_partition_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertLastAssignedPartitionId".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertLastAssignedPartitionId {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertLastAssignedPartitionId".to_string())?,
            last_assigned_partition_id: intermediate_rep.last_assigned_partition_id.into_iter().next().ok_or_else(|| "last-assigned-partition-id missing in AssertLastAssignedPartitionId".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertLastAssignedPartitionId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertLastAssignedPartitionId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertLastAssignedPartitionId>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertLastAssignedPartitionId - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertLastAssignedPartitionId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertLastAssignedPartitionId as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertLastAssignedPartitionId - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table branch or tag identified by the requirement's `ref` must reference the requirement's `snapshot-id`; if `snapshot-id` is `null` or missing, the ref must not already exist
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertRefSnapshotId {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "ref")]
    pub r#ref: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

}


impl AssertRefSnapshotId {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, r#ref: String, snapshot_id: i64, ) -> AssertRefSnapshotId {
        AssertRefSnapshotId {
            r#type,
            r#ref,
            snapshot_id,
        }
    }
}

/// Converts the AssertRefSnapshotId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertRefSnapshotId {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("ref".to_string()),
            Some(self.r#ref.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertRefSnapshotId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertRefSnapshotId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub r#ref: Vec<String>,
            pub snapshot_id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertRefSnapshotId".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ref" => intermediate_rep.r#ref.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertRefSnapshotId".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertRefSnapshotId {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertRefSnapshotId".to_string())?,
            r#ref: intermediate_rep.r#ref.into_iter().next().ok_or_else(|| "ref missing in AssertRefSnapshotId".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in AssertRefSnapshotId".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertRefSnapshotId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertRefSnapshotId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertRefSnapshotId>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertRefSnapshotId - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertRefSnapshotId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertRefSnapshotId as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertRefSnapshotId - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The table UUID must match the requirement's `uuid`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertTableUuid {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "uuid")]
    pub uuid: String,

}


impl AssertTableUuid {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, uuid: String, ) -> AssertTableUuid {
        AssertTableUuid {
            r#type,
            uuid,
        }
    }
}

/// Converts the AssertTableUuid value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertTableUuid {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("uuid".to_string()),
            Some(self.uuid.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertTableUuid value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertTableUuid {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub uuid: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertTableUuid".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "uuid" => intermediate_rep.uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertTableUuid".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertTableUuid {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertTableUuid".to_string())?,
            uuid: intermediate_rep.uuid.into_iter().next().ok_or_else(|| "uuid missing in AssertTableUuid".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertTableUuid> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertTableUuid>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertTableUuid>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertTableUuid - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertTableUuid> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertTableUuid as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertTableUuid - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The view UUID must match the requirement's `uuid`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssertViewUuid {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "uuid")]
    pub uuid: String,

}


impl AssertViewUuid {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, uuid: String, ) -> AssertViewUuid {
        AssertViewUuid {
            r#type,
            uuid,
        }
    }
}

/// Converts the AssertViewUuid value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssertViewUuid {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("uuid".to_string()),
            Some(self.uuid.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssertViewUuid value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssertViewUuid {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub uuid: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssertViewUuid".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "uuid" => intermediate_rep.uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssertViewUuid".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssertViewUuid {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in AssertViewUuid".to_string())?,
            uuid: intermediate_rep.uuid.into_iter().next().ok_or_else(|| "uuid missing in AssertViewUuid".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssertViewUuid> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssertViewUuid>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssertViewUuid>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssertViewUuid - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssertViewUuid> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssertViewUuid as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssertViewUuid - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Assigning a UUID to a table/view should only be done when creating the table/view. It is not safe to re-assign the UUID if a table/view already has a UUID assigned
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AssignUuidUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "uuid")]
    pub uuid: String,

}


impl AssignUuidUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, uuid: String, ) -> AssignUuidUpdate {
        AssignUuidUpdate {
            action,
            uuid,
        }
    }
}

/// Converts the AssignUuidUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AssignUuidUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("uuid".to_string()),
            Some(self.uuid.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AssignUuidUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AssignUuidUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub uuid: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AssignUuidUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "uuid" => intermediate_rep.uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AssignUuidUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AssignUuidUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in AssignUuidUpdate".to_string())?,
            uuid: intermediate_rep.uuid.into_iter().next().ok_or_else(|| "uuid missing in AssignUuidUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AssignUuidUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AssignUuidUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AssignUuidUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AssignUuidUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AssignUuidUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AssignUuidUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AssignUuidUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BaseUpdate {
    #[serde(rename = "action")]
    pub action: String,

}


impl BaseUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, ) -> BaseUpdate {
        BaseUpdate {
            action,
        }
    }
}

/// Converts the BaseUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BaseUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BaseUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BaseUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BaseUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BaseUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BaseUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in BaseUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BaseUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BaseUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BaseUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BaseUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BaseUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BaseUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BaseUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Binary type values are stored and serialized as an uppercase hexadecimal string
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BinaryTypeValue(String);

impl std::convert::From<String> for BinaryTypeValue {
    fn from(x: String) -> Self {
        BinaryTypeValue(x)
    }
}

impl std::string::ToString for BinaryTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for BinaryTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(BinaryTypeValue(x.to_string()))
    }
}

impl std::convert::From<BinaryTypeValue> for String {
    fn from(x: BinaryTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for BinaryTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for BinaryTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BlobMetadata {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "sequence-number")]
    pub sequence_number: i64,

    #[serde(rename = "fields")]
    pub fields: Vec<i32>,

    #[serde(rename = "properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<serde_json::Value>,

}


impl BlobMetadata {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, snapshot_id: i64, sequence_number: i64, fields: Vec<i32>, ) -> BlobMetadata {
        BlobMetadata {
            r#type,
            snapshot_id,
            sequence_number,
            fields,
            properties: None,
        }
    }
}

/// Converts the BlobMetadata value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BlobMetadata {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            Some("sequence-number".to_string()),
            Some(self.sequence_number.to_string()),


            Some("fields".to_string()),
            Some(self.fields.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping properties in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BlobMetadata value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BlobMetadata {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub sequence_number: Vec<i64>,
            pub fields: Vec<Vec<i32>>,
            pub properties: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BlobMetadata".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sequence-number" => intermediate_rep.sequence_number.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in BlobMetadata".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "properties" => intermediate_rep.properties.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BlobMetadata".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BlobMetadata {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in BlobMetadata".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in BlobMetadata".to_string())?,
            sequence_number: intermediate_rep.sequence_number.into_iter().next().ok_or_else(|| "sequence-number missing in BlobMetadata".to_string())?,
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in BlobMetadata".to_string())?,
            properties: intermediate_rep.properties.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BlobMetadata> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BlobMetadata>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BlobMetadata>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BlobMetadata - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BlobMetadata> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BlobMetadata as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BlobMetadata - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BooleanTypeValue(bool);

impl std::convert::From<bool> for BooleanTypeValue {
    fn from(x: bool) -> Self {
        BooleanTypeValue(x)
    }
}

impl std::convert::From<BooleanTypeValue> for bool {
    fn from(x: BooleanTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for BooleanTypeValue {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for BooleanTypeValue {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}


/// Server-provided configuration for the catalog.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CatalogConfig {
    /// Properties that should be used to override client configuration; applied after defaults and client configuration.
    #[serde(rename = "overrides")]
    pub overrides: std::collections::HashMap<String, String>,

    /// Properties that should be used as default configuration; applied before client configuration.
    #[serde(rename = "defaults")]
    pub defaults: std::collections::HashMap<String, String>,

}


impl CatalogConfig {
    #[allow(clippy::new_without_default)]
    pub fn new(overrides: std::collections::HashMap<String, String>, defaults: std::collections::HashMap<String, String>, ) -> CatalogConfig {
        CatalogConfig {
            overrides,
            defaults,
        }
    }
}

/// Converts the CatalogConfig value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CatalogConfig {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping overrides in query parameter serialization

            // Skipping defaults in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CatalogConfig value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CatalogConfig {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub overrides: Vec<std::collections::HashMap<String, String>>,
            pub defaults: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CatalogConfig".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "overrides" => return std::result::Result::Err("Parsing a container in this style is not supported in CatalogConfig".to_string()),
                    "defaults" => return std::result::Result::Err("Parsing a container in this style is not supported in CatalogConfig".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CatalogConfig".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CatalogConfig {
            overrides: intermediate_rep.overrides.into_iter().next().ok_or_else(|| "overrides missing in CatalogConfig".to_string())?,
            defaults: intermediate_rep.defaults.into_iter().next().ok_or_else(|| "defaults missing in CatalogConfig".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CatalogConfig> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CatalogConfig>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CatalogConfig>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CatalogConfig - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CatalogConfig> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CatalogConfig as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CatalogConfig - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CommitReport {
    #[serde(rename = "table-name")]
    pub table_name: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "sequence-number")]
    pub sequence_number: i64,

    #[serde(rename = "operation")]
    pub operation: String,

    #[serde(rename = "metrics")]
    pub metrics: std::collections::HashMap<String, models::MetricResult>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

}


impl CommitReport {
    #[allow(clippy::new_without_default)]
    pub fn new(table_name: String, snapshot_id: i64, sequence_number: i64, operation: String, metrics: std::collections::HashMap<String, models::MetricResult>, ) -> CommitReport {
        CommitReport {
            table_name,
            snapshot_id,
            sequence_number,
            operation,
            metrics,
            metadata: None,
        }
    }
}

/// Converts the CommitReport value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CommitReport {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("table-name".to_string()),
            Some(self.table_name.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            Some("sequence-number".to_string()),
            Some(self.sequence_number.to_string()),


            Some("operation".to_string()),
            Some(self.operation.to_string()),

            // Skipping metrics in query parameter serialization
            // Skipping metrics in query parameter serialization

            // Skipping metadata in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CommitReport value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CommitReport {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub table_name: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub sequence_number: Vec<i64>,
            pub operation: Vec<String>,
            pub metrics: Vec<std::collections::HashMap<String, models::MetricResult>>,
            pub metadata: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CommitReport".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "table-name" => intermediate_rep.table_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sequence-number" => intermediate_rep.sequence_number.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "operation" => intermediate_rep.operation.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "metrics" => return std::result::Result::Err("Parsing a container in this style is not supported in CommitReport".to_string()),
                    "metadata" => return std::result::Result::Err("Parsing a container in this style is not supported in CommitReport".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CommitReport".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CommitReport {
            table_name: intermediate_rep.table_name.into_iter().next().ok_or_else(|| "table-name missing in CommitReport".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in CommitReport".to_string())?,
            sequence_number: intermediate_rep.sequence_number.into_iter().next().ok_or_else(|| "sequence-number missing in CommitReport".to_string())?,
            operation: intermediate_rep.operation.into_iter().next().ok_or_else(|| "operation missing in CommitReport".to_string())?,
            metrics: intermediate_rep.metrics.into_iter().next().ok_or_else(|| "metrics missing in CommitReport".to_string())?,
            metadata: intermediate_rep.metadata.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CommitReport> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CommitReport>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CommitReport>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CommitReport - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CommitReport> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CommitReport as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CommitReport - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CommitTableRequest {
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identifier: Option<models::TableIdentifier>,

    #[serde(rename = "requirements")]
    pub requirements: Vec<models::TableRequirement>,

    #[serde(rename = "updates")]
    pub updates: Vec<models::TableUpdate>,

}


impl CommitTableRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(requirements: Vec<models::TableRequirement>, updates: Vec<models::TableUpdate>, ) -> CommitTableRequest {
        CommitTableRequest {
            identifier: None,
            requirements,
            updates,
        }
    }
}

/// Converts the CommitTableRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CommitTableRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping identifier in query parameter serialization

            // Skipping requirements in query parameter serialization

            // Skipping updates in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CommitTableRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CommitTableRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub identifier: Vec<models::TableIdentifier>,
            pub requirements: Vec<Vec<models::TableRequirement>>,
            pub updates: Vec<Vec<models::TableUpdate>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CommitTableRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "identifier" => intermediate_rep.identifier.push(<models::TableIdentifier as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "requirements" => return std::result::Result::Err("Parsing a container in this style is not supported in CommitTableRequest".to_string()),
                    "updates" => return std::result::Result::Err("Parsing a container in this style is not supported in CommitTableRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CommitTableRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CommitTableRequest {
            identifier: intermediate_rep.identifier.into_iter().next(),
            requirements: intermediate_rep.requirements.into_iter().next().ok_or_else(|| "requirements missing in CommitTableRequest".to_string())?,
            updates: intermediate_rep.updates.into_iter().next().ok_or_else(|| "updates missing in CommitTableRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CommitTableRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CommitTableRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CommitTableRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CommitTableRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CommitTableRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CommitTableRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CommitTableRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CommitTableResponse {
    #[serde(rename = "metadata-location")]
    pub metadata_location: String,

    #[serde(rename = "metadata")]
    pub metadata: models::TableMetadata,

}


impl CommitTableResponse {
    #[allow(clippy::new_without_default)]
    pub fn new(metadata_location: String, metadata: models::TableMetadata, ) -> CommitTableResponse {
        CommitTableResponse {
            metadata_location,
            metadata,
        }
    }
}

/// Converts the CommitTableResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CommitTableResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("metadata-location".to_string()),
            Some(self.metadata_location.to_string()),

            // Skipping metadata in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CommitTableResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CommitTableResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub metadata_location: Vec<String>,
            pub metadata: Vec<models::TableMetadata>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CommitTableResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "metadata-location" => intermediate_rep.metadata_location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "metadata" => intermediate_rep.metadata.push(<models::TableMetadata as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CommitTableResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CommitTableResponse {
            metadata_location: intermediate_rep.metadata_location.into_iter().next().ok_or_else(|| "metadata-location missing in CommitTableResponse".to_string())?,
            metadata: intermediate_rep.metadata.into_iter().next().ok_or_else(|| "metadata missing in CommitTableResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CommitTableResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CommitTableResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CommitTableResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CommitTableResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CommitTableResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CommitTableResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CommitTableResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CommitTransactionRequest {
    #[serde(rename = "table-changes")]
    pub table_changes: Vec<models::CommitTableRequest>,

}


impl CommitTransactionRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(table_changes: Vec<models::CommitTableRequest>, ) -> CommitTransactionRequest {
        CommitTransactionRequest {
            table_changes,
        }
    }
}

/// Converts the CommitTransactionRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CommitTransactionRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping table-changes in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CommitTransactionRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CommitTransactionRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub table_changes: Vec<Vec<models::CommitTableRequest>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CommitTransactionRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "table-changes" => return std::result::Result::Err("Parsing a container in this style is not supported in CommitTransactionRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CommitTransactionRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CommitTransactionRequest {
            table_changes: intermediate_rep.table_changes.into_iter().next().ok_or_else(|| "table-changes missing in CommitTransactionRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CommitTransactionRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CommitTransactionRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CommitTransactionRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CommitTransactionRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CommitTransactionRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CommitTransactionRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CommitTransactionRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CommitViewRequest {
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identifier: Option<models::TableIdentifier>,

    #[serde(rename = "requirements")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requirements: Option<Vec<models::ViewRequirement>>,

    #[serde(rename = "updates")]
    pub updates: Vec<models::ViewUpdate>,

}


impl CommitViewRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(updates: Vec<models::ViewUpdate>, ) -> CommitViewRequest {
        CommitViewRequest {
            identifier: None,
            requirements: None,
            updates,
        }
    }
}

/// Converts the CommitViewRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CommitViewRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping identifier in query parameter serialization

            // Skipping requirements in query parameter serialization

            // Skipping updates in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CommitViewRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CommitViewRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub identifier: Vec<models::TableIdentifier>,
            pub requirements: Vec<Vec<models::ViewRequirement>>,
            pub updates: Vec<Vec<models::ViewUpdate>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CommitViewRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "identifier" => intermediate_rep.identifier.push(<models::TableIdentifier as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "requirements" => return std::result::Result::Err("Parsing a container in this style is not supported in CommitViewRequest".to_string()),
                    "updates" => return std::result::Result::Err("Parsing a container in this style is not supported in CommitViewRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CommitViewRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CommitViewRequest {
            identifier: intermediate_rep.identifier.into_iter().next(),
            requirements: intermediate_rep.requirements.into_iter().next(),
            updates: intermediate_rep.updates.into_iter().next().ok_or_else(|| "updates missing in CommitViewRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CommitViewRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CommitViewRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CommitViewRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CommitViewRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CommitViewRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CommitViewRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CommitViewRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContentFile {
    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "file-path")]
    pub file_path: String,

    #[serde(rename = "file-format")]
    pub file_format: models::FileFormat,

    #[serde(rename = "spec-id")]
    pub spec_id: i32,

    /// A list of partition field values ordered based on the fields of the partition spec specified by the `spec-id`
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition: Option<Vec<models::PrimitiveTypeValue>>,

    /// Total file size in bytes
    #[serde(rename = "file-size-in-bytes")]
    pub file_size_in_bytes: i64,

    /// Number of records in the file
    #[serde(rename = "record-count")]
    pub record_count: i64,

    /// Binary type values are stored and serialized as an uppercase hexadecimal string
    #[serde(rename = "key-metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_metadata: Option<String>,

    /// List of splittable offsets
    #[serde(rename = "split-offsets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub split_offsets: Option<Vec<i64>>,

    #[serde(rename = "sort-order-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_order_id: Option<i32>,

}


impl ContentFile {
    #[allow(clippy::new_without_default)]
    pub fn new(content: String, file_path: String, file_format: models::FileFormat, spec_id: i32, file_size_in_bytes: i64, record_count: i64, ) -> ContentFile {
        ContentFile {
            content,
            file_path,
            file_format,
            spec_id,
            partition: None,
            file_size_in_bytes,
            record_count,
            key_metadata: None,
            split_offsets: None,
            sort_order_id: None,
        }
    }
}

/// Converts the ContentFile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ContentFile {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("content".to_string()),
            Some(self.content.to_string()),


            Some("file-path".to_string()),
            Some(self.file_path.to_string()),

            // Skipping file-format in query parameter serialization


            Some("spec-id".to_string()),
            Some(self.spec_id.to_string()),

            // Skipping partition in query parameter serialization


            Some("file-size-in-bytes".to_string()),
            Some(self.file_size_in_bytes.to_string()),


            Some("record-count".to_string()),
            Some(self.record_count.to_string()),


            self.key_metadata.as_ref().map(|key_metadata| {
                [
                    "key-metadata".to_string(),
                    key_metadata.to_string(),
                ].join(",")
            }),


            self.split_offsets.as_ref().map(|split_offsets| {
                [
                    "split-offsets".to_string(),
                    split_offsets.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.sort_order_id.as_ref().map(|sort_order_id| {
                [
                    "sort-order-id".to_string(),
                    sort_order_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContentFile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContentFile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub content: Vec<String>,
            pub file_path: Vec<String>,
            pub file_format: Vec<models::FileFormat>,
            pub spec_id: Vec<i32>,
            pub partition: Vec<Vec<models::PrimitiveTypeValue>>,
            pub file_size_in_bytes: Vec<i64>,
            pub record_count: Vec<i64>,
            pub key_metadata: Vec<String>,
            pub split_offsets: Vec<Vec<i64>>,
            pub sort_order_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContentFile".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "content" => intermediate_rep.content.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-path" => intermediate_rep.file_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-format" => intermediate_rep.file_format.push(<models::FileFormat as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spec-id" => intermediate_rep.spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "partition" => return std::result::Result::Err("Parsing a container in this style is not supported in ContentFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "file-size-in-bytes" => intermediate_rep.file_size_in_bytes.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "record-count" => intermediate_rep.record_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key-metadata" => intermediate_rep.key_metadata.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "split-offsets" => return std::result::Result::Err("Parsing a container in this style is not supported in ContentFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "sort-order-id" => intermediate_rep.sort_order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContentFile".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContentFile {
            content: intermediate_rep.content.into_iter().next().ok_or_else(|| "content missing in ContentFile".to_string())?,
            file_path: intermediate_rep.file_path.into_iter().next().ok_or_else(|| "file-path missing in ContentFile".to_string())?,
            file_format: intermediate_rep.file_format.into_iter().next().ok_or_else(|| "file-format missing in ContentFile".to_string())?,
            spec_id: intermediate_rep.spec_id.into_iter().next().ok_or_else(|| "spec-id missing in ContentFile".to_string())?,
            partition: intermediate_rep.partition.into_iter().next(),
            file_size_in_bytes: intermediate_rep.file_size_in_bytes.into_iter().next().ok_or_else(|| "file-size-in-bytes missing in ContentFile".to_string())?,
            record_count: intermediate_rep.record_count.into_iter().next().ok_or_else(|| "record-count missing in ContentFile".to_string())?,
            key_metadata: intermediate_rep.key_metadata.into_iter().next(),
            split_offsets: intermediate_rep.split_offsets.into_iter().next(),
            sort_order_id: intermediate_rep.sort_order_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContentFile> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ContentFile>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContentFile>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ContentFile - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ContentFile> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContentFile as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ContentFile - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CountMap {
    /// List of integer column ids for each corresponding value
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys: Option<Vec<models::IntegerTypeValue>>,

    /// List of Long values, matched to 'keys' by index
    #[serde(rename = "values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<models::LongTypeValue>>,

}


impl CountMap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CountMap {
        CountMap {
            keys: None,
            values: None,
        }
    }
}

/// Converts the CountMap value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CountMap {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.keys.as_ref().map(|keys| {
                [
                    "keys".to_string(),
                    keys.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.values.as_ref().map(|values| {
                [
                    "values".to_string(),
                    values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CountMap value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CountMap {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub keys: Vec<Vec<models::IntegerTypeValue>>,
            pub values: Vec<Vec<models::LongTypeValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CountMap".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "keys" => return std::result::Result::Err("Parsing a container in this style is not supported in CountMap".to_string()),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in CountMap".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CountMap".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CountMap {
            keys: intermediate_rep.keys.into_iter().next(),
            values: intermediate_rep.values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CountMap> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CountMap>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CountMap>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CountMap - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CountMap> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CountMap as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CountMap - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CounterResult {
    #[serde(rename = "unit")]
    pub unit: String,

    #[serde(rename = "value")]
    pub value: i64,

}


impl CounterResult {
    #[allow(clippy::new_without_default)]
    pub fn new(unit: String, value: i64, ) -> CounterResult {
        CounterResult {
            unit,
            value,
        }
    }
}

/// Converts the CounterResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CounterResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("unit".to_string()),
            Some(self.unit.to_string()),


            Some("value".to_string()),
            Some(self.value.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CounterResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CounterResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub unit: Vec<String>,
            pub value: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CounterResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "unit" => intermediate_rep.unit.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CounterResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CounterResult {
            unit: intermediate_rep.unit.into_iter().next().ok_or_else(|| "unit missing in CounterResult".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in CounterResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CounterResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CounterResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CounterResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CounterResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CounterResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CounterResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CounterResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateNamespaceRequest {
    /// Reference to one or more levels of a namespace
    #[serde(rename = "namespace")]
    pub namespace: Vec<String>,

    /// Configured string to string map of properties for the namespace
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,

}


impl CreateNamespaceRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(namespace: Vec<String>, ) -> CreateNamespaceRequest {
        CreateNamespaceRequest {
            namespace,
            properties: None,
        }
    }
}

/// Converts the CreateNamespaceRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateNamespaceRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("namespace".to_string()),
            Some(self.namespace.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping properties in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateNamespaceRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateNamespaceRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub namespace: Vec<Vec<String>>,
            pub properties: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateNamespaceRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "namespace" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateNamespaceRequest".to_string()),
                    "properties" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateNamespaceRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateNamespaceRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateNamespaceRequest {
            namespace: intermediate_rep.namespace.into_iter().next().ok_or_else(|| "namespace missing in CreateNamespaceRequest".to_string())?,
            properties: intermediate_rep.properties.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateNamespaceRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateNamespaceRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateNamespaceRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateNamespaceRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateNamespaceRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateNamespaceRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateNamespaceRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateNamespaceResponse {
    /// Reference to one or more levels of a namespace
    #[serde(rename = "namespace")]
    pub namespace: Vec<String>,

    /// Properties stored on the namespace, if supported by the server.
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,

}


impl CreateNamespaceResponse {
    #[allow(clippy::new_without_default)]
    pub fn new(namespace: Vec<String>, ) -> CreateNamespaceResponse {
        CreateNamespaceResponse {
            namespace,
            properties: None,
        }
    }
}

/// Converts the CreateNamespaceResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateNamespaceResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("namespace".to_string()),
            Some(self.namespace.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping properties in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateNamespaceResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateNamespaceResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub namespace: Vec<Vec<String>>,
            pub properties: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateNamespaceResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "namespace" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateNamespaceResponse".to_string()),
                    "properties" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateNamespaceResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateNamespaceResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateNamespaceResponse {
            namespace: intermediate_rep.namespace.into_iter().next().ok_or_else(|| "namespace missing in CreateNamespaceResponse".to_string())?,
            properties: intermediate_rep.properties.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateNamespaceResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateNamespaceResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateNamespaceResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateNamespaceResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateNamespaceResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateNamespaceResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateNamespaceResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateTableRequest {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "schema")]
    pub schema: models::Schema,

    #[serde(rename = "partition-spec")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition_spec: Option<models::PartitionSpec>,

    #[serde(rename = "write-order")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub write_order: Option<models::SortOrder>,

    #[serde(rename = "stage-create")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stage_create: Option<bool>,

    #[serde(rename = "properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,

}


impl CreateTableRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, schema: models::Schema, ) -> CreateTableRequest {
        CreateTableRequest {
            name,
            location: None,
            schema,
            partition_spec: None,
            write_order: None,
            stage_create: None,
            properties: None,
        }
    }
}

/// Converts the CreateTableRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateTableRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            self.location.as_ref().map(|location| {
                [
                    "location".to_string(),
                    location.to_string(),
                ].join(",")
            }),

            // Skipping schema in query parameter serialization

            // Skipping partition-spec in query parameter serialization

            // Skipping write-order in query parameter serialization


            self.stage_create.as_ref().map(|stage_create| {
                [
                    "stage-create".to_string(),
                    stage_create.to_string(),
                ].join(",")
            }),

            // Skipping properties in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateTableRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateTableRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub location: Vec<String>,
            pub schema: Vec<models::Schema>,
            pub partition_spec: Vec<models::PartitionSpec>,
            pub write_order: Vec<models::SortOrder>,
            pub stage_create: Vec<bool>,
            pub properties: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateTableRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "location" => intermediate_rep.location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema" => intermediate_rep.schema.push(<models::Schema as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "partition-spec" => intermediate_rep.partition_spec.push(<models::PartitionSpec as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "write-order" => intermediate_rep.write_order.push(<models::SortOrder as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "stage-create" => intermediate_rep.stage_create.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "properties" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateTableRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateTableRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateTableRequest {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in CreateTableRequest".to_string())?,
            location: intermediate_rep.location.into_iter().next(),
            schema: intermediate_rep.schema.into_iter().next().ok_or_else(|| "schema missing in CreateTableRequest".to_string())?,
            partition_spec: intermediate_rep.partition_spec.into_iter().next(),
            write_order: intermediate_rep.write_order.into_iter().next(),
            stage_create: intermediate_rep.stage_create.into_iter().next(),
            properties: intermediate_rep.properties.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateTableRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateTableRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateTableRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateTableRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateTableRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateTableRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateTableRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CreateViewRequest {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "schema")]
    pub schema: models::Schema,

    #[serde(rename = "view-version")]
    pub view_version: models::ViewVersion,

    #[serde(rename = "properties")]
    pub properties: std::collections::HashMap<String, String>,

}


impl CreateViewRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, schema: models::Schema, view_version: models::ViewVersion, properties: std::collections::HashMap<String, String>, ) -> CreateViewRequest {
        CreateViewRequest {
            name,
            location: None,
            schema,
            view_version,
            properties,
        }
    }
}

/// Converts the CreateViewRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateViewRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            self.location.as_ref().map(|location| {
                [
                    "location".to_string(),
                    location.to_string(),
                ].join(",")
            }),

            // Skipping schema in query parameter serialization

            // Skipping view-version in query parameter serialization

            // Skipping properties in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateViewRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateViewRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub location: Vec<String>,
            pub schema: Vec<models::Schema>,
            pub view_version: Vec<models::ViewVersion>,
            pub properties: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateViewRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "location" => intermediate_rep.location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema" => intermediate_rep.schema.push(<models::Schema as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "view-version" => intermediate_rep.view_version.push(<models::ViewVersion as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "properties" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateViewRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateViewRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateViewRequest {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in CreateViewRequest".to_string())?,
            location: intermediate_rep.location.into_iter().next(),
            schema: intermediate_rep.schema.into_iter().next().ok_or_else(|| "schema missing in CreateViewRequest".to_string())?,
            view_version: intermediate_rep.view_version.into_iter().next().ok_or_else(|| "view-version missing in CreateViewRequest".to_string())?,
            properties: intermediate_rep.properties.into_iter().next().ok_or_else(|| "properties missing in CreateViewRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CreateViewRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateViewRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateViewRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateViewRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateViewRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateViewRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateViewRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataFile {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "column-sizes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub column_sizes: Option<models::CountMap>,

    #[serde(rename = "value-counts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value_counts: Option<models::CountMap>,

    #[serde(rename = "null-value-counts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub null_value_counts: Option<models::CountMap>,

    #[serde(rename = "nan-value-counts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nan_value_counts: Option<models::CountMap>,

    #[serde(rename = "lower-bounds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lower_bounds: Option<models::ValueMap>,

    #[serde(rename = "upper-bounds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub upper_bounds: Option<models::ValueMap>,

    #[serde(rename = "file-path")]
    pub file_path: String,

    #[serde(rename = "file-format")]
    pub file_format: models::FileFormat,

    #[serde(rename = "spec-id")]
    pub spec_id: i32,

    /// A list of partition field values ordered based on the fields of the partition spec specified by the `spec-id`
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition: Option<Vec<models::PrimitiveTypeValue>>,

    /// Total file size in bytes
    #[serde(rename = "file-size-in-bytes")]
    pub file_size_in_bytes: i64,

    /// Number of records in the file
    #[serde(rename = "record-count")]
    pub record_count: i64,

    /// Binary type values are stored and serialized as an uppercase hexadecimal string
    #[serde(rename = "key-metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_metadata: Option<String>,

    /// List of splittable offsets
    #[serde(rename = "split-offsets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub split_offsets: Option<Vec<i64>>,

    #[serde(rename = "sort-order-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_order_id: Option<i32>,

}


impl DataFile {
    #[allow(clippy::new_without_default)]
    pub fn new(content: String, file_path: String, file_format: models::FileFormat, spec_id: i32, file_size_in_bytes: i64, record_count: i64, ) -> DataFile {
        DataFile {
            content,
            column_sizes: None,
            value_counts: None,
            null_value_counts: None,
            nan_value_counts: None,
            lower_bounds: None,
            upper_bounds: None,
            file_path,
            file_format,
            spec_id,
            partition: None,
            file_size_in_bytes,
            record_count,
            key_metadata: None,
            split_offsets: None,
            sort_order_id: None,
        }
    }
}

/// Converts the DataFile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataFile {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("content".to_string()),
            Some(self.content.to_string()),

            // Skipping column-sizes in query parameter serialization

            // Skipping value-counts in query parameter serialization

            // Skipping null-value-counts in query parameter serialization

            // Skipping nan-value-counts in query parameter serialization

            // Skipping lower-bounds in query parameter serialization

            // Skipping upper-bounds in query parameter serialization


            Some("file-path".to_string()),
            Some(self.file_path.to_string()),

            // Skipping file-format in query parameter serialization


            Some("spec-id".to_string()),
            Some(self.spec_id.to_string()),

            // Skipping partition in query parameter serialization


            Some("file-size-in-bytes".to_string()),
            Some(self.file_size_in_bytes.to_string()),


            Some("record-count".to_string()),
            Some(self.record_count.to_string()),


            self.key_metadata.as_ref().map(|key_metadata| {
                [
                    "key-metadata".to_string(),
                    key_metadata.to_string(),
                ].join(",")
            }),


            self.split_offsets.as_ref().map(|split_offsets| {
                [
                    "split-offsets".to_string(),
                    split_offsets.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.sort_order_id.as_ref().map(|sort_order_id| {
                [
                    "sort-order-id".to_string(),
                    sort_order_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataFile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataFile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub content: Vec<String>,
            pub column_sizes: Vec<models::CountMap>,
            pub value_counts: Vec<models::CountMap>,
            pub null_value_counts: Vec<models::CountMap>,
            pub nan_value_counts: Vec<models::CountMap>,
            pub lower_bounds: Vec<models::ValueMap>,
            pub upper_bounds: Vec<models::ValueMap>,
            pub file_path: Vec<String>,
            pub file_format: Vec<models::FileFormat>,
            pub spec_id: Vec<i32>,
            pub partition: Vec<Vec<models::PrimitiveTypeValue>>,
            pub file_size_in_bytes: Vec<i64>,
            pub record_count: Vec<i64>,
            pub key_metadata: Vec<String>,
            pub split_offsets: Vec<Vec<i64>>,
            pub sort_order_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DataFile".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "content" => intermediate_rep.content.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "column-sizes" => intermediate_rep.column_sizes.push(<models::CountMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value-counts" => intermediate_rep.value_counts.push(<models::CountMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "null-value-counts" => intermediate_rep.null_value_counts.push(<models::CountMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nan-value-counts" => intermediate_rep.nan_value_counts.push(<models::CountMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lower-bounds" => intermediate_rep.lower_bounds.push(<models::ValueMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "upper-bounds" => intermediate_rep.upper_bounds.push(<models::ValueMap as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-path" => intermediate_rep.file_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-format" => intermediate_rep.file_format.push(<models::FileFormat as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spec-id" => intermediate_rep.spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "partition" => return std::result::Result::Err("Parsing a container in this style is not supported in DataFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "file-size-in-bytes" => intermediate_rep.file_size_in_bytes.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "record-count" => intermediate_rep.record_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key-metadata" => intermediate_rep.key_metadata.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "split-offsets" => return std::result::Result::Err("Parsing a container in this style is not supported in DataFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "sort-order-id" => intermediate_rep.sort_order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataFile".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataFile {
            content: intermediate_rep.content.into_iter().next().ok_or_else(|| "content missing in DataFile".to_string())?,
            column_sizes: intermediate_rep.column_sizes.into_iter().next(),
            value_counts: intermediate_rep.value_counts.into_iter().next(),
            null_value_counts: intermediate_rep.null_value_counts.into_iter().next(),
            nan_value_counts: intermediate_rep.nan_value_counts.into_iter().next(),
            lower_bounds: intermediate_rep.lower_bounds.into_iter().next(),
            upper_bounds: intermediate_rep.upper_bounds.into_iter().next(),
            file_path: intermediate_rep.file_path.into_iter().next().ok_or_else(|| "file-path missing in DataFile".to_string())?,
            file_format: intermediate_rep.file_format.into_iter().next().ok_or_else(|| "file-format missing in DataFile".to_string())?,
            spec_id: intermediate_rep.spec_id.into_iter().next().ok_or_else(|| "spec-id missing in DataFile".to_string())?,
            partition: intermediate_rep.partition.into_iter().next(),
            file_size_in_bytes: intermediate_rep.file_size_in_bytes.into_iter().next().ok_or_else(|| "file-size-in-bytes missing in DataFile".to_string())?,
            record_count: intermediate_rep.record_count.into_iter().next().ok_or_else(|| "record-count missing in DataFile".to_string())?,
            key_metadata: intermediate_rep.key_metadata.into_iter().next(),
            split_offsets: intermediate_rep.split_offsets.into_iter().next(),
            sort_order_id: intermediate_rep.sort_order_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataFile> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataFile>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DataFile>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DataFile - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DataFile> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DataFile as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DataFile - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Date type values follow the 'YYYY-MM-DD' ISO-8601 standard date format
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DateTypeValue(chrono::naive::NaiveDate);

impl std::convert::From<chrono::naive::NaiveDate> for DateTypeValue {
    fn from(x: chrono::naive::NaiveDate) -> Self {
        DateTypeValue(x)
    }
}

impl std::convert::From<DateTypeValue> for chrono::naive::NaiveDate {
    fn from(x: DateTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for DateTypeValue {
    type Target = chrono::naive::NaiveDate;
    fn deref(&self) -> &chrono::naive::NaiveDate {
        &self.0
    }
}

impl std::ops::DerefMut for DateTypeValue {
    fn deref_mut(&mut self) -> &mut chrono::naive::NaiveDate {
        &mut self.0
    }
}


/// Decimal type values are serialized as strings. Decimals with a positive scale serialize as numeric plain text, while decimals with a negative scale use scientific notation and the exponent will be equal to the negated scale. For instance, a decimal with a positive scale is '123.4500', with zero scale is '2', and with a negative scale is '2E+20'
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DecimalTypeValue(String);

impl std::convert::From<String> for DecimalTypeValue {
    fn from(x: String) -> Self {
        DecimalTypeValue(x)
    }
}

impl std::string::ToString for DecimalTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for DecimalTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(DecimalTypeValue(x.to_string()))
    }
}

impl std::convert::From<DecimalTypeValue> for String {
    fn from(x: DecimalTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for DecimalTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for DecimalTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DoubleTypeValue(f64);

impl std::convert::From<f64> for DoubleTypeValue {
    fn from(x: f64) -> Self {
        DoubleTypeValue(x)
    }
}

impl std::convert::From<DoubleTypeValue> for f64 {
    fn from(x: DoubleTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for DoubleTypeValue {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl std::ops::DerefMut for DoubleTypeValue {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EqualityDeleteFile {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "content")]
    pub content: String,

    /// List of equality field IDs
    #[serde(rename = "equality-ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub equality_ids: Option<Vec<i32>>,

    #[serde(rename = "file-path")]
    pub file_path: String,

    #[serde(rename = "file-format")]
    pub file_format: models::FileFormat,

    #[serde(rename = "spec-id")]
    pub spec_id: i32,

    /// A list of partition field values ordered based on the fields of the partition spec specified by the `spec-id`
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition: Option<Vec<models::PrimitiveTypeValue>>,

    /// Total file size in bytes
    #[serde(rename = "file-size-in-bytes")]
    pub file_size_in_bytes: i64,

    /// Number of records in the file
    #[serde(rename = "record-count")]
    pub record_count: i64,

    /// Binary type values are stored and serialized as an uppercase hexadecimal string
    #[serde(rename = "key-metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_metadata: Option<String>,

    /// List of splittable offsets
    #[serde(rename = "split-offsets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub split_offsets: Option<Vec<i64>>,

    #[serde(rename = "sort-order-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_order_id: Option<i32>,

}


impl EqualityDeleteFile {
    #[allow(clippy::new_without_default)]
    pub fn new(content: String, file_path: String, file_format: models::FileFormat, spec_id: i32, file_size_in_bytes: i64, record_count: i64, ) -> EqualityDeleteFile {
        EqualityDeleteFile {
            content,
            equality_ids: None,
            file_path,
            file_format,
            spec_id,
            partition: None,
            file_size_in_bytes,
            record_count,
            key_metadata: None,
            split_offsets: None,
            sort_order_id: None,
        }
    }
}

/// Converts the EqualityDeleteFile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EqualityDeleteFile {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("content".to_string()),
            Some(self.content.to_string()),


            self.equality_ids.as_ref().map(|equality_ids| {
                [
                    "equality-ids".to_string(),
                    equality_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            Some("file-path".to_string()),
            Some(self.file_path.to_string()),

            // Skipping file-format in query parameter serialization


            Some("spec-id".to_string()),
            Some(self.spec_id.to_string()),

            // Skipping partition in query parameter serialization


            Some("file-size-in-bytes".to_string()),
            Some(self.file_size_in_bytes.to_string()),


            Some("record-count".to_string()),
            Some(self.record_count.to_string()),


            self.key_metadata.as_ref().map(|key_metadata| {
                [
                    "key-metadata".to_string(),
                    key_metadata.to_string(),
                ].join(",")
            }),


            self.split_offsets.as_ref().map(|split_offsets| {
                [
                    "split-offsets".to_string(),
                    split_offsets.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.sort_order_id.as_ref().map(|sort_order_id| {
                [
                    "sort-order-id".to_string(),
                    sort_order_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EqualityDeleteFile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EqualityDeleteFile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub content: Vec<String>,
            pub equality_ids: Vec<Vec<i32>>,
            pub file_path: Vec<String>,
            pub file_format: Vec<models::FileFormat>,
            pub spec_id: Vec<i32>,
            pub partition: Vec<Vec<models::PrimitiveTypeValue>>,
            pub file_size_in_bytes: Vec<i64>,
            pub record_count: Vec<i64>,
            pub key_metadata: Vec<String>,
            pub split_offsets: Vec<Vec<i64>>,
            pub sort_order_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing EqualityDeleteFile".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "content" => intermediate_rep.content.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "equality-ids" => return std::result::Result::Err("Parsing a container in this style is not supported in EqualityDeleteFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "file-path" => intermediate_rep.file_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-format" => intermediate_rep.file_format.push(<models::FileFormat as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spec-id" => intermediate_rep.spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "partition" => return std::result::Result::Err("Parsing a container in this style is not supported in EqualityDeleteFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "file-size-in-bytes" => intermediate_rep.file_size_in_bytes.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "record-count" => intermediate_rep.record_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key-metadata" => intermediate_rep.key_metadata.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "split-offsets" => return std::result::Result::Err("Parsing a container in this style is not supported in EqualityDeleteFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "sort-order-id" => intermediate_rep.sort_order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing EqualityDeleteFile".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EqualityDeleteFile {
            content: intermediate_rep.content.into_iter().next().ok_or_else(|| "content missing in EqualityDeleteFile".to_string())?,
            equality_ids: intermediate_rep.equality_ids.into_iter().next(),
            file_path: intermediate_rep.file_path.into_iter().next().ok_or_else(|| "file-path missing in EqualityDeleteFile".to_string())?,
            file_format: intermediate_rep.file_format.into_iter().next().ok_or_else(|| "file-format missing in EqualityDeleteFile".to_string())?,
            spec_id: intermediate_rep.spec_id.into_iter().next().ok_or_else(|| "spec-id missing in EqualityDeleteFile".to_string())?,
            partition: intermediate_rep.partition.into_iter().next(),
            file_size_in_bytes: intermediate_rep.file_size_in_bytes.into_iter().next().ok_or_else(|| "file-size-in-bytes missing in EqualityDeleteFile".to_string())?,
            record_count: intermediate_rep.record_count.into_iter().next().ok_or_else(|| "record-count missing in EqualityDeleteFile".to_string())?,
            key_metadata: intermediate_rep.key_metadata.into_iter().next(),
            split_offsets: intermediate_rep.split_offsets.into_iter().next(),
            sort_order_id: intermediate_rep.sort_order_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EqualityDeleteFile> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EqualityDeleteFile>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EqualityDeleteFile>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for EqualityDeleteFile - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<EqualityDeleteFile> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EqualityDeleteFile as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into EqualityDeleteFile - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// JSON error payload returned in a response with further details on the error
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorModel {
    /// Human-readable error message
    #[serde(rename = "message")]
    pub message: String,

    /// Internal type definition of the error
    #[serde(rename = "type")]
    pub r#type: String,

    /// HTTP response code
    #[serde(rename = "code")]
    #[validate(
            range(min = 400, max = 600),
        )]
    pub code: u16,

    #[serde(rename = "stack")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stack: Option<Vec<String>>,

}


impl ErrorModel {
    #[allow(clippy::new_without_default)]
    pub fn new(message: String, r#type: String, code: u16, ) -> ErrorModel {
        ErrorModel {
            message,
            r#type,
            code,
            stack: None,
        }
    }
}

/// Converts the ErrorModel value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ErrorModel {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("message".to_string()),
            Some(self.message.to_string()),


            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("code".to_string()),
            Some(self.code.to_string()),


            self.stack.as_ref().map(|stack| {
                [
                    "stack".to_string(),
                    stack.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorModel value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorModel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
            pub r#type: Vec<String>,
            pub code: Vec<u16>,
            pub stack: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ErrorModel".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(<u16 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "stack" => return std::result::Result::Err("Parsing a container in this style is not supported in ErrorModel".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ErrorModel".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorModel {
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in ErrorModel".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ErrorModel".to_string())?,
            code: intermediate_rep.code.into_iter().next().ok_or_else(|| "code missing in ErrorModel".to_string())?,
            stack: intermediate_rep.stack.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorModel> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorModel>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ErrorModel>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ErrorModel - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ErrorModel> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ErrorModel as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ErrorModel - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Expression {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "left")]
    pub left: models::Expression,

    #[serde(rename = "right")]
    pub right: models::Expression,

    #[serde(rename = "child")]
    pub child: models::Expression,

    #[serde(rename = "term")]
    pub term: models::Term,

    #[serde(rename = "values")]
    pub values: Vec<serde_json::Value>,

    #[serde(rename = "value")]
    pub value: serde_json::Value,

}


impl Expression {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, left: models::Expression, right: models::Expression, child: models::Expression, term: models::Term, values: Vec<serde_json::Value>, value: serde_json::Value, ) -> Expression {
        Expression {
            r#type,
            left,
            right,
            child,
            term,
            values,
            value,
        }
    }
}

/// Converts the Expression value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Expression {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping left in query parameter serialization

            // Skipping right in query parameter serialization

            // Skipping child in query parameter serialization

            // Skipping term in query parameter serialization

            // Skipping values in query parameter serialization

            // Skipping value in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Expression value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Expression {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub left: Vec<models::Expression>,
            pub right: Vec<models::Expression>,
            pub child: Vec<models::Expression>,
            pub term: Vec<models::Term>,
            pub values: Vec<Vec<serde_json::Value>>,
            pub value: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Expression".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "left" => intermediate_rep.left.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "right" => intermediate_rep.right.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "child" => intermediate_rep.child.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "term" => intermediate_rep.term.push(<models::Term as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in Expression".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Expression".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Expression {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Expression".to_string())?,
            left: intermediate_rep.left.into_iter().next().ok_or_else(|| "left missing in Expression".to_string())?,
            right: intermediate_rep.right.into_iter().next().ok_or_else(|| "right missing in Expression".to_string())?,
            child: intermediate_rep.child.into_iter().next().ok_or_else(|| "child missing in Expression".to_string())?,
            term: intermediate_rep.term.into_iter().next().ok_or_else(|| "term missing in Expression".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in Expression".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in Expression".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Expression> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Expression>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Expression>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Expression - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Expression> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Expression as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Expression - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExpressionType(String);

impl std::convert::From<String> for ExpressionType {
    fn from(x: String) -> Self {
        ExpressionType(x)
    }
}

impl std::string::ToString for ExpressionType {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for ExpressionType {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(ExpressionType(x.to_string()))
    }
}

impl std::convert::From<ExpressionType> for String {
    fn from(x: ExpressionType) -> Self {
        x.0
    }
}

impl std::ops::Deref for ExpressionType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for ExpressionType {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum FileFormat {
    #[serde(rename = "avro")]
    Avro,
    #[serde(rename = "orc")]
    Orc,
    #[serde(rename = "parquet")]
    Parquet,
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileFormat::Avro => write!(f, "avro"),
            FileFormat::Orc => write!(f, "orc"),
            FileFormat::Parquet => write!(f, "parquet"),
        }
    }
}

impl std::str::FromStr for FileFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "avro" => std::result::Result::Ok(FileFormat::Avro),
            "orc" => std::result::Result::Ok(FileFormat::Orc),
            "parquet" => std::result::Result::Ok(FileFormat::Parquet),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Fixed length type values are stored and serialized as an uppercase hexadecimal string preserving the fixed length
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FixedTypeValue(String);

impl std::convert::From<String> for FixedTypeValue {
    fn from(x: String) -> Self {
        FixedTypeValue(x)
    }
}

impl std::string::ToString for FixedTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for FixedTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(FixedTypeValue(x.to_string()))
    }
}

impl std::convert::From<FixedTypeValue> for String {
    fn from(x: FixedTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for FixedTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for FixedTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FloatTypeValue(f32);

impl std::convert::From<f32> for FloatTypeValue {
    fn from(x: f32) -> Self {
        FloatTypeValue(x)
    }
}

impl std::convert::From<FloatTypeValue> for f32 {
    fn from(x: FloatTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for FloatTypeValue {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl std::ops::DerefMut for FloatTypeValue {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetNamespaceResponse {
    /// Reference to one or more levels of a namespace
    #[serde(rename = "namespace")]
    pub namespace: Vec<String>,

    /// Properties stored on the namespace, if supported by the server. If the server does not support namespace properties, it should return null for this field. If namespace properties are supported, but none are set, it should return an empty object.
    #[serde(rename = "properties")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<swagger::Nullable<std::collections::HashMap<String, String>>>,

}


impl GetNamespaceResponse {
    #[allow(clippy::new_without_default)]
    pub fn new(namespace: Vec<String>, ) -> GetNamespaceResponse {
        GetNamespaceResponse {
            namespace,
            properties: None,
        }
    }
}

/// Converts the GetNamespaceResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GetNamespaceResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("namespace".to_string()),
            Some(self.namespace.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping properties in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GetNamespaceResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GetNamespaceResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub namespace: Vec<Vec<String>>,
            pub properties: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GetNamespaceResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "namespace" => return std::result::Result::Err("Parsing a container in this style is not supported in GetNamespaceResponse".to_string()),
                    "properties" => return std::result::Result::Err("Parsing a container in this style is not supported in GetNamespaceResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing GetNamespaceResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GetNamespaceResponse {
            namespace: intermediate_rep.namespace.into_iter().next().ok_or_else(|| "namespace missing in GetNamespaceResponse".to_string())?,
            properties: std::result::Result::Err("Nullable types not supported in GetNamespaceResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GetNamespaceResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GetNamespaceResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GetNamespaceResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GetNamespaceResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GetNamespaceResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GetNamespaceResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GetNamespaceResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// JSON wrapper for all error responses (non-2xx)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IcebergErrorResponse {
    #[serde(rename = "error")]
    pub error: models::ErrorModel,

}


impl IcebergErrorResponse {
    #[allow(clippy::new_without_default)]
    pub fn new(error: models::ErrorModel, ) -> IcebergErrorResponse {
        IcebergErrorResponse {
            error,
        }
    }
}

/// Converts the IcebergErrorResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for IcebergErrorResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping error in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a IcebergErrorResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for IcebergErrorResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub error: Vec<models::ErrorModel>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing IcebergErrorResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "error" => intermediate_rep.error.push(<models::ErrorModel as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing IcebergErrorResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(IcebergErrorResponse {
            error: intermediate_rep.error.into_iter().next().ok_or_else(|| "error missing in IcebergErrorResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<IcebergErrorResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<IcebergErrorResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<IcebergErrorResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for IcebergErrorResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<IcebergErrorResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <IcebergErrorResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into IcebergErrorResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IntegerTypeValue(i32);

impl std::convert::From<i32> for IntegerTypeValue {
    fn from(x: i32) -> Self {
        IntegerTypeValue(x)
    }
}

impl std::convert::From<IntegerTypeValue> for i32 {
    fn from(x: IntegerTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for IntegerTypeValue {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for IntegerTypeValue {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListNamespacesResponse {
    /// An opaque token that allows clients to make use of pagination for list APIs (e.g. ListTables). Clients may initiate the first paginated request by sending an empty query parameter `pageToken` to the server. Servers that support pagination should identify the `pageToken` parameter and return a `next-page-token` in the response if there are more results available.  After the initial request, the value of `next-page-token` from each response must be used as the `pageToken` parameter value for the next request. The server must return `null` value for the `next-page-token` in the last response. Servers that support pagination must return all results in a single response with the value of `next-page-token` set to `null` if the query parameter `pageToken` is not set in the request. Servers that do not support pagination should ignore the `pageToken` parameter and return all results in a single response. The `next-page-token` must be omitted from the response. Clients must interpret either `null` or missing response value of `next-page-token` as the end of the listing results.
    #[serde(rename = "next-page-token")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<swagger::Nullable<String>>,

    #[serde(rename = "namespaces")]
    #[validate(
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub namespaces: Option<Vec<models::Namespace>>,

}


impl ListNamespacesResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ListNamespacesResponse {
        ListNamespacesResponse {
            next_page_token: None,
            namespaces: None,
        }
    }
}

/// Converts the ListNamespacesResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ListNamespacesResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.next_page_token.as_ref().map(|next_page_token| {
                [
                    "next-page-token".to_string(),
                    next_page_token.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),

            // Skipping namespaces in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListNamespacesResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListNamespacesResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub next_page_token: Vec<String>,
            pub namespaces: Vec<Vec<models::Namespace>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListNamespacesResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "next-page-token" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ListNamespacesResponse".to_string()),
                    "namespaces" => return std::result::Result::Err("Parsing a container in this style is not supported in ListNamespacesResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListNamespacesResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListNamespacesResponse {
            next_page_token: std::result::Result::Err("Nullable types not supported in ListNamespacesResponse".to_string())?,
            namespaces: intermediate_rep.namespaces.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListNamespacesResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ListNamespacesResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ListNamespacesResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ListNamespacesResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ListNamespacesResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListNamespacesResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ListNamespacesResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListTablesResponse {
    /// An opaque token that allows clients to make use of pagination for list APIs (e.g. ListTables). Clients may initiate the first paginated request by sending an empty query parameter `pageToken` to the server. Servers that support pagination should identify the `pageToken` parameter and return a `next-page-token` in the response if there are more results available.  After the initial request, the value of `next-page-token` from each response must be used as the `pageToken` parameter value for the next request. The server must return `null` value for the `next-page-token` in the last response. Servers that support pagination must return all results in a single response with the value of `next-page-token` set to `null` if the query parameter `pageToken` is not set in the request. Servers that do not support pagination should ignore the `pageToken` parameter and return all results in a single response. The `next-page-token` must be omitted from the response. Clients must interpret either `null` or missing response value of `next-page-token` as the end of the listing results.
    #[serde(rename = "next-page-token")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_page_token: Option<swagger::Nullable<String>>,

    #[serde(rename = "identifiers")]
    #[validate(
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identifiers: Option<Vec<models::TableIdentifier>>,

}


impl ListTablesResponse {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ListTablesResponse {
        ListTablesResponse {
            next_page_token: None,
            identifiers: None,
        }
    }
}

/// Converts the ListTablesResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ListTablesResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.next_page_token.as_ref().map(|next_page_token| {
                [
                    "next-page-token".to_string(),
                    next_page_token.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),

            // Skipping identifiers in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListTablesResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListTablesResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub next_page_token: Vec<String>,
            pub identifiers: Vec<Vec<models::TableIdentifier>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListTablesResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "next-page-token" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ListTablesResponse".to_string()),
                    "identifiers" => return std::result::Result::Err("Parsing a container in this style is not supported in ListTablesResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListTablesResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListTablesResponse {
            next_page_token: std::result::Result::Err("Nullable types not supported in ListTablesResponse".to_string())?,
            identifiers: intermediate_rep.identifiers.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListTablesResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ListTablesResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ListTablesResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ListTablesResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ListTablesResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListTablesResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ListTablesResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListType {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "element-id")]
    pub element_id: i32,

    #[serde(rename = "element")]
    pub element: models::Type,

    #[serde(rename = "element-required")]
    pub element_required: bool,

}


impl ListType {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, element_id: i32, element: models::Type, element_required: bool, ) -> ListType {
        ListType {
            r#type,
            element_id,
            element,
            element_required,
        }
    }
}

/// Converts the ListType value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ListType {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("element-id".to_string()),
            Some(self.element_id.to_string()),

            // Skipping element in query parameter serialization


            Some("element-required".to_string()),
            Some(self.element_required.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ListType value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ListType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub element_id: Vec<i32>,
            pub element: Vec<models::Type>,
            pub element_required: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ListType".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "element-id" => intermediate_rep.element_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "element" => intermediate_rep.element.push(<models::Type as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "element-required" => intermediate_rep.element_required.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ListType".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ListType {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ListType".to_string())?,
            element_id: intermediate_rep.element_id.into_iter().next().ok_or_else(|| "element-id missing in ListType".to_string())?,
            element: intermediate_rep.element.into_iter().next().ok_or_else(|| "element missing in ListType".to_string())?,
            element_required: intermediate_rep.element_required.into_iter().next().ok_or_else(|| "element-required missing in ListType".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ListType> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ListType>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ListType>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ListType - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ListType> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ListType as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ListType - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LiteralExpression {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "term")]
    pub term: models::Term,

    #[serde(rename = "value")]
    pub value: serde_json::Value,

}


impl LiteralExpression {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, term: models::Term, value: serde_json::Value, ) -> LiteralExpression {
        LiteralExpression {
            r#type,
            term,
            value,
        }
    }
}

/// Converts the LiteralExpression value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LiteralExpression {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping term in query parameter serialization

            // Skipping value in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LiteralExpression value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LiteralExpression {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub term: Vec<models::Term>,
            pub value: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LiteralExpression".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "term" => intermediate_rep.term.push(<models::Term as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LiteralExpression".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LiteralExpression {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in LiteralExpression".to_string())?,
            term: intermediate_rep.term.into_iter().next().ok_or_else(|| "term missing in LiteralExpression".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in LiteralExpression".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LiteralExpression> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LiteralExpression>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LiteralExpression>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LiteralExpression - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LiteralExpression> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LiteralExpression as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LiteralExpression - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Result used when a table is successfully loaded.   The table metadata JSON is returned in the `metadata` field. The corresponding file location of table metadata should be returned in the `metadata-location` field, unless the metadata is not yet committed. For example, a create transaction may return metadata that is staged but not committed. Clients can check whether metadata has changed by comparing metadata locations after the table has been created.   The `config` map returns table-specific configuration for the table's resources, including its HTTP client and FileIO. For example, config may contain a specific FileIO implementation class for the table depending on its underlying storage.   The following configurations should be respected by clients:  ## General Configurations  - `token`: Authorization bearer token to use for table requests if OAuth2 security is enabled   ## AWS Configurations  The following configurations should be respected when working with tables stored in AWS S3  - `client.region`: region to configure client for making requests to AWS  - `s3.access-key-id`: id for for credentials that provide access to the data in S3  - `s3.secret-access-key`: secret for credentials that provide access to data in S3   - `s3.session-token`: if present, this value should be used for as the session token   - `s3.remote-signing-enabled`: if `true` remote signing should be performed as described in the `s3-signer-open-api.yaml` specification 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LoadTableResult {
    /// May be null if the table is staged as part of a transaction
    #[serde(rename = "metadata-location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata_location: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: models::TableMetadata,

    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,

}


impl LoadTableResult {
    #[allow(clippy::new_without_default)]
    pub fn new(metadata: models::TableMetadata, ) -> LoadTableResult {
        LoadTableResult {
            metadata_location: None,
            metadata,
            config: None,
        }
    }
}

/// Converts the LoadTableResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LoadTableResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.metadata_location.as_ref().map(|metadata_location| {
                [
                    "metadata-location".to_string(),
                    metadata_location.to_string(),
                ].join(",")
            }),

            // Skipping metadata in query parameter serialization

            // Skipping config in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LoadTableResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LoadTableResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub metadata_location: Vec<String>,
            pub metadata: Vec<models::TableMetadata>,
            pub config: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LoadTableResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "metadata-location" => intermediate_rep.metadata_location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "metadata" => intermediate_rep.metadata.push(<models::TableMetadata as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "config" => return std::result::Result::Err("Parsing a container in this style is not supported in LoadTableResult".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing LoadTableResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LoadTableResult {
            metadata_location: intermediate_rep.metadata_location.into_iter().next(),
            metadata: intermediate_rep.metadata.into_iter().next().ok_or_else(|| "metadata missing in LoadTableResult".to_string())?,
            config: intermediate_rep.config.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LoadTableResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LoadTableResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LoadTableResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LoadTableResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LoadTableResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LoadTableResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LoadTableResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Result used when a view is successfully loaded.   The view metadata JSON is returned in the `metadata` field. The corresponding file location of view metadata is returned in the `metadata-location` field. Clients can check whether metadata has changed by comparing metadata locations after the view has been created.  The `config` map returns view-specific configuration for the view's resources.  The following configurations should be respected by clients:  ## General Configurations  - `token`: Authorization bearer token to use for view requests if OAuth2 security is enabled 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LoadViewResult {
    #[serde(rename = "metadata-location")]
    pub metadata_location: String,

    #[serde(rename = "metadata")]
    pub metadata: models::ViewMetadata,

    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,

}


impl LoadViewResult {
    #[allow(clippy::new_without_default)]
    pub fn new(metadata_location: String, metadata: models::ViewMetadata, ) -> LoadViewResult {
        LoadViewResult {
            metadata_location,
            metadata,
            config: None,
        }
    }
}

/// Converts the LoadViewResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LoadViewResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("metadata-location".to_string()),
            Some(self.metadata_location.to_string()),

            // Skipping metadata in query parameter serialization

            // Skipping config in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LoadViewResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LoadViewResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub metadata_location: Vec<String>,
            pub metadata: Vec<models::ViewMetadata>,
            pub config: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LoadViewResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "metadata-location" => intermediate_rep.metadata_location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "metadata" => intermediate_rep.metadata.push(<models::ViewMetadata as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "config" => return std::result::Result::Err("Parsing a container in this style is not supported in LoadViewResult".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing LoadViewResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LoadViewResult {
            metadata_location: intermediate_rep.metadata_location.into_iter().next().ok_or_else(|| "metadata-location missing in LoadViewResult".to_string())?,
            metadata: intermediate_rep.metadata.into_iter().next().ok_or_else(|| "metadata missing in LoadViewResult".to_string())?,
            config: intermediate_rep.config.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LoadViewResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LoadViewResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LoadViewResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LoadViewResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LoadViewResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LoadViewResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LoadViewResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LongTypeValue(i64);

impl std::convert::From<i64> for LongTypeValue {
    fn from(x: i64) -> Self {
        LongTypeValue(x)
    }
}

impl std::convert::From<LongTypeValue> for i64 {
    fn from(x: LongTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for LongTypeValue {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl std::ops::DerefMut for LongTypeValue {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MapType {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "key-id")]
    pub key_id: i32,

    #[serde(rename = "key")]
    pub key: models::Type,

    #[serde(rename = "value-id")]
    pub value_id: i32,

    #[serde(rename = "value")]
    pub value: models::Type,

    #[serde(rename = "value-required")]
    pub value_required: bool,

}


impl MapType {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, key_id: i32, key: models::Type, value_id: i32, value: models::Type, value_required: bool, ) -> MapType {
        MapType {
            r#type,
            key_id,
            key,
            value_id,
            value,
            value_required,
        }
    }
}

/// Converts the MapType value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MapType {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("key-id".to_string()),
            Some(self.key_id.to_string()),

            // Skipping key in query parameter serialization


            Some("value-id".to_string()),
            Some(self.value_id.to_string()),

            // Skipping value in query parameter serialization


            Some("value-required".to_string()),
            Some(self.value_required.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MapType value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MapType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub key_id: Vec<i32>,
            pub key: Vec<models::Type>,
            pub value_id: Vec<i32>,
            pub value: Vec<models::Type>,
            pub value_required: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MapType".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key-id" => intermediate_rep.key_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<models::Type as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value-id" => intermediate_rep.value_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<models::Type as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value-required" => intermediate_rep.value_required.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MapType".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MapType {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in MapType".to_string())?,
            key_id: intermediate_rep.key_id.into_iter().next().ok_or_else(|| "key-id missing in MapType".to_string())?,
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in MapType".to_string())?,
            value_id: intermediate_rep.value_id.into_iter().next().ok_or_else(|| "value-id missing in MapType".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in MapType".to_string())?,
            value_required: intermediate_rep.value_required.into_iter().next().ok_or_else(|| "value-required missing in MapType".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MapType> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MapType>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MapType>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MapType - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MapType> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MapType as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MapType - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MetadataLogInner {
    #[serde(rename = "metadata-file")]
    pub metadata_file: String,

    #[serde(rename = "timestamp-ms")]
    pub timestamp_ms: i64,

}


impl MetadataLogInner {
    #[allow(clippy::new_without_default)]
    pub fn new(metadata_file: String, timestamp_ms: i64, ) -> MetadataLogInner {
        MetadataLogInner {
            metadata_file,
            timestamp_ms,
        }
    }
}

/// Converts the MetadataLogInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MetadataLogInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("metadata-file".to_string()),
            Some(self.metadata_file.to_string()),


            Some("timestamp-ms".to_string()),
            Some(self.timestamp_ms.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MetadataLogInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MetadataLogInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub metadata_file: Vec<String>,
            pub timestamp_ms: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MetadataLogInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "metadata-file" => intermediate_rep.metadata_file.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp-ms" => intermediate_rep.timestamp_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MetadataLogInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MetadataLogInner {
            metadata_file: intermediate_rep.metadata_file.into_iter().next().ok_or_else(|| "metadata-file missing in MetadataLogInner".to_string())?,
            timestamp_ms: intermediate_rep.timestamp_ms.into_iter().next().ok_or_else(|| "timestamp-ms missing in MetadataLogInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MetadataLogInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MetadataLogInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MetadataLogInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MetadataLogInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MetadataLogInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MetadataLogInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MetadataLogInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MetricResult {
    #[serde(rename = "unit")]
    pub unit: String,

    #[serde(rename = "value")]
    pub value: i64,

    #[serde(rename = "time-unit")]
    pub time_unit: String,

    #[serde(rename = "count")]
    pub count: i64,

    #[serde(rename = "total-duration")]
    pub total_duration: i64,

}


impl MetricResult {
    #[allow(clippy::new_without_default)]
    pub fn new(unit: String, value: i64, time_unit: String, count: i64, total_duration: i64, ) -> MetricResult {
        MetricResult {
            unit,
            value,
            time_unit,
            count,
            total_duration,
        }
    }
}

/// Converts the MetricResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MetricResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("unit".to_string()),
            Some(self.unit.to_string()),


            Some("value".to_string()),
            Some(self.value.to_string()),


            Some("time-unit".to_string()),
            Some(self.time_unit.to_string()),


            Some("count".to_string()),
            Some(self.count.to_string()),


            Some("total-duration".to_string()),
            Some(self.total_duration.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MetricResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MetricResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub unit: Vec<String>,
            pub value: Vec<i64>,
            pub time_unit: Vec<String>,
            pub count: Vec<i64>,
            pub total_duration: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MetricResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "unit" => intermediate_rep.unit.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "time-unit" => intermediate_rep.time_unit.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "total-duration" => intermediate_rep.total_duration.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MetricResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MetricResult {
            unit: intermediate_rep.unit.into_iter().next().ok_or_else(|| "unit missing in MetricResult".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in MetricResult".to_string())?,
            time_unit: intermediate_rep.time_unit.into_iter().next().ok_or_else(|| "time-unit missing in MetricResult".to_string())?,
            count: intermediate_rep.count.into_iter().next().ok_or_else(|| "count missing in MetricResult".to_string())?,
            total_duration: intermediate_rep.total_duration.into_iter().next().ok_or_else(|| "total-duration missing in MetricResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MetricResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MetricResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MetricResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MetricResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MetricResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MetricResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MetricResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NotExpression {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "child")]
    pub child: models::Expression,

}


impl NotExpression {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, child: models::Expression, ) -> NotExpression {
        NotExpression {
            r#type,
            child,
        }
    }
}

/// Converts the NotExpression value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for NotExpression {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping child in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NotExpression value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NotExpression {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub child: Vec<models::Expression>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NotExpression".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "child" => intermediate_rep.child.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NotExpression".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NotExpression {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in NotExpression".to_string())?,
            child: intermediate_rep.child.into_iter().next().ok_or_else(|| "child missing in NotExpression".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NotExpression> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<NotExpression>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NotExpression>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NotExpression - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<NotExpression> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NotExpression as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NotExpression - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum NullOrder {
    #[serde(rename = "nulls-first")]
    First,
    #[serde(rename = "nulls-last")]
    Last,
}

impl std::fmt::Display for NullOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NullOrder::First => write!(f, "nulls-first"),
            NullOrder::Last => write!(f, "nulls-last"),
        }
    }
}

impl std::str::FromStr for NullOrder {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "nulls-first" => std::result::Result::Ok(NullOrder::First),
            "nulls-last" => std::result::Result::Ok(NullOrder::Last),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OAuthError {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "error")]
    pub error: String,

    #[serde(rename = "error_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_description: Option<String>,

    #[serde(rename = "error_uri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_uri: Option<String>,

}


impl OAuthError {
    #[allow(clippy::new_without_default)]
    pub fn new(error: String, ) -> OAuthError {
        OAuthError {
            error,
            error_description: None,
            error_uri: None,
        }
    }
}

/// Converts the OAuthError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OAuthError {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("error".to_string()),
            Some(self.error.to_string()),


            self.error_description.as_ref().map(|error_description| {
                [
                    "error_description".to_string(),
                    error_description.to_string(),
                ].join(",")
            }),


            self.error_uri.as_ref().map(|error_uri| {
                [
                    "error_uri".to_string(),
                    error_uri.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OAuthError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OAuthError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub error: Vec<String>,
            pub error_description: Vec<String>,
            pub error_uri: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OAuthError".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "error" => intermediate_rep.error.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "error_description" => intermediate_rep.error_description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "error_uri" => intermediate_rep.error_uri.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing OAuthError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OAuthError {
            error: intermediate_rep.error.into_iter().next().ok_or_else(|| "error missing in OAuthError".to_string())?,
            error_description: intermediate_rep.error_description.into_iter().next(),
            error_uri: intermediate_rep.error_uri.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OAuthError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OAuthError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OAuthError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OAuthError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OAuthError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OAuthError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OAuthError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OAuthTokenResponse {
    /// The access token, for client credentials or token exchange
    #[serde(rename = "access_token")]
    pub access_token: String,

    /// Access token type for client credentials or token exchange  See https://datatracker.ietf.org/doc/html/rfc6749#section-7.1
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "token_type")]
    pub token_type: String,

    /// Lifetime of the access token in seconds for client credentials or token exchange
    #[serde(rename = "expires_in")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_in: Option<i32>,

    #[serde(rename = "issued_token_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issued_token_type: Option<models::TokenType>,

    /// Refresh token for client credentials or token exchange
    #[serde(rename = "refresh_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_token: Option<String>,

    /// Authorization scope for client credentials or token exchange
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scope: Option<String>,

}


impl OAuthTokenResponse {
    #[allow(clippy::new_without_default)]
    pub fn new(access_token: String, token_type: String, ) -> OAuthTokenResponse {
        OAuthTokenResponse {
            access_token,
            token_type,
            expires_in: None,
            issued_token_type: None,
            refresh_token: None,
            scope: None,
        }
    }
}

/// Converts the OAuthTokenResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OAuthTokenResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("access_token".to_string()),
            Some(self.access_token.to_string()),


            Some("token_type".to_string()),
            Some(self.token_type.to_string()),


            self.expires_in.as_ref().map(|expires_in| {
                [
                    "expires_in".to_string(),
                    expires_in.to_string(),
                ].join(",")
            }),

            // Skipping issued_token_type in query parameter serialization


            self.refresh_token.as_ref().map(|refresh_token| {
                [
                    "refresh_token".to_string(),
                    refresh_token.to_string(),
                ].join(",")
            }),


            self.scope.as_ref().map(|scope| {
                [
                    "scope".to_string(),
                    scope.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OAuthTokenResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OAuthTokenResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub access_token: Vec<String>,
            pub token_type: Vec<String>,
            pub expires_in: Vec<i32>,
            pub issued_token_type: Vec<models::TokenType>,
            pub refresh_token: Vec<String>,
            pub scope: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OAuthTokenResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "access_token" => intermediate_rep.access_token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "token_type" => intermediate_rep.token_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "expires_in" => intermediate_rep.expires_in.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "issued_token_type" => intermediate_rep.issued_token_type.push(<models::TokenType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "refresh_token" => intermediate_rep.refresh_token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "scope" => intermediate_rep.scope.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing OAuthTokenResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OAuthTokenResponse {
            access_token: intermediate_rep.access_token.into_iter().next().ok_or_else(|| "access_token missing in OAuthTokenResponse".to_string())?,
            token_type: intermediate_rep.token_type.into_iter().next().ok_or_else(|| "token_type missing in OAuthTokenResponse".to_string())?,
            expires_in: intermediate_rep.expires_in.into_iter().next(),
            issued_token_type: intermediate_rep.issued_token_type.into_iter().next(),
            refresh_token: intermediate_rep.refresh_token.into_iter().next(),
            scope: intermediate_rep.scope.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OAuthTokenResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OAuthTokenResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OAuthTokenResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OAuthTokenResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OAuthTokenResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OAuthTokenResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OAuthTokenResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// An opaque token that allows clients to make use of pagination for list APIs (e.g. ListTables). Clients may initiate the first paginated request by sending an empty query parameter `pageToken` to the server. Servers that support pagination should identify the `pageToken` parameter and return a `next-page-token` in the response if there are more results available.  After the initial request, the value of `next-page-token` from each response must be used as the `pageToken` parameter value for the next request. The server must return `null` value for the `next-page-token` in the last response. Servers that support pagination must return all results in a single response with the value of `next-page-token` set to `null` if the query parameter `pageToken` is not set in the request. Servers that do not support pagination should ignore the `pageToken` parameter and return all results in a single response. The `next-page-token` must be omitted from the response. Clients must interpret either `null` or missing response value of `next-page-token` as the end of the listing results.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PageToken(String);

impl std::convert::From<String> for PageToken {
    fn from(x: String) -> Self {
        PageToken(x)
    }
}

impl std::string::ToString for PageToken {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for PageToken {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(PageToken(x.to_string()))
    }
}

impl std::convert::From<PageToken> for String {
    fn from(x: PageToken) -> Self {
        x.0
    }
}

impl std::ops::Deref for PageToken {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for PageToken {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PartitionField {
    #[serde(rename = "field-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field_id: Option<i32>,

    #[serde(rename = "source-id")]
    pub source_id: i32,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "transform")]
    pub transform: String,

}


impl PartitionField {
    #[allow(clippy::new_without_default)]
    pub fn new(source_id: i32, name: String, transform: String, ) -> PartitionField {
        PartitionField {
            field_id: None,
            source_id,
            name,
            transform,
        }
    }
}

/// Converts the PartitionField value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PartitionField {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.field_id.as_ref().map(|field_id| {
                [
                    "field-id".to_string(),
                    field_id.to_string(),
                ].join(",")
            }),


            Some("source-id".to_string()),
            Some(self.source_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("transform".to_string()),
            Some(self.transform.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PartitionField value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PartitionField {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub field_id: Vec<i32>,
            pub source_id: Vec<i32>,
            pub name: Vec<String>,
            pub transform: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PartitionField".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "field-id" => intermediate_rep.field_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "source-id" => intermediate_rep.source_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "transform" => intermediate_rep.transform.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PartitionField".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PartitionField {
            field_id: intermediate_rep.field_id.into_iter().next(),
            source_id: intermediate_rep.source_id.into_iter().next().ok_or_else(|| "source-id missing in PartitionField".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in PartitionField".to_string())?,
            transform: intermediate_rep.transform.into_iter().next().ok_or_else(|| "transform missing in PartitionField".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PartitionField> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PartitionField>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PartitionField>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PartitionField - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PartitionField> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PartitionField as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PartitionField - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PartitionSpec {
    #[serde(rename = "spec-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spec_id: Option<i32>,

    #[serde(rename = "fields")]
    pub fields: Vec<models::PartitionField>,

}


impl PartitionSpec {
    #[allow(clippy::new_without_default)]
    pub fn new(fields: Vec<models::PartitionField>, ) -> PartitionSpec {
        PartitionSpec {
            spec_id: None,
            fields,
        }
    }
}

/// Converts the PartitionSpec value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PartitionSpec {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.spec_id.as_ref().map(|spec_id| {
                [
                    "spec-id".to_string(),
                    spec_id.to_string(),
                ].join(",")
            }),

            // Skipping fields in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PartitionSpec value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PartitionSpec {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub spec_id: Vec<i32>,
            pub fields: Vec<Vec<models::PartitionField>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PartitionSpec".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "spec-id" => intermediate_rep.spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in PartitionSpec".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PartitionSpec".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PartitionSpec {
            spec_id: intermediate_rep.spec_id.into_iter().next(),
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in PartitionSpec".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PartitionSpec> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PartitionSpec>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PartitionSpec>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PartitionSpec - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PartitionSpec> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PartitionSpec as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PartitionSpec - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PartitionStatisticsFile {
    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "statistics-path")]
    pub statistics_path: String,

    #[serde(rename = "file-size-in-bytes")]
    pub file_size_in_bytes: i64,

}


impl PartitionStatisticsFile {
    #[allow(clippy::new_without_default)]
    pub fn new(snapshot_id: i64, statistics_path: String, file_size_in_bytes: i64, ) -> PartitionStatisticsFile {
        PartitionStatisticsFile {
            snapshot_id,
            statistics_path,
            file_size_in_bytes,
        }
    }
}

/// Converts the PartitionStatisticsFile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PartitionStatisticsFile {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            Some("statistics-path".to_string()),
            Some(self.statistics_path.to_string()),


            Some("file-size-in-bytes".to_string()),
            Some(self.file_size_in_bytes.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PartitionStatisticsFile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PartitionStatisticsFile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub snapshot_id: Vec<i64>,
            pub statistics_path: Vec<String>,
            pub file_size_in_bytes: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PartitionStatisticsFile".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "statistics-path" => intermediate_rep.statistics_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-size-in-bytes" => intermediate_rep.file_size_in_bytes.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PartitionStatisticsFile".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PartitionStatisticsFile {
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in PartitionStatisticsFile".to_string())?,
            statistics_path: intermediate_rep.statistics_path.into_iter().next().ok_or_else(|| "statistics-path missing in PartitionStatisticsFile".to_string())?,
            file_size_in_bytes: intermediate_rep.file_size_in_bytes.into_iter().next().ok_or_else(|| "file-size-in-bytes missing in PartitionStatisticsFile".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PartitionStatisticsFile> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PartitionStatisticsFile>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PartitionStatisticsFile>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PartitionStatisticsFile - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PartitionStatisticsFile> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PartitionStatisticsFile as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PartitionStatisticsFile - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PositionDeleteFile {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "file-path")]
    pub file_path: String,

    #[serde(rename = "file-format")]
    pub file_format: models::FileFormat,

    #[serde(rename = "spec-id")]
    pub spec_id: i32,

    /// A list of partition field values ordered based on the fields of the partition spec specified by the `spec-id`
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition: Option<Vec<models::PrimitiveTypeValue>>,

    /// Total file size in bytes
    #[serde(rename = "file-size-in-bytes")]
    pub file_size_in_bytes: i64,

    /// Number of records in the file
    #[serde(rename = "record-count")]
    pub record_count: i64,

    /// Binary type values are stored and serialized as an uppercase hexadecimal string
    #[serde(rename = "key-metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_metadata: Option<String>,

    /// List of splittable offsets
    #[serde(rename = "split-offsets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub split_offsets: Option<Vec<i64>>,

    #[serde(rename = "sort-order-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_order_id: Option<i32>,

}


impl PositionDeleteFile {
    #[allow(clippy::new_without_default)]
    pub fn new(content: String, file_path: String, file_format: models::FileFormat, spec_id: i32, file_size_in_bytes: i64, record_count: i64, ) -> PositionDeleteFile {
        PositionDeleteFile {
            content,
            file_path,
            file_format,
            spec_id,
            partition: None,
            file_size_in_bytes,
            record_count,
            key_metadata: None,
            split_offsets: None,
            sort_order_id: None,
        }
    }
}

/// Converts the PositionDeleteFile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PositionDeleteFile {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("content".to_string()),
            Some(self.content.to_string()),


            Some("file-path".to_string()),
            Some(self.file_path.to_string()),

            // Skipping file-format in query parameter serialization


            Some("spec-id".to_string()),
            Some(self.spec_id.to_string()),

            // Skipping partition in query parameter serialization


            Some("file-size-in-bytes".to_string()),
            Some(self.file_size_in_bytes.to_string()),


            Some("record-count".to_string()),
            Some(self.record_count.to_string()),


            self.key_metadata.as_ref().map(|key_metadata| {
                [
                    "key-metadata".to_string(),
                    key_metadata.to_string(),
                ].join(",")
            }),


            self.split_offsets.as_ref().map(|split_offsets| {
                [
                    "split-offsets".to_string(),
                    split_offsets.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),


            self.sort_order_id.as_ref().map(|sort_order_id| {
                [
                    "sort-order-id".to_string(),
                    sort_order_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PositionDeleteFile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PositionDeleteFile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub content: Vec<String>,
            pub file_path: Vec<String>,
            pub file_format: Vec<models::FileFormat>,
            pub spec_id: Vec<i32>,
            pub partition: Vec<Vec<models::PrimitiveTypeValue>>,
            pub file_size_in_bytes: Vec<i64>,
            pub record_count: Vec<i64>,
            pub key_metadata: Vec<String>,
            pub split_offsets: Vec<Vec<i64>>,
            pub sort_order_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PositionDeleteFile".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "content" => intermediate_rep.content.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-path" => intermediate_rep.file_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-format" => intermediate_rep.file_format.push(<models::FileFormat as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spec-id" => intermediate_rep.spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "partition" => return std::result::Result::Err("Parsing a container in this style is not supported in PositionDeleteFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "file-size-in-bytes" => intermediate_rep.file_size_in_bytes.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "record-count" => intermediate_rep.record_count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key-metadata" => intermediate_rep.key_metadata.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "split-offsets" => return std::result::Result::Err("Parsing a container in this style is not supported in PositionDeleteFile".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "sort-order-id" => intermediate_rep.sort_order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PositionDeleteFile".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PositionDeleteFile {
            content: intermediate_rep.content.into_iter().next().ok_or_else(|| "content missing in PositionDeleteFile".to_string())?,
            file_path: intermediate_rep.file_path.into_iter().next().ok_or_else(|| "file-path missing in PositionDeleteFile".to_string())?,
            file_format: intermediate_rep.file_format.into_iter().next().ok_or_else(|| "file-format missing in PositionDeleteFile".to_string())?,
            spec_id: intermediate_rep.spec_id.into_iter().next().ok_or_else(|| "spec-id missing in PositionDeleteFile".to_string())?,
            partition: intermediate_rep.partition.into_iter().next(),
            file_size_in_bytes: intermediate_rep.file_size_in_bytes.into_iter().next().ok_or_else(|| "file-size-in-bytes missing in PositionDeleteFile".to_string())?,
            record_count: intermediate_rep.record_count.into_iter().next().ok_or_else(|| "record-count missing in PositionDeleteFile".to_string())?,
            key_metadata: intermediate_rep.key_metadata.into_iter().next(),
            split_offsets: intermediate_rep.split_offsets.into_iter().next(),
            sort_order_id: intermediate_rep.sort_order_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PositionDeleteFile> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PositionDeleteFile>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PositionDeleteFile>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PositionDeleteFile - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PositionDeleteFile> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PositionDeleteFile as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PositionDeleteFile - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PrimitiveType(String);

impl std::convert::From<String> for PrimitiveType {
    fn from(x: String) -> Self {
        PrimitiveType(x)
    }
}

impl std::string::ToString for PrimitiveType {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for PrimitiveType {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(PrimitiveType(x.to_string()))
    }
}

impl std::convert::From<PrimitiveType> for String {
    fn from(x: PrimitiveType) -> Self {
        x.0
    }
}

impl std::ops::Deref for PrimitiveType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for PrimitiveType {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PrimitiveTypeValue {
}


impl PrimitiveTypeValue {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PrimitiveTypeValue {
        PrimitiveTypeValue {
        }
    }
}

/// Converts the PrimitiveTypeValue value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PrimitiveTypeValue {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PrimitiveTypeValue value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PrimitiveTypeValue {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PrimitiveTypeValue".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing PrimitiveTypeValue".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PrimitiveTypeValue {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PrimitiveTypeValue> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PrimitiveTypeValue>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PrimitiveTypeValue>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PrimitiveTypeValue - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PrimitiveTypeValue> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PrimitiveTypeValue as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PrimitiveTypeValue - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Reference(String);

impl std::convert::From<String> for Reference {
    fn from(x: String) -> Self {
        Reference(x)
    }
}

impl std::string::ToString for Reference {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Reference {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Reference(x.to_string()))
    }
}

impl std::convert::From<Reference> for String {
    fn from(x: Reference) -> Self {
        x.0
    }
}

impl std::ops::Deref for Reference {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Reference {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegisterTableRequest {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "metadata-location")]
    pub metadata_location: String,

}


impl RegisterTableRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, metadata_location: String, ) -> RegisterTableRequest {
        RegisterTableRequest {
            name,
            metadata_location,
        }
    }
}

/// Converts the RegisterTableRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RegisterTableRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("metadata-location".to_string()),
            Some(self.metadata_location.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RegisterTableRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RegisterTableRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub metadata_location: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RegisterTableRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "metadata-location" => intermediate_rep.metadata_location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RegisterTableRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RegisterTableRequest {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in RegisterTableRequest".to_string())?,
            metadata_location: intermediate_rep.metadata_location.into_iter().next().ok_or_else(|| "metadata-location missing in RegisterTableRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RegisterTableRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RegisterTableRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RegisterTableRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RegisterTableRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RegisterTableRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RegisterTableRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RegisterTableRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RemovePartitionStatisticsUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

}


impl RemovePartitionStatisticsUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, snapshot_id: i64, ) -> RemovePartitionStatisticsUpdate {
        RemovePartitionStatisticsUpdate {
            action,
            snapshot_id,
        }
    }
}

/// Converts the RemovePartitionStatisticsUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RemovePartitionStatisticsUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RemovePartitionStatisticsUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RemovePartitionStatisticsUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub snapshot_id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RemovePartitionStatisticsUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RemovePartitionStatisticsUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RemovePartitionStatisticsUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in RemovePartitionStatisticsUpdate".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in RemovePartitionStatisticsUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RemovePartitionStatisticsUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RemovePartitionStatisticsUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RemovePartitionStatisticsUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RemovePartitionStatisticsUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RemovePartitionStatisticsUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RemovePartitionStatisticsUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RemovePartitionStatisticsUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RemovePropertiesUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "removals")]
    pub removals: Vec<String>,

}


impl RemovePropertiesUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, removals: Vec<String>, ) -> RemovePropertiesUpdate {
        RemovePropertiesUpdate {
            action,
            removals,
        }
    }
}

/// Converts the RemovePropertiesUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RemovePropertiesUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("removals".to_string()),
            Some(self.removals.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RemovePropertiesUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RemovePropertiesUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub removals: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RemovePropertiesUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "removals" => return std::result::Result::Err("Parsing a container in this style is not supported in RemovePropertiesUpdate".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RemovePropertiesUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RemovePropertiesUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in RemovePropertiesUpdate".to_string())?,
            removals: intermediate_rep.removals.into_iter().next().ok_or_else(|| "removals missing in RemovePropertiesUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RemovePropertiesUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RemovePropertiesUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RemovePropertiesUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RemovePropertiesUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RemovePropertiesUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RemovePropertiesUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RemovePropertiesUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RemoveSnapshotRefUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "ref-name")]
    pub ref_name: String,

}


impl RemoveSnapshotRefUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, ref_name: String, ) -> RemoveSnapshotRefUpdate {
        RemoveSnapshotRefUpdate {
            action,
            ref_name,
        }
    }
}

/// Converts the RemoveSnapshotRefUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RemoveSnapshotRefUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("ref-name".to_string()),
            Some(self.ref_name.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RemoveSnapshotRefUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RemoveSnapshotRefUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub ref_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RemoveSnapshotRefUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ref-name" => intermediate_rep.ref_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RemoveSnapshotRefUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RemoveSnapshotRefUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in RemoveSnapshotRefUpdate".to_string())?,
            ref_name: intermediate_rep.ref_name.into_iter().next().ok_or_else(|| "ref-name missing in RemoveSnapshotRefUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RemoveSnapshotRefUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RemoveSnapshotRefUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RemoveSnapshotRefUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RemoveSnapshotRefUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RemoveSnapshotRefUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RemoveSnapshotRefUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RemoveSnapshotRefUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RemoveSnapshotsUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "snapshot-ids")]
    pub snapshot_ids: Vec<i64>,

}


impl RemoveSnapshotsUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, snapshot_ids: Vec<i64>, ) -> RemoveSnapshotsUpdate {
        RemoveSnapshotsUpdate {
            action,
            snapshot_ids,
        }
    }
}

/// Converts the RemoveSnapshotsUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RemoveSnapshotsUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("snapshot-ids".to_string()),
            Some(self.snapshot_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RemoveSnapshotsUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RemoveSnapshotsUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub snapshot_ids: Vec<Vec<i64>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RemoveSnapshotsUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "snapshot-ids" => return std::result::Result::Err("Parsing a container in this style is not supported in RemoveSnapshotsUpdate".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RemoveSnapshotsUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RemoveSnapshotsUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in RemoveSnapshotsUpdate".to_string())?,
            snapshot_ids: intermediate_rep.snapshot_ids.into_iter().next().ok_or_else(|| "snapshot-ids missing in RemoveSnapshotsUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RemoveSnapshotsUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RemoveSnapshotsUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RemoveSnapshotsUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RemoveSnapshotsUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RemoveSnapshotsUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RemoveSnapshotsUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RemoveSnapshotsUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RemoveStatisticsUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

}


impl RemoveStatisticsUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, snapshot_id: i64, ) -> RemoveStatisticsUpdate {
        RemoveStatisticsUpdate {
            action,
            snapshot_id,
        }
    }
}

/// Converts the RemoveStatisticsUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RemoveStatisticsUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RemoveStatisticsUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RemoveStatisticsUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub snapshot_id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RemoveStatisticsUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RemoveStatisticsUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RemoveStatisticsUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in RemoveStatisticsUpdate".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in RemoveStatisticsUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RemoveStatisticsUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RemoveStatisticsUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RemoveStatisticsUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RemoveStatisticsUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RemoveStatisticsUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RemoveStatisticsUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RemoveStatisticsUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RenameTableRequest {
    #[serde(rename = "source")]
    pub source: models::TableIdentifier,

    #[serde(rename = "destination")]
    pub destination: models::TableIdentifier,

}


impl RenameTableRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(source: models::TableIdentifier, destination: models::TableIdentifier, ) -> RenameTableRequest {
        RenameTableRequest {
            source,
            destination,
        }
    }
}

/// Converts the RenameTableRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RenameTableRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping source in query parameter serialization

            // Skipping destination in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RenameTableRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RenameTableRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub source: Vec<models::TableIdentifier>,
            pub destination: Vec<models::TableIdentifier>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RenameTableRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "source" => intermediate_rep.source.push(<models::TableIdentifier as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "destination" => intermediate_rep.destination.push(<models::TableIdentifier as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RenameTableRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RenameTableRequest {
            source: intermediate_rep.source.into_iter().next().ok_or_else(|| "source missing in RenameTableRequest".to_string())?,
            destination: intermediate_rep.destination.into_iter().next().ok_or_else(|| "destination missing in RenameTableRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RenameTableRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RenameTableRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RenameTableRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RenameTableRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RenameTableRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RenameTableRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RenameTableRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReportMetricsRequest {
    #[serde(rename = "report-type")]
    pub report_type: String,

    #[serde(rename = "table-name")]
    pub table_name: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "filter")]
    pub filter: models::Expression,

    #[serde(rename = "schema-id")]
    pub schema_id: i32,

    #[serde(rename = "projected-field-ids")]
    pub projected_field_ids: Vec<i32>,

    #[serde(rename = "projected-field-names")]
    pub projected_field_names: Vec<String>,

    #[serde(rename = "metrics")]
    pub metrics: std::collections::HashMap<String, models::MetricResult>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "sequence-number")]
    pub sequence_number: i64,

    #[serde(rename = "operation")]
    pub operation: String,

}


impl ReportMetricsRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(report_type: String, table_name: String, snapshot_id: i64, filter: models::Expression, schema_id: i32, projected_field_ids: Vec<i32>, projected_field_names: Vec<String>, metrics: std::collections::HashMap<String, models::MetricResult>, sequence_number: i64, operation: String, ) -> ReportMetricsRequest {
        ReportMetricsRequest {
            report_type,
            table_name,
            snapshot_id,
            filter,
            schema_id,
            projected_field_ids,
            projected_field_names,
            metrics,
            metadata: None,
            sequence_number,
            operation,
        }
    }
}

/// Converts the ReportMetricsRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ReportMetricsRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("report-type".to_string()),
            Some(self.report_type.to_string()),


            Some("table-name".to_string()),
            Some(self.table_name.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),

            // Skipping filter in query parameter serialization


            Some("schema-id".to_string()),
            Some(self.schema_id.to_string()),


            Some("projected-field-ids".to_string()),
            Some(self.projected_field_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("projected-field-names".to_string()),
            Some(self.projected_field_names.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping metrics in query parameter serialization
            // Skipping metrics in query parameter serialization

            // Skipping metadata in query parameter serialization


            Some("sequence-number".to_string()),
            Some(self.sequence_number.to_string()),


            Some("operation".to_string()),
            Some(self.operation.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ReportMetricsRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ReportMetricsRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub report_type: Vec<String>,
            pub table_name: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub filter: Vec<models::Expression>,
            pub schema_id: Vec<i32>,
            pub projected_field_ids: Vec<Vec<i32>>,
            pub projected_field_names: Vec<Vec<String>>,
            pub metrics: Vec<std::collections::HashMap<String, models::MetricResult>>,
            pub metadata: Vec<std::collections::HashMap<String, String>>,
            pub sequence_number: Vec<i64>,
            pub operation: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ReportMetricsRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "report-type" => intermediate_rep.report_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "table-name" => intermediate_rep.table_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "filter" => intermediate_rep.filter.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema-id" => intermediate_rep.schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "projected-field-ids" => return std::result::Result::Err("Parsing a container in this style is not supported in ReportMetricsRequest".to_string()),
                    "projected-field-names" => return std::result::Result::Err("Parsing a container in this style is not supported in ReportMetricsRequest".to_string()),
                    "metrics" => return std::result::Result::Err("Parsing a container in this style is not supported in ReportMetricsRequest".to_string()),
                    "metadata" => return std::result::Result::Err("Parsing a container in this style is not supported in ReportMetricsRequest".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "sequence-number" => intermediate_rep.sequence_number.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "operation" => intermediate_rep.operation.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ReportMetricsRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ReportMetricsRequest {
            report_type: intermediate_rep.report_type.into_iter().next().ok_or_else(|| "report-type missing in ReportMetricsRequest".to_string())?,
            table_name: intermediate_rep.table_name.into_iter().next().ok_or_else(|| "table-name missing in ReportMetricsRequest".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in ReportMetricsRequest".to_string())?,
            filter: intermediate_rep.filter.into_iter().next().ok_or_else(|| "filter missing in ReportMetricsRequest".to_string())?,
            schema_id: intermediate_rep.schema_id.into_iter().next().ok_or_else(|| "schema-id missing in ReportMetricsRequest".to_string())?,
            projected_field_ids: intermediate_rep.projected_field_ids.into_iter().next().ok_or_else(|| "projected-field-ids missing in ReportMetricsRequest".to_string())?,
            projected_field_names: intermediate_rep.projected_field_names.into_iter().next().ok_or_else(|| "projected-field-names missing in ReportMetricsRequest".to_string())?,
            metrics: intermediate_rep.metrics.into_iter().next().ok_or_else(|| "metrics missing in ReportMetricsRequest".to_string())?,
            metadata: intermediate_rep.metadata.into_iter().next(),
            sequence_number: intermediate_rep.sequence_number.into_iter().next().ok_or_else(|| "sequence-number missing in ReportMetricsRequest".to_string())?,
            operation: intermediate_rep.operation.into_iter().next().ok_or_else(|| "operation missing in ReportMetricsRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ReportMetricsRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ReportMetricsRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ReportMetricsRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ReportMetricsRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ReportMetricsRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ReportMetricsRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ReportMetricsRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ScanReport {
    #[serde(rename = "table-name")]
    pub table_name: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "filter")]
    pub filter: models::Expression,

    #[serde(rename = "schema-id")]
    pub schema_id: i32,

    #[serde(rename = "projected-field-ids")]
    pub projected_field_ids: Vec<i32>,

    #[serde(rename = "projected-field-names")]
    pub projected_field_names: Vec<String>,

    #[serde(rename = "metrics")]
    pub metrics: std::collections::HashMap<String, models::MetricResult>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

}


impl ScanReport {
    #[allow(clippy::new_without_default)]
    pub fn new(table_name: String, snapshot_id: i64, filter: models::Expression, schema_id: i32, projected_field_ids: Vec<i32>, projected_field_names: Vec<String>, metrics: std::collections::HashMap<String, models::MetricResult>, ) -> ScanReport {
        ScanReport {
            table_name,
            snapshot_id,
            filter,
            schema_id,
            projected_field_ids,
            projected_field_names,
            metrics,
            metadata: None,
        }
    }
}

/// Converts the ScanReport value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ScanReport {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("table-name".to_string()),
            Some(self.table_name.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),

            // Skipping filter in query parameter serialization


            Some("schema-id".to_string()),
            Some(self.schema_id.to_string()),


            Some("projected-field-ids".to_string()),
            Some(self.projected_field_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("projected-field-names".to_string()),
            Some(self.projected_field_names.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping metrics in query parameter serialization
            // Skipping metrics in query parameter serialization

            // Skipping metadata in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ScanReport value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ScanReport {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub table_name: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub filter: Vec<models::Expression>,
            pub schema_id: Vec<i32>,
            pub projected_field_ids: Vec<Vec<i32>>,
            pub projected_field_names: Vec<Vec<String>>,
            pub metrics: Vec<std::collections::HashMap<String, models::MetricResult>>,
            pub metadata: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ScanReport".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "table-name" => intermediate_rep.table_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "filter" => intermediate_rep.filter.push(<models::Expression as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema-id" => intermediate_rep.schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "projected-field-ids" => return std::result::Result::Err("Parsing a container in this style is not supported in ScanReport".to_string()),
                    "projected-field-names" => return std::result::Result::Err("Parsing a container in this style is not supported in ScanReport".to_string()),
                    "metrics" => return std::result::Result::Err("Parsing a container in this style is not supported in ScanReport".to_string()),
                    "metadata" => return std::result::Result::Err("Parsing a container in this style is not supported in ScanReport".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ScanReport".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ScanReport {
            table_name: intermediate_rep.table_name.into_iter().next().ok_or_else(|| "table-name missing in ScanReport".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in ScanReport".to_string())?,
            filter: intermediate_rep.filter.into_iter().next().ok_or_else(|| "filter missing in ScanReport".to_string())?,
            schema_id: intermediate_rep.schema_id.into_iter().next().ok_or_else(|| "schema-id missing in ScanReport".to_string())?,
            projected_field_ids: intermediate_rep.projected_field_ids.into_iter().next().ok_or_else(|| "projected-field-ids missing in ScanReport".to_string())?,
            projected_field_names: intermediate_rep.projected_field_names.into_iter().next().ok_or_else(|| "projected-field-names missing in ScanReport".to_string())?,
            metrics: intermediate_rep.metrics.into_iter().next().ok_or_else(|| "metrics missing in ScanReport".to_string())?,
            metadata: intermediate_rep.metadata.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ScanReport> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ScanReport>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ScanReport>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ScanReport - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ScanReport> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ScanReport as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ScanReport - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Schema {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "fields")]
    pub fields: Vec<models::StructField>,

    #[serde(rename = "schema-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_id: Option<i32>,

    #[serde(rename = "identifier-field-ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identifier_field_ids: Option<Vec<i32>>,

}


impl Schema {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, fields: Vec<models::StructField>, ) -> Schema {
        Schema {
            r#type,
            fields,
            schema_id: None,
            identifier_field_ids: None,
        }
    }
}

/// Converts the Schema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Schema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping fields in query parameter serialization


            self.schema_id.as_ref().map(|schema_id| {
                [
                    "schema-id".to_string(),
                    schema_id.to_string(),
                ].join(",")
            }),


            self.identifier_field_ids.as_ref().map(|identifier_field_ids| {
                [
                    "identifier-field-ids".to_string(),
                    identifier_field_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Schema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Schema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub fields: Vec<Vec<models::StructField>>,
            pub schema_id: Vec<i32>,
            pub identifier_field_ids: Vec<Vec<i32>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Schema".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in Schema".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "schema-id" => intermediate_rep.schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "identifier-field-ids" => return std::result::Result::Err("Parsing a container in this style is not supported in Schema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Schema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Schema {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Schema".to_string())?,
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in Schema".to_string())?,
            schema_id: intermediate_rep.schema_id.into_iter().next(),
            identifier_field_ids: intermediate_rep.identifier_field_ids.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Schema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Schema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Schema>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Schema - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Schema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Schema as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Schema - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetCurrentSchemaUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    /// Schema ID to set as current, or -1 to set last added schema
    #[serde(rename = "schema-id")]
    pub schema_id: i32,

}


impl SetCurrentSchemaUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, schema_id: i32, ) -> SetCurrentSchemaUpdate {
        SetCurrentSchemaUpdate {
            action,
            schema_id,
        }
    }
}

/// Converts the SetCurrentSchemaUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetCurrentSchemaUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("schema-id".to_string()),
            Some(self.schema_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetCurrentSchemaUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetCurrentSchemaUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub schema_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetCurrentSchemaUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema-id" => intermediate_rep.schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetCurrentSchemaUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetCurrentSchemaUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetCurrentSchemaUpdate".to_string())?,
            schema_id: intermediate_rep.schema_id.into_iter().next().ok_or_else(|| "schema-id missing in SetCurrentSchemaUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetCurrentSchemaUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetCurrentSchemaUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetCurrentSchemaUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetCurrentSchemaUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetCurrentSchemaUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetCurrentSchemaUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetCurrentSchemaUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetCurrentViewVersionUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    /// The view version id to set as current, or -1 to set last added view version id
    #[serde(rename = "view-version-id")]
    pub view_version_id: i32,

}


impl SetCurrentViewVersionUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, view_version_id: i32, ) -> SetCurrentViewVersionUpdate {
        SetCurrentViewVersionUpdate {
            action,
            view_version_id,
        }
    }
}

/// Converts the SetCurrentViewVersionUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetCurrentViewVersionUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("view-version-id".to_string()),
            Some(self.view_version_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetCurrentViewVersionUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetCurrentViewVersionUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub view_version_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetCurrentViewVersionUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "view-version-id" => intermediate_rep.view_version_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetCurrentViewVersionUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetCurrentViewVersionUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetCurrentViewVersionUpdate".to_string())?,
            view_version_id: intermediate_rep.view_version_id.into_iter().next().ok_or_else(|| "view-version-id missing in SetCurrentViewVersionUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetCurrentViewVersionUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetCurrentViewVersionUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetCurrentViewVersionUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetCurrentViewVersionUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetCurrentViewVersionUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetCurrentViewVersionUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetCurrentViewVersionUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetDefaultSortOrderUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    /// Sort order ID to set as the default, or -1 to set last added sort order
    #[serde(rename = "sort-order-id")]
    pub sort_order_id: i32,

}


impl SetDefaultSortOrderUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, sort_order_id: i32, ) -> SetDefaultSortOrderUpdate {
        SetDefaultSortOrderUpdate {
            action,
            sort_order_id,
        }
    }
}

/// Converts the SetDefaultSortOrderUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetDefaultSortOrderUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("sort-order-id".to_string()),
            Some(self.sort_order_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetDefaultSortOrderUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetDefaultSortOrderUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub sort_order_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetDefaultSortOrderUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sort-order-id" => intermediate_rep.sort_order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetDefaultSortOrderUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetDefaultSortOrderUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetDefaultSortOrderUpdate".to_string())?,
            sort_order_id: intermediate_rep.sort_order_id.into_iter().next().ok_or_else(|| "sort-order-id missing in SetDefaultSortOrderUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetDefaultSortOrderUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetDefaultSortOrderUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetDefaultSortOrderUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetDefaultSortOrderUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetDefaultSortOrderUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetDefaultSortOrderUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetDefaultSortOrderUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetDefaultSpecUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    /// Partition spec ID to set as the default, or -1 to set last added spec
    #[serde(rename = "spec-id")]
    pub spec_id: i32,

}


impl SetDefaultSpecUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, spec_id: i32, ) -> SetDefaultSpecUpdate {
        SetDefaultSpecUpdate {
            action,
            spec_id,
        }
    }
}

/// Converts the SetDefaultSpecUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetDefaultSpecUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("spec-id".to_string()),
            Some(self.spec_id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetDefaultSpecUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetDefaultSpecUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub spec_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetDefaultSpecUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "spec-id" => intermediate_rep.spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetDefaultSpecUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetDefaultSpecUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetDefaultSpecUpdate".to_string())?,
            spec_id: intermediate_rep.spec_id.into_iter().next().ok_or_else(|| "spec-id missing in SetDefaultSpecUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetDefaultSpecUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetDefaultSpecUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetDefaultSpecUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetDefaultSpecUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetDefaultSpecUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetDefaultSpecUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetDefaultSpecUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetExpression {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "term")]
    pub term: models::Term,

    #[serde(rename = "values")]
    pub values: Vec<serde_json::Value>,

}


impl SetExpression {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, term: models::Term, values: Vec<serde_json::Value>, ) -> SetExpression {
        SetExpression {
            r#type,
            term,
            values,
        }
    }
}

/// Converts the SetExpression value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetExpression {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping term in query parameter serialization

            // Skipping values in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetExpression value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetExpression {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub term: Vec<models::Term>,
            pub values: Vec<Vec<serde_json::Value>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetExpression".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "term" => intermediate_rep.term.push(<models::Term as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in SetExpression".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetExpression".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetExpression {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in SetExpression".to_string())?,
            term: intermediate_rep.term.into_iter().next().ok_or_else(|| "term missing in SetExpression".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in SetExpression".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetExpression> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetExpression>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetExpression>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetExpression - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetExpression> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetExpression as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetExpression - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetLocationUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "location")]
    pub location: String,

}


impl SetLocationUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, location: String, ) -> SetLocationUpdate {
        SetLocationUpdate {
            action,
            location,
        }
    }
}

/// Converts the SetLocationUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetLocationUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("location".to_string()),
            Some(self.location.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetLocationUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetLocationUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub location: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetLocationUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "location" => intermediate_rep.location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetLocationUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetLocationUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetLocationUpdate".to_string())?,
            location: intermediate_rep.location.into_iter().next().ok_or_else(|| "location missing in SetLocationUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetLocationUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetLocationUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetLocationUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetLocationUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetLocationUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetLocationUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetLocationUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetPartitionStatisticsUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "partition-statistics")]
    pub partition_statistics: models::PartitionStatisticsFile,

}


impl SetPartitionStatisticsUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, partition_statistics: models::PartitionStatisticsFile, ) -> SetPartitionStatisticsUpdate {
        SetPartitionStatisticsUpdate {
            action,
            partition_statistics,
        }
    }
}

/// Converts the SetPartitionStatisticsUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetPartitionStatisticsUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

            // Skipping partition-statistics in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetPartitionStatisticsUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetPartitionStatisticsUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub partition_statistics: Vec<models::PartitionStatisticsFile>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetPartitionStatisticsUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "partition-statistics" => intermediate_rep.partition_statistics.push(<models::PartitionStatisticsFile as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetPartitionStatisticsUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetPartitionStatisticsUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetPartitionStatisticsUpdate".to_string())?,
            partition_statistics: intermediate_rep.partition_statistics.into_iter().next().ok_or_else(|| "partition-statistics missing in SetPartitionStatisticsUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetPartitionStatisticsUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetPartitionStatisticsUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetPartitionStatisticsUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetPartitionStatisticsUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetPartitionStatisticsUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetPartitionStatisticsUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetPartitionStatisticsUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetPropertiesUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "updates")]
    pub updates: std::collections::HashMap<String, String>,

}


impl SetPropertiesUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, updates: std::collections::HashMap<String, String>, ) -> SetPropertiesUpdate {
        SetPropertiesUpdate {
            action,
            updates,
        }
    }
}

/// Converts the SetPropertiesUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetPropertiesUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

            // Skipping updates in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetPropertiesUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetPropertiesUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub updates: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetPropertiesUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "updates" => return std::result::Result::Err("Parsing a container in this style is not supported in SetPropertiesUpdate".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetPropertiesUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetPropertiesUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetPropertiesUpdate".to_string())?,
            updates: intermediate_rep.updates.into_iter().next().ok_or_else(|| "updates missing in SetPropertiesUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetPropertiesUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetPropertiesUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetPropertiesUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetPropertiesUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetPropertiesUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetPropertiesUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetPropertiesUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetSnapshotRefUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "ref-name")]
    pub ref_name: String,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "max-ref-age-ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_ref_age_ms: Option<i64>,

    #[serde(rename = "max-snapshot-age-ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_snapshot_age_ms: Option<i64>,

    #[serde(rename = "min-snapshots-to-keep")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_snapshots_to_keep: Option<i32>,

}


impl SetSnapshotRefUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, ref_name: String, r#type: String, snapshot_id: i64, ) -> SetSnapshotRefUpdate {
        SetSnapshotRefUpdate {
            action,
            ref_name,
            r#type,
            snapshot_id,
            max_ref_age_ms: None,
            max_snapshot_age_ms: None,
            min_snapshots_to_keep: None,
        }
    }
}

/// Converts the SetSnapshotRefUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetSnapshotRefUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("ref-name".to_string()),
            Some(self.ref_name.to_string()),


            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            self.max_ref_age_ms.as_ref().map(|max_ref_age_ms| {
                [
                    "max-ref-age-ms".to_string(),
                    max_ref_age_ms.to_string(),
                ].join(",")
            }),


            self.max_snapshot_age_ms.as_ref().map(|max_snapshot_age_ms| {
                [
                    "max-snapshot-age-ms".to_string(),
                    max_snapshot_age_ms.to_string(),
                ].join(",")
            }),


            self.min_snapshots_to_keep.as_ref().map(|min_snapshots_to_keep| {
                [
                    "min-snapshots-to-keep".to_string(),
                    min_snapshots_to_keep.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetSnapshotRefUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetSnapshotRefUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub ref_name: Vec<String>,
            pub r#type: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub max_ref_age_ms: Vec<i64>,
            pub max_snapshot_age_ms: Vec<i64>,
            pub min_snapshots_to_keep: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetSnapshotRefUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ref-name" => intermediate_rep.ref_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max-ref-age-ms" => intermediate_rep.max_ref_age_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max-snapshot-age-ms" => intermediate_rep.max_snapshot_age_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "min-snapshots-to-keep" => intermediate_rep.min_snapshots_to_keep.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetSnapshotRefUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetSnapshotRefUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetSnapshotRefUpdate".to_string())?,
            ref_name: intermediate_rep.ref_name.into_iter().next().ok_or_else(|| "ref-name missing in SetSnapshotRefUpdate".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in SetSnapshotRefUpdate".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in SetSnapshotRefUpdate".to_string())?,
            max_ref_age_ms: intermediate_rep.max_ref_age_ms.into_iter().next(),
            max_snapshot_age_ms: intermediate_rep.max_snapshot_age_ms.into_iter().next(),
            min_snapshots_to_keep: intermediate_rep.min_snapshots_to_keep.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetSnapshotRefUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetSnapshotRefUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetSnapshotRefUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetSnapshotRefUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetSnapshotRefUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetSnapshotRefUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetSnapshotRefUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SetStatisticsUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "statistics")]
    pub statistics: models::StatisticsFile,

}


impl SetStatisticsUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, snapshot_id: i64, statistics: models::StatisticsFile, ) -> SetStatisticsUpdate {
        SetStatisticsUpdate {
            action,
            snapshot_id,
            statistics,
        }
    }
}

/// Converts the SetStatisticsUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SetStatisticsUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),

            // Skipping statistics in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SetStatisticsUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SetStatisticsUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub statistics: Vec<models::StatisticsFile>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SetStatisticsUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "statistics" => intermediate_rep.statistics.push(<models::StatisticsFile as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SetStatisticsUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SetStatisticsUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in SetStatisticsUpdate".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in SetStatisticsUpdate".to_string())?,
            statistics: intermediate_rep.statistics.into_iter().next().ok_or_else(|| "statistics missing in SetStatisticsUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SetStatisticsUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SetStatisticsUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SetStatisticsUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SetStatisticsUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SetStatisticsUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SetStatisticsUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SetStatisticsUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Snapshot {
    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "parent-snapshot-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_snapshot_id: Option<i64>,

    #[serde(rename = "sequence-number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sequence_number: Option<i64>,

    #[serde(rename = "timestamp-ms")]
    pub timestamp_ms: i64,

    /// Location of the snapshot's manifest list file
    #[serde(rename = "manifest-list")]
    pub manifest_list: String,

    #[serde(rename = "summary")]
    pub summary: models::SnapshotSummary,

    #[serde(rename = "schema-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_id: Option<i32>,

}


impl Snapshot {
    #[allow(clippy::new_without_default)]
    pub fn new(snapshot_id: i64, timestamp_ms: i64, manifest_list: String, summary: models::SnapshotSummary, ) -> Snapshot {
        Snapshot {
            snapshot_id,
            parent_snapshot_id: None,
            sequence_number: None,
            timestamp_ms,
            manifest_list,
            summary,
            schema_id: None,
        }
    }
}

/// Converts the Snapshot value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Snapshot {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            self.parent_snapshot_id.as_ref().map(|parent_snapshot_id| {
                [
                    "parent-snapshot-id".to_string(),
                    parent_snapshot_id.to_string(),
                ].join(",")
            }),


            self.sequence_number.as_ref().map(|sequence_number| {
                [
                    "sequence-number".to_string(),
                    sequence_number.to_string(),
                ].join(",")
            }),


            Some("timestamp-ms".to_string()),
            Some(self.timestamp_ms.to_string()),


            Some("manifest-list".to_string()),
            Some(self.manifest_list.to_string()),

            // Skipping summary in query parameter serialization


            self.schema_id.as_ref().map(|schema_id| {
                [
                    "schema-id".to_string(),
                    schema_id.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Snapshot value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Snapshot {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub snapshot_id: Vec<i64>,
            pub parent_snapshot_id: Vec<i64>,
            pub sequence_number: Vec<i64>,
            pub timestamp_ms: Vec<i64>,
            pub manifest_list: Vec<String>,
            pub summary: Vec<models::SnapshotSummary>,
            pub schema_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Snapshot".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "parent-snapshot-id" => intermediate_rep.parent_snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sequence-number" => intermediate_rep.sequence_number.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp-ms" => intermediate_rep.timestamp_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "manifest-list" => intermediate_rep.manifest_list.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "summary" => intermediate_rep.summary.push(<models::SnapshotSummary as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema-id" => intermediate_rep.schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Snapshot".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Snapshot {
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in Snapshot".to_string())?,
            parent_snapshot_id: intermediate_rep.parent_snapshot_id.into_iter().next(),
            sequence_number: intermediate_rep.sequence_number.into_iter().next(),
            timestamp_ms: intermediate_rep.timestamp_ms.into_iter().next().ok_or_else(|| "timestamp-ms missing in Snapshot".to_string())?,
            manifest_list: intermediate_rep.manifest_list.into_iter().next().ok_or_else(|| "manifest-list missing in Snapshot".to_string())?,
            summary: intermediate_rep.summary.into_iter().next().ok_or_else(|| "summary missing in Snapshot".to_string())?,
            schema_id: intermediate_rep.schema_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Snapshot> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Snapshot>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Snapshot>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Snapshot - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Snapshot> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Snapshot as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Snapshot - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotLogInner {
    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "timestamp-ms")]
    pub timestamp_ms: i64,

}


impl SnapshotLogInner {
    #[allow(clippy::new_without_default)]
    pub fn new(snapshot_id: i64, timestamp_ms: i64, ) -> SnapshotLogInner {
        SnapshotLogInner {
            snapshot_id,
            timestamp_ms,
        }
    }
}

/// Converts the SnapshotLogInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SnapshotLogInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            Some("timestamp-ms".to_string()),
            Some(self.timestamp_ms.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SnapshotLogInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SnapshotLogInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub snapshot_id: Vec<i64>,
            pub timestamp_ms: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SnapshotLogInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp-ms" => intermediate_rep.timestamp_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SnapshotLogInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SnapshotLogInner {
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in SnapshotLogInner".to_string())?,
            timestamp_ms: intermediate_rep.timestamp_ms.into_iter().next().ok_or_else(|| "timestamp-ms missing in SnapshotLogInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SnapshotLogInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SnapshotLogInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SnapshotLogInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SnapshotLogInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SnapshotLogInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SnapshotLogInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SnapshotLogInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotReference {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "max-ref-age-ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_ref_age_ms: Option<i64>,

    #[serde(rename = "max-snapshot-age-ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_snapshot_age_ms: Option<i64>,

    #[serde(rename = "min-snapshots-to-keep")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_snapshots_to_keep: Option<i32>,

}


impl SnapshotReference {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, snapshot_id: i64, ) -> SnapshotReference {
        SnapshotReference {
            r#type,
            snapshot_id,
            max_ref_age_ms: None,
            max_snapshot_age_ms: None,
            min_snapshots_to_keep: None,
        }
    }
}

/// Converts the SnapshotReference value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SnapshotReference {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            self.max_ref_age_ms.as_ref().map(|max_ref_age_ms| {
                [
                    "max-ref-age-ms".to_string(),
                    max_ref_age_ms.to_string(),
                ].join(",")
            }),


            self.max_snapshot_age_ms.as_ref().map(|max_snapshot_age_ms| {
                [
                    "max-snapshot-age-ms".to_string(),
                    max_snapshot_age_ms.to_string(),
                ].join(",")
            }),


            self.min_snapshots_to_keep.as_ref().map(|min_snapshots_to_keep| {
                [
                    "min-snapshots-to-keep".to_string(),
                    min_snapshots_to_keep.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SnapshotReference value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SnapshotReference {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub max_ref_age_ms: Vec<i64>,
            pub max_snapshot_age_ms: Vec<i64>,
            pub min_snapshots_to_keep: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SnapshotReference".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max-ref-age-ms" => intermediate_rep.max_ref_age_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max-snapshot-age-ms" => intermediate_rep.max_snapshot_age_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "min-snapshots-to-keep" => intermediate_rep.min_snapshots_to_keep.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SnapshotReference".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SnapshotReference {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in SnapshotReference".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in SnapshotReference".to_string())?,
            max_ref_age_ms: intermediate_rep.max_ref_age_ms.into_iter().next(),
            max_snapshot_age_ms: intermediate_rep.max_snapshot_age_ms.into_iter().next(),
            min_snapshots_to_keep: intermediate_rep.min_snapshots_to_keep.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SnapshotReference> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SnapshotReference>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SnapshotReference>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SnapshotReference - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SnapshotReference> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SnapshotReference as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SnapshotReference - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SnapshotSummary {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "operation")]
    pub operation: String,

}


impl SnapshotSummary {
    #[allow(clippy::new_without_default)]
    pub fn new(operation: String, ) -> SnapshotSummary {
        SnapshotSummary {
            operation,
        }
    }
}

/// Converts the SnapshotSummary value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SnapshotSummary {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("operation".to_string()),
            Some(self.operation.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SnapshotSummary value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SnapshotSummary {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub operation: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SnapshotSummary".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "operation" => intermediate_rep.operation.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SnapshotSummary".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SnapshotSummary {
            operation: intermediate_rep.operation.into_iter().next().ok_or_else(|| "operation missing in SnapshotSummary".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SnapshotSummary> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SnapshotSummary>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SnapshotSummary>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SnapshotSummary - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SnapshotSummary> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SnapshotSummary as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SnapshotSummary - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SortDirection::Asc => write!(f, "asc"),
            SortDirection::Desc => write!(f, "desc"),
        }
    }
}

impl std::str::FromStr for SortDirection {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "asc" => std::result::Result::Ok(SortDirection::Asc),
            "desc" => std::result::Result::Ok(SortDirection::Desc),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SortField {
    #[serde(rename = "source-id")]
    pub source_id: i32,

    #[serde(rename = "transform")]
    pub transform: String,

    #[serde(rename = "direction")]
    pub direction: models::SortDirection,

    #[serde(rename = "null-order")]
    pub null_order: models::NullOrder,

}


impl SortField {
    #[allow(clippy::new_without_default)]
    pub fn new(source_id: i32, transform: String, direction: models::SortDirection, null_order: models::NullOrder, ) -> SortField {
        SortField {
            source_id,
            transform,
            direction,
            null_order,
        }
    }
}

/// Converts the SortField value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SortField {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("source-id".to_string()),
            Some(self.source_id.to_string()),


            Some("transform".to_string()),
            Some(self.transform.to_string()),

            // Skipping direction in query parameter serialization

            // Skipping null-order in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SortField value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SortField {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub source_id: Vec<i32>,
            pub transform: Vec<String>,
            pub direction: Vec<models::SortDirection>,
            pub null_order: Vec<models::NullOrder>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SortField".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "source-id" => intermediate_rep.source_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "transform" => intermediate_rep.transform.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "direction" => intermediate_rep.direction.push(<models::SortDirection as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "null-order" => intermediate_rep.null_order.push(<models::NullOrder as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SortField".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SortField {
            source_id: intermediate_rep.source_id.into_iter().next().ok_or_else(|| "source-id missing in SortField".to_string())?,
            transform: intermediate_rep.transform.into_iter().next().ok_or_else(|| "transform missing in SortField".to_string())?,
            direction: intermediate_rep.direction.into_iter().next().ok_or_else(|| "direction missing in SortField".to_string())?,
            null_order: intermediate_rep.null_order.into_iter().next().ok_or_else(|| "null-order missing in SortField".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SortField> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SortField>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SortField>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SortField - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SortField> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SortField as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SortField - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SortOrder {
    #[serde(rename = "order-id")]
    pub order_id: i32,

    #[serde(rename = "fields")]
    pub fields: Vec<models::SortField>,

}


impl SortOrder {
    #[allow(clippy::new_without_default)]
    pub fn new(order_id: i32, fields: Vec<models::SortField>, ) -> SortOrder {
        SortOrder {
            order_id,
            fields,
        }
    }
}

/// Converts the SortOrder value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SortOrder {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("order-id".to_string()),
            Some(self.order_id.to_string()),

            // Skipping fields in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SortOrder value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SortOrder {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub order_id: Vec<i32>,
            pub fields: Vec<Vec<models::SortField>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SortOrder".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "order-id" => intermediate_rep.order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in SortOrder".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SortOrder".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SortOrder {
            order_id: intermediate_rep.order_id.into_iter().next().ok_or_else(|| "order-id missing in SortOrder".to_string())?,
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in SortOrder".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SortOrder> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SortOrder>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SortOrder>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SortOrder - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SortOrder> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SortOrder as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SortOrder - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SqlViewRepresentation {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "sql")]
    pub sql: String,

    #[serde(rename = "dialect")]
    pub dialect: String,

}


impl SqlViewRepresentation {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, sql: String, dialect: String, ) -> SqlViewRepresentation {
        SqlViewRepresentation {
            r#type,
            sql,
            dialect,
        }
    }
}

/// Converts the SqlViewRepresentation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SqlViewRepresentation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("sql".to_string()),
            Some(self.sql.to_string()),


            Some("dialect".to_string()),
            Some(self.dialect.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SqlViewRepresentation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SqlViewRepresentation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub sql: Vec<String>,
            pub dialect: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SqlViewRepresentation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sql" => intermediate_rep.sql.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "dialect" => intermediate_rep.dialect.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SqlViewRepresentation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SqlViewRepresentation {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in SqlViewRepresentation".to_string())?,
            sql: intermediate_rep.sql.into_iter().next().ok_or_else(|| "sql missing in SqlViewRepresentation".to_string())?,
            dialect: intermediate_rep.dialect.into_iter().next().ok_or_else(|| "dialect missing in SqlViewRepresentation".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SqlViewRepresentation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SqlViewRepresentation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SqlViewRepresentation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SqlViewRepresentation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SqlViewRepresentation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SqlViewRepresentation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SqlViewRepresentation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StatisticsFile {
    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "statistics-path")]
    pub statistics_path: String,

    #[serde(rename = "file-size-in-bytes")]
    pub file_size_in_bytes: i64,

    #[serde(rename = "file-footer-size-in-bytes")]
    pub file_footer_size_in_bytes: i64,

    #[serde(rename = "blob-metadata")]
    pub blob_metadata: Vec<models::BlobMetadata>,

}


impl StatisticsFile {
    #[allow(clippy::new_without_default)]
    pub fn new(snapshot_id: i64, statistics_path: String, file_size_in_bytes: i64, file_footer_size_in_bytes: i64, blob_metadata: Vec<models::BlobMetadata>, ) -> StatisticsFile {
        StatisticsFile {
            snapshot_id,
            statistics_path,
            file_size_in_bytes,
            file_footer_size_in_bytes,
            blob_metadata,
        }
    }
}

/// Converts the StatisticsFile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StatisticsFile {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            Some("statistics-path".to_string()),
            Some(self.statistics_path.to_string()),


            Some("file-size-in-bytes".to_string()),
            Some(self.file_size_in_bytes.to_string()),


            Some("file-footer-size-in-bytes".to_string()),
            Some(self.file_footer_size_in_bytes.to_string()),

            // Skipping blob-metadata in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StatisticsFile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StatisticsFile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub snapshot_id: Vec<i64>,
            pub statistics_path: Vec<String>,
            pub file_size_in_bytes: Vec<i64>,
            pub file_footer_size_in_bytes: Vec<i64>,
            pub blob_metadata: Vec<Vec<models::BlobMetadata>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StatisticsFile".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "statistics-path" => intermediate_rep.statistics_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-size-in-bytes" => intermediate_rep.file_size_in_bytes.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "file-footer-size-in-bytes" => intermediate_rep.file_footer_size_in_bytes.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "blob-metadata" => return std::result::Result::Err("Parsing a container in this style is not supported in StatisticsFile".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing StatisticsFile".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StatisticsFile {
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in StatisticsFile".to_string())?,
            statistics_path: intermediate_rep.statistics_path.into_iter().next().ok_or_else(|| "statistics-path missing in StatisticsFile".to_string())?,
            file_size_in_bytes: intermediate_rep.file_size_in_bytes.into_iter().next().ok_or_else(|| "file-size-in-bytes missing in StatisticsFile".to_string())?,
            file_footer_size_in_bytes: intermediate_rep.file_footer_size_in_bytes.into_iter().next().ok_or_else(|| "file-footer-size-in-bytes missing in StatisticsFile".to_string())?,
            blob_metadata: intermediate_rep.blob_metadata.into_iter().next().ok_or_else(|| "blob-metadata missing in StatisticsFile".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StatisticsFile> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StatisticsFile>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StatisticsFile>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StatisticsFile - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StatisticsFile> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StatisticsFile as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StatisticsFile - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StringTypeValue(String);

impl std::convert::From<String> for StringTypeValue {
    fn from(x: String) -> Self {
        StringTypeValue(x)
    }
}

impl std::string::ToString for StringTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for StringTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(StringTypeValue(x.to_string()))
    }
}

impl std::convert::From<StringTypeValue> for String {
    fn from(x: StringTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for StringTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for StringTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StructField {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub r#type: models::Type,

    #[serde(rename = "required")]
    pub required: bool,

    #[serde(rename = "doc")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub doc: Option<String>,

}


impl StructField {
    #[allow(clippy::new_without_default)]
    pub fn new(id: i32, name: String, r#type: models::Type, required: bool, ) -> StructField {
        StructField {
            id,
            name,
            r#type,
            required,
            doc: None,
        }
    }
}

/// Converts the StructField value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StructField {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),

            // Skipping type in query parameter serialization


            Some("required".to_string()),
            Some(self.required.to_string()),


            self.doc.as_ref().map(|doc| {
                [
                    "doc".to_string(),
                    doc.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StructField value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StructField {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub name: Vec<String>,
            pub r#type: Vec<models::Type>,
            pub required: Vec<bool>,
            pub doc: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StructField".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::Type as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "required" => intermediate_rep.required.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "doc" => intermediate_rep.doc.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing StructField".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StructField {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in StructField".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in StructField".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in StructField".to_string())?,
            required: intermediate_rep.required.into_iter().next().ok_or_else(|| "required missing in StructField".to_string())?,
            doc: intermediate_rep.doc.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StructField> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StructField>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StructField>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StructField - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StructField> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StructField as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StructField - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StructType {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "fields")]
    pub fields: Vec<models::StructField>,

}


impl StructType {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, fields: Vec<models::StructField>, ) -> StructType {
        StructType {
            r#type,
            fields,
        }
    }
}

/// Converts the StructType value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StructType {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping fields in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StructType value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StructType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub fields: Vec<Vec<models::StructField>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StructType".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in StructType".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing StructType".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StructType {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in StructType".to_string())?,
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in StructType".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StructType> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StructType>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StructType>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StructType - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StructType> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StructType as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StructType - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TableIdentifier {
    /// Reference to one or more levels of a namespace
    #[serde(rename = "namespace")]
    pub namespace: Vec<String>,

    #[serde(rename = "name")]
    pub name: String,

}


impl TableIdentifier {
    #[allow(clippy::new_without_default)]
    pub fn new(namespace: Vec<String>, name: String, ) -> TableIdentifier {
        TableIdentifier {
            namespace,
            name,
        }
    }
}

/// Converts the TableIdentifier value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TableIdentifier {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("namespace".to_string()),
            Some(self.namespace.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("name".to_string()),
            Some(self.name.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TableIdentifier value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TableIdentifier {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub namespace: Vec<Vec<String>>,
            pub name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TableIdentifier".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "namespace" => return std::result::Result::Err("Parsing a container in this style is not supported in TableIdentifier".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TableIdentifier".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TableIdentifier {
            namespace: intermediate_rep.namespace.into_iter().next().ok_or_else(|| "namespace missing in TableIdentifier".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in TableIdentifier".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TableIdentifier> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TableIdentifier>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TableIdentifier>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TableIdentifier - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TableIdentifier> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TableIdentifier as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TableIdentifier - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TableMetadata {
    #[serde(rename = "format-version")]
    #[validate(
            range(min = 1, max = 2),
        )]
    pub format_version: u8,

    #[serde(rename = "table-uuid")]
    pub table_uuid: String,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "last-updated-ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_updated_ms: Option<i64>,

    #[serde(rename = "properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<models::Schema>>,

    #[serde(rename = "current-schema-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_schema_id: Option<i32>,

    #[serde(rename = "last-column-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_column_id: Option<i32>,

    #[serde(rename = "partition-specs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition_specs: Option<Vec<models::PartitionSpec>>,

    #[serde(rename = "default-spec-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_spec_id: Option<i32>,

    #[serde(rename = "last-partition-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_partition_id: Option<i32>,

    #[serde(rename = "sort-orders")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sort_orders: Option<Vec<models::SortOrder>>,

    #[serde(rename = "default-sort-order-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_sort_order_id: Option<i32>,

    #[serde(rename = "snapshots")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshots: Option<Vec<models::Snapshot>>,

    #[serde(rename = "refs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refs: Option<std::collections::HashMap<String, models::SnapshotReference>>,

    #[serde(rename = "current-snapshot-id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_snapshot_id: Option<i64>,

    #[serde(rename = "last-sequence-number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_sequence_number: Option<i64>,

    #[serde(rename = "snapshot-log")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub snapshot_log: Option<Vec<models::SnapshotLogInner>>,

    #[serde(rename = "metadata-log")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata_log: Option<Vec<models::MetadataLogInner>>,

    #[serde(rename = "statistics-files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statistics_files: Option<Vec<models::StatisticsFile>>,

    #[serde(rename = "partition-statistics-files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub partition_statistics_files: Option<Vec<models::PartitionStatisticsFile>>,

}


impl TableMetadata {
    #[allow(clippy::new_without_default)]
    pub fn new(format_version: u8, table_uuid: String, ) -> TableMetadata {
        TableMetadata {
            format_version,
            table_uuid,
            location: None,
            last_updated_ms: None,
            properties: None,
            schemas: None,
            current_schema_id: None,
            last_column_id: None,
            partition_specs: None,
            default_spec_id: None,
            last_partition_id: None,
            sort_orders: None,
            default_sort_order_id: None,
            snapshots: None,
            refs: None,
            current_snapshot_id: None,
            last_sequence_number: None,
            snapshot_log: None,
            metadata_log: None,
            statistics_files: None,
            partition_statistics_files: None,
        }
    }
}

/// Converts the TableMetadata value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TableMetadata {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("format-version".to_string()),
            Some(self.format_version.to_string()),


            Some("table-uuid".to_string()),
            Some(self.table_uuid.to_string()),


            self.location.as_ref().map(|location| {
                [
                    "location".to_string(),
                    location.to_string(),
                ].join(",")
            }),


            self.last_updated_ms.as_ref().map(|last_updated_ms| {
                [
                    "last-updated-ms".to_string(),
                    last_updated_ms.to_string(),
                ].join(",")
            }),

            // Skipping properties in query parameter serialization

            // Skipping schemas in query parameter serialization


            self.current_schema_id.as_ref().map(|current_schema_id| {
                [
                    "current-schema-id".to_string(),
                    current_schema_id.to_string(),
                ].join(",")
            }),


            self.last_column_id.as_ref().map(|last_column_id| {
                [
                    "last-column-id".to_string(),
                    last_column_id.to_string(),
                ].join(",")
            }),

            // Skipping partition-specs in query parameter serialization


            self.default_spec_id.as_ref().map(|default_spec_id| {
                [
                    "default-spec-id".to_string(),
                    default_spec_id.to_string(),
                ].join(",")
            }),


            self.last_partition_id.as_ref().map(|last_partition_id| {
                [
                    "last-partition-id".to_string(),
                    last_partition_id.to_string(),
                ].join(",")
            }),

            // Skipping sort-orders in query parameter serialization


            self.default_sort_order_id.as_ref().map(|default_sort_order_id| {
                [
                    "default-sort-order-id".to_string(),
                    default_sort_order_id.to_string(),
                ].join(",")
            }),

            // Skipping snapshots in query parameter serialization

            // Skipping refs in query parameter serialization
            // Skipping refs in query parameter serialization


            self.current_snapshot_id.as_ref().map(|current_snapshot_id| {
                [
                    "current-snapshot-id".to_string(),
                    current_snapshot_id.to_string(),
                ].join(",")
            }),


            self.last_sequence_number.as_ref().map(|last_sequence_number| {
                [
                    "last-sequence-number".to_string(),
                    last_sequence_number.to_string(),
                ].join(",")
            }),

            // Skipping snapshot-log in query parameter serialization

            // Skipping metadata-log in query parameter serialization

            // Skipping statistics-files in query parameter serialization

            // Skipping partition-statistics-files in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TableMetadata value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TableMetadata {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub format_version: Vec<u8>,
            pub table_uuid: Vec<String>,
            pub location: Vec<String>,
            pub last_updated_ms: Vec<i64>,
            pub properties: Vec<std::collections::HashMap<String, String>>,
            pub schemas: Vec<Vec<models::Schema>>,
            pub current_schema_id: Vec<i32>,
            pub last_column_id: Vec<i32>,
            pub partition_specs: Vec<Vec<models::PartitionSpec>>,
            pub default_spec_id: Vec<i32>,
            pub last_partition_id: Vec<i32>,
            pub sort_orders: Vec<Vec<models::SortOrder>>,
            pub default_sort_order_id: Vec<i32>,
            pub snapshots: Vec<Vec<models::Snapshot>>,
            pub refs: Vec<std::collections::HashMap<String, models::SnapshotReference>>,
            pub current_snapshot_id: Vec<i64>,
            pub last_sequence_number: Vec<i64>,
            pub snapshot_log: Vec<Vec<models::SnapshotLogInner>>,
            pub metadata_log: Vec<Vec<models::MetadataLogInner>>,
            pub statistics_files: Vec<Vec<models::StatisticsFile>>,
            pub partition_statistics_files: Vec<Vec<models::PartitionStatisticsFile>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TableMetadata".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "format-version" => intermediate_rep.format_version.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "table-uuid" => intermediate_rep.table_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "location" => intermediate_rep.location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last-updated-ms" => intermediate_rep.last_updated_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "properties" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    "schemas" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "current-schema-id" => intermediate_rep.current_schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last-column-id" => intermediate_rep.last_column_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "partition-specs" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "default-spec-id" => intermediate_rep.default_spec_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last-partition-id" => intermediate_rep.last_partition_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "sort-orders" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "default-sort-order-id" => intermediate_rep.default_sort_order_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "snapshots" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    "refs" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "current-snapshot-id" => intermediate_rep.current_snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last-sequence-number" => intermediate_rep.last_sequence_number.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "snapshot-log" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    "metadata-log" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    "statistics-files" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    "partition-statistics-files" => return std::result::Result::Err("Parsing a container in this style is not supported in TableMetadata".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TableMetadata".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TableMetadata {
            format_version: intermediate_rep.format_version.into_iter().next().ok_or_else(|| "format-version missing in TableMetadata".to_string())?,
            table_uuid: intermediate_rep.table_uuid.into_iter().next().ok_or_else(|| "table-uuid missing in TableMetadata".to_string())?,
            location: intermediate_rep.location.into_iter().next(),
            last_updated_ms: intermediate_rep.last_updated_ms.into_iter().next(),
            properties: intermediate_rep.properties.into_iter().next(),
            schemas: intermediate_rep.schemas.into_iter().next(),
            current_schema_id: intermediate_rep.current_schema_id.into_iter().next(),
            last_column_id: intermediate_rep.last_column_id.into_iter().next(),
            partition_specs: intermediate_rep.partition_specs.into_iter().next(),
            default_spec_id: intermediate_rep.default_spec_id.into_iter().next(),
            last_partition_id: intermediate_rep.last_partition_id.into_iter().next(),
            sort_orders: intermediate_rep.sort_orders.into_iter().next(),
            default_sort_order_id: intermediate_rep.default_sort_order_id.into_iter().next(),
            snapshots: intermediate_rep.snapshots.into_iter().next(),
            refs: intermediate_rep.refs.into_iter().next(),
            current_snapshot_id: intermediate_rep.current_snapshot_id.into_iter().next(),
            last_sequence_number: intermediate_rep.last_sequence_number.into_iter().next(),
            snapshot_log: intermediate_rep.snapshot_log.into_iter().next(),
            metadata_log: intermediate_rep.metadata_log.into_iter().next(),
            statistics_files: intermediate_rep.statistics_files.into_iter().next(),
            partition_statistics_files: intermediate_rep.partition_statistics_files.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TableMetadata> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TableMetadata>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TableMetadata>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TableMetadata - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TableMetadata> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TableMetadata as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TableMetadata - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TableRequirement {
    #[serde(rename = "type")]
    pub r#type: String,

}


impl TableRequirement {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, ) -> TableRequirement {
        TableRequirement {
            r#type,
        }
    }
}

/// Converts the TableRequirement value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TableRequirement {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TableRequirement value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TableRequirement {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TableRequirement".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TableRequirement".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TableRequirement {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in TableRequirement".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TableRequirement> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TableRequirement>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TableRequirement>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TableRequirement - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TableRequirement> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TableRequirement as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TableRequirement - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TableUpdate {
    #[serde(rename = "action")]
    pub action: String,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "snapshot-id")]
    pub snapshot_id: i64,

    #[serde(rename = "max-ref-age-ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_ref_age_ms: Option<i64>,

    #[serde(rename = "max-snapshot-age-ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_snapshot_age_ms: Option<i64>,

    #[serde(rename = "min-snapshots-to-keep")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_snapshots_to_keep: Option<i32>,

}


impl TableUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, r#type: String, snapshot_id: i64, ) -> TableUpdate {
        TableUpdate {
            action,
            r#type,
            snapshot_id,
            max_ref_age_ms: None,
            max_snapshot_age_ms: None,
            min_snapshots_to_keep: None,
        }
    }
}

/// Converts the TableUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TableUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("snapshot-id".to_string()),
            Some(self.snapshot_id.to_string()),


            self.max_ref_age_ms.as_ref().map(|max_ref_age_ms| {
                [
                    "max-ref-age-ms".to_string(),
                    max_ref_age_ms.to_string(),
                ].join(",")
            }),


            self.max_snapshot_age_ms.as_ref().map(|max_snapshot_age_ms| {
                [
                    "max-snapshot-age-ms".to_string(),
                    max_snapshot_age_ms.to_string(),
                ].join(",")
            }),


            self.min_snapshots_to_keep.as_ref().map(|min_snapshots_to_keep| {
                [
                    "min-snapshots-to-keep".to_string(),
                    min_snapshots_to_keep.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TableUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TableUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub r#type: Vec<String>,
            pub snapshot_id: Vec<i64>,
            pub max_ref_age_ms: Vec<i64>,
            pub max_snapshot_age_ms: Vec<i64>,
            pub min_snapshots_to_keep: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TableUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "snapshot-id" => intermediate_rep.snapshot_id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max-ref-age-ms" => intermediate_rep.max_ref_age_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max-snapshot-age-ms" => intermediate_rep.max_snapshot_age_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "min-snapshots-to-keep" => intermediate_rep.min_snapshots_to_keep.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TableUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TableUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in TableUpdate".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in TableUpdate".to_string())?,
            snapshot_id: intermediate_rep.snapshot_id.into_iter().next().ok_or_else(|| "snapshot-id missing in TableUpdate".to_string())?,
            max_ref_age_ms: intermediate_rep.max_ref_age_ms.into_iter().next(),
            max_snapshot_age_ms: intermediate_rep.max_snapshot_age_ms.into_iter().next(),
            min_snapshots_to_keep: intermediate_rep.min_snapshots_to_keep.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TableUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TableUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TableUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TableUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TableUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TableUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TableUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Term {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "transform")]
    pub transform: String,

    #[serde(rename = "term")]
    pub term: String,

}


impl Term {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, transform: String, term: String, ) -> Term {
        Term {
            r#type,
            transform,
            term,
        }
    }
}

/// Converts the Term value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Term {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("transform".to_string()),
            Some(self.transform.to_string()),


            Some("term".to_string()),
            Some(self.term.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Term value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Term {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub transform: Vec<String>,
            pub term: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Term".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "transform" => intermediate_rep.transform.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "term" => intermediate_rep.term.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Term".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Term {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Term".to_string())?,
            transform: intermediate_rep.transform.into_iter().next().ok_or_else(|| "transform missing in Term".to_string())?,
            term: intermediate_rep.term.into_iter().next().ok_or_else(|| "term missing in Term".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Term> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Term>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Term>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Term - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Term> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Term as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Term - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Time type values follow the 'HH:MM:SS.ssssss' ISO-8601 format with microsecond precision
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimeTypeValue(String);

impl std::convert::From<String> for TimeTypeValue {
    fn from(x: String) -> Self {
        TimeTypeValue(x)
    }
}

impl std::string::ToString for TimeTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for TimeTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TimeTypeValue(x.to_string()))
    }
}

impl std::convert::From<TimeTypeValue> for String {
    fn from(x: TimeTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for TimeTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TimeTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimerResult {
    #[serde(rename = "time-unit")]
    pub time_unit: String,

    #[serde(rename = "count")]
    pub count: i64,

    #[serde(rename = "total-duration")]
    pub total_duration: i64,

}


impl TimerResult {
    #[allow(clippy::new_without_default)]
    pub fn new(time_unit: String, count: i64, total_duration: i64, ) -> TimerResult {
        TimerResult {
            time_unit,
            count,
            total_duration,
        }
    }
}

/// Converts the TimerResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TimerResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("time-unit".to_string()),
            Some(self.time_unit.to_string()),


            Some("count".to_string()),
            Some(self.count.to_string()),


            Some("total-duration".to_string()),
            Some(self.total_duration.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TimerResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TimerResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub time_unit: Vec<String>,
            pub count: Vec<i64>,
            pub total_duration: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TimerResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "time-unit" => intermediate_rep.time_unit.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "total-duration" => intermediate_rep.total_duration.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TimerResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TimerResult {
            time_unit: intermediate_rep.time_unit.into_iter().next().ok_or_else(|| "time-unit missing in TimerResult".to_string())?,
            count: intermediate_rep.count.into_iter().next().ok_or_else(|| "count missing in TimerResult".to_string())?,
            total_duration: intermediate_rep.total_duration.into_iter().next().ok_or_else(|| "total-duration missing in TimerResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TimerResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TimerResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TimerResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TimerResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TimerResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TimerResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TimerResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Timestamp_ns type values follow the 'YYYY-MM-DDTHH:MM:SS.sssssssss' ISO-8601 format with nanosecond precision
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimestampNanoTypeValue(String);

impl std::convert::From<String> for TimestampNanoTypeValue {
    fn from(x: String) -> Self {
        TimestampNanoTypeValue(x)
    }
}

impl std::string::ToString for TimestampNanoTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for TimestampNanoTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TimestampNanoTypeValue(x.to_string()))
    }
}

impl std::convert::From<TimestampNanoTypeValue> for String {
    fn from(x: TimestampNanoTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for TimestampNanoTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TimestampNanoTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Timestamp type values follow the 'YYYY-MM-DDTHH:MM:SS.ssssss' ISO-8601 format with microsecond precision
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimestampTypeValue(String);

impl std::convert::From<String> for TimestampTypeValue {
    fn from(x: String) -> Self {
        TimestampTypeValue(x)
    }
}

impl std::string::ToString for TimestampTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for TimestampTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TimestampTypeValue(x.to_string()))
    }
}

impl std::convert::From<TimestampTypeValue> for String {
    fn from(x: TimestampTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for TimestampTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TimestampTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Timestamp_ns type values follow the 'YYYY-MM-DDTHH:MM:SS.sssssssss+00:00' ISO-8601 format with nanosecond precision, and a timezone offset (+00:00 for UTC)
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimestampTzNanoTypeValue(String);

impl std::convert::From<String> for TimestampTzNanoTypeValue {
    fn from(x: String) -> Self {
        TimestampTzNanoTypeValue(x)
    }
}

impl std::string::ToString for TimestampTzNanoTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for TimestampTzNanoTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TimestampTzNanoTypeValue(x.to_string()))
    }
}

impl std::convert::From<TimestampTzNanoTypeValue> for String {
    fn from(x: TimestampTzNanoTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for TimestampTzNanoTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TimestampTzNanoTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// TimestampTz type values follow the 'YYYY-MM-DDTHH:MM:SS.ssssss+00:00' ISO-8601 format with microsecond precision, and a timezone offset (+00:00 for UTC)
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimestampTzTypeValue(String);

impl std::convert::From<String> for TimestampTzTypeValue {
    fn from(x: String) -> Self {
        TimestampTzTypeValue(x)
    }
}

impl std::string::ToString for TimestampTzTypeValue {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for TimestampTzTypeValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TimestampTzTypeValue(x.to_string()))
    }
}

impl std::convert::From<TimestampTzTypeValue> for String {
    fn from(x: TimestampTzTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for TimestampTzTypeValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TimestampTzTypeValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Token type identifier, from RFC 8693 Section 3  See https://datatracker.ietf.org/doc/html/rfc8693#section-3
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TokenType {
    #[serde(rename = "urn:ietf:params:oauth:token-type:access_token")]
    AccessToken,
    #[serde(rename = "urn:ietf:params:oauth:token-type:refresh_token")]
    RefreshToken,
    #[serde(rename = "urn:ietf:params:oauth:token-type:id_token")]
    IdToken,
    #[serde(rename = "urn:ietf:params:oauth:token-type:saml1")]
    Saml1,
    #[serde(rename = "urn:ietf:params:oauth:token-type:saml2")]
    Saml2,
    #[serde(rename = "urn:ietf:params:oauth:token-type:jwt")]
    Jwt,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenType::AccessToken => write!(f, "urn:ietf:params:oauth:token-type:access_token"),
            TokenType::RefreshToken => write!(f, "urn:ietf:params:oauth:token-type:refresh_token"),
            TokenType::IdToken => write!(f, "urn:ietf:params:oauth:token-type:id_token"),
            TokenType::Saml1 => write!(f, "urn:ietf:params:oauth:token-type:saml1"),
            TokenType::Saml2 => write!(f, "urn:ietf:params:oauth:token-type:saml2"),
            TokenType::Jwt => write!(f, "urn:ietf:params:oauth:token-type:jwt"),
        }
    }
}

impl std::str::FromStr for TokenType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "urn:ietf:params:oauth:token-type:access_token" => std::result::Result::Ok(TokenType::AccessToken),
            "urn:ietf:params:oauth:token-type:refresh_token" => std::result::Result::Ok(TokenType::RefreshToken),
            "urn:ietf:params:oauth:token-type:id_token" => std::result::Result::Ok(TokenType::IdToken),
            "urn:ietf:params:oauth:token-type:saml1" => std::result::Result::Ok(TokenType::Saml1),
            "urn:ietf:params:oauth:token-type:saml2" => std::result::Result::Ok(TokenType::Saml2),
            "urn:ietf:params:oauth:token-type:jwt" => std::result::Result::Ok(TokenType::Jwt),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Transform(String);

impl std::convert::From<String> for Transform {
    fn from(x: String) -> Self {
        Transform(x)
    }
}

impl std::string::ToString for Transform {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Transform {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Transform(x.to_string()))
    }
}

impl std::convert::From<Transform> for String {
    fn from(x: Transform) -> Self {
        x.0
    }
}

impl std::ops::Deref for Transform {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Transform {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TransformTerm {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "transform")]
    pub transform: String,

    #[serde(rename = "term")]
    pub term: String,

}


impl TransformTerm {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, transform: String, term: String, ) -> TransformTerm {
        TransformTerm {
            r#type,
            transform,
            term,
        }
    }
}

/// Converts the TransformTerm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TransformTerm {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("transform".to_string()),
            Some(self.transform.to_string()),


            Some("term".to_string()),
            Some(self.term.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TransformTerm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TransformTerm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub transform: Vec<String>,
            pub term: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TransformTerm".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "transform" => intermediate_rep.transform.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "term" => intermediate_rep.term.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TransformTerm".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TransformTerm {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in TransformTerm".to_string())?,
            transform: intermediate_rep.transform.into_iter().next().ok_or_else(|| "transform missing in TransformTerm".to_string())?,
            term: intermediate_rep.term.into_iter().next().ok_or_else(|| "term missing in TransformTerm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TransformTerm> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TransformTerm>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TransformTerm>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TransformTerm - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TransformTerm> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TransformTerm as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TransformTerm - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Type {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "fields")]
    pub fields: Vec<models::StructField>,

    #[serde(rename = "element-id")]
    pub element_id: i32,

    #[serde(rename = "element")]
    pub element: models::Type,

    #[serde(rename = "element-required")]
    pub element_required: bool,

    #[serde(rename = "key-id")]
    pub key_id: i32,

    #[serde(rename = "key")]
    pub key: models::Type,

    #[serde(rename = "value-id")]
    pub value_id: i32,

    #[serde(rename = "value")]
    pub value: models::Type,

    #[serde(rename = "value-required")]
    pub value_required: bool,

}


impl Type {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, fields: Vec<models::StructField>, element_id: i32, element: models::Type, element_required: bool, key_id: i32, key: models::Type, value_id: i32, value: models::Type, value_required: bool, ) -> Type {
        Type {
            r#type,
            fields,
            element_id,
            element,
            element_required,
            key_id,
            key,
            value_id,
            value,
            value_required,
        }
    }
}

/// Converts the Type value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Type {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping fields in query parameter serialization


            Some("element-id".to_string()),
            Some(self.element_id.to_string()),

            // Skipping element in query parameter serialization


            Some("element-required".to_string()),
            Some(self.element_required.to_string()),


            Some("key-id".to_string()),
            Some(self.key_id.to_string()),

            // Skipping key in query parameter serialization


            Some("value-id".to_string()),
            Some(self.value_id.to_string()),

            // Skipping value in query parameter serialization


            Some("value-required".to_string()),
            Some(self.value_required.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Type value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub fields: Vec<Vec<models::StructField>>,
            pub element_id: Vec<i32>,
            pub element: Vec<models::Type>,
            pub element_required: Vec<bool>,
            pub key_id: Vec<i32>,
            pub key: Vec<models::Type>,
            pub value_id: Vec<i32>,
            pub value: Vec<models::Type>,
            pub value_required: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Type".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in Type".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "element-id" => intermediate_rep.element_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "element" => intermediate_rep.element.push(<models::Type as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "element-required" => intermediate_rep.element_required.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key-id" => intermediate_rep.key_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<models::Type as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value-id" => intermediate_rep.value_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<models::Type as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value-required" => intermediate_rep.value_required.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Type".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Type {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Type".to_string())?,
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in Type".to_string())?,
            element_id: intermediate_rep.element_id.into_iter().next().ok_or_else(|| "element-id missing in Type".to_string())?,
            element: intermediate_rep.element.into_iter().next().ok_or_else(|| "element missing in Type".to_string())?,
            element_required: intermediate_rep.element_required.into_iter().next().ok_or_else(|| "element-required missing in Type".to_string())?,
            key_id: intermediate_rep.key_id.into_iter().next().ok_or_else(|| "key-id missing in Type".to_string())?,
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in Type".to_string())?,
            value_id: intermediate_rep.value_id.into_iter().next().ok_or_else(|| "value-id missing in Type".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in Type".to_string())?,
            value_required: intermediate_rep.value_required.into_iter().next().ok_or_else(|| "value-required missing in Type".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Type> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Type>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Type>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Type - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Type> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Type as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Type - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnaryExpression {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "term")]
    pub term: models::Term,

    #[serde(rename = "value")]
    pub value: serde_json::Value,

}


impl UnaryExpression {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, term: models::Term, value: serde_json::Value, ) -> UnaryExpression {
        UnaryExpression {
            r#type,
            term,
            value,
        }
    }
}

/// Converts the UnaryExpression value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UnaryExpression {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping term in query parameter serialization

            // Skipping value in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnaryExpression value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnaryExpression {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub term: Vec<models::Term>,
            pub value: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UnaryExpression".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "term" => intermediate_rep.term.push(<models::Term as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UnaryExpression".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnaryExpression {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in UnaryExpression".to_string())?,
            term: intermediate_rep.term.into_iter().next().ok_or_else(|| "term missing in UnaryExpression".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in UnaryExpression".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnaryExpression> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UnaryExpression>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UnaryExpression>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UnaryExpression - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UnaryExpression> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UnaryExpression as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UnaryExpression - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNamespacePropertiesRequest {
    #[serde(rename = "removals")]
    #[validate(
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub removals: Option<Vec<String>>,

    #[serde(rename = "updates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updates: Option<std::collections::HashMap<String, String>>,

}


impl UpdateNamespacePropertiesRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateNamespacePropertiesRequest {
        UpdateNamespacePropertiesRequest {
            removals: None,
            updates: None,
        }
    }
}

/// Converts the UpdateNamespacePropertiesRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateNamespacePropertiesRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.removals.as_ref().map(|removals| {
                [
                    "removals".to_string(),
                    removals.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping updates in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateNamespacePropertiesRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateNamespacePropertiesRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub removals: Vec<Vec<String>>,
            pub updates: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateNamespacePropertiesRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "removals" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateNamespacePropertiesRequest".to_string()),
                    "updates" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateNamespacePropertiesRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateNamespacePropertiesRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateNamespacePropertiesRequest {
            removals: intermediate_rep.removals.into_iter().next(),
            updates: intermediate_rep.updates.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateNamespacePropertiesRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateNamespacePropertiesRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateNamespacePropertiesRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateNamespacePropertiesRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateNamespacePropertiesRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateNamespacePropertiesRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateNamespacePropertiesRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateNamespacePropertiesResponse {
    /// List of property keys that were added or updated
    #[serde(rename = "updated")]
    #[validate(
        )]
    pub updated: Vec<String>,

    /// List of properties that were removed
    #[serde(rename = "removed")]
    pub removed: Vec<String>,

    /// List of properties requested for removal that were not found in the namespace's properties. Represents a partial success response. Server's do not need to implement this.
    #[serde(rename = "missing")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub missing: Option<swagger::Nullable<Vec<String>>>,

}


impl UpdateNamespacePropertiesResponse {
    #[allow(clippy::new_without_default)]
    pub fn new(updated: Vec<String>, removed: Vec<String>, ) -> UpdateNamespacePropertiesResponse {
        UpdateNamespacePropertiesResponse {
            updated,
            removed,
            missing: None,
        }
    }
}

/// Converts the UpdateNamespacePropertiesResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateNamespacePropertiesResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("updated".to_string()),
            Some(self.updated.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("removed".to_string()),
            Some(self.removed.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            self.missing.as_ref().map(|missing| {
                [
                    "missing".to_string(),
                    missing.as_ref().map_or("null".to_string(), |x| x.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateNamespacePropertiesResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateNamespacePropertiesResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub updated: Vec<Vec<String>>,
            pub removed: Vec<Vec<String>>,
            pub missing: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateNamespacePropertiesResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "updated" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateNamespacePropertiesResponse".to_string()),
                    "removed" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateNamespacePropertiesResponse".to_string()),
                    "missing" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateNamespacePropertiesResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateNamespacePropertiesResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateNamespacePropertiesResponse {
            updated: intermediate_rep.updated.into_iter().next().ok_or_else(|| "updated missing in UpdateNamespacePropertiesResponse".to_string())?,
            removed: intermediate_rep.removed.into_iter().next().ok_or_else(|| "removed missing in UpdateNamespacePropertiesResponse".to_string())?,
            missing: std::result::Result::Err("Nullable types not supported in UpdateNamespacePropertiesResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateNamespacePropertiesResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateNamespacePropertiesResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateNamespacePropertiesResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateNamespacePropertiesResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateNamespacePropertiesResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateNamespacePropertiesResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateNamespacePropertiesResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpgradeFormatVersionUpdate {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "format-version")]
    pub format_version: i32,

}


impl UpgradeFormatVersionUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, format_version: i32, ) -> UpgradeFormatVersionUpdate {
        UpgradeFormatVersionUpdate {
            action,
            format_version,
        }
    }
}

/// Converts the UpgradeFormatVersionUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpgradeFormatVersionUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),


            Some("format-version".to_string()),
            Some(self.format_version.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpgradeFormatVersionUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpgradeFormatVersionUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
            pub format_version: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpgradeFormatVersionUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "format-version" => intermediate_rep.format_version.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpgradeFormatVersionUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpgradeFormatVersionUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in UpgradeFormatVersionUpdate".to_string())?,
            format_version: intermediate_rep.format_version.into_iter().next().ok_or_else(|| "format-version missing in UpgradeFormatVersionUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpgradeFormatVersionUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpgradeFormatVersionUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpgradeFormatVersionUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpgradeFormatVersionUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpgradeFormatVersionUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpgradeFormatVersionUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpgradeFormatVersionUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// UUID type values are serialized as a 36-character lowercase string in standard UUID format as specified by RFC-4122
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UuidTypeValue(uuid::Uuid);

impl std::convert::From<uuid::Uuid> for UuidTypeValue {
    fn from(x: uuid::Uuid) -> Self {
        UuidTypeValue(x)
    }
}

impl std::convert::From<UuidTypeValue> for uuid::Uuid {
    fn from(x: UuidTypeValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for UuidTypeValue {
    type Target = uuid::Uuid;
    fn deref(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl std::ops::DerefMut for UuidTypeValue {
    fn deref_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ValueMap {
    /// List of integer column ids for each corresponding value
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys: Option<Vec<models::IntegerTypeValue>>,

    /// List of primitive type values, matched to 'keys' by index
    #[serde(rename = "values")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub values: Option<Vec<models::PrimitiveTypeValue>>,

}


impl ValueMap {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ValueMap {
        ValueMap {
            keys: None,
            values: None,
        }
    }
}

/// Converts the ValueMap value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ValueMap {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.keys.as_ref().map(|keys| {
                [
                    "keys".to_string(),
                    keys.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping values in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ValueMap value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ValueMap {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub keys: Vec<Vec<models::IntegerTypeValue>>,
            pub values: Vec<Vec<models::PrimitiveTypeValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ValueMap".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "keys" => return std::result::Result::Err("Parsing a container in this style is not supported in ValueMap".to_string()),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in ValueMap".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ValueMap".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ValueMap {
            keys: intermediate_rep.keys.into_iter().next(),
            values: intermediate_rep.values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ValueMap> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ValueMap>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ValueMap>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ValueMap - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ValueMap> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ValueMap as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ValueMap - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ViewHistoryEntry {
    #[serde(rename = "version-id")]
    pub version_id: i32,

    #[serde(rename = "timestamp-ms")]
    pub timestamp_ms: i64,

}


impl ViewHistoryEntry {
    #[allow(clippy::new_without_default)]
    pub fn new(version_id: i32, timestamp_ms: i64, ) -> ViewHistoryEntry {
        ViewHistoryEntry {
            version_id,
            timestamp_ms,
        }
    }
}

/// Converts the ViewHistoryEntry value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ViewHistoryEntry {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("version-id".to_string()),
            Some(self.version_id.to_string()),


            Some("timestamp-ms".to_string()),
            Some(self.timestamp_ms.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ViewHistoryEntry value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ViewHistoryEntry {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub version_id: Vec<i32>,
            pub timestamp_ms: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ViewHistoryEntry".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "version-id" => intermediate_rep.version_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp-ms" => intermediate_rep.timestamp_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ViewHistoryEntry".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ViewHistoryEntry {
            version_id: intermediate_rep.version_id.into_iter().next().ok_or_else(|| "version-id missing in ViewHistoryEntry".to_string())?,
            timestamp_ms: intermediate_rep.timestamp_ms.into_iter().next().ok_or_else(|| "timestamp-ms missing in ViewHistoryEntry".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ViewHistoryEntry> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ViewHistoryEntry>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ViewHistoryEntry>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ViewHistoryEntry - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ViewHistoryEntry> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ViewHistoryEntry as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ViewHistoryEntry - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ViewMetadata {
    #[serde(rename = "view-uuid")]
    pub view_uuid: String,

    #[serde(rename = "format-version")]
    #[validate(
            range(min = 1, max = 1),
        )]
    pub format_version: u8,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "current-version-id")]
    pub current_version_id: i32,

    #[serde(rename = "versions")]
    pub versions: Vec<models::ViewVersion>,

    #[serde(rename = "version-log")]
    pub version_log: Vec<models::ViewHistoryEntry>,

    #[serde(rename = "schemas")]
    pub schemas: Vec<models::Schema>,

    #[serde(rename = "properties")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,

}


impl ViewMetadata {
    #[allow(clippy::new_without_default)]
    pub fn new(view_uuid: String, format_version: u8, location: String, current_version_id: i32, versions: Vec<models::ViewVersion>, version_log: Vec<models::ViewHistoryEntry>, schemas: Vec<models::Schema>, ) -> ViewMetadata {
        ViewMetadata {
            view_uuid,
            format_version,
            location,
            current_version_id,
            versions,
            version_log,
            schemas,
            properties: None,
        }
    }
}

/// Converts the ViewMetadata value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ViewMetadata {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("view-uuid".to_string()),
            Some(self.view_uuid.to_string()),


            Some("format-version".to_string()),
            Some(self.format_version.to_string()),


            Some("location".to_string()),
            Some(self.location.to_string()),


            Some("current-version-id".to_string()),
            Some(self.current_version_id.to_string()),

            // Skipping versions in query parameter serialization

            // Skipping version-log in query parameter serialization

            // Skipping schemas in query parameter serialization

            // Skipping properties in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ViewMetadata value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ViewMetadata {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub view_uuid: Vec<String>,
            pub format_version: Vec<u8>,
            pub location: Vec<String>,
            pub current_version_id: Vec<i32>,
            pub versions: Vec<Vec<models::ViewVersion>>,
            pub version_log: Vec<Vec<models::ViewHistoryEntry>>,
            pub schemas: Vec<Vec<models::Schema>>,
            pub properties: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ViewMetadata".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "view-uuid" => intermediate_rep.view_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "format-version" => intermediate_rep.format_version.push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "location" => intermediate_rep.location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "current-version-id" => intermediate_rep.current_version_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "versions" => return std::result::Result::Err("Parsing a container in this style is not supported in ViewMetadata".to_string()),
                    "version-log" => return std::result::Result::Err("Parsing a container in this style is not supported in ViewMetadata".to_string()),
                    "schemas" => return std::result::Result::Err("Parsing a container in this style is not supported in ViewMetadata".to_string()),
                    "properties" => return std::result::Result::Err("Parsing a container in this style is not supported in ViewMetadata".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ViewMetadata".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ViewMetadata {
            view_uuid: intermediate_rep.view_uuid.into_iter().next().ok_or_else(|| "view-uuid missing in ViewMetadata".to_string())?,
            format_version: intermediate_rep.format_version.into_iter().next().ok_or_else(|| "format-version missing in ViewMetadata".to_string())?,
            location: intermediate_rep.location.into_iter().next().ok_or_else(|| "location missing in ViewMetadata".to_string())?,
            current_version_id: intermediate_rep.current_version_id.into_iter().next().ok_or_else(|| "current-version-id missing in ViewMetadata".to_string())?,
            versions: intermediate_rep.versions.into_iter().next().ok_or_else(|| "versions missing in ViewMetadata".to_string())?,
            version_log: intermediate_rep.version_log.into_iter().next().ok_or_else(|| "version-log missing in ViewMetadata".to_string())?,
            schemas: intermediate_rep.schemas.into_iter().next().ok_or_else(|| "schemas missing in ViewMetadata".to_string())?,
            properties: intermediate_rep.properties.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ViewMetadata> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ViewMetadata>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ViewMetadata>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ViewMetadata - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ViewMetadata> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ViewMetadata as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ViewMetadata - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ViewRepresentation {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "sql")]
    pub sql: String,

    #[serde(rename = "dialect")]
    pub dialect: String,

}


impl ViewRepresentation {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, sql: String, dialect: String, ) -> ViewRepresentation {
        ViewRepresentation {
            r#type,
            sql,
            dialect,
        }
    }
}

/// Converts the ViewRepresentation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ViewRepresentation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),


            Some("sql".to_string()),
            Some(self.sql.to_string()),


            Some("dialect".to_string()),
            Some(self.dialect.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ViewRepresentation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ViewRepresentation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub sql: Vec<String>,
            pub dialect: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ViewRepresentation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sql" => intermediate_rep.sql.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "dialect" => intermediate_rep.dialect.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ViewRepresentation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ViewRepresentation {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ViewRepresentation".to_string())?,
            sql: intermediate_rep.sql.into_iter().next().ok_or_else(|| "sql missing in ViewRepresentation".to_string())?,
            dialect: intermediate_rep.dialect.into_iter().next().ok_or_else(|| "dialect missing in ViewRepresentation".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ViewRepresentation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ViewRepresentation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ViewRepresentation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ViewRepresentation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ViewRepresentation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ViewRepresentation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ViewRepresentation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ViewRequirement {
    #[serde(rename = "type")]
    pub r#type: String,

}


impl ViewRequirement {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, ) -> ViewRequirement {
        ViewRequirement {
            r#type,
        }
    }
}

/// Converts the ViewRequirement value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ViewRequirement {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ViewRequirement value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ViewRequirement {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ViewRequirement".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ViewRequirement".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ViewRequirement {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ViewRequirement".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ViewRequirement> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ViewRequirement>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ViewRequirement>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ViewRequirement - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ViewRequirement> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ViewRequirement as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ViewRequirement - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ViewUpdate {
    #[serde(rename = "action")]
    pub action: String,

}


impl ViewUpdate {
    #[allow(clippy::new_without_default)]
    pub fn new(action: String, ) -> ViewUpdate {
        ViewUpdate {
            action,
        }
    }
}

/// Converts the ViewUpdate value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ViewUpdate {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("action".to_string()),
            Some(self.action.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ViewUpdate value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ViewUpdate {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ViewUpdate".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ViewUpdate".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ViewUpdate {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in ViewUpdate".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ViewUpdate> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ViewUpdate>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ViewUpdate>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ViewUpdate - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ViewUpdate> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ViewUpdate as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ViewUpdate - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ViewVersion {
    #[serde(rename = "version-id")]
    pub version_id: i32,

    #[serde(rename = "timestamp-ms")]
    pub timestamp_ms: i64,

    /// Schema ID to set as current, or -1 to set last added schema
    #[serde(rename = "schema-id")]
    pub schema_id: i32,

    #[serde(rename = "summary")]
    pub summary: std::collections::HashMap<String, String>,

    #[serde(rename = "representations")]
    pub representations: Vec<models::ViewRepresentation>,

    #[serde(rename = "default-catalog")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_catalog: Option<String>,

    /// Reference to one or more levels of a namespace
    #[serde(rename = "default-namespace")]
    pub default_namespace: Vec<String>,

}


impl ViewVersion {
    #[allow(clippy::new_without_default)]
    pub fn new(version_id: i32, timestamp_ms: i64, schema_id: i32, summary: std::collections::HashMap<String, String>, representations: Vec<models::ViewRepresentation>, default_namespace: Vec<String>, ) -> ViewVersion {
        ViewVersion {
            version_id,
            timestamp_ms,
            schema_id,
            summary,
            representations,
            default_catalog: None,
            default_namespace,
        }
    }
}

/// Converts the ViewVersion value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ViewVersion {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("version-id".to_string()),
            Some(self.version_id.to_string()),


            Some("timestamp-ms".to_string()),
            Some(self.timestamp_ms.to_string()),


            Some("schema-id".to_string()),
            Some(self.schema_id.to_string()),

            // Skipping summary in query parameter serialization

            // Skipping representations in query parameter serialization


            self.default_catalog.as_ref().map(|default_catalog| {
                [
                    "default-catalog".to_string(),
                    default_catalog.to_string(),
                ].join(",")
            }),


            Some("default-namespace".to_string()),
            Some(self.default_namespace.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ViewVersion value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ViewVersion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub version_id: Vec<i32>,
            pub timestamp_ms: Vec<i64>,
            pub schema_id: Vec<i32>,
            pub summary: Vec<std::collections::HashMap<String, String>>,
            pub representations: Vec<Vec<models::ViewRepresentation>>,
            pub default_catalog: Vec<String>,
            pub default_namespace: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ViewVersion".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "version-id" => intermediate_rep.version_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp-ms" => intermediate_rep.timestamp_ms.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "schema-id" => intermediate_rep.schema_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "summary" => return std::result::Result::Err("Parsing a container in this style is not supported in ViewVersion".to_string()),
                    "representations" => return std::result::Result::Err("Parsing a container in this style is not supported in ViewVersion".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "default-catalog" => intermediate_rep.default_catalog.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "default-namespace" => return std::result::Result::Err("Parsing a container in this style is not supported in ViewVersion".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ViewVersion".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ViewVersion {
            version_id: intermediate_rep.version_id.into_iter().next().ok_or_else(|| "version-id missing in ViewVersion".to_string())?,
            timestamp_ms: intermediate_rep.timestamp_ms.into_iter().next().ok_or_else(|| "timestamp-ms missing in ViewVersion".to_string())?,
            schema_id: intermediate_rep.schema_id.into_iter().next().ok_or_else(|| "schema-id missing in ViewVersion".to_string())?,
            summary: intermediate_rep.summary.into_iter().next().ok_or_else(|| "summary missing in ViewVersion".to_string())?,
            representations: intermediate_rep.representations.into_iter().next().ok_or_else(|| "representations missing in ViewVersion".to_string())?,
            default_catalog: intermediate_rep.default_catalog.into_iter().next(),
            default_namespace: intermediate_rep.default_namespace.into_iter().next().ok_or_else(|| "default-namespace missing in ViewVersion".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ViewVersion> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ViewVersion>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ViewVersion>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ViewVersion - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ViewVersion> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ViewVersion as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ViewVersion - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

