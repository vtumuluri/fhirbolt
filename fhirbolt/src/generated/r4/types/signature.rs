// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Signature {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::Coding>>,
    pub r#when: super::super::types::Instant,
    pub r#who: Box<super::super::types::Reference>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#target_format: Option<super::super::types::Code>,
    pub r#sig_format: Option<super::super::types::Code>,
    pub r#data: Option<super::super::types::Base64Binary>,
}
impl serde::ser::Serialize for Signature {
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
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if let Some(some) = self.r#when.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("when", &some)?;
        }
        if self.r#when.id.is_some() || !self.r#when.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#when.id,
                extension: &self.r#when.extension,
            };
            state.serialize_entry("_when", &primitive_element)?;
        }
        state.serialize_entry("who", &self.r#who)?;
        if let Some(some) = self.r#on_behalf_of.as_ref() {
            state.serialize_entry("onBehalfOf", some)?;
        }
        if let Some(some) = self.r#target_format.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("targetFormat", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_targetFormat", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#sig_format.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("sigFormat", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sigFormat", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#data.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("data", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_data", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Signature {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "when")]
            When,
            #[serde(rename = "_when")]
            WhenPrimitiveElement,
            #[serde(rename = "who")]
            Who,
            #[serde(rename = "onBehalfOf")]
            OnBehalfOf,
            #[serde(rename = "targetFormat")]
            TargetFormat,
            #[serde(rename = "_targetFormat")]
            TargetFormatPrimitiveElement,
            #[serde(rename = "sigFormat")]
            SigFormat,
            #[serde(rename = "_sigFormat")]
            SigFormatPrimitiveElement,
            #[serde(rename = "data")]
            Data,
            #[serde(rename = "_data")]
            DataPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Signature;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Signature")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Signature, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#when: Option<super::super::types::Instant> = None;
                let mut r#who: Option<Box<super::super::types::Reference>> = None;
                let mut r#on_behalf_of: Option<Box<super::super::types::Reference>> = None;
                let mut r#target_format: Option<super::super::types::Code> = None;
                let mut r#sig_format: Option<super::super::types::Code> = None;
                let mut r#data: Option<super::super::types::Base64Binary> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::When => {
                            let some = r#when.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("when"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::WhenPrimitiveElement => {
                            let some = r#when.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_when"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Who => {
                            if r#who.is_some() {
                                return Err(serde::de::Error::duplicate_field("who"));
                            }
                            r#who = Some(map_access.next_value()?);
                        }
                        Field::OnBehalfOf => {
                            if r#on_behalf_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                            }
                            r#on_behalf_of = Some(map_access.next_value()?);
                        }
                        Field::TargetFormat => {
                            let some = r#target_format.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetFormat"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::TargetFormatPrimitiveElement => {
                            let some = r#target_format.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_targetFormat"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::SigFormat => {
                            let some = r#sig_format.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sigFormat"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::SigFormatPrimitiveElement => {
                            let some = r#sig_format.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sigFormat"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Data => {
                            let some = r#data.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::DataPrimitiveElement => {
                            let some = r#data.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_data"));
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
                Ok(Signature {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#when: r#when.ok_or(serde::de::Error::missing_field("when"))?,
                    r#who: r#who.ok_or(serde::de::Error::missing_field("who"))?,
                    r#on_behalf_of,
                    r#target_format,
                    r#sig_format,
                    r#data,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
