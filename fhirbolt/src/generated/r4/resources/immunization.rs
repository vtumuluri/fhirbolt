// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ImmunizationOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ImmunizationOccurrence {
    fn default() -> ImmunizationOccurrence {
        ImmunizationOccurrence::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ImmunizationProtocolAppliedDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ImmunizationProtocolAppliedDoseNumber {
    fn default() -> ImmunizationProtocolAppliedDoseNumber {
        ImmunizationProtocolAppliedDoseNumber::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ImmunizationProtocolAppliedSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ImmunizationProtocolAppliedSeriesDoses {
    fn default() -> ImmunizationProtocolAppliedSeriesDoses {
        ImmunizationProtocolAppliedSeriesDoses::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImmunizationPerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ImmunizationPerformer {
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
        if let Some(some) = self.r#function.as_ref() {
            state.serialize_entry("function", some)?;
        }
        state.serialize_entry("actor", &self.r#actor)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationPerformer {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "actor")]
            Actor,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationPerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationPerformer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImmunizationPerformer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#function: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Function => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            r#function = Some(map_access.next_value()?);
                        }
                        Field::Actor => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            r#actor = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ImmunizationPerformer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#function,
                    r#actor: r#actor.ok_or(serde::de::Error::missing_field("actor"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImmunizationEducation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#document_type: Option<super::super::types::String>,
    pub r#reference: Option<super::super::types::Uri>,
    pub r#publication_date: Option<super::super::types::DateTime>,
    pub r#presentation_date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for ImmunizationEducation {
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
        if let Some(some) = self.r#document_type.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("documentType", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_documentType", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#reference.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("reference", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_reference", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#publication_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("publicationDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_publicationDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#presentation_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("presentationDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_presentationDate", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationEducation {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "documentType")]
            DocumentType,
            #[serde(rename = "_documentType")]
            DocumentTypePrimitiveElement,
            #[serde(rename = "reference")]
            Reference,
            #[serde(rename = "_reference")]
            ReferencePrimitiveElement,
            #[serde(rename = "publicationDate")]
            PublicationDate,
            #[serde(rename = "_publicationDate")]
            PublicationDatePrimitiveElement,
            #[serde(rename = "presentationDate")]
            PresentationDate,
            #[serde(rename = "_presentationDate")]
            PresentationDatePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationEducation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationEducation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImmunizationEducation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#document_type: Option<super::super::types::String> = None;
                let mut r#reference: Option<super::super::types::Uri> = None;
                let mut r#publication_date: Option<super::super::types::DateTime> = None;
                let mut r#presentation_date: Option<super::super::types::DateTime> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::DocumentType => {
                            let some = r#document_type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentType"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DocumentTypePrimitiveElement => {
                            let some = r#document_type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_documentType"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Reference => {
                            let some = r#reference.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ReferencePrimitiveElement => {
                            let some = r#reference.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_reference"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PublicationDate => {
                            let some = r#publication_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicationDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PublicationDatePrimitiveElement => {
                            let some = r#publication_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_publicationDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PresentationDate => {
                            let some = r#presentation_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentationDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PresentationDatePrimitiveElement => {
                            let some = r#presentation_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_presentationDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImmunizationEducation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#document_type,
                    r#reference,
                    r#publication_date,
                    r#presentation_date,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImmunizationReaction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#detail: Option<Box<super::super::types::Reference>>,
    pub r#reported: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for ImmunizationReaction {
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
        if let Some(some) = self.r#detail.as_ref() {
            state.serialize_entry("detail", some)?;
        }
        if let Some(some) = self.r#reported.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("reported", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_reported", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationReaction {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "detail")]
            Detail,
            #[serde(rename = "reported")]
            Reported,
            #[serde(rename = "_reported")]
            ReportedPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationReaction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImmunizationReaction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#detail: Option<Box<super::super::types::Reference>> = None;
                let mut r#reported: Option<super::super::types::Boolean> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Date => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DatePrimitiveElement => {
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
                        Field::Detail => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            r#detail = Some(map_access.next_value()?);
                        }
                        Field::Reported => {
                            let some = r#reported.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("reported"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ReportedPrimitiveElement => {
                            let some = r#reported.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_reported"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ImmunizationReaction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#date,
                    r#detail,
                    r#reported,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImmunizationProtocolApplied {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#series: Option<super::super::types::String>,
    pub r#authority: Option<Box<super::super::types::Reference>>,
    pub r#target_disease: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#dose_number: ImmunizationProtocolAppliedDoseNumber,
    pub r#series_doses: Option<ImmunizationProtocolAppliedSeriesDoses>,
}
impl serde::ser::Serialize for ImmunizationProtocolApplied {
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
        if let Some(some) = self.r#authority.as_ref() {
            state.serialize_entry("authority", some)?;
        }
        if !self.r#target_disease.is_empty() {
            state.serialize_entry("targetDisease", &self.r#target_disease)?;
        }
        match self.r#dose_number {
            ImmunizationProtocolAppliedDoseNumber::PositiveInt(ref value) => {
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
            ImmunizationProtocolAppliedDoseNumber::String(ref value) => {
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
            ImmunizationProtocolAppliedDoseNumber::Invalid => {
                return Err(serde::ser::Error::custom("dose_number is a required field"))
            }
        }
        if let Some(some) = self.r#series_doses.as_ref() {
            match some {
                ImmunizationProtocolAppliedSeriesDoses::PositiveInt(ref value) => {
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
                ImmunizationProtocolAppliedSeriesDoses::String(ref value) => {
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
                ImmunizationProtocolAppliedSeriesDoses::Invalid => {
                    return Err(serde::ser::Error::custom("series_doses is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImmunizationProtocolApplied {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "series")]
            Series,
            #[serde(rename = "_series")]
            SeriesPrimitiveElement,
            #[serde(rename = "authority")]
            Authority,
            #[serde(rename = "targetDisease")]
            TargetDisease,
            #[serde(rename = "doseNumberPositiveInt")]
            DoseNumberPositiveInt,
            #[serde(rename = "_doseNumberPositiveInt")]
            DoseNumberPositiveIntPrimitiveElement,
            #[serde(rename = "doseNumberString")]
            DoseNumberString,
            #[serde(rename = "_doseNumberString")]
            DoseNumberStringPrimitiveElement,
            #[serde(rename = "seriesDosesPositiveInt")]
            SeriesDosesPositiveInt,
            #[serde(rename = "_seriesDosesPositiveInt")]
            SeriesDosesPositiveIntPrimitiveElement,
            #[serde(rename = "seriesDosesString")]
            SeriesDosesString,
            #[serde(rename = "_seriesDosesString")]
            SeriesDosesStringPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImmunizationProtocolApplied;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImmunizationProtocolApplied")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImmunizationProtocolApplied, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#series: Option<super::super::types::String> = None;
                let mut r#authority: Option<Box<super::super::types::Reference>> = None;
                let mut r#target_disease: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#dose_number: Option<ImmunizationProtocolAppliedDoseNumber> = None;
                let mut r#series_doses: Option<ImmunizationProtocolAppliedSeriesDoses> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Series => {
                            let some = r#series.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("series"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SeriesPrimitiveElement => {
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
                        Field::Authority => {
                            if r#authority.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            r#authority = Some(map_access.next_value()?);
                        }
                        Field::TargetDisease => {
                            if r#target_disease.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetDisease"));
                            }
                            r#target_disease = Some(map_access.next_value()?);
                        }
                        Field::DoseNumberPositiveInt => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationProtocolAppliedDoseNumber::PositiveInt(
                                    Default::default(),
                                ),
                            );
                            if let ImmunizationProtocolAppliedDoseNumber::PositiveInt(variant) =
                                r#enum
                            {
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
                        Field::DoseNumberPositiveIntPrimitiveElement => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationProtocolAppliedDoseNumber::PositiveInt(
                                    Default::default(),
                                ),
                            );
                            if let ImmunizationProtocolAppliedDoseNumber::PositiveInt(variant) =
                                r#enum
                            {
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
                        Field::DoseNumberString => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationProtocolAppliedDoseNumber::String(Default::default()),
                            );
                            if let ImmunizationProtocolAppliedDoseNumber::String(variant) = r#enum {
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
                        Field::DoseNumberStringPrimitiveElement => {
                            let r#enum = r#dose_number.get_or_insert(
                                ImmunizationProtocolAppliedDoseNumber::String(Default::default()),
                            );
                            if let ImmunizationProtocolAppliedDoseNumber::String(variant) = r#enum {
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
                        Field::SeriesDosesPositiveInt => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationProtocolAppliedSeriesDoses::PositiveInt(
                                    Default::default(),
                                ),
                            );
                            if let ImmunizationProtocolAppliedSeriesDoses::PositiveInt(variant) =
                                r#enum
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
                        Field::SeriesDosesPositiveIntPrimitiveElement => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationProtocolAppliedSeriesDoses::PositiveInt(
                                    Default::default(),
                                ),
                            );
                            if let ImmunizationProtocolAppliedSeriesDoses::PositiveInt(variant) =
                                r#enum
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
                        Field::SeriesDosesString => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationProtocolAppliedSeriesDoses::String(Default::default()),
                            );
                            if let ImmunizationProtocolAppliedSeriesDoses::String(variant) = r#enum
                            {
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
                        Field::SeriesDosesStringPrimitiveElement => {
                            let r#enum = r#series_doses.get_or_insert(
                                ImmunizationProtocolAppliedSeriesDoses::String(Default::default()),
                            );
                            if let ImmunizationProtocolAppliedSeriesDoses::String(variant) = r#enum
                            {
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
                    }
                }
                Ok(ImmunizationProtocolApplied {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#series,
                    r#authority,
                    r#target_disease: r#target_disease.unwrap_or(vec![]),
                    r#dose_number: r#dose_number
                        .ok_or(serde::de::Error::missing_field("dose_number"))?,
                    r#series_doses,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Immunization {
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
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#vaccine_code: Box<super::super::types::CodeableConcept>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#occurrence: ImmunizationOccurrence,
    pub r#recorded: Option<super::super::types::DateTime>,
    pub r#primary_source: Option<super::super::types::Boolean>,
    pub r#report_origin: Option<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#lot_number: Option<super::super::types::String>,
    pub r#expiration_date: Option<super::super::types::Date>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#dose_quantity: Option<Box<super::super::types::Quantity>>,
    pub r#performer: Vec<ImmunizationPerformer>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#is_subpotent: Option<super::super::types::Boolean>,
    pub r#subpotent_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#education: Vec<ImmunizationEducation>,
    pub r#program_eligibility: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#funding_source: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reaction: Vec<ImmunizationReaction>,
    pub r#protocol_applied: Vec<ImmunizationProtocolApplied>,
}
impl serde::ser::Serialize for Immunization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Immunization")?;
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
        if let Some(some) = self.r#status_reason.as_ref() {
            state.serialize_entry("statusReason", some)?;
        }
        state.serialize_entry("vaccineCode", &self.r#vaccine_code)?;
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        match self.r#occurrence {
            ImmunizationOccurrence::DateTime(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("occurrenceDateTime", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                }
            }
            ImmunizationOccurrence::String(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("occurrenceString", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_occurrenceString", &primitive_element)?;
                }
            }
            ImmunizationOccurrence::Invalid => {
                return Err(serde::ser::Error::custom("occurrence is a required field"))
            }
        }
        if let Some(some) = self.r#recorded.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("recorded", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_recorded", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#primary_source.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("primarySource", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_primarySource", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#report_origin.as_ref() {
            state.serialize_entry("reportOrigin", some)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            state.serialize_entry("location", some)?;
        }
        if let Some(some) = self.r#manufacturer.as_ref() {
            state.serialize_entry("manufacturer", some)?;
        }
        if let Some(some) = self.r#lot_number.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lotNumber", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lotNumber", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#expiration_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("expirationDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_expirationDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#site.as_ref() {
            state.serialize_entry("site", some)?;
        }
        if let Some(some) = self.r#route.as_ref() {
            state.serialize_entry("route", some)?;
        }
        if let Some(some) = self.r#dose_quantity.as_ref() {
            state.serialize_entry("doseQuantity", some)?;
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if let Some(some) = self.r#is_subpotent.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isSubpotent", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isSubpotent", &primitive_element)?;
            }
        }
        if !self.r#subpotent_reason.is_empty() {
            state.serialize_entry("subpotentReason", &self.r#subpotent_reason)?;
        }
        if !self.r#education.is_empty() {
            state.serialize_entry("education", &self.r#education)?;
        }
        if !self.r#program_eligibility.is_empty() {
            state.serialize_entry("programEligibility", &self.r#program_eligibility)?;
        }
        if let Some(some) = self.r#funding_source.as_ref() {
            state.serialize_entry("fundingSource", some)?;
        }
        if !self.r#reaction.is_empty() {
            state.serialize_entry("reaction", &self.r#reaction)?;
        }
        if !self.r#protocol_applied.is_empty() {
            state.serialize_entry("protocolApplied", &self.r#protocol_applied)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Immunization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "vaccineCode")]
            VaccineCode,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "occurrenceString")]
            OccurrenceString,
            #[serde(rename = "_occurrenceString")]
            OccurrenceStringPrimitiveElement,
            #[serde(rename = "recorded")]
            Recorded,
            #[serde(rename = "_recorded")]
            RecordedPrimitiveElement,
            #[serde(rename = "primarySource")]
            PrimarySource,
            #[serde(rename = "_primarySource")]
            PrimarySourcePrimitiveElement,
            #[serde(rename = "reportOrigin")]
            ReportOrigin,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "lotNumber")]
            LotNumber,
            #[serde(rename = "_lotNumber")]
            LotNumberPrimitiveElement,
            #[serde(rename = "expirationDate")]
            ExpirationDate,
            #[serde(rename = "_expirationDate")]
            ExpirationDatePrimitiveElement,
            #[serde(rename = "site")]
            Site,
            #[serde(rename = "route")]
            Route,
            #[serde(rename = "doseQuantity")]
            DoseQuantity,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "isSubpotent")]
            IsSubpotent,
            #[serde(rename = "_isSubpotent")]
            IsSubpotentPrimitiveElement,
            #[serde(rename = "subpotentReason")]
            SubpotentReason,
            #[serde(rename = "education")]
            Education,
            #[serde(rename = "programEligibility")]
            ProgramEligibility,
            #[serde(rename = "fundingSource")]
            FundingSource,
            #[serde(rename = "reaction")]
            Reaction,
            #[serde(rename = "protocolApplied")]
            ProtocolApplied,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Immunization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Immunization")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Immunization, V::Error>
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
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#vaccine_code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#occurrence: Option<ImmunizationOccurrence> = None;
                let mut r#recorded: Option<super::super::types::DateTime> = None;
                let mut r#primary_source: Option<super::super::types::Boolean> = None;
                let mut r#report_origin: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#manufacturer: Option<Box<super::super::types::Reference>> = None;
                let mut r#lot_number: Option<super::super::types::String> = None;
                let mut r#expiration_date: Option<super::super::types::Date> = None;
                let mut r#site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#route: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#dose_quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#performer: Option<Vec<ImmunizationPerformer>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#is_subpotent: Option<super::super::types::Boolean> = None;
                let mut r#subpotent_reason: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#education: Option<Vec<ImmunizationEducation>> = None;
                let mut r#program_eligibility: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#funding_source: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reaction: Option<Vec<ImmunizationReaction>> = None;
                let mut r#protocol_applied: Option<Vec<ImmunizationProtocolApplied>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRules => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
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
                        Field::Language => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LanguagePrimitiveElement => {
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
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        Field::Contained => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::Status => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusPrimitiveElement => {
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
                        Field::StatusReason => {
                            if r#status_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusReason"));
                            }
                            r#status_reason = Some(map_access.next_value()?);
                        }
                        Field::VaccineCode => {
                            if r#vaccine_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaccineCode"));
                            }
                            r#vaccine_code = Some(map_access.next_value()?);
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        Field::OccurrenceDateTime => {
                            let r#enum = r#occurrence.get_or_insert(
                                ImmunizationOccurrence::DateTime(Default::default()),
                            );
                            if let ImmunizationOccurrence::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("occurrence[x]"));
                            }
                        }
                        Field::OccurrenceDateTimePrimitiveElement => {
                            let r#enum = r#occurrence.get_or_insert(
                                ImmunizationOccurrence::DateTime(Default::default()),
                            );
                            if let ImmunizationOccurrence::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_occurrenceDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_occurrence[x]"));
                            }
                        }
                        Field::OccurrenceString => {
                            let r#enum = r#occurrence
                                .get_or_insert(ImmunizationOccurrence::String(Default::default()));
                            if let ImmunizationOccurrence::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("occurrence[x]"));
                            }
                        }
                        Field::OccurrenceStringPrimitiveElement => {
                            let r#enum = r#occurrence
                                .get_or_insert(ImmunizationOccurrence::String(Default::default()));
                            if let ImmunizationOccurrence::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_occurrenceString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_occurrence[x]"));
                            }
                        }
                        Field::Recorded => {
                            let some = r#recorded.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("recorded"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RecordedPrimitiveElement => {
                            let some = r#recorded.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_recorded"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PrimarySource => {
                            let some = r#primary_source.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("primarySource"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PrimarySourcePrimitiveElement => {
                            let some = r#primary_source.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_primarySource"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ReportOrigin => {
                            if r#report_origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportOrigin"));
                            }
                            r#report_origin = Some(map_access.next_value()?);
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        Field::Manufacturer => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(map_access.next_value()?);
                        }
                        Field::LotNumber => {
                            let some = r#lot_number.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lotNumber"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LotNumberPrimitiveElement => {
                            let some = r#lot_number.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lotNumber"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ExpirationDate => {
                            let some = r#expiration_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ExpirationDatePrimitiveElement => {
                            let some = r#expiration_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_expirationDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Site => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            r#site = Some(map_access.next_value()?);
                        }
                        Field::Route => {
                            if r#route.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            r#route = Some(map_access.next_value()?);
                        }
                        Field::DoseQuantity => {
                            if r#dose_quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseQuantity"));
                            }
                            r#dose_quantity = Some(map_access.next_value()?);
                        }
                        Field::Performer => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        Field::ReasonCode => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        Field::ReasonReference => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        Field::IsSubpotent => {
                            let some = r#is_subpotent.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSubpotent"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IsSubpotentPrimitiveElement => {
                            let some = r#is_subpotent.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isSubpotent"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::SubpotentReason => {
                            if r#subpotent_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("subpotentReason"));
                            }
                            r#subpotent_reason = Some(map_access.next_value()?);
                        }
                        Field::Education => {
                            if r#education.is_some() {
                                return Err(serde::de::Error::duplicate_field("education"));
                            }
                            r#education = Some(map_access.next_value()?);
                        }
                        Field::ProgramEligibility => {
                            if r#program_eligibility.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "programEligibility",
                                ));
                            }
                            r#program_eligibility = Some(map_access.next_value()?);
                        }
                        Field::FundingSource => {
                            if r#funding_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundingSource"));
                            }
                            r#funding_source = Some(map_access.next_value()?);
                        }
                        Field::Reaction => {
                            if r#reaction.is_some() {
                                return Err(serde::de::Error::duplicate_field("reaction"));
                            }
                            r#reaction = Some(map_access.next_value()?);
                        }
                        Field::ProtocolApplied => {
                            if r#protocol_applied.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolApplied"));
                            }
                            r#protocol_applied = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(Immunization {
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
                    r#status_reason,
                    r#vaccine_code: r#vaccine_code
                        .ok_or(serde::de::Error::missing_field("vaccine_code"))?,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#encounter,
                    r#occurrence: r#occurrence
                        .ok_or(serde::de::Error::missing_field("occurrence"))?,
                    r#recorded,
                    r#primary_source,
                    r#report_origin,
                    r#location,
                    r#manufacturer,
                    r#lot_number,
                    r#expiration_date,
                    r#site,
                    r#route,
                    r#dose_quantity,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#is_subpotent,
                    r#subpotent_reason: r#subpotent_reason.unwrap_or(vec![]),
                    r#education: r#education.unwrap_or(vec![]),
                    r#program_eligibility: r#program_eligibility.unwrap_or(vec![]),
                    r#funding_source,
                    r#reaction: r#reaction.unwrap_or(vec![]),
                    r#protocol_applied: r#protocol_applied.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
