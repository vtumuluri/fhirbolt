// Generated on 2023-04-16 by fhirbolt-codegen v0.2.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::SupplyRequestParameter,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("code", ctx))?;
        }
        if let Some(some) = self.value.r#value.as_ref() {
            match some {
                fhirbolt_model::r4::resources::SupplyRequestParameterValue::CodeableConcept(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4::resources::SupplyRequestParameterValue::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
                }
                fhirbolt_model::r4::resources::SupplyRequestParameterValue::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
                }
                fhirbolt_model::r4::resources::SupplyRequestParameterValue::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueBoolean", ctx))?;
                    }
                }
                fhirbolt_model::r4::resources::SupplyRequestParameterValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::SupplyRequestParameter>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::SupplyRequestParameter>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::SupplyRequestParameter>>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::SupplyRequestParameter,
    >
{
    type Value = fhirbolt_model::r4::resources::SupplyRequestParameter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::SupplyRequestParameter,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::SupplyRequestParameter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SupplyRequestParameter")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::SupplyRequestParameter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "code",
                            "valueCodeableConcept",
                            "valueQuantity",
                            "valueRange",
                            "valueBoolean",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#value: Option<
                    fhirbolt_model::r4::resources::SupplyRequestParameterValue,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4 :: resources :: SupplyRequestParameterValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r4 :: resources :: SupplyRequestParameterValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4::resources::SupplyRequestParameterValue::Range(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4::types::Range>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: SupplyRequestParameterValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: SupplyRequestParameterValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: SupplyRequestParameterValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: SupplyRequestParameterValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: SupplyRequestParameterValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4::resources::SupplyRequestParameter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::SupplyRequestParameter>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::SupplyRequestParameter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::SupplyRequestParameter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::SupplyRequestParameter>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::SupplyRequestParameter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::SupplyRequestParameter>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::SupplyRequestParameter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::SupplyRequestParameter>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::SupplyRequestParameter>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::SupplyRequestParameter>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::SupplyRequestParameter>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::SupplyRequestParameter>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4::resources::SupplyRequestParameter>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4::resources::SupplyRequest {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::SupplyRequest>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SupplyRequest")?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("implicitRules", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("language", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#language.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
            }
        }
        if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        if !self.value.r#contained.is_empty() {
            self.with_context(&self.value.r#contained, |ctx| {
                state.serialize_entry("contained", ctx)
            })?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#status.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_status", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#status.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
            }
        }
        if let Some(some) = self.value.r#category.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("category", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#priority.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("priority", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_priority", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#priority.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("priority", ctx))?;
            }
        }
        match self.value.r#item {
            fhirbolt_model::r4::resources::SupplyRequestItem::CodeableConcept(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("itemCodeableConcept", ctx)
                })?;
            }
            fhirbolt_model::r4::resources::SupplyRequestItem::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("itemReference", ctx))?;
            }
            fhirbolt_model::r4::resources::SupplyRequestItem::Invalid => {
                return Err(serde::ser::Error::custom("item is a required field"))
            }
        }
        self.with_context(&self.value.r#quantity, |ctx| {
            state.serialize_entry("quantity", ctx)
        })?;
        if !self.value.r#parameter.is_empty() {
            self.with_context(&self.value.r#parameter, |ctx| {
                state.serialize_entry("parameter", ctx)
            })?;
        }
        if let Some(some) = self.value.r#occurrence.as_ref() {
            match some {
                fhirbolt_model::r4::resources::SupplyRequestOccurrence::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("occurrenceDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_occurrenceDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("occurrenceDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4::resources::SupplyRequestOccurrence::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("occurrencePeriod", ctx))?;
                }
                fhirbolt_model::r4::resources::SupplyRequestOccurrence::Timing(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("occurrenceTiming", ctx))?;
                }
                fhirbolt_model::r4::resources::SupplyRequestOccurrence::Invalid => {
                    return Err(serde::ser::Error::custom("occurrence is invalid"))
                }
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#authored_on.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("authoredOn", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_authoredOn", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#authored_on.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("authoredOn", ctx))?;
            }
        }
        if let Some(some) = self.value.r#requester.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requester", ctx))?;
        }
        if !self.value.r#supplier.is_empty() {
            self.with_context(&self.value.r#supplier, |ctx| {
                state.serialize_entry("supplier", ctx)
            })?;
        }
        if !self.value.r#reason_code.is_empty() {
            self.with_context(&self.value.r#reason_code, |ctx| {
                state.serialize_entry("reasonCode", ctx)
            })?;
        }
        if !self.value.r#reason_reference.is_empty() {
            self.with_context(&self.value.r#reason_reference, |ctx| {
                state.serialize_entry("reasonReference", ctx)
            })?;
        }
        if let Some(some) = self.value.r#deliver_from.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("deliverFrom", ctx))?;
        }
        if let Some(some) = self.value.r#deliver_to.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("deliverTo", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::SupplyRequest>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::SupplyRequest>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::SupplyRequest>>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::SupplyRequest>
{
    type Value = fhirbolt_model::r4::resources::SupplyRequest;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::SupplyRequest,
    >
{
    type Value = fhirbolt_model::r4::resources::SupplyRequest;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::SupplyRequest,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::SupplyRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SupplyRequest")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::SupplyRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                            "category",
                            "priority",
                            "itemCodeableConcept",
                            "itemReference",
                            "quantity",
                            "parameter",
                            "occurrenceDateTime",
                            "occurrencePeriod",
                            "occurrenceTiming",
                            "authoredOn",
                            "requester",
                            "supplier",
                            "reasonCode",
                            "reasonReference",
                            "deliverFrom",
                            "deliverTo",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r4::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4::types::Identifier>>> =
                    None;
                let mut r#status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#category: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#priority: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#item: Option<fhirbolt_model::r4::resources::SupplyRequestItem> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#parameter: Option<
                    Vec<fhirbolt_model::r4::resources::SupplyRequestParameter>,
                > = None;
                let mut r#occurrence: Option<
                    fhirbolt_model::r4::resources::SupplyRequestOccurrence,
                > = None;
                let mut r#authored_on: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#requester: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#supplier: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#reason_code: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#reason_reference: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> =
                    None;
                let mut r#deliver_from: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#deliver_to: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
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
                            r#meta = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Meta>>(),
                            )?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                r#implicit_rules = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Uri>(),
                                )?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("language");
                            }
                        }
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<Box<fhirbolt_model::r4::Resource>>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::Status => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Priority => {
                            if self.0.from_json {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::PriorityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_priority"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("priority");
                            }
                        }
                        Field::ItemCodeableConcept => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "itemCodeableConcept",
                                ));
                            }
                            r#item = Some (fhirbolt_model :: r4 :: resources :: SupplyRequestItem :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ItemReference => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemReference"));
                            }
                            r#item =
                                Some(fhirbolt_model::r4::resources::SupplyRequestItem::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::Parameter => {
                            if self.0.from_json {
                                if r#parameter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parameter"));
                                }
                                r#parameter =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::SupplyRequestParameter,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#parameter.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: SupplyRequestParameter > ()) ?) ;
                            }
                        }
                        Field::OccurrenceDateTime => {
                            if self.0.from_json {
                                let r#enum = r#occurrence . get_or_insert (fhirbolt_model :: r4 :: resources :: SupplyRequestOccurrence :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: SupplyRequestOccurrence :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("occurrenceDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("occurrence[x]")) ; }
                            } else {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceDateTime",
                                    ));
                                }
                                r#occurrence = Some (fhirbolt_model :: r4 :: resources :: SupplyRequestOccurrence :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::OccurrenceDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#occurrence . get_or_insert (fhirbolt_model :: r4 :: resources :: SupplyRequestOccurrence :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: SupplyRequestOccurrence :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_occurrenceDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_occurrence[x]")) ; }
                            } else {
                                return unknown_field_error("occurrenceDateTime");
                            }
                        }
                        Field::OccurrencePeriod => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrencePeriod"));
                            }
                            r#occurrence = Some(
                                fhirbolt_model::r4::resources::SupplyRequestOccurrence::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Period>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::OccurrenceTiming => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrenceTiming"));
                            }
                            r#occurrence = Some(
                                fhirbolt_model::r4::resources::SupplyRequestOccurrence::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Timing>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::AuthoredOn => {
                            if self.0.from_json {
                                let some = r#authored_on.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authoredOn"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#authored_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authoredOn"));
                                }
                                r#authored_on = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::AuthoredOnPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#authored_on.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_authoredOn"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("authoredOn");
                            }
                        }
                        Field::Requester => {
                            if r#requester.is_some() {
                                return Err(serde::de::Error::duplicate_field("requester"));
                            }
                            r#requester = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Supplier => {
                            if self.0.from_json {
                                if r#supplier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("supplier"));
                                }
                                r#supplier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#supplier.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ReasonCode => {
                            if self.0.from_json {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#reason_code.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::ReasonReference => {
                            if self.0.from_json {
                                if r#reason_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#reason_reference.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::DeliverFrom => {
                            if r#deliver_from.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliverFrom"));
                            }
                            r#deliver_from = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::DeliverTo => {
                            if r#deliver_to.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliverTo"));
                            }
                            r#deliver_to = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4::resources::SupplyRequest {
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
                    r#item: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#item.unwrap_or(Default::default())
                    } else {
                        r#item.ok_or(serde::de::Error::missing_field("item[x]"))?
                    },
                    r#quantity: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#quantity.unwrap_or(Default::default())
                    } else {
                        r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?
                    },
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
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::SupplyRequest>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::SupplyRequest>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::SupplyRequest>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::SupplyRequest>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::SupplyRequest>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::SupplyRequest>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::SupplyRequest>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::SupplyRequest>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::SupplyRequest>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::SupplyRequest>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::SupplyRequest>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::SupplyRequest>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4::resources::SupplyRequest>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
