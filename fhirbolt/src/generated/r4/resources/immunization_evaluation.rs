// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ImmunizationEvaluationDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
impl Default for ImmunizationEvaluationDoseNumber {
    fn default() -> ImmunizationEvaluationDoseNumber {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ImmunizationEvaluationSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
impl Default for ImmunizationEvaluationSeriesDoses {
    fn default() -> ImmunizationEvaluationSeriesDoses {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImmunizationEvaluation {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#authority: Option<Box<super::super::types::Reference>>,
    pub r#target_disease: Box<super::super::types::CodeableConcept>,
    pub r#immunization_event: Box<super::super::types::Reference>,
    pub r#dose_status: Box<super::super::types::CodeableConcept>,
    pub r#dose_status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#series: Option<super::super::types::String>,
    pub r#dose_number: Option<ImmunizationEvaluationDoseNumber>,
    pub r#series_doses: Option<ImmunizationEvaluationSeriesDoses>,
}
impl serde::ser::Serialize for ImmunizationEvaluation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ImmunizationEvaluation")?;
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
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#authority.as_ref() {
            state.serialize_entry("authority", some)?;
        }
        state.serialize_entry("targetDisease", &self.r#target_disease)?;
        state.serialize_entry("immunizationEvent", &self.r#immunization_event)?;
        state.serialize_entry("doseStatus", &self.r#dose_status)?;
        if !self.r#dose_status_reason.is_empty() {
            state.serialize_entry("doseStatusReason", &self.r#dose_status_reason)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#series.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("series", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_series", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#dose_number.as_ref() {
            match some {
                ImmunizationEvaluationDoseNumber::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("doseNumberPositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_doseNumberPositiveInt", &primitive_element)?;
                    }
                }
                ImmunizationEvaluationDoseNumber::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("doseNumberString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_doseNumberString", &primitive_element)?;
                    }
                }
            }
        }
        if let Some(some) = self.r#series_doses.as_ref() {
            match some {
                ImmunizationEvaluationSeriesDoses::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("seriesDosesPositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_seriesDosesPositiveInt", &primitive_element)?;
                    }
                }
                ImmunizationEvaluationSeriesDoses::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("seriesDosesString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_seriesDosesString", &primitive_element)?;
                    }
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationEvaluation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationEvaluation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationEvaluation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImmunizationEvaluation, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#authority: Option<Box<super::super::types::Reference>> = None;
                let mut r#target_disease: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#immunization_event: Option<Box<super::super::types::Reference>> = None;
                let mut r#dose_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#dose_status_reason: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#series: Option<super::super::types::String> = None;
                let mut r#dose_number: Option<ImmunizationEvaluationDoseNumber> = None;
                let mut r#series_doses: Option<ImmunizationEvaluationSeriesDoses> = None;
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
                        "patient" => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        "date" => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_date" => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_date"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "authority" => {
                            if r#authority.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            r#authority = Some(map_access.next_value()?);
                        }
                        "targetDisease" => {
                            if r#target_disease.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetDisease"));
                            }
                            r#target_disease = Some(map_access.next_value()?);
                        }
                        "immunizationEvent" => {
                            if r#immunization_event.is_some() {
                                return Err(serde::de::Error::duplicate_field("immunizationEvent"));
                            }
                            r#immunization_event = Some(map_access.next_value()?);
                        }
                        "doseStatus" => {
                            if r#dose_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseStatus"));
                            }
                            r#dose_status = Some(map_access.next_value()?);
                        }
                        "doseStatusReason" => {
                            if r#dose_status_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseStatusReason"));
                            }
                            r#dose_status_reason = Some(map_access.next_value()?);
                        }
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "series" => {
                            let some = r#series.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("series"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_series" => {
                            let some = r#series.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_series"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "doseNumberPositiveInt" => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationEvaluationDoseNumber::PositiveInt(Default::default()),
                            );
                            if let ImmunizationEvaluationDoseNumber::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "doseNumberPositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("doseNumber[x]"));
                            }
                        }
                        "_doseNumberPositiveInt" => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationEvaluationDoseNumber::PositiveInt(Default::default()),
                            );
                            if let ImmunizationEvaluationDoseNumber::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_doseNumberPositiveInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_doseNumber[x]"));
                            }
                        }
                        "doseNumberString" => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationEvaluationDoseNumber::String(Default::default()),
                            );
                            if let ImmunizationEvaluationDoseNumber::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "doseNumberString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("doseNumber[x]"));
                            }
                        }
                        "_doseNumberString" => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationEvaluationDoseNumber::String(Default::default()),
                            );
                            if let ImmunizationEvaluationDoseNumber::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_doseNumberString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_doseNumber[x]"));
                            }
                        }
                        "seriesDosesPositiveInt" => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationEvaluationSeriesDoses::PositiveInt(Default::default()),
                            );
                            if let ImmunizationEvaluationSeriesDoses::PositiveInt(variant) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "seriesDosesPositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("seriesDoses[x]"));
                            }
                        }
                        "_seriesDosesPositiveInt" => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationEvaluationSeriesDoses::PositiveInt(Default::default()),
                            );
                            if let ImmunizationEvaluationSeriesDoses::PositiveInt(variant) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_seriesDosesPositiveInt",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_seriesDoses[x]"));
                            }
                        }
                        "seriesDosesString" => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationEvaluationSeriesDoses::String(Default::default()),
                            );
                            if let ImmunizationEvaluationSeriesDoses::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "seriesDosesString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("seriesDoses[x]"));
                            }
                        }
                        "_seriesDosesString" => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationEvaluationSeriesDoses::String(Default::default()),
                            );
                            if let ImmunizationEvaluationSeriesDoses::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_seriesDosesString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_seriesDoses[x]"));
                            }
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
                                    "status",
                                    "patient",
                                    "date",
                                    "authority",
                                    "target_disease",
                                    "immunization_event",
                                    "dose_status",
                                    "dose_status_reason",
                                    "description",
                                    "series",
                                    "dose_number",
                                    "series_doses",
                                ],
                            ))
                        }
                    }
                }
                Ok(ImmunizationEvaluation {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#date,
                    r#authority,
                    r#target_disease: r#target_disease
                        .ok_or(serde::de::Error::missing_field("target_disease"))?,
                    r#immunization_event: r#immunization_event
                        .ok_or(serde::de::Error::missing_field("immunization_event"))?,
                    r#dose_status: r#dose_status
                        .ok_or(serde::de::Error::missing_field("dose_status"))?,
                    r#dose_status_reason: r#dose_status_reason.unwrap_or(vec![]),
                    r#description,
                    r#series,
                    r#dose_number,
                    r#series_doses,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}