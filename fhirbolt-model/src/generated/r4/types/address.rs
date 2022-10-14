// Generated on 2022-10-14 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.\n\nNeed to be able to record postal addresses, along with notes about their use."]
#[derive(Default, Debug, Clone)]
pub struct Address {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The purpose of this address."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "Distinguishes between physical addresses (those you can visit) and mailing addresses (e.g. PO Boxes and care-of addresses). Most addresses are both."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Specifies the entire address as it should be displayed e.g. on a postal label. This may be provided instead of or as well as the specific parts."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "This component contains the house number, apartment number, street name, street direction,  P.O. Box number, delivery hints, and similar address information."]
    pub r#line: Vec<super::super::types::String>,
    #[doc = "The name of the city, town, suburb, village or other community or delivery center."]
    pub r#city: Option<super::super::types::String>,
    #[doc = "The name of the administrative area (county)."]
    pub r#district: Option<super::super::types::String>,
    #[doc = "Sub-unit of a country with limited sovereignty in a federally organized country. A code may be used if codes are in common use (e.g. US 2 letter state codes)."]
    pub r#state: Option<super::super::types::String>,
    #[doc = "A postal code designating a region defined by the postal service."]
    pub r#postal_code: Option<super::super::types::String>,
    #[doc = "Country - a nation as commonly understood or generally accepted."]
    pub r#country: Option<super::super::types::String>,
    #[doc = "Time period when address was/is in use."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl crate::AnyResource for Address {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
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
            Unknown(String),
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "use",
                                        "type",
                                        "text",
                                        "line",
                                        "city",
                                        "district",
                                        "state",
                                        "postalCode",
                                        "country",
                                        "period",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}