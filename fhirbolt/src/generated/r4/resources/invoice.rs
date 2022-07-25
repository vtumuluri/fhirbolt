// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum InvoiceLineItemChargeItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for InvoiceLineItemChargeItem {
    fn default() -> InvoiceLineItemChargeItem {
        InvoiceLineItemChargeItem::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct InvoiceParticipant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for InvoiceParticipant {
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
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        state.serialize_entry("actor", &self.r#actor)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for InvoiceParticipant {
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
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "actor")]
            Actor,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InvoiceParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("InvoiceParticipant")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<InvoiceParticipant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Actor => {
                                if r#actor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actor"));
                                }
                                r#actor = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &["id", "extension", "modifierExtension", "role", "actor"],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(InvoiceParticipant {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#role,
                        r#actor: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#actor.unwrap_or(Default::default())
                        } else {
                            r#actor.ok_or(serde::de::Error::missing_field("actor"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct InvoiceLineItemPriceComponent {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#amount: Option<Box<super::super::types::Money>>,
}
impl serde::ser::Serialize for InvoiceLineItemPriceComponent {
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
            let some = Ok(some)?;
            state.serialize_entry("type", &some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("factor", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for InvoiceLineItemPriceComponent {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "amount")]
            Amount,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InvoiceLineItemPriceComponent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("InvoiceLineItemPriceComponent")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<InvoiceLineItemPriceComponent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::FactorPrimitiveElement => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Amount => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                r#amount = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "code",
                                            "factor",
                                            "amount",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(InvoiceLineItemPriceComponent {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#code,
                        r#factor,
                        r#amount,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct InvoiceLineItem {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: Option<super::super::types::PositiveInt>,
    pub r#charge_item: InvoiceLineItemChargeItem,
    pub r#price_component: Vec<InvoiceLineItemPriceComponent>,
}
impl serde::ser::Serialize for InvoiceLineItem {
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
        if let Some(some) = self.r#sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("sequence", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sequence", &primitive_element)?;
            }
        }
        match self.r#charge_item {
            InvoiceLineItemChargeItem::Reference(ref value) => {
                state.serialize_entry("chargeItemReference", value)?;
            }
            InvoiceLineItemChargeItem::CodeableConcept(ref value) => {
                state.serialize_entry("chargeItemCodeableConcept", value)?;
            }
            InvoiceLineItemChargeItem::Invalid => {
                return Err(serde::ser::Error::custom("charge_item is a required field"))
            }
        }
        if !self.r#price_component.is_empty() {
            state.serialize_entry("priceComponent", &self.r#price_component)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for InvoiceLineItem {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "chargeItemReference")]
            ChargeItemReference,
            #[serde(rename = "chargeItemCodeableConcept")]
            ChargeItemCodeableConcept,
            #[serde(rename = "priceComponent")]
            PriceComponent,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InvoiceLineItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("InvoiceLineItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<InvoiceLineItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#charge_item: Option<InvoiceLineItemChargeItem> = None;
                let mut r#price_component: Option<Vec<InvoiceLineItemPriceComponent>> = None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ChargeItemReference => {
                                if r#charge_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "chargeItemReference",
                                    ));
                                }
                                r#charge_item = Some(InvoiceLineItemChargeItem::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ChargeItemCodeableConcept => {
                                if r#charge_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "chargeItemCodeableConcept",
                                    ));
                                }
                                r#charge_item = Some(InvoiceLineItemChargeItem::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::PriceComponent => {
                                if r#price_component.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "priceComponent",
                                    ));
                                }
                                r#price_component = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "chargeItemReference",
                                            "chargeItemCodeableConcept",
                                            "priceComponent",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(InvoiceLineItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence,
                        r#charge_item: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#charge_item.unwrap_or(Default::default())
                        } else {
                            r#charge_item.ok_or(serde::de::Error::missing_field("chargeItem[x]"))?
                        },
                        r#price_component: r#price_component.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Invoice {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#cancelled_reason: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#participant: Vec<InvoiceParticipant>,
    pub r#issuer: Option<Box<super::super::types::Reference>>,
    pub r#account: Option<Box<super::super::types::Reference>>,
    pub r#line_item: Vec<InvoiceLineItem>,
    pub r#total_price_component: Vec<InvoiceLineItemPriceComponent>,
    pub r#total_net: Option<Box<super::super::types::Money>>,
    pub r#total_gross: Option<Box<super::super::types::Money>>,
    pub r#payment_terms: Option<super::super::types::Markdown>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for Invoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Invoice")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
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
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#cancelled_reason.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("cancelledReason", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_cancelledReason", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if let Some(some) = self.r#recipient.as_ref() {
            state.serialize_entry("recipient", some)?;
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("date", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if !self.r#participant.is_empty() {
            state.serialize_entry("participant", &self.r#participant)?;
        }
        if let Some(some) = self.r#issuer.as_ref() {
            state.serialize_entry("issuer", some)?;
        }
        if let Some(some) = self.r#account.as_ref() {
            state.serialize_entry("account", some)?;
        }
        if !self.r#line_item.is_empty() {
            state.serialize_entry("lineItem", &self.r#line_item)?;
        }
        if !self.r#total_price_component.is_empty() {
            state.serialize_entry("totalPriceComponent", &self.r#total_price_component)?;
        }
        if let Some(some) = self.r#total_net.as_ref() {
            state.serialize_entry("totalNet", some)?;
        }
        if let Some(some) = self.r#total_gross.as_ref() {
            state.serialize_entry("totalGross", some)?;
        }
        if let Some(some) = self.r#payment_terms.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("paymentTerms", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_paymentTerms", &primitive_element)?;
            }
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Invoice {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "cancelledReason")]
            CancelledReason,
            #[serde(rename = "_cancelledReason")]
            CancelledReasonPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "recipient")]
            Recipient,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "participant")]
            Participant,
            #[serde(rename = "issuer")]
            Issuer,
            #[serde(rename = "account")]
            Account,
            #[serde(rename = "lineItem")]
            LineItem,
            #[serde(rename = "totalPriceComponent")]
            TotalPriceComponent,
            #[serde(rename = "totalNet")]
            TotalNet,
            #[serde(rename = "totalGross")]
            TotalGross,
            #[serde(rename = "paymentTerms")]
            PaymentTerms,
            #[serde(rename = "_paymentTerms")]
            PaymentTermsPrimitiveElement,
            #[serde(rename = "note")]
            Note,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Invoice;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Invoice")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Invoice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#cancelled_reason: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#recipient: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#participant: Option<Vec<InvoiceParticipant>> = None;
                let mut r#issuer: Option<Box<super::super::types::Reference>> = None;
                let mut r#account: Option<Box<super::super::types::Reference>> = None;
                let mut r#line_item: Option<Vec<InvoiceLineItem>> = None;
                let mut r#total_price_component: Option<Vec<InvoiceLineItemPriceComponent>> = None;
                let mut r#total_net: Option<Box<super::super::types::Money>> = None;
                let mut r#total_gross: Option<Box<super::super::types::Money>> = None;
                let mut r#payment_terms: Option<super::super::types::Markdown> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                crate::json::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Invoice" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Invoice",
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::CancelledReason => {
                                let some = r#cancelled_reason.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cancelledReason",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CancelledReasonPrimitiveElement => {
                                let some = r#cancelled_reason.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_cancelledReason",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Recipient => {
                                if r#recipient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recipient"));
                                }
                                r#recipient = Some(map_access.next_value()?);
                            }
                            Field::Date => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Participant => {
                                if r#participant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("participant"));
                                }
                                r#participant = Some(map_access.next_value()?);
                            }
                            Field::Issuer => {
                                if r#issuer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issuer"));
                                }
                                r#issuer = Some(map_access.next_value()?);
                            }
                            Field::Account => {
                                if r#account.is_some() {
                                    return Err(serde::de::Error::duplicate_field("account"));
                                }
                                r#account = Some(map_access.next_value()?);
                            }
                            Field::LineItem => {
                                if r#line_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lineItem"));
                                }
                                r#line_item = Some(map_access.next_value()?);
                            }
                            Field::TotalPriceComponent => {
                                if r#total_price_component.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "totalPriceComponent",
                                    ));
                                }
                                r#total_price_component = Some(map_access.next_value()?);
                            }
                            Field::TotalNet => {
                                if r#total_net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("totalNet"));
                                }
                                r#total_net = Some(map_access.next_value()?);
                            }
                            Field::TotalGross => {
                                if r#total_gross.is_some() {
                                    return Err(serde::de::Error::duplicate_field("totalGross"));
                                }
                                r#total_gross = Some(map_access.next_value()?);
                            }
                            Field::PaymentTerms => {
                                let some = r#payment_terms.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("paymentTerms"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PaymentTermsPrimitiveElement => {
                                let some = r#payment_terms.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_paymentTerms"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::json::de::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "status",
                                            "cancelledReason",
                                            "type",
                                            "subject",
                                            "recipient",
                                            "date",
                                            "participant",
                                            "issuer",
                                            "account",
                                            "lineItem",
                                            "totalPriceComponent",
                                            "totalNet",
                                            "totalGross",
                                            "paymentTerms",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(Invoice {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if config.mode == crate::json::de::DeserializationMode::Lax {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#cancelled_reason,
                        r#type,
                        r#subject,
                        r#recipient,
                        r#date,
                        r#participant: r#participant.unwrap_or(vec![]),
                        r#issuer,
                        r#account,
                        r#line_item: r#line_item.unwrap_or(vec![]),
                        r#total_price_component: r#total_price_component.unwrap_or(vec![]),
                        r#total_net,
                        r#total_gross,
                        r#payment_terms,
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
