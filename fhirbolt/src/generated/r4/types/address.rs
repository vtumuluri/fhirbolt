// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct Address {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#use: Option<super::super::types::Code>,
    pub r#type: Option<super::super::types::Code>,
    pub r#text: Option<super::super::types::String>,
    pub r#line: Vec<super::super::types::String>,
    pub r#city: Option<super::super::types::String>,
    pub r#district: Option<super::super::types::String>,
    pub r#state: Option<super::super::types::String>,
    pub r#postal_code: Option<super::super::types::String>,
    pub r#country: Option<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for Address {
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
        if let Some(some) = self.r#use.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("use", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_use", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("type", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_type", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("text", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if !self.r#line.is_empty() {
            let values = self
                .r#line
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("line", &values)?;
            }
            let requires_elements = self
                .r#line
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#line
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
                state.serialize_entry("_line", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#city.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("city", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_city", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#district.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("district", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_district", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#state.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("state", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_state", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#postal_code.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("postalCode", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_postalCode", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#country.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("country", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_country", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Address {
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
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "_use")]
            UsePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "line")]
            Line,
            #[serde(rename = "_line")]
            LinePrimitiveElement,
            #[serde(rename = "city")]
            City,
            #[serde(rename = "_city")]
            CityPrimitiveElement,
            #[serde(rename = "district")]
            District,
            #[serde(rename = "_district")]
            DistrictPrimitiveElement,
            #[serde(rename = "state")]
            State,
            #[serde(rename = "_state")]
            StatePrimitiveElement,
            #[serde(rename = "postalCode")]
            PostalCode,
            #[serde(rename = "_postalCode")]
            PostalCodePrimitiveElement,
            #[serde(rename = "country")]
            Country,
            #[serde(rename = "_country")]
            CountryPrimitiveElement,
            #[serde(rename = "period")]
            Period,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Address;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Address")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Address, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#use: Option<super::super::types::Code> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#line: Option<Vec<super::super::types::String>> = None;
                let mut r#city: Option<super::super::types::String> = None;
                let mut r#district: Option<super::super::types::String> = None;
                let mut r#state: Option<super::super::types::String> = None;
                let mut r#postal_code: Option<super::super::types::String> = None;
                let mut r#country: Option<super::super::types::String> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                        Field::Use => {
                            let some = r#use.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("use"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
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
                        Field::Type => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
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
                        Field::Text => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::TextPrimitiveElement => {
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
                        Field::Line => {
                            let values: Vec<Option<_>> = map_access.next_value()?;
                            let vec = r#line.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(values.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                if let Some(value) = value {
                                    vec[i].value = Some(value);
                                }
                            }
                        }
                        Field::LinePrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#line.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(elements.len())
                                    .collect::<Vec<_>>(),
                            );
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
                                return Err(serde::de::Error::duplicate_field("_line"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::City => {
                            let some = r#city.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("city"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::CityPrimitiveElement => {
                            let some = r#city.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_city"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::District => {
                            let some = r#district.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("district"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::DistrictPrimitiveElement => {
                            let some = r#district.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_district"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::State => {
                            let some = r#state.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::StatePrimitiveElement => {
                            let some = r#state.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_state"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PostalCode => {
                            let some = r#postal_code.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("postalCode"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::PostalCodePrimitiveElement => {
                            let some = r#postal_code.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_postalCode"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Country => {
                            let some = r#country.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::CountryPrimitiveElement => {
                            let some = r#country.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_country"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(Address {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#use,
                    r#type,
                    r#text,
                    r#line: r#line.unwrap_or(vec![]),
                    r#city,
                    r#district,
                    r#state,
                    r#postal_code,
                    r#country,
                    r#period,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
