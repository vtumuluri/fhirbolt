// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ObservationEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Instant(Box<super::super::types::Instant>),
}
impl Default for ObservationEffective {
    fn default() -> ObservationEffective {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ObservationValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
impl Default for ObservationValue {
    fn default() -> ObservationValue {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ObservationComponentValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
impl Default for ObservationComponentValue {
    fn default() -> ObservationComponentValue {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ObservationReferenceRange {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#low: Option<Box<super::super::types::Quantity>>,
    pub r#high: Option<Box<super::super::types::Quantity>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#applies_to: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#age: Option<Box<super::super::types::Range>>,
    pub r#text: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ObservationReferenceRange {
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
        if let Some(some) = self.r#low.as_ref() {
            state.serialize_entry("low", some)?;
        }
        if let Some(some) = self.r#high.as_ref() {
            state.serialize_entry("high", some)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if !self.r#applies_to.is_empty() {
            state.serialize_entry("appliesTo", &self.r#applies_to)?;
        }
        if let Some(some) = self.r#age.as_ref() {
            state.serialize_entry("age", some)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationReferenceRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObservationReferenceRange;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationReferenceRange")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ObservationReferenceRange, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#low: Option<Box<super::super::types::Quantity>> = None;
                let mut r#high: Option<Box<super::super::types::Quantity>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#applies_to: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#age: Option<Box<super::super::types::Range>> = None;
                let mut r#text: Option<super::super::types::String> = None;
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
                        "low" => {
                            if r#low.is_some() {
                                return Err(serde::de::Error::duplicate_field("low"));
                            }
                            r#low = Some(map_access.next_value()?);
                        }
                        "high" => {
                            if r#high.is_some() {
                                return Err(serde::de::Error::duplicate_field("high"));
                            }
                            r#high = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "appliesTo" => {
                            if r#applies_to.is_some() {
                                return Err(serde::de::Error::duplicate_field("appliesTo"));
                            }
                            r#applies_to = Some(map_access.next_value()?);
                        }
                        "age" => {
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("age"));
                            }
                            r#age = Some(map_access.next_value()?);
                        }
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "low",
                                    "high",
                                    "type",
                                    "applies_to",
                                    "age",
                                    "text",
                                ],
                            ))
                        }
                    }
                }
                Ok(ObservationReferenceRange {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#low,
                    r#high,
                    r#type,
                    r#applies_to: r#applies_to.unwrap_or(vec![]),
                    r#age,
                    r#text,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ObservationComponent {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#value: Option<ObservationComponentValue>,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reference_range: Vec<ObservationReferenceRange>,
}
impl serde::ser::Serialize for ObservationComponent {
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
        state.serialize_entry("code", &self.r#code)?;
        if let Some(some) = self.r#value.as_ref() {
            match some {
                ObservationComponentValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ObservationComponentValue::CodeableConcept(ref value) => {
                    state.serialize_entry("valueCodeableConcept", value)?;
                }
                ObservationComponentValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                ObservationComponentValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBoolean", &primitive_element)?;
                    }
                }
                ObservationComponentValue::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueInteger", &primitive_element)?;
                    }
                }
                ObservationComponentValue::Range(ref value) => {
                    state.serialize_entry("valueRange", value)?;
                }
                ObservationComponentValue::Ratio(ref value) => {
                    state.serialize_entry("valueRatio", value)?;
                }
                ObservationComponentValue::SampledData(ref value) => {
                    state.serialize_entry("valueSampledData", value)?;
                }
                ObservationComponentValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueTime", &primitive_element)?;
                    }
                }
                ObservationComponentValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDateTime", &primitive_element)?;
                    }
                }
                ObservationComponentValue::Period(ref value) => {
                    state.serialize_entry("valuePeriod", value)?;
                }
            }
        }
        if let Some(some) = self.r#data_absent_reason.as_ref() {
            state.serialize_entry("dataAbsentReason", some)?;
        }
        if !self.r#interpretation.is_empty() {
            state.serialize_entry("interpretation", &self.r#interpretation)?;
        }
        if !self.r#reference_range.is_empty() {
            state.serialize_entry("referenceRange", &self.r#reference_range)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObservationComponent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationComponent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ObservationComponent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<ObservationComponentValue> = None;
                let mut r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#interpretation: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reference_range: Option<Vec<ObservationReferenceRange>> = None;
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
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "valueQuantity" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(ObservationComponentValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        "valueCodeableConcept" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some(ObservationComponentValue::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "valueString" => {
                            let r#enum = r#value.get_or_insert(ObservationComponentValue::String(
                                Default::default(),
                            ));
                            if let ObservationComponentValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueString" => {
                            let r#enum = r#value.get_or_insert(ObservationComponentValue::String(
                                Default::default(),
                            ));
                            if let ObservationComponentValue::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueBoolean" => {
                            let r#enum = r#value.get_or_insert(ObservationComponentValue::Boolean(
                                Default::default(),
                            ));
                            if let ObservationComponentValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBoolean" => {
                            let r#enum = r#value.get_or_insert(ObservationComponentValue::Boolean(
                                Default::default(),
                            ));
                            if let ObservationComponentValue::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueInteger" => {
                            let r#enum = r#value.get_or_insert(ObservationComponentValue::Integer(
                                Default::default(),
                            ));
                            if let ObservationComponentValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueInteger" => {
                            let r#enum = r#value.get_or_insert(ObservationComponentValue::Integer(
                                Default::default(),
                            ));
                            if let ObservationComponentValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInteger"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueRange" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value =
                                Some(ObservationComponentValue::Range(map_access.next_value()?));
                        }
                        "valueRatio" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value =
                                Some(ObservationComponentValue::Ratio(map_access.next_value()?));
                        }
                        "valueSampledData" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some(ObservationComponentValue::SampledData(
                                map_access.next_value()?,
                            ));
                        }
                        "valueTime" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationComponentValue::Time(Default::default()));
                            if let ObservationComponentValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueTime" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationComponentValue::Time(Default::default()));
                            if let ObservationComponentValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueTime"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueDateTime" => {
                            let r#enum = r#value.get_or_insert(
                                ObservationComponentValue::DateTime(Default::default()),
                            );
                            if let ObservationComponentValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDateTime" => {
                            let r#enum = r#value.get_or_insert(
                                ObservationComponentValue::DateTime(Default::default()),
                            );
                            if let ObservationComponentValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valuePeriod" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value =
                                Some(ObservationComponentValue::Period(map_access.next_value()?));
                        }
                        "dataAbsentReason" => {
                            if r#data_absent_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataAbsentReason"));
                            }
                            r#data_absent_reason = Some(map_access.next_value()?);
                        }
                        "interpretation" => {
                            if r#interpretation.is_some() {
                                return Err(serde::de::Error::duplicate_field("interpretation"));
                            }
                            r#interpretation = Some(map_access.next_value()?);
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
                                    "code",
                                    "value",
                                    "data_absent_reason",
                                    "interpretation",
                                    "reference_range",
                                ],
                            ))
                        }
                    }
                }
                Ok(ObservationComponent {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#value,
                    r#data_absent_reason,
                    r#interpretation: r#interpretation.unwrap_or(vec![]),
                    r#reference_range: r#reference_range.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Observation {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#effective: Option<ObservationEffective>,
    pub r#issued: Option<super::super::types::Instant>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#value: Option<ObservationValue>,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#reference_range: Vec<ObservationReferenceRange>,
    pub r#has_member: Vec<Box<super::super::types::Reference>>,
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    pub r#component: Vec<ObservationComponent>,
}
impl serde::ser::Serialize for Observation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Observation")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
        }
        if !self.r#part_of.is_empty() {
            state.serialize_entry("partOf", &self.r#part_of)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        state.serialize_entry("code", &self.r#code)?;
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if !self.r#focus.is_empty() {
            state.serialize_entry("focus", &self.r#focus)?;
        }
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#effective.as_ref() {
            match some {
                ObservationEffective::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("effectiveDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_effectiveDateTime", &primitive_element)?;
                    }
                }
                ObservationEffective::Period(ref value) => {
                    state.serialize_entry("effectivePeriod", value)?;
                }
                ObservationEffective::Timing(ref value) => {
                    state.serialize_entry("effectiveTiming", value)?;
                }
                ObservationEffective::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("effectiveInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_effectiveInstant", &primitive_element)?;
                    }
                }
            }
        }
        if let Some(some) = self.r#issued.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("issued", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_issued", &primitive_element)?;
            }
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                ObservationValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ObservationValue::CodeableConcept(ref value) => {
                    state.serialize_entry("valueCodeableConcept", value)?;
                }
                ObservationValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                ObservationValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBoolean", &primitive_element)?;
                    }
                }
                ObservationValue::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueInteger", &primitive_element)?;
                    }
                }
                ObservationValue::Range(ref value) => {
                    state.serialize_entry("valueRange", value)?;
                }
                ObservationValue::Ratio(ref value) => {
                    state.serialize_entry("valueRatio", value)?;
                }
                ObservationValue::SampledData(ref value) => {
                    state.serialize_entry("valueSampledData", value)?;
                }
                ObservationValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueTime", &primitive_element)?;
                    }
                }
                ObservationValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDateTime", &primitive_element)?;
                    }
                }
                ObservationValue::Period(ref value) => {
                    state.serialize_entry("valuePeriod", value)?;
                }
            }
        }
        if let Some(some) = self.r#data_absent_reason.as_ref() {
            state.serialize_entry("dataAbsentReason", some)?;
        }
        if !self.r#interpretation.is_empty() {
            state.serialize_entry("interpretation", &self.r#interpretation)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if let Some(some) = self.r#method.as_ref() {
            state.serialize_entry("method", some)?;
        }
        if let Some(some) = self.r#specimen.as_ref() {
            state.serialize_entry("specimen", some)?;
        }
        if let Some(some) = self.r#device.as_ref() {
            state.serialize_entry("device", some)?;
        }
        if !self.r#reference_range.is_empty() {
            state.serialize_entry("referenceRange", &self.r#reference_range)?;
        }
        if !self.r#has_member.is_empty() {
            state.serialize_entry("hasMember", &self.r#has_member)?;
        }
        if !self.r#derived_from.is_empty() {
            state.serialize_entry("derivedFrom", &self.r#derived_from)?;
        }
        if !self.r#component.is_empty() {
            state.serialize_entry("component", &self.r#component)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Observation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Observation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Observation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Observation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#focus: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#effective: Option<ObservationEffective> = None;
                let mut r#issued: Option<super::super::types::Instant> = None;
                let mut r#performer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#value: Option<ObservationValue> = None;
                let mut r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#interpretation: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#specimen: Option<Box<super::super::types::Reference>> = None;
                let mut r#device: Option<Box<super::super::types::Reference>> = None;
                let mut r#reference_range: Option<Vec<ObservationReferenceRange>> = None;
                let mut r#has_member: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#derived_from: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#component: Option<Vec<ObservationComponent>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "basedOn" => {
                            if r#based_on.is_some() {
                                return Err(serde::de::Error::duplicate_field("basedOn"));
                            }
                            r#based_on = Some(map_access.next_value()?);
                        }
                        "partOf" => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            r#part_of = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "focus" => {
                            if r#focus.is_some() {
                                return Err(serde::de::Error::duplicate_field("focus"));
                            }
                            r#focus = Some(map_access.next_value()?);
                        }
                        "encounter" => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        "effectiveDateTime" => {
                            let r#enum = r#effective
                                .get_or_insert(ObservationEffective::DateTime(Default::default()));
                            if let ObservationEffective::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectiveDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("effective[x]"));
                            }
                        }
                        "_effectiveDateTime" => {
                            let r#enum = r#effective
                                .get_or_insert(ObservationEffective::DateTime(Default::default()));
                            if let ObservationEffective::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_effectiveDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_effective[x]"));
                            }
                        }
                        "effectivePeriod" => {
                            if r#effective.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectivePeriod"));
                            }
                            r#effective =
                                Some(ObservationEffective::Period(map_access.next_value()?));
                        }
                        "effectiveTiming" => {
                            if r#effective.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveTiming"));
                            }
                            r#effective =
                                Some(ObservationEffective::Timing(map_access.next_value()?));
                        }
                        "effectiveInstant" => {
                            let r#enum = r#effective
                                .get_or_insert(ObservationEffective::Instant(Default::default()));
                            if let ObservationEffective::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectiveInstant",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("effective[x]"));
                            }
                        }
                        "_effectiveInstant" => {
                            let r#enum = r#effective
                                .get_or_insert(ObservationEffective::Instant(Default::default()));
                            if let ObservationEffective::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_effectiveInstant",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_effective[x]"));
                            }
                        }
                        "issued" => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("issued"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_issued" => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_issued"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "performer" => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        "valueQuantity" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(ObservationValue::Quantity(map_access.next_value()?));
                        }
                        "valueCodeableConcept" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value =
                                Some(ObservationValue::CodeableConcept(map_access.next_value()?));
                        }
                        "valueString" => {
                            let r#enum =
                                r#value.get_or_insert(ObservationValue::String(Default::default()));
                            if let ObservationValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueString" => {
                            let r#enum =
                                r#value.get_or_insert(ObservationValue::String(Default::default()));
                            if let ObservationValue::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueBoolean" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationValue::Boolean(Default::default()));
                            if let ObservationValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBoolean" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationValue::Boolean(Default::default()));
                            if let ObservationValue::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueInteger" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationValue::Integer(Default::default()));
                            if let ObservationValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueInteger" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationValue::Integer(Default::default()));
                            if let ObservationValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInteger"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueRange" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(ObservationValue::Range(map_access.next_value()?));
                        }
                        "valueRatio" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value = Some(ObservationValue::Ratio(map_access.next_value()?));
                        }
                        "valueSampledData" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some(ObservationValue::SampledData(map_access.next_value()?));
                        }
                        "valueTime" => {
                            let r#enum =
                                r#value.get_or_insert(ObservationValue::Time(Default::default()));
                            if let ObservationValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueTime" => {
                            let r#enum =
                                r#value.get_or_insert(ObservationValue::Time(Default::default()));
                            if let ObservationValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueTime"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueDateTime" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationValue::DateTime(Default::default()));
                            if let ObservationValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDateTime" => {
                            let r#enum = r#value
                                .get_or_insert(ObservationValue::DateTime(Default::default()));
                            if let ObservationValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valuePeriod" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value = Some(ObservationValue::Period(map_access.next_value()?));
                        }
                        "dataAbsentReason" => {
                            if r#data_absent_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataAbsentReason"));
                            }
                            r#data_absent_reason = Some(map_access.next_value()?);
                        }
                        "interpretation" => {
                            if r#interpretation.is_some() {
                                return Err(serde::de::Error::duplicate_field("interpretation"));
                            }
                            r#interpretation = Some(map_access.next_value()?);
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        "bodySite" => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        "method" => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some(map_access.next_value()?);
                        }
                        "specimen" => {
                            if r#specimen.is_some() {
                                return Err(serde::de::Error::duplicate_field("specimen"));
                            }
                            r#specimen = Some(map_access.next_value()?);
                        }
                        "device" => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(map_access.next_value()?);
                        }
                        "referenceRange" => {
                            if r#reference_range.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceRange"));
                            }
                            r#reference_range = Some(map_access.next_value()?);
                        }
                        "hasMember" => {
                            if r#has_member.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasMember"));
                            }
                            r#has_member = Some(map_access.next_value()?);
                        }
                        "derivedFrom" => {
                            if r#derived_from.is_some() {
                                return Err(serde::de::Error::duplicate_field("derivedFrom"));
                            }
                            r#derived_from = Some(map_access.next_value()?);
                        }
                        "component" => {
                            if r#component.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            r#component = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "based_on",
                                    "part_of",
                                    "status",
                                    "category",
                                    "code",
                                    "subject",
                                    "focus",
                                    "encounter",
                                    "effective",
                                    "issued",
                                    "performer",
                                    "value",
                                    "data_absent_reason",
                                    "interpretation",
                                    "note",
                                    "body_site",
                                    "method",
                                    "specimen",
                                    "device",
                                    "reference_range",
                                    "has_member",
                                    "derived_from",
                                    "component",
                                ],
                            ))
                        }
                    }
                }
                Ok(Observation {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#category: r#category.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#subject,
                    r#focus: r#focus.unwrap_or(vec![]),
                    r#encounter,
                    r#effective,
                    r#issued,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#value,
                    r#data_absent_reason,
                    r#interpretation: r#interpretation.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#body_site,
                    r#method,
                    r#specimen,
                    r#device,
                    r#reference_range: r#reference_range.unwrap_or(vec![]),
                    r#has_member: r#has_member.unwrap_or(vec![]),
                    r#derived_from: r#derived_from.unwrap_or(vec![]),
                    r#component: r#component.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}