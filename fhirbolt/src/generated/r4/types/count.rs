// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Count {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#comparator: Option<super::super::types::Code>,
    pub r#unit: Option<super::super::types::String>,
    pub r#system: Option<super::super::types::Uri>,
    pub r#code: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for Count {
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
        if let Some(some) = self.r#value.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("value", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_value", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#comparator.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("comparator", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_comparator", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#unit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("unit", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_unit", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#system.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("system", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_system", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#code.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_code", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Count")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Count, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#value: Option<super::super::types::Decimal> = None;
                let mut r#comparator: Option<super::super::types::Code> = None;
                let mut r#unit: Option<super::super::types::String> = None;
                let mut r#system: Option<super::super::types::Uri> = None;
                let mut r#code: Option<super::super::types::Code> = None;
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
                        "value" => {
                            let some = r#value.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_value" => {
                            let some = r#value.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_value"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "comparator" => {
                            let some = r#comparator.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparator"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_comparator" => {
                            let some = r#comparator.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_comparator"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "unit" => {
                            let some = r#unit.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_unit" => {
                            let some = r#unit.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_unit"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "system" => {
                            let some = r#system.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("system"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_system" => {
                            let some = r#system.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_system"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "code" => {
                            let some = r#code.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_code" => {
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
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "value",
                                    "comparator",
                                    "unit",
                                    "system",
                                    "code",
                                ],
                            ))
                        }
                    }
                }
                Ok(Count {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#value,
                    r#comparator,
                    r#unit,
                    r#system,
                    r#code,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}