// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ActivityDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ActivityDefinitionSubject {
    fn default() -> ActivityDefinitionSubject {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ActivityDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    Duration(Box<super::super::types::Duration>),
}
impl Default for ActivityDefinitionTiming {
    fn default() -> ActivityDefinitionTiming {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ActivityDefinitionProduct {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
impl Default for ActivityDefinitionProduct {
    fn default() -> ActivityDefinitionProduct {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ActivityDefinitionParticipant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ActivityDefinitionParticipant {
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
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ActivityDefinitionParticipant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ActivityDefinitionParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ActivityDefinitionParticipant")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ActivityDefinitionParticipant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "role" => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "type", "role"],
                            ))
                        }
                    }
                }
                Ok(ActivityDefinitionParticipant {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#role,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ActivityDefinitionDynamicValue {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: super::super::types::String,
    pub r#expression: Box<super::super::types::Expression>,
}
impl serde::ser::Serialize for ActivityDefinitionDynamicValue {
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
        if let Some(some) = self.r#path.value.as_ref() {
            state.serialize_entry("path", some)?;
        }
        if self.r#path.id.is_some() || !self.r#path.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#path.id,
                extension: &self.r#path.extension,
            };
            state.serialize_entry("_path", &primitive_element)?;
        }
        state.serialize_entry("expression", &self.r#expression)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ActivityDefinitionDynamicValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ActivityDefinitionDynamicValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ActivityDefinitionDynamicValue")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ActivityDefinitionDynamicValue, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#expression: Option<Box<super::super::types::Expression>> = None;
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
                        "path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "expression" => {
                            if r#expression.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            r#expression = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "path",
                                    "expression",
                                ],
                            ))
                        }
                    }
                }
                Ok(ActivityDefinitionDynamicValue {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#path: r#path.ok_or(serde::de::Error::missing_field("path"))?,
                    r#expression: r#expression
                        .ok_or(serde::de::Error::missing_field("expression"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ActivityDefinition {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#version: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#title: Option<super::super::types::String>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#subject: Option<ActivityDefinitionSubject>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#usage: Option<super::super::types::String>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#kind: Option<super::super::types::Code>,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#intent: Option<super::super::types::Code>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#timing: Option<ActivityDefinitionTiming>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#participant: Vec<ActivityDefinitionParticipant>,
    pub r#product: Option<ActivityDefinitionProduct>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#specimen_requirement: Vec<Box<super::super::types::Reference>>,
    pub r#observation_requirement: Vec<Box<super::super::types::Reference>>,
    pub r#observation_result_requirement: Vec<Box<super::super::types::Reference>>,
    pub r#transform: Option<super::super::types::Canonical>,
    pub r#dynamic_value: Vec<ActivityDefinitionDynamicValue>,
}
impl serde::ser::Serialize for ActivityDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ActivityDefinition")?;
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
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("url", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_url", &primitive_element)?;
            }
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#version.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("version", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_version", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
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
        if let Some(some) = self.r#subtitle.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("subtitle", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_subtitle", &primitive_element)?;
            }
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
        if let Some(some) = self.r#experimental.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("experimental", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_experimental", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#subject.as_ref() {
            match some {
                ActivityDefinitionSubject::CodeableConcept(ref value) => {
                    state.serialize_entry("subjectCodeableConcept", value)?;
                }
                ActivityDefinitionSubject::Reference(ref value) => {
                    state.serialize_entry("subjectReference", value)?;
                }
            }
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
        if let Some(some) = self.r#publisher.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("publisher", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_publisher", &primitive_element)?;
            }
        }
        if !self.r#contact.is_empty() {
            state.serialize_entry("contact", &self.r#contact)?;
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
        if !self.r#use_context.is_empty() {
            state.serialize_entry("useContext", &self.r#use_context)?;
        }
        if !self.r#jurisdiction.is_empty() {
            state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
        }
        if let Some(some) = self.r#purpose.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("purpose", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_purpose", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#usage.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("usage", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_usage", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#copyright.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("copyright", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_copyright", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#approval_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("approvalDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_approvalDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#last_review_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lastReviewDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastReviewDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#effective_period.as_ref() {
            state.serialize_entry("effectivePeriod", some)?;
        }
        if !self.r#topic.is_empty() {
            state.serialize_entry("topic", &self.r#topic)?;
        }
        if !self.r#author.is_empty() {
            state.serialize_entry("author", &self.r#author)?;
        }
        if !self.r#editor.is_empty() {
            state.serialize_entry("editor", &self.r#editor)?;
        }
        if !self.r#reviewer.is_empty() {
            state.serialize_entry("reviewer", &self.r#reviewer)?;
        }
        if !self.r#endorser.is_empty() {
            state.serialize_entry("endorser", &self.r#endorser)?;
        }
        if !self.r#related_artifact.is_empty() {
            state.serialize_entry("relatedArtifact", &self.r#related_artifact)?;
        }
        if !self.r#library.is_empty() {
            let values: Vec<_> = self.r#library.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("library", &values)?;
            }
            let requires_elements = self
                .r#library
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#library
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_library", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#kind.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("kind", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_kind", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#profile.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("profile", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_profile", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#intent.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("intent", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_intent", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#priority.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("priority", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_priority", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#do_not_perform.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("doNotPerform", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_doNotPerform", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#timing.as_ref() {
            match some {
                ActivityDefinitionTiming::Timing(ref value) => {
                    state.serialize_entry("timingTiming", value)?;
                }
                ActivityDefinitionTiming::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("timingDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timingDateTime", &primitive_element)?;
                    }
                }
                ActivityDefinitionTiming::Age(ref value) => {
                    state.serialize_entry("timingAge", value)?;
                }
                ActivityDefinitionTiming::Period(ref value) => {
                    state.serialize_entry("timingPeriod", value)?;
                }
                ActivityDefinitionTiming::Range(ref value) => {
                    state.serialize_entry("timingRange", value)?;
                }
                ActivityDefinitionTiming::Duration(ref value) => {
                    state.serialize_entry("timingDuration", value)?;
                }
            }
        }
        if let Some(some) = self.r#location.as_ref() {
            state.serialize_entry("location", some)?;
        }
        if !self.r#participant.is_empty() {
            state.serialize_entry("participant", &self.r#participant)?;
        }
        if let Some(some) = self.r#product.as_ref() {
            match some {
                ActivityDefinitionProduct::Reference(ref value) => {
                    state.serialize_entry("productReference", value)?;
                }
                ActivityDefinitionProduct::CodeableConcept(ref value) => {
                    state.serialize_entry("productCodeableConcept", value)?;
                }
            }
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if !self.r#dosage.is_empty() {
            state.serialize_entry("dosage", &self.r#dosage)?;
        }
        if !self.r#body_site.is_empty() {
            state.serialize_entry("bodySite", &self.r#body_site)?;
        }
        if !self.r#specimen_requirement.is_empty() {
            state.serialize_entry("specimenRequirement", &self.r#specimen_requirement)?;
        }
        if !self.r#observation_requirement.is_empty() {
            state.serialize_entry("observationRequirement", &self.r#observation_requirement)?;
        }
        if !self.r#observation_result_requirement.is_empty() {
            state.serialize_entry(
                "observationResultRequirement",
                &self.r#observation_result_requirement,
            )?;
        }
        if let Some(some) = self.r#transform.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("transform", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_transform", &primitive_element)?;
            }
        }
        if !self.r#dynamic_value.is_empty() {
            state.serialize_entry("dynamicValue", &self.r#dynamic_value)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ActivityDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ActivityDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ActivityDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ActivityDefinition, V::Error>
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
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#subtitle: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#subject: Option<ActivityDefinitionSubject> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#purpose: Option<super::super::types::Markdown> = None;
                let mut r#usage: Option<super::super::types::String> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
                let mut r#approval_date: Option<super::super::types::Date> = None;
                let mut r#last_review_date: Option<super::super::types::Date> = None;
                let mut r#effective_period: Option<Box<super::super::types::Period>> = None;
                let mut r#topic: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#author: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#editor: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#reviewer: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#endorser: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#related_artifact: Option<Vec<Box<super::super::types::RelatedArtifact>>> =
                    None;
                let mut r#library: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#kind: Option<super::super::types::Code> = None;
                let mut r#profile: Option<super::super::types::Canonical> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#intent: Option<super::super::types::Code> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#timing: Option<ActivityDefinitionTiming> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#participant: Option<Vec<ActivityDefinitionParticipant>> = None;
                let mut r#product: Option<ActivityDefinitionProduct> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#dosage: Option<Vec<Box<super::super::types::Dosage>>> = None;
                let mut r#body_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#specimen_requirement: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#observation_requirement: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                let mut r#observation_result_requirement: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                let mut r#transform: Option<super::super::types::Canonical> = None;
                let mut r#dynamic_value: Option<Vec<ActivityDefinitionDynamicValue>> = None;
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
                        "url" => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_url" => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_url"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "version" => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_version" => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
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
                        "subtitle" => {
                            let some = r#subtitle.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("subtitle"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_subtitle" => {
                            let some = r#subtitle.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_subtitle"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                        "experimental" => {
                            let some = r#experimental.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimental"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_experimental" => {
                            let some = r#experimental.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_experimental"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "subjectCodeableConcept" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subjectCodeableConcept",
                                ));
                            }
                            r#subject = Some(ActivityDefinitionSubject::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "subjectReference" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectReference"));
                            }
                            r#subject = Some(ActivityDefinitionSubject::Reference(
                                map_access.next_value()?,
                            ));
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
                        "publisher" => {
                            let some = r#publisher.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_publisher" => {
                            let some = r#publisher.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_publisher"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "contact" => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            r#contact = Some(map_access.next_value()?);
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
                        "useContext" => {
                            if r#use_context.is_some() {
                                return Err(serde::de::Error::duplicate_field("useContext"));
                            }
                            r#use_context = Some(map_access.next_value()?);
                        }
                        "jurisdiction" => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            r#jurisdiction = Some(map_access.next_value()?);
                        }
                        "purpose" => {
                            let some = r#purpose.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_purpose" => {
                            let some = r#purpose.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_purpose"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "usage" => {
                            let some = r#usage.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_usage" => {
                            let some = r#usage.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_usage"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "copyright" => {
                            let some = r#copyright.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("copyright"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_copyright" => {
                            let some = r#copyright.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_copyright"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "approvalDate" => {
                            let some = r#approval_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("approvalDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_approvalDate" => {
                            let some = r#approval_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_approvalDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "lastReviewDate" => {
                            let some = r#last_review_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastReviewDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_lastReviewDate" => {
                            let some = r#last_review_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lastReviewDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "effectivePeriod" => {
                            if r#effective_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectivePeriod"));
                            }
                            r#effective_period = Some(map_access.next_value()?);
                        }
                        "topic" => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topic"));
                            }
                            r#topic = Some(map_access.next_value()?);
                        }
                        "author" => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            r#author = Some(map_access.next_value()?);
                        }
                        "editor" => {
                            if r#editor.is_some() {
                                return Err(serde::de::Error::duplicate_field("editor"));
                            }
                            r#editor = Some(map_access.next_value()?);
                        }
                        "reviewer" => {
                            if r#reviewer.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewer"));
                            }
                            r#reviewer = Some(map_access.next_value()?);
                        }
                        "endorser" => {
                            if r#endorser.is_some() {
                                return Err(serde::de::Error::duplicate_field("endorser"));
                            }
                            r#endorser = Some(map_access.next_value()?);
                        }
                        "relatedArtifact" => {
                            if r#related_artifact.is_some() {
                                return Err(serde::de::Error::duplicate_field("relatedArtifact"));
                            }
                            r#related_artifact = Some(map_access.next_value()?);
                        }
                        "library" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#library.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("library"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_library" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#library.get_or_insert(Vec::with_capacity(elements.len()));
                            if vec.len() != elements.len() {
                                return Err(serde::de::Error::invalid_length(
                                    elements.len(),
                                    &"primitive values length",
                                ));
                            }
                            if vec
                                .iter()
                                .any(|e| e.id.is_some() || !e.extension.is_empty())
                            {
                                return Err(serde::de::Error::duplicate_field("_library"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "kind" => {
                            let some = r#kind.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_kind" => {
                            let some = r#kind.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_kind"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "profile" => {
                            let some = r#profile.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_profile" => {
                            let some = r#profile.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_profile"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "intent" => {
                            let some = r#intent.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("intent"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_intent" => {
                            let some = r#intent.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_intent"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "priority" => {
                            let some = r#priority.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_priority" => {
                            let some = r#priority.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_priority"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "doNotPerform" => {
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("doNotPerform"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_doNotPerform" => {
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_doNotPerform"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "timingTiming" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingTiming"));
                            }
                            r#timing =
                                Some(ActivityDefinitionTiming::Timing(map_access.next_value()?));
                        }
                        "timingDateTime" => {
                            let r#enum = r#timing.get_or_insert(
                                ActivityDefinitionTiming::DateTime(Default::default()),
                            );
                            if let ActivityDefinitionTiming::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "timingDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("timing[x]"));
                            }
                        }
                        "_timingDateTime" => {
                            let r#enum = r#timing.get_or_insert(
                                ActivityDefinitionTiming::DateTime(Default::default()),
                            );
                            if let ActivityDefinitionTiming::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_timingDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_timing[x]"));
                            }
                        }
                        "timingAge" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingAge"));
                            }
                            r#timing =
                                Some(ActivityDefinitionTiming::Age(map_access.next_value()?));
                        }
                        "timingPeriod" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingPeriod"));
                            }
                            r#timing =
                                Some(ActivityDefinitionTiming::Period(map_access.next_value()?));
                        }
                        "timingRange" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingRange"));
                            }
                            r#timing =
                                Some(ActivityDefinitionTiming::Range(map_access.next_value()?));
                        }
                        "timingDuration" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingDuration"));
                            }
                            r#timing =
                                Some(ActivityDefinitionTiming::Duration(map_access.next_value()?));
                        }
                        "location" => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        "participant" => {
                            if r#participant.is_some() {
                                return Err(serde::de::Error::duplicate_field("participant"));
                            }
                            r#participant = Some(map_access.next_value()?);
                        }
                        "productReference" => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field("productReference"));
                            }
                            r#product = Some(ActivityDefinitionProduct::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "productCodeableConcept" => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productCodeableConcept",
                                ));
                            }
                            r#product = Some(ActivityDefinitionProduct::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "dosage" => {
                            if r#dosage.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosage"));
                            }
                            r#dosage = Some(map_access.next_value()?);
                        }
                        "bodySite" => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        "specimenRequirement" => {
                            if r#specimen_requirement.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "specimenRequirement",
                                ));
                            }
                            r#specimen_requirement = Some(map_access.next_value()?);
                        }
                        "observationRequirement" => {
                            if r#observation_requirement.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "observationRequirement",
                                ));
                            }
                            r#observation_requirement = Some(map_access.next_value()?);
                        }
                        "observationResultRequirement" => {
                            if r#observation_result_requirement.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "observationResultRequirement",
                                ));
                            }
                            r#observation_result_requirement = Some(map_access.next_value()?);
                        }
                        "transform" => {
                            let some = r#transform.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("transform"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_transform" => {
                            let some = r#transform.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_transform"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "dynamicValue" => {
                            if r#dynamic_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicValue"));
                            }
                            r#dynamic_value = Some(map_access.next_value()?);
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
                                    "url",
                                    "identifier",
                                    "version",
                                    "name",
                                    "title",
                                    "subtitle",
                                    "status",
                                    "experimental",
                                    "subject",
                                    "date",
                                    "publisher",
                                    "contact",
                                    "description",
                                    "use_context",
                                    "jurisdiction",
                                    "purpose",
                                    "usage",
                                    "copyright",
                                    "approval_date",
                                    "last_review_date",
                                    "effective_period",
                                    "topic",
                                    "author",
                                    "editor",
                                    "reviewer",
                                    "endorser",
                                    "related_artifact",
                                    "library",
                                    "kind",
                                    "profile",
                                    "code",
                                    "intent",
                                    "priority",
                                    "do_not_perform",
                                    "timing",
                                    "location",
                                    "participant",
                                    "product",
                                    "quantity",
                                    "dosage",
                                    "body_site",
                                    "specimen_requirement",
                                    "observation_requirement",
                                    "observation_result_requirement",
                                    "transform",
                                    "dynamic_value",
                                ],
                            ))
                        }
                    }
                }
                Ok(ActivityDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url,
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#version,
                    r#name,
                    r#title,
                    r#subtitle,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#experimental,
                    r#subject,
                    r#date,
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#description,
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                    r#purpose,
                    r#usage,
                    r#copyright,
                    r#approval_date,
                    r#last_review_date,
                    r#effective_period,
                    r#topic: r#topic.unwrap_or(vec![]),
                    r#author: r#author.unwrap_or(vec![]),
                    r#editor: r#editor.unwrap_or(vec![]),
                    r#reviewer: r#reviewer.unwrap_or(vec![]),
                    r#endorser: r#endorser.unwrap_or(vec![]),
                    r#related_artifact: r#related_artifact.unwrap_or(vec![]),
                    r#library: r#library.unwrap_or(vec![]),
                    r#kind,
                    r#profile,
                    r#code,
                    r#intent,
                    r#priority,
                    r#do_not_perform,
                    r#timing,
                    r#location,
                    r#participant: r#participant.unwrap_or(vec![]),
                    r#product,
                    r#quantity,
                    r#dosage: r#dosage.unwrap_or(vec![]),
                    r#body_site: r#body_site.unwrap_or(vec![]),
                    r#specimen_requirement: r#specimen_requirement.unwrap_or(vec![]),
                    r#observation_requirement: r#observation_requirement.unwrap_or(vec![]),
                    r#observation_result_requirement: r#observation_result_requirement
                        .unwrap_or(vec![]),
                    r#transform,
                    r#dynamic_value: r#dynamic_value.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
