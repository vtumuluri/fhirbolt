// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct ImagingStudySeriesPerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ImagingStudySeriesPerformer {
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
impl<'de> serde::de::Deserialize<'de> for ImagingStudySeriesPerformer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudySeriesPerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudySeriesPerformer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImagingStudySeriesPerformer, V::Error>
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
                        "function" => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            r#function = Some(map_access.next_value()?);
                        }
                        "actor" => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            r#actor = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "function", "actor"],
                            ))
                        }
                    }
                }
                Ok(ImagingStudySeriesPerformer {
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
pub struct ImagingStudySeriesInstance {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#uid: super::super::types::Id,
    pub r#sop_class: Box<super::super::types::Coding>,
    pub r#number: Option<super::super::types::UnsignedInt>,
    pub r#title: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ImagingStudySeriesInstance {
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
        if let Some(some) = self.r#uid.value.as_ref() {
            state.serialize_entry("uid", some)?;
        }
        if self.r#uid.id.is_some() || !self.r#uid.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#uid.id,
                extension: &self.r#uid.extension,
            };
            state.serialize_entry("_uid", &primitive_element)?;
        }
        state.serialize_entry("sopClass", &self.r#sop_class)?;
        if let Some(some) = self.r#number.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("number", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_number", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#title.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("title", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_title", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImagingStudySeriesInstance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudySeriesInstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudySeriesInstance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImagingStudySeriesInstance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#uid: Option<super::super::types::Id> = None;
                let mut r#sop_class: Option<Box<super::super::types::Coding>> = None;
                let mut r#number: Option<super::super::types::UnsignedInt> = None;
                let mut r#title: Option<super::super::types::String> = None;
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
                        "uid" => {
                            let some = r#uid.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_uid" => {
                            let some = r#uid.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_uid"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "sopClass" => {
                            if r#sop_class.is_some() {
                                return Err(serde::de::Error::duplicate_field("sopClass"));
                            }
                            r#sop_class = Some(map_access.next_value()?);
                        }
                        "number" => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_number" => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_number"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "title" => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_title" => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_title"));
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
                                    "uid",
                                    "sop_class",
                                    "number",
                                    "title",
                                ],
                            ))
                        }
                    }
                }
                Ok(ImagingStudySeriesInstance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#uid: r#uid.ok_or(serde::de::Error::missing_field("uid"))?,
                    r#sop_class: r#sop_class.ok_or(serde::de::Error::missing_field("sop_class"))?,
                    r#number,
                    r#title,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImagingStudySeries {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#uid: super::super::types::Id,
    pub r#number: Option<super::super::types::UnsignedInt>,
    pub r#modality: Box<super::super::types::Coding>,
    pub r#description: Option<super::super::types::String>,
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#body_site: Option<Box<super::super::types::Coding>>,
    pub r#laterality: Option<Box<super::super::types::Coding>>,
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    pub r#started: Option<super::super::types::DateTime>,
    pub r#performer: Vec<ImagingStudySeriesPerformer>,
    pub r#instance: Vec<ImagingStudySeriesInstance>,
}
impl serde::ser::Serialize for ImagingStudySeries {
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
        if let Some(some) = self.r#uid.value.as_ref() {
            state.serialize_entry("uid", some)?;
        }
        if self.r#uid.id.is_some() || !self.r#uid.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#uid.id,
                extension: &self.r#uid.extension,
            };
            state.serialize_entry("_uid", &primitive_element)?;
        }
        if let Some(some) = self.r#number.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("number", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_number", &primitive_element)?;
            }
        }
        state.serialize_entry("modality", &self.r#modality)?;
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
        if let Some(some) = self.r#number_of_instances.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("numberOfInstances", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_numberOfInstances", &primitive_element)?;
            }
        }
        if !self.r#endpoint.is_empty() {
            state.serialize_entry("endpoint", &self.r#endpoint)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if let Some(some) = self.r#laterality.as_ref() {
            state.serialize_entry("laterality", some)?;
        }
        if !self.r#specimen.is_empty() {
            state.serialize_entry("specimen", &self.r#specimen)?;
        }
        if let Some(some) = self.r#started.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("started", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_started", &primitive_element)?;
            }
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if !self.r#instance.is_empty() {
            state.serialize_entry("instance", &self.r#instance)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImagingStudySeries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudySeries;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudySeries")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImagingStudySeries, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#uid: Option<super::super::types::Id> = None;
                let mut r#number: Option<super::super::types::UnsignedInt> = None;
                let mut r#modality: Option<Box<super::super::types::Coding>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#number_of_instances: Option<super::super::types::UnsignedInt> = None;
                let mut r#endpoint: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Box<super::super::types::Coding>> = None;
                let mut r#laterality: Option<Box<super::super::types::Coding>> = None;
                let mut r#specimen: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#started: Option<super::super::types::DateTime> = None;
                let mut r#performer: Option<Vec<ImagingStudySeriesPerformer>> = None;
                let mut r#instance: Option<Vec<ImagingStudySeriesInstance>> = None;
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
                        "uid" => {
                            let some = r#uid.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_uid" => {
                            let some = r#uid.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_uid"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "number" => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_number" => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_number"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "modality" => {
                            if r#modality.is_some() {
                                return Err(serde::de::Error::duplicate_field("modality"));
                            }
                            r#modality = Some(map_access.next_value()?);
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
                        "numberOfInstances" => {
                            let some = r#number_of_instances.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfInstances"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_numberOfInstances" => {
                            let some = r#number_of_instances.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_numberOfInstances",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "endpoint" => {
                            if r#endpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            r#endpoint = Some(map_access.next_value()?);
                        }
                        "bodySite" => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        "laterality" => {
                            if r#laterality.is_some() {
                                return Err(serde::de::Error::duplicate_field("laterality"));
                            }
                            r#laterality = Some(map_access.next_value()?);
                        }
                        "specimen" => {
                            if r#specimen.is_some() {
                                return Err(serde::de::Error::duplicate_field("specimen"));
                            }
                            r#specimen = Some(map_access.next_value()?);
                        }
                        "started" => {
                            let some = r#started.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_started" => {
                            let some = r#started.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_started"));
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
                        "instance" => {
                            if r#instance.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            r#instance = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "uid",
                                    "number",
                                    "modality",
                                    "description",
                                    "number_of_instances",
                                    "endpoint",
                                    "body_site",
                                    "laterality",
                                    "specimen",
                                    "started",
                                    "performer",
                                    "instance",
                                ],
                            ))
                        }
                    }
                }
                Ok(ImagingStudySeries {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#uid: r#uid.ok_or(serde::de::Error::missing_field("uid"))?,
                    r#number,
                    r#modality: r#modality.ok_or(serde::de::Error::missing_field("modality"))?,
                    r#description,
                    r#number_of_instances,
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                    r#body_site,
                    r#laterality,
                    r#specimen: r#specimen.unwrap_or(vec![]),
                    r#started,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#instance: r#instance.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ImagingStudy {
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
    pub r#modality: Vec<Box<super::super::types::Coding>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#started: Option<super::super::types::DateTime>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#referrer: Option<Box<super::super::types::Reference>>,
    pub r#interpreter: Vec<Box<super::super::types::Reference>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#number_of_series: Option<super::super::types::UnsignedInt>,
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    pub r#procedure_reference: Option<Box<super::super::types::Reference>>,
    pub r#procedure_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#description: Option<super::super::types::String>,
    pub r#series: Vec<ImagingStudySeries>,
}
impl serde::ser::Serialize for ImagingStudy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ImagingStudy")?;
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
        if !self.r#modality.is_empty() {
            state.serialize_entry("modality", &self.r#modality)?;
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#started.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("started", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_started", &primitive_element)?;
            }
        }
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
        }
        if let Some(some) = self.r#referrer.as_ref() {
            state.serialize_entry("referrer", some)?;
        }
        if !self.r#interpreter.is_empty() {
            state.serialize_entry("interpreter", &self.r#interpreter)?;
        }
        if !self.r#endpoint.is_empty() {
            state.serialize_entry("endpoint", &self.r#endpoint)?;
        }
        if let Some(some) = self.r#number_of_series.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("numberOfSeries", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_numberOfSeries", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#number_of_instances.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("numberOfInstances", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_numberOfInstances", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#procedure_reference.as_ref() {
            state.serialize_entry("procedureReference", some)?;
        }
        if !self.r#procedure_code.is_empty() {
            state.serialize_entry("procedureCode", &self.r#procedure_code)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            state.serialize_entry("location", some)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
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
        if !self.r#series.is_empty() {
            state.serialize_entry("series", &self.r#series)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ImagingStudy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudy")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImagingStudy, V::Error>
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
                let mut r#modality: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#started: Option<super::super::types::DateTime> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#referrer: Option<Box<super::super::types::Reference>> = None;
                let mut r#interpreter: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#endpoint: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#number_of_series: Option<super::super::types::UnsignedInt> = None;
                let mut r#number_of_instances: Option<super::super::types::UnsignedInt> = None;
                let mut r#procedure_reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#procedure_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#series: Option<Vec<ImagingStudySeries>> = None;
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
                        "modality" => {
                            if r#modality.is_some() {
                                return Err(serde::de::Error::duplicate_field("modality"));
                            }
                            r#modality = Some(map_access.next_value()?);
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "encounter" => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        "started" => {
                            let some = r#started.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_started" => {
                            let some = r#started.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_started"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "basedOn" => {
                            if r#based_on.is_some() {
                                return Err(serde::de::Error::duplicate_field("basedOn"));
                            }
                            r#based_on = Some(map_access.next_value()?);
                        }
                        "referrer" => {
                            if r#referrer.is_some() {
                                return Err(serde::de::Error::duplicate_field("referrer"));
                            }
                            r#referrer = Some(map_access.next_value()?);
                        }
                        "interpreter" => {
                            if r#interpreter.is_some() {
                                return Err(serde::de::Error::duplicate_field("interpreter"));
                            }
                            r#interpreter = Some(map_access.next_value()?);
                        }
                        "endpoint" => {
                            if r#endpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            r#endpoint = Some(map_access.next_value()?);
                        }
                        "numberOfSeries" => {
                            let some = r#number_of_series.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfSeries"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_numberOfSeries" => {
                            let some = r#number_of_series.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_numberOfSeries"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "numberOfInstances" => {
                            let some = r#number_of_instances.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfInstances"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_numberOfInstances" => {
                            let some = r#number_of_instances.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_numberOfInstances",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "procedureReference" => {
                            if r#procedure_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "procedureReference",
                                ));
                            }
                            r#procedure_reference = Some(map_access.next_value()?);
                        }
                        "procedureCode" => {
                            if r#procedure_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("procedureCode"));
                            }
                            r#procedure_code = Some(map_access.next_value()?);
                        }
                        "location" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        "reasonCode" => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        "reasonReference" => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
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
                            if r#series.is_some() {
                                return Err(serde::de::Error::duplicate_field("series"));
                            }
                            r#series = Some(map_access.next_value()?);
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
                                    "modality",
                                    "subject",
                                    "encounter",
                                    "started",
                                    "based_on",
                                    "referrer",
                                    "interpreter",
                                    "endpoint",
                                    "number_of_series",
                                    "number_of_instances",
                                    "procedure_reference",
                                    "procedure_code",
                                    "location",
                                    "reason_code",
                                    "reason_reference",
                                    "note",
                                    "description",
                                    "series",
                                ],
                            ))
                        }
                    }
                }
                Ok(ImagingStudy {
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
                    r#modality: r#modality.unwrap_or(vec![]),
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#encounter,
                    r#started,
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#referrer,
                    r#interpreter: r#interpreter.unwrap_or(vec![]),
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                    r#number_of_series,
                    r#number_of_instances,
                    r#procedure_reference,
                    r#procedure_code: r#procedure_code.unwrap_or(vec![]),
                    r#location,
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#description,
                    r#series: r#series.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}