// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct GraphDefinitionLinkTargetCompartment {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#use: super::super::types::Code,
    pub r#code: super::super::types::Code,
    pub r#rule: super::super::types::Code,
    pub r#expression: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
}
impl serde::ser::Serialize for GraphDefinitionLinkTargetCompartment {
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
        if let Some(some) = self.r#use.value.as_ref() {
            state.serialize_entry("use", some)?;
        }
        if self.r#use.id.is_some() || !self.r#use.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#use.id,
                extension: &self.r#use.extension,
            };
            state.serialize_entry("_use", &primitive_element)?;
        }
        if let Some(some) = self.r#code.value.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#code.id,
                extension: &self.r#code.extension,
            };
            state.serialize_entry("_code", &primitive_element)?;
        }
        if let Some(some) = self.r#rule.value.as_ref() {
            state.serialize_entry("rule", some)?;
        }
        if self.r#rule.id.is_some() || !self.r#rule.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#rule.id,
                extension: &self.r#rule.extension,
            };
            state.serialize_entry("_rule", &primitive_element)?;
        }
        if let Some(some) = self.r#expression.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("expression", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_expression", &primitive_element)?;
            }
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
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for GraphDefinitionLinkTargetCompartment {
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
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "_use")]
            UsePrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "rule")]
            Rule,
            #[serde(rename = "_rule")]
            RulePrimitiveElement,
            #[serde(rename = "expression")]
            Expression,
            #[serde(rename = "_expression")]
            ExpressionPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GraphDefinitionLinkTargetCompartment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GraphDefinitionLinkTargetCompartment")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<GraphDefinitionLinkTargetCompartment, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#use: Option<super::super::types::Code> = None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#rule: Option<super::super::types::Code> = None;
                let mut r#expression: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
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
                        Field::Use => {
                            let some = r#use.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("use"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UsePrimitiveElement => {
                            let some = r#use.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_use"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Code => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CodePrimitiveElement => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_code"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Rule => {
                            let some = r#rule.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RulePrimitiveElement => {
                            let some = r#rule.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_rule"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Expression => {
                            let some = r#expression.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ExpressionPrimitiveElement => {
                            let some = r#expression.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_expression"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
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
                    }
                }
                Ok(GraphDefinitionLinkTargetCompartment {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#use: r#use.ok_or(serde::de::Error::missing_field("use"))?,
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#rule: r#rule.ok_or(serde::de::Error::missing_field("rule"))?,
                    r#expression,
                    r#description,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct GraphDefinitionLinkTarget {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#params: Option<super::super::types::String>,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#compartment: Vec<GraphDefinitionLinkTargetCompartment>,
    pub r#link: Vec<GraphDefinitionLink>,
}
impl serde::ser::Serialize for GraphDefinitionLinkTarget {
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
        if let Some(some) = self.r#params.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("params", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_params", &primitive_element)?;
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
        if !self.r#compartment.is_empty() {
            state.serialize_entry("compartment", &self.r#compartment)?;
        }
        if !self.r#link.is_empty() {
            state.serialize_entry("link", &self.r#link)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for GraphDefinitionLinkTarget {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "params")]
            Params,
            #[serde(rename = "_params")]
            ParamsPrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "_profile")]
            ProfilePrimitiveElement,
            #[serde(rename = "compartment")]
            Compartment,
            #[serde(rename = "link")]
            Link,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GraphDefinitionLinkTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GraphDefinitionLinkTarget")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<GraphDefinitionLinkTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#params: Option<super::super::types::String> = None;
                let mut r#profile: Option<super::super::types::Canonical> = None;
                let mut r#compartment: Option<Vec<GraphDefinitionLinkTargetCompartment>> = None;
                let mut r#link: Option<Vec<GraphDefinitionLink>> = None;
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
                        Field::Type => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TypePrimitiveElement => {
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
                        Field::Params => {
                            let some = r#params.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ParamsPrimitiveElement => {
                            let some = r#params.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_params"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Profile => {
                            let some = r#profile.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ProfilePrimitiveElement => {
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
                        Field::Compartment => {
                            if r#compartment.is_some() {
                                return Err(serde::de::Error::duplicate_field("compartment"));
                            }
                            r#compartment = Some(map_access.next_value()?);
                        }
                        Field::Link => {
                            if r#link.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            r#link = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(GraphDefinitionLinkTarget {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#params,
                    r#profile,
                    r#compartment: r#compartment.unwrap_or(vec![]),
                    r#link: r#link.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct GraphDefinitionLink {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: Option<super::super::types::String>,
    pub r#slice_name: Option<super::super::types::String>,
    pub r#min: Option<super::super::types::Integer>,
    pub r#max: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#target: Vec<GraphDefinitionLinkTarget>,
}
impl serde::ser::Serialize for GraphDefinitionLink {
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
        if let Some(some) = self.r#path.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("path", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_path", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#slice_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sliceName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sliceName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#min.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("min", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_min", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#max.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("max", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_max", &primitive_element)?;
            }
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
        if !self.r#target.is_empty() {
            state.serialize_entry("target", &self.r#target)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for GraphDefinitionLink {
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
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "sliceName")]
            SliceName,
            #[serde(rename = "_sliceName")]
            SliceNamePrimitiveElement,
            #[serde(rename = "min")]
            Min,
            #[serde(rename = "_min")]
            MinPrimitiveElement,
            #[serde(rename = "max")]
            Max,
            #[serde(rename = "_max")]
            MaxPrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "target")]
            Target,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GraphDefinitionLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GraphDefinitionLink")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<GraphDefinitionLink, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#slice_name: Option<super::super::types::String> = None;
                let mut r#min: Option<super::super::types::Integer> = None;
                let mut r#max: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#target: Option<Vec<GraphDefinitionLinkTarget>> = None;
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
                        Field::Path => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PathPrimitiveElement => {
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
                        Field::SliceName => {
                            let some = r#slice_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sliceName"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SliceNamePrimitiveElement => {
                            let some = r#slice_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sliceName"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Min => {
                            let some = r#min.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MinPrimitiveElement => {
                            let some = r#min.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_min"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Max => {
                            let some = r#max.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MaxPrimitiveElement => {
                            let some = r#max.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_max"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
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
                        Field::Target => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            r#target = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(GraphDefinitionLink {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#path,
                    r#slice_name,
                    r#min,
                    r#max,
                    r#description,
                    r#target: r#target.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct GraphDefinition {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#version: Option<super::super::types::String>,
    pub r#name: super::super::types::String,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#start: super::super::types::Code,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#link: Vec<GraphDefinitionLink>,
}
impl serde::ser::Serialize for GraphDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "GraphDefinition")?;
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
        if let Some(some) = self.r#name.value.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#name.id,
                extension: &self.r#name.extension,
            };
            state.serialize_entry("_name", &primitive_element)?;
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
        if let Some(some) = self.r#start.value.as_ref() {
            state.serialize_entry("start", some)?;
        }
        if self.r#start.id.is_some() || !self.r#start.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#start.id,
                extension: &self.r#start.extension,
            };
            state.serialize_entry("_start", &primitive_element)?;
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
        if !self.r#link.is_empty() {
            state.serialize_entry("link", &self.r#link)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for GraphDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
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
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "experimental")]
            Experimental,
            #[serde(rename = "_experimental")]
            ExperimentalPrimitiveElement,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "_publisher")]
            PublisherPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "purpose")]
            Purpose,
            #[serde(rename = "_purpose")]
            PurposePrimitiveElement,
            #[serde(rename = "start")]
            Start,
            #[serde(rename = "_start")]
            StartPrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "_profile")]
            ProfilePrimitiveElement,
            #[serde(rename = "link")]
            Link,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GraphDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GraphDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<GraphDefinition, V::Error>
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
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#purpose: Option<super::super::types::Markdown> = None;
                let mut r#start: Option<super::super::types::Code> = None;
                let mut r#profile: Option<super::super::types::Canonical> = None;
                let mut r#link: Option<Vec<GraphDefinitionLink>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "GraphDefinition" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"GraphDefinition",
                                ));
                            }
                        }
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
                        Field::Url => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UrlPrimitiveElement => {
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
                        Field::Version => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::VersionPrimitiveElement => {
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
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
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
                        Field::Experimental => {
                            let some = r#experimental.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimental"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ExperimentalPrimitiveElement => {
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
                        Field::Publisher => {
                            let some = r#publisher.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PublisherPrimitiveElement => {
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
                        Field::Contact => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            r#contact = Some(map_access.next_value()?);
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
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
                        Field::UseContext => {
                            if r#use_context.is_some() {
                                return Err(serde::de::Error::duplicate_field("useContext"));
                            }
                            r#use_context = Some(map_access.next_value()?);
                        }
                        Field::Jurisdiction => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            r#jurisdiction = Some(map_access.next_value()?);
                        }
                        Field::Purpose => {
                            let some = r#purpose.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PurposePrimitiveElement => {
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
                        Field::Start => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StartPrimitiveElement => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_start"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Profile => {
                            let some = r#profile.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ProfilePrimitiveElement => {
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
                        Field::Link => {
                            if r#link.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            r#link = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(GraphDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url,
                    r#version,
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#experimental,
                    r#date,
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#description,
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                    r#purpose,
                    r#start: r#start.ok_or(serde::de::Error::missing_field("start"))?,
                    r#profile,
                    r#link: r#link.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
