// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SubstanceAmountAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
impl Default for SubstanceAmountAmount {
    fn default() -> SubstanceAmountAmount {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceAmountReferenceRange {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#low_limit: Option<Box<super::super::types::Quantity>>,
    pub r#high_limit: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for SubstanceAmountReferenceRange {
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
        if let Some(some) = self.r#low_limit.as_ref() {
            state.serialize_entry("lowLimit", some)?;
        }
        if let Some(some) = self.r#high_limit.as_ref() {
            state.serialize_entry("highLimit", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceAmountReferenceRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceAmountReferenceRange;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceAmountReferenceRange")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceAmountReferenceRange, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#low_limit: Option<Box<super::super::types::Quantity>> = None;
                let mut r#high_limit: Option<Box<super::super::types::Quantity>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "lowLimit" => {
                            if r#low_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("lowLimit"));
                            }
                            r#low_limit = Some(map_access.next_value()?);
                        }
                        "highLimit" => {
                            if r#high_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("highLimit"));
                            }
                            r#high_limit = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "low_limit", "high_limit"],
                            ))
                        }
                    }
                }
                Ok(SubstanceAmountReferenceRange {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#low_limit,
                    r#high_limit,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceAmount {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: Option<SubstanceAmountAmount>,
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount_text: Option<super::super::types::String>,
    pub r#reference_range: Option<Box<super::super::types::Element>>,
}
impl serde::ser::Serialize for SubstanceAmount {
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
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#amount.as_ref() {
            match some {
                SubstanceAmountAmount::Quantity(ref value) => {
                    state.serialize_entry("amountQuantity", value)?;
                }
                SubstanceAmountAmount::Range(ref value) => {
                    state.serialize_entry("amountRange", value)?;
                }
                SubstanceAmountAmount::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("amountString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_amountString", &primitive_element)?;
                    }
                }
            }
        }
        if let Some(some) = self.r#amount_type.as_ref() {
            state.serialize_entry("amountType", some)?;
        }
        if let Some(some) = self.r#amount_text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("amountText", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_amountText", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#reference_range.as_ref() {
            state.serialize_entry("referenceRange", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceAmount;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceAmount")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceAmount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#amount: Option<SubstanceAmountAmount> = None;
                let mut r#amount_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount_text: Option<super::super::types::String> = None;
                let mut r#reference_range: Option<Box<super::super::types::Element>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "amountQuantity" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountQuantity"));
                            }
                            r#amount =
                                Some(SubstanceAmountAmount::Quantity(map_access.next_value()?));
                        }
                        "amountRange" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountRange"));
                            }
                            r#amount = Some(SubstanceAmountAmount::Range(map_access.next_value()?));
                        }
                        "amountString" => {
                            let r#enum = r#amount
                                .get_or_insert(SubstanceAmountAmount::String(Default::default()));
                            if let SubstanceAmountAmount::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("amount[x]"));
                            }
                        }
                        "_amountString" => {
                            let r#enum = r#amount
                                .get_or_insert(SubstanceAmountAmount::String(Default::default()));
                            if let SubstanceAmountAmount::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_amountString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_amount[x]"));
                            }
                        }
                        "amountType" => {
                            if r#amount_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountType"));
                            }
                            r#amount_type = Some(map_access.next_value()?);
                        }
                        "amountText" => {
                            let some = r#amount_text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountText"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_amountText" => {
                            let some = r#amount_text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_amountText"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "referenceRange" => {
                            if r#reference_range.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceRange"));
                            }
                            r#reference_range = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "amount",
                                    "amount_type",
                                    "amount_text",
                                    "reference_range",
                                ],
                            ))
                        }
                    }
                }
                Ok(SubstanceAmount {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#amount,
                    r#amount_type,
                    r#amount_text,
                    r#reference_range,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
