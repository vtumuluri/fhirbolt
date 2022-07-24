// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum UsageContextValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for UsageContextValue {
    fn default() -> UsageContextValue {
        UsageContextValue::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct UsageContext {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::Coding>,
    pub r#value: UsageContextValue,
}
impl serde::ser::Serialize for UsageContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        state.serialize_entry("code", &self.r#code)?;
        match self.r#value {
            UsageContextValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            UsageContextValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            UsageContextValue::Range(ref value) => {
                state.serialize_entry("valueRange", value)?;
            }
            UsageContextValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            UsageContextValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for UsageContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "valueReference")]
            ValueReference,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UsageContext;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("UsageContext")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<UsageContext, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#code: Option<Box<super::super::types::Coding>> = None;
                let mut r#value: Option<UsageContextValue> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value =
                                Some(UsageContextValue::CodeableConcept(map_access.next_value()?));
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(UsageContextValue::Quantity(map_access.next_value()?));
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(UsageContextValue::Range(map_access.next_value()?));
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(UsageContextValue::Reference(map_access.next_value()?));
                        }
                    }
                }
                Ok(UsageContext {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#value: r#value.ok_or(serde::de::Error::missing_field("value[x]"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
