// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SupplyRequestItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for SupplyRequestItem {
    fn default() -> SupplyRequestItem {
        SupplyRequestItem::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum SupplyRequestParameterValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Boolean(Box<super::super::types::Boolean>),
    Invalid,
}
impl Default for SupplyRequestParameterValue {
    fn default() -> SupplyRequestParameterValue {
        SupplyRequestParameterValue::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum SupplyRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for SupplyRequestOccurrence {
    fn default() -> SupplyRequestOccurrence {
        SupplyRequestOccurrence::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct SupplyRequestParameter {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#value: Option<SupplyRequestParameterValue>,
}
impl serde::ser::Serialize for SupplyRequestParameter {
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
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                SupplyRequestParameterValue::CodeableConcept(ref value) => {
                    state.serialize_entry("valueCodeableConcept", value)?;
                }
                SupplyRequestParameterValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                SupplyRequestParameterValue::Range(ref value) => {
                    state.serialize_entry("valueRange", value)?;
                }
                SupplyRequestParameterValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueBoolean", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBoolean", &primitive_element)?;
                    }
                }
                SupplyRequestParameterValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SupplyRequestParameter {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SupplyRequestParameter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SupplyRequestParameter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SupplyRequestParameter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<SupplyRequestParameterValue> = None;
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some(SupplyRequestParameterValue::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(SupplyRequestParameterValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value =
                                Some(SupplyRequestParameterValue::Range(map_access.next_value()?));
                        }
                        Field::ValueBoolean => {
                            let r#enum = r#value.get_or_insert(
                                SupplyRequestParameterValue::Boolean(Default::default()),
                            );
                            if let SupplyRequestParameterValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                let value: _ = map_access.next_value()?;
                                variant.value = Some(value);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                SupplyRequestParameterValue::Boolean(Default::default()),
                            );
                            if let SupplyRequestParameterValue::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                    }
                }
                Ok(SupplyRequestParameter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SupplyRequest {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#item: SupplyRequestItem,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#parameter: Vec<SupplyRequestParameter>,
    pub r#occurrence: Option<SupplyRequestOccurrence>,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#supplier: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#deliver_from: Option<Box<super::super::types::Reference>>,
    pub r#deliver_to: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for SupplyRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SupplyRequest")?;
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
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("status", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        if let Some(some) = self.r#priority.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("priority", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_priority", &primitive_element)?;
            }
        }
        match self.r#item {
            SupplyRequestItem::CodeableConcept(ref value) => {
                state.serialize_entry("itemCodeableConcept", value)?;
            }
            SupplyRequestItem::Reference(ref value) => {
                state.serialize_entry("itemReference", value)?;
            }
            SupplyRequestItem::Invalid => {
                return Err(serde::ser::Error::custom("item is a required field"))
            }
        }
        state.serialize_entry("quantity", &self.r#quantity)?;
        if !self.r#parameter.is_empty() {
            state.serialize_entry("parameter", &self.r#parameter)?;
        }
        if let Some(some) = self.r#occurrence.as_ref() {
            match some {
                SupplyRequestOccurrence::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("occurrenceDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                    }
                }
                SupplyRequestOccurrence::Period(ref value) => {
                    state.serialize_entry("occurrencePeriod", value)?;
                }
                SupplyRequestOccurrence::Timing(ref value) => {
                    state.serialize_entry("occurrenceTiming", value)?;
                }
                SupplyRequestOccurrence::Invalid => {
                    return Err(serde::ser::Error::custom("occurrence is invalid"))
                }
            }
        }
        if let Some(some) = self.r#authored_on.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("authoredOn", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_authoredOn", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#requester.as_ref() {
            state.serialize_entry("requester", some)?;
        }
        if !self.r#supplier.is_empty() {
            state.serialize_entry("supplier", &self.r#supplier)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if let Some(some) = self.r#deliver_from.as_ref() {
            state.serialize_entry("deliverFrom", some)?;
        }
        if let Some(some) = self.r#deliver_to.as_ref() {
            state.serialize_entry("deliverTo", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SupplyRequest {
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
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "_priority")]
            PriorityPrimitiveElement,
            #[serde(rename = "itemCodeableConcept")]
            ItemCodeableConcept,
            #[serde(rename = "itemReference")]
            ItemReference,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "parameter")]
            Parameter,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "occurrencePeriod")]
            OccurrencePeriod,
            #[serde(rename = "occurrenceTiming")]
            OccurrenceTiming,
            #[serde(rename = "authoredOn")]
            AuthoredOn,
            #[serde(rename = "_authoredOn")]
            AuthoredOnPrimitiveElement,
            #[serde(rename = "requester")]
            Requester,
            #[serde(rename = "supplier")]
            Supplier,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "deliverFrom")]
            DeliverFrom,
            #[serde(rename = "deliverTo")]
            DeliverTo,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SupplyRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SupplyRequest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SupplyRequest, V::Error>
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
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#item: Option<SupplyRequestItem> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#parameter: Option<Vec<SupplyRequestParameter>> = None;
                let mut r#occurrence: Option<SupplyRequestOccurrence> = None;
                let mut r#authored_on: Option<super::super::types::DateTime> = None;
                let mut r#requester: Option<Box<super::super::types::Reference>> = None;
                let mut r#supplier: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#deliver_from: Option<Box<super::super::types::Reference>> = None;
                let mut r#deliver_to: Option<Box<super::super::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "SupplyRequest" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"SupplyRequest",
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        Field::Priority => {
                            let some = r#priority.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::PriorityPrimitiveElement => {
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
                        Field::ItemCodeableConcept => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "itemCodeableConcept",
                                ));
                            }
                            r#item =
                                Some(SupplyRequestItem::CodeableConcept(map_access.next_value()?));
                        }
                        Field::ItemReference => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemReference"));
                            }
                            r#item = Some(SupplyRequestItem::Reference(map_access.next_value()?));
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        Field::Parameter => {
                            if r#parameter.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            r#parameter = Some(map_access.next_value()?);
                        }
                        Field::OccurrenceDateTime => {
                            let r#enum = r#occurrence.get_or_insert(
                                SupplyRequestOccurrence::DateTime(Default::default()),
                            );
                            if let SupplyRequestOccurrence::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceDateTime",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                variant.value = Some(value);
                            } else {
                                return Err(serde::de::Error::duplicate_field("occurrence[x]"));
                            }
                        }
                        Field::OccurrenceDateTimePrimitiveElement => {
                            let r#enum = r#occurrence.get_or_insert(
                                SupplyRequestOccurrence::DateTime(Default::default()),
                            );
                            if let SupplyRequestOccurrence::DateTime(variant) = r#enum {
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
                        Field::OccurrencePeriod => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrencePeriod"));
                            }
                            r#occurrence =
                                Some(SupplyRequestOccurrence::Period(map_access.next_value()?));
                        }
                        Field::OccurrenceTiming => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrenceTiming"));
                            }
                            r#occurrence =
                                Some(SupplyRequestOccurrence::Timing(map_access.next_value()?));
                        }
                        Field::AuthoredOn => {
                            let some = r#authored_on.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoredOn"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::AuthoredOnPrimitiveElement => {
                            let some = r#authored_on.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_authoredOn"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Requester => {
                            if r#requester.is_some() {
                                return Err(serde::de::Error::duplicate_field("requester"));
                            }
                            r#requester = Some(map_access.next_value()?);
                        }
                        Field::Supplier => {
                            if r#supplier.is_some() {
                                return Err(serde::de::Error::duplicate_field("supplier"));
                            }
                            r#supplier = Some(map_access.next_value()?);
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
                        Field::DeliverFrom => {
                            if r#deliver_from.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliverFrom"));
                            }
                            r#deliver_from = Some(map_access.next_value()?);
                        }
                        Field::DeliverTo => {
                            if r#deliver_to.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliverTo"));
                            }
                            r#deliver_to = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SupplyRequest {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status,
                    r#category,
                    r#priority,
                    r#item: r#item.ok_or(serde::de::Error::missing_field("item[x]"))?,
                    r#quantity: r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?,
                    r#parameter: r#parameter.unwrap_or(vec![]),
                    r#occurrence,
                    r#authored_on,
                    r#requester,
                    r#supplier: r#supplier.unwrap_or(vec![]),
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#deliver_from,
                    r#deliver_to,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
