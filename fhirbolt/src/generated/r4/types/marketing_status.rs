// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct MarketingStatus {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#country: Box<super::super::types::CodeableConcept>,
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Box<super::super::types::CodeableConcept>,
    pub r#date_range: Box<super::super::types::Period>,
    pub r#restore_date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for MarketingStatus {
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
        state.serialize_entry("country", &self.r#country)?;
        if let Some(some) = self.r#jurisdiction.as_ref() {
            state.serialize_entry("jurisdiction", some)?;
        }
        state.serialize_entry("status", &self.r#status)?;
        state.serialize_entry("dateRange", &self.r#date_range)?;
        if let Some(some) = self.r#restore_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("restoreDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_restoreDate", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MarketingStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MarketingStatus;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MarketingStatus")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MarketingStatus, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#country: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#jurisdiction: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#date_range: Option<Box<super::super::types::Period>> = None;
                let mut r#restore_date: Option<super::super::types::DateTime> = None;
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
                        "country" => {
                            if r#country.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            r#country = Some(map_access.next_value()?);
                        }
                        "jurisdiction" => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            r#jurisdiction = Some(map_access.next_value()?);
                        }
                        "status" => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some(map_access.next_value()?);
                        }
                        "dateRange" => {
                            if r#date_range.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateRange"));
                            }
                            r#date_range = Some(map_access.next_value()?);
                        }
                        "restoreDate" => {
                            let some = r#restore_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("restoreDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_restoreDate" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#restore_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_restoreDate"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
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
                                    "country",
                                    "jurisdiction",
                                    "status",
                                    "date_range",
                                    "restore_date",
                                ],
                            ))
                        }
                    }
                }
                Ok(MarketingStatus {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#country: r#country.ok_or(serde::de::Error::missing_field("country"))?,
                    r#jurisdiction,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#date_range: r#date_range
                        .ok_or(serde::de::Error::missing_field("date_range"))?,
                    r#restore_date,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
