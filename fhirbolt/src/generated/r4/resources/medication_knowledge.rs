// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationKnowledgeIngredientItem {
    fn default() -> MedicationKnowledgeIngredientItem {
        MedicationKnowledgeIngredientItem::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationKnowledgeAdministrationGuidelinesIndication {
    fn default() -> MedicationKnowledgeAdministrationGuidelinesIndication {
        MedicationKnowledgeAdministrationGuidelinesIndication::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic {
    fn default() -> MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic
    {
        MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationKnowledgeDrugCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    Invalid,
}
impl Default for MedicationKnowledgeDrugCharacteristicValue {
    fn default() -> MedicationKnowledgeDrugCharacteristicValue {
        MedicationKnowledgeDrugCharacteristicValue::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeRelatedMedicationKnowledge {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationKnowledgeRelatedMedicationKnowledge {
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
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#reference.is_empty() {
            state.serialize_entry("reference", &self.r#reference)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeRelatedMedicationKnowledge {
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
            #[serde(rename = "reference")]
            Reference,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeRelatedMedicationKnowledge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRelatedMedicationKnowledge")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeRelatedMedicationKnowledge, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reference: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Reference => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeRelatedMedicationKnowledge {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#reference: r#reference.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeMonograph {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationKnowledgeMonograph {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#source.as_ref() {
            state.serialize_entry("source", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeMonograph {
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
            #[serde(rename = "source")]
            Source,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeMonograph;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMonograph")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeMonograph, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#source: Option<Box<super::super::types::Reference>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeMonograph {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#source,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeIngredient {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item: MedicationKnowledgeIngredientItem,
    pub r#is_active: Option<super::super::types::Boolean>,
    pub r#strength: Option<Box<super::super::types::Ratio>>,
}
impl serde::ser::Serialize for MedicationKnowledgeIngredient {
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
        match self.r#item {
            MedicationKnowledgeIngredientItem::CodeableConcept(ref value) => {
                state.serialize_entry("itemCodeableConcept", value)?;
            }
            MedicationKnowledgeIngredientItem::Reference(ref value) => {
                state.serialize_entry("itemReference", value)?;
            }
            MedicationKnowledgeIngredientItem::Invalid => {
                return Err(serde::ser::Error::custom("item is a required field"))
            }
        }
        if let Some(some) = self.r#is_active.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("isActive", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_isActive", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#strength.as_ref() {
            state.serialize_entry("strength", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeIngredient {
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
            #[serde(rename = "itemCodeableConcept")]
            ItemCodeableConcept,
            #[serde(rename = "itemReference")]
            ItemReference,
            #[serde(rename = "isActive")]
            IsActive,
            #[serde(rename = "_isActive")]
            IsActivePrimitiveElement,
            #[serde(rename = "strength")]
            Strength,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeIngredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeIngredient")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeIngredient, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item: Option<MedicationKnowledgeIngredientItem> = None;
                let mut r#is_active: Option<super::super::types::Boolean> = None;
                let mut r#strength: Option<Box<super::super::types::Ratio>> = None;
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
                        Field::ItemCodeableConcept => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "itemCodeableConcept",
                                ));
                            }
                            r#item = Some(MedicationKnowledgeIngredientItem::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ItemReference => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemReference"));
                            }
                            r#item = Some(MedicationKnowledgeIngredientItem::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::IsActive => {
                            let some = r#is_active.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IsActivePrimitiveElement => {
                            let some = r#is_active.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_isActive"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Strength => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            r#strength = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeIngredient {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item: r#item.ok_or(serde::de::Error::missing_field("item[x]"))?,
                    r#is_active,
                    r#strength,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeCost {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#source: Option<super::super::types::String>,
    pub r#cost: Box<super::super::types::Money>,
}
impl serde::ser::Serialize for MedicationKnowledgeCost {
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
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#source.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_source", &primitive_element)?;
            }
        }
        state.serialize_entry("cost", &self.r#cost)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeCost {
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
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "_source")]
            SourcePrimitiveElement,
            #[serde(rename = "cost")]
            Cost,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeCost;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeCost")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicationKnowledgeCost, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#source: Option<super::super::types::String> = None;
                let mut r#cost: Option<Box<super::super::types::Money>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Source => {
                            let some = r#source.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SourcePrimitiveElement => {
                            let some = r#source.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Cost => {
                            if r#cost.is_some() {
                                return Err(serde::de::Error::duplicate_field("cost"));
                            }
                            r#cost = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeCost {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#source,
                    r#cost: r#cost.ok_or(serde::de::Error::missing_field("cost"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeMonitoringProgram {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#name: Option<super::super::types::String>,
}
impl serde::ser::Serialize for MedicationKnowledgeMonitoringProgram {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
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
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeMonitoringProgram {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeMonitoringProgram;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMonitoringProgram")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeMonitoringProgram, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#name: Option<super::super::types::String> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
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
                    }
                }
                Ok(MedicationKnowledgeMonitoringProgram {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#name,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesDosage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
}
impl serde::ser::Serialize for MedicationKnowledgeAdministrationGuidelinesDosage {
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
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#dosage.is_empty() {
            state.serialize_entry("dosage", &self.r#dosage)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeAdministrationGuidelinesDosage {
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
            #[serde(rename = "dosage")]
            Dosage,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeAdministrationGuidelinesDosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeAdministrationGuidelinesDosage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeAdministrationGuidelinesDosage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#dosage: Option<Vec<Box<super::super::types::Dosage>>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Dosage => {
                            if r#dosage.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosage"));
                            }
                            r#dosage = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeAdministrationGuidelinesDosage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#dosage: r#dosage.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#characteristic:
        MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic,
    pub r#value: Vec<super::super::types::String>,
}
impl serde::ser::Serialize for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
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
        match self . r#characteristic { MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: CodeableConcept (ref value) => { state . serialize_entry ("characteristicCodeableConcept" , value) ? ; } , MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: Quantity (ref value) => { state . serialize_entry ("characteristicQuantity" , value) ? ; } , MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: Invalid => { return Err (serde :: ser :: Error :: custom ("characteristic is a required field")) } }
        if !self.r#value.is_empty() {
            let values: Vec<_> = self.r#value.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("value", &values)?;
            }
            let requires_elements = self
                .r#value
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#value
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
                state.serialize_entry("_value", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de>
    for MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics
{
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
            #[serde(rename = "characteristicCodeableConcept")]
            CharacteristicCodeableConcept,
            #[serde(rename = "characteristicQuantity")]
            CharacteristicQuantity,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter
                    .write_str("MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#characteristic: Option<
                    MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic,
                > = None;
                let mut r#value: Option<Vec<super::super::types::String>> = None;
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
                        Field::CharacteristicCodeableConcept => {
                            if r#characteristic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "characteristicCodeableConcept",
                                ));
                            }
                            r#characteristic = Some (MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: CodeableConcept (map_access . next_value () ?)) ;
                        }
                        Field::CharacteristicQuantity => {
                            if r#characteristic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "characteristicQuantity",
                                ));
                            }
                            r#characteristic = Some (MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: Quantity (map_access . next_value () ?)) ;
                        }
                        Field::Value => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#value.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#value.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_value"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                    }
                }
                Ok(
                    MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#characteristic: r#characteristic
                            .ok_or(serde::de::Error::missing_field("characteristic[x]"))?,
                        r#value: r#value.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeAdministrationGuidelines {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#dosage: Vec<MedicationKnowledgeAdministrationGuidelinesDosage>,
    pub r#indication: Option<MedicationKnowledgeAdministrationGuidelinesIndication>,
    pub r#patient_characteristics:
        Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>,
}
impl serde::ser::Serialize for MedicationKnowledgeAdministrationGuidelines {
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
        if !self.r#dosage.is_empty() {
            state.serialize_entry("dosage", &self.r#dosage)?;
        }
        if let Some(some) = self.r#indication.as_ref() {
            match some {
                MedicationKnowledgeAdministrationGuidelinesIndication::CodeableConcept(
                    ref value,
                ) => {
                    state.serialize_entry("indicationCodeableConcept", value)?;
                }
                MedicationKnowledgeAdministrationGuidelinesIndication::Reference(ref value) => {
                    state.serialize_entry("indicationReference", value)?;
                }
                MedicationKnowledgeAdministrationGuidelinesIndication::Invalid => {
                    return Err(serde::ser::Error::custom("indication is invalid"))
                }
            }
        }
        if !self.r#patient_characteristics.is_empty() {
            state.serialize_entry("patientCharacteristics", &self.r#patient_characteristics)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeAdministrationGuidelines {
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
            #[serde(rename = "dosage")]
            Dosage,
            #[serde(rename = "indicationCodeableConcept")]
            IndicationCodeableConcept,
            #[serde(rename = "indicationReference")]
            IndicationReference,
            #[serde(rename = "patientCharacteristics")]
            PatientCharacteristics,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeAdministrationGuidelines;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeAdministrationGuidelines")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeAdministrationGuidelines, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#dosage: Option<Vec<MedicationKnowledgeAdministrationGuidelinesDosage>> =
                    None;
                let mut r#indication: Option<
                    MedicationKnowledgeAdministrationGuidelinesIndication,
                > = None;
                let mut r#patient_characteristics: Option<
                    Vec<MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics>,
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
                        Field::Dosage => {
                            if r#dosage.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosage"));
                            }
                            r#dosage = Some(map_access.next_value()?);
                        }
                        Field::IndicationCodeableConcept => {
                            if r#indication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "indicationCodeableConcept",
                                ));
                            }
                            r#indication = Some (MedicationKnowledgeAdministrationGuidelinesIndication :: CodeableConcept (map_access . next_value () ?)) ;
                        }
                        Field::IndicationReference => {
                            if r#indication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "indicationReference",
                                ));
                            }
                            r#indication = Some(
                                MedicationKnowledgeAdministrationGuidelinesIndication::Reference(
                                    map_access.next_value()?,
                                ),
                            );
                        }
                        Field::PatientCharacteristics => {
                            if r#patient_characteristics.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patientCharacteristics",
                                ));
                            }
                            r#patient_characteristics = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeAdministrationGuidelines {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#dosage: r#dosage.unwrap_or(vec![]),
                    r#indication,
                    r#patient_characteristics: r#patient_characteristics.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeMedicineClassification {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for MedicationKnowledgeMedicineClassification {
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
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#classification.is_empty() {
            state.serialize_entry("classification", &self.r#classification)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeMedicineClassification {
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
            #[serde(rename = "classification")]
            Classification,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeMedicineClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMedicineClassification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeMedicineClassification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#classification: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Classification => {
                            if r#classification.is_some() {
                                return Err(serde::de::Error::duplicate_field("classification"));
                            }
                            r#classification = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeMedicineClassification {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#classification: r#classification.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgePackaging {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
}
impl serde::ser::Serialize for MedicationKnowledgePackaging {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgePackaging {
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
            #[serde(rename = "quantity")]
            Quantity,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgePackaging;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgePackaging")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgePackaging, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgePackaging {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#quantity,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeDrugCharacteristic {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#value: Option<MedicationKnowledgeDrugCharacteristicValue>,
}
impl serde::ser::Serialize for MedicationKnowledgeDrugCharacteristic {
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                MedicationKnowledgeDrugCharacteristicValue::CodeableConcept(ref value) => {
                    state.serialize_entry("valueCodeableConcept", value)?;
                }
                MedicationKnowledgeDrugCharacteristicValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                MedicationKnowledgeDrugCharacteristicValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                MedicationKnowledgeDrugCharacteristicValue::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                    }
                }
                MedicationKnowledgeDrugCharacteristicValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeDrugCharacteristic {
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
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueBase64Binary")]
            ValueBase64Binary,
            #[serde(rename = "_valueBase64Binary")]
            ValueBase64BinaryPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeDrugCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeDrugCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeDrugCharacteristic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<MedicationKnowledgeDrugCharacteristicValue> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value =
                                Some(MedicationKnowledgeDrugCharacteristicValue::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                        }
                        Field::ValueString => {
                            let r#enum = r#value.get_or_insert(
                                MedicationKnowledgeDrugCharacteristicValue::String(
                                    Default::default(),
                                ),
                            );
                            if let MedicationKnowledgeDrugCharacteristicValue::String(variant) =
                                r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                MedicationKnowledgeDrugCharacteristicValue::String(
                                    Default::default(),
                                ),
                            );
                            if let MedicationKnowledgeDrugCharacteristicValue::String(variant) =
                                r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
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
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(MedicationKnowledgeDrugCharacteristicValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        Field::ValueBase64Binary => {
                            let r#enum = r#value.get_or_insert(
                                MedicationKnowledgeDrugCharacteristicValue::Base64Binary(
                                    Default::default(),
                                ),
                            );
                            if let MedicationKnowledgeDrugCharacteristicValue::Base64Binary(
                                variant,
                            ) = r#enum
                            {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                MedicationKnowledgeDrugCharacteristicValue::Base64Binary(
                                    Default::default(),
                                ),
                            );
                            if let MedicationKnowledgeDrugCharacteristicValue::Base64Binary(
                                variant,
                            ) = r#enum
                            {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueBase64Binary",
                                    ));
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
                Ok(MedicationKnowledgeDrugCharacteristic {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySubstitution {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#allowed: super::super::types::Boolean,
}
impl serde::ser::Serialize for MedicationKnowledgeRegulatorySubstitution {
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
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#allowed.value.as_ref() {
            state.serialize_entry("allowed", some)?;
        }
        if self.r#allowed.id.is_some() || !self.r#allowed.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#allowed.id,
                extension: &self.r#allowed.extension,
            };
            state.serialize_entry("_allowed", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeRegulatorySubstitution {
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
            #[serde(rename = "allowed")]
            Allowed,
            #[serde(rename = "_allowed")]
            AllowedPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeRegulatorySubstitution;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatorySubstitution")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeRegulatorySubstitution, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#allowed: Option<super::super::types::Boolean> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Allowed => {
                            let some = r#allowed.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowed"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AllowedPrimitiveElement => {
                            let some = r#allowed.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_allowed"));
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
                Ok(MedicationKnowledgeRegulatorySubstitution {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#allowed: r#allowed.ok_or(serde::de::Error::missing_field("allowed"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeRegulatorySchedule {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for MedicationKnowledgeRegulatorySchedule {
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
        state.serialize_entry("schedule", &self.r#schedule)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeRegulatorySchedule {
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
            #[serde(rename = "schedule")]
            Schedule,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeRegulatorySchedule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatorySchedule")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeRegulatorySchedule, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#schedule: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::Schedule => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            r#schedule = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeRegulatorySchedule {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#schedule: r#schedule.ok_or(serde::de::Error::missing_field("schedule"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeRegulatoryMaxDispense {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#period: Option<Box<super::super::types::Duration>>,
}
impl serde::ser::Serialize for MedicationKnowledgeRegulatoryMaxDispense {
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
        state.serialize_entry("quantity", &self.r#quantity)?;
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeRegulatoryMaxDispense {
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
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "period")]
            Period,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeRegulatoryMaxDispense;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatoryMaxDispense")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeRegulatoryMaxDispense, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#period: Option<Box<super::super::types::Duration>> = None;
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
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeRegulatoryMaxDispense {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#quantity: r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?,
                    r#period,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeRegulatory {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#regulatory_authority: Box<super::super::types::Reference>,
    pub r#substitution: Vec<MedicationKnowledgeRegulatorySubstitution>,
    pub r#schedule: Vec<MedicationKnowledgeRegulatorySchedule>,
    pub r#max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense>,
}
impl serde::ser::Serialize for MedicationKnowledgeRegulatory {
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
        state.serialize_entry("regulatoryAuthority", &self.r#regulatory_authority)?;
        if !self.r#substitution.is_empty() {
            state.serialize_entry("substitution", &self.r#substitution)?;
        }
        if !self.r#schedule.is_empty() {
            state.serialize_entry("schedule", &self.r#schedule)?;
        }
        if let Some(some) = self.r#max_dispense.as_ref() {
            state.serialize_entry("maxDispense", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeRegulatory {
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
            #[serde(rename = "regulatoryAuthority")]
            RegulatoryAuthority,
            #[serde(rename = "substitution")]
            Substitution,
            #[serde(rename = "schedule")]
            Schedule,
            #[serde(rename = "maxDispense")]
            MaxDispense,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeRegulatory;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatory")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeRegulatory, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#regulatory_authority: Option<Box<super::super::types::Reference>> = None;
                let mut r#substitution: Option<Vec<MedicationKnowledgeRegulatorySubstitution>> =
                    None;
                let mut r#schedule: Option<Vec<MedicationKnowledgeRegulatorySchedule>> = None;
                let mut r#max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispense> = None;
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
                        Field::RegulatoryAuthority => {
                            if r#regulatory_authority.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "regulatoryAuthority",
                                ));
                            }
                            r#regulatory_authority = Some(map_access.next_value()?);
                        }
                        Field::Substitution => {
                            if r#substitution.is_some() {
                                return Err(serde::de::Error::duplicate_field("substitution"));
                            }
                            r#substitution = Some(map_access.next_value()?);
                        }
                        Field::Schedule => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            r#schedule = Some(map_access.next_value()?);
                        }
                        Field::MaxDispense => {
                            if r#max_dispense.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDispense"));
                            }
                            r#max_dispense = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeRegulatory {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#regulatory_authority: r#regulatory_authority
                        .ok_or(serde::de::Error::missing_field("regulatoryAuthority"))?,
                    r#substitution: r#substitution.unwrap_or(vec![]),
                    r#schedule: r#schedule.unwrap_or(vec![]),
                    r#max_dispense,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledgeKinetics {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#area_under_curve: Vec<Box<super::super::types::Quantity>>,
    pub r#lethal_dose_50: Vec<Box<super::super::types::Quantity>>,
    pub r#half_life_period: Option<Box<super::super::types::Duration>>,
}
impl serde::ser::Serialize for MedicationKnowledgeKinetics {
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
        if !self.r#area_under_curve.is_empty() {
            state.serialize_entry("areaUnderCurve", &self.r#area_under_curve)?;
        }
        if !self.r#lethal_dose_50.is_empty() {
            state.serialize_entry("lethalDose50", &self.r#lethal_dose_50)?;
        }
        if let Some(some) = self.r#half_life_period.as_ref() {
            state.serialize_entry("halfLifePeriod", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledgeKinetics {
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
            #[serde(rename = "areaUnderCurve")]
            AreaUnderCurve,
            #[serde(rename = "lethalDose50")]
            LethalDose50,
            #[serde(rename = "halfLifePeriod")]
            HalfLifePeriod,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledgeKinetics;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeKinetics")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationKnowledgeKinetics, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#area_under_curve: Option<Vec<Box<super::super::types::Quantity>>> = None;
                let mut r#lethal_dose_50: Option<Vec<Box<super::super::types::Quantity>>> = None;
                let mut r#half_life_period: Option<Box<super::super::types::Duration>> = None;
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
                        Field::AreaUnderCurve => {
                            if r#area_under_curve.is_some() {
                                return Err(serde::de::Error::duplicate_field("areaUnderCurve"));
                            }
                            r#area_under_curve = Some(map_access.next_value()?);
                        }
                        Field::LethalDose50 => {
                            if r#lethal_dose_50.is_some() {
                                return Err(serde::de::Error::duplicate_field("lethalDose50"));
                            }
                            r#lethal_dose_50 = Some(map_access.next_value()?);
                        }
                        Field::HalfLifePeriod => {
                            if r#half_life_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("halfLifePeriod"));
                            }
                            r#half_life_period = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledgeKinetics {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#area_under_curve: r#area_under_curve.unwrap_or(vec![]),
                    r#lethal_dose_50: r#lethal_dose_50.unwrap_or(vec![]),
                    r#half_life_period,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationKnowledge {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#dose_form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    pub r#synonym: Vec<super::super::types::String>,
    pub r#related_medication_knowledge: Vec<MedicationKnowledgeRelatedMedicationKnowledge>,
    pub r#associated_medication: Vec<Box<super::super::types::Reference>>,
    pub r#product_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#monograph: Vec<MedicationKnowledgeMonograph>,
    pub r#ingredient: Vec<MedicationKnowledgeIngredient>,
    pub r#preparation_instruction: Option<super::super::types::Markdown>,
    pub r#intended_route: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#cost: Vec<MedicationKnowledgeCost>,
    pub r#monitoring_program: Vec<MedicationKnowledgeMonitoringProgram>,
    pub r#administration_guidelines: Vec<MedicationKnowledgeAdministrationGuidelines>,
    pub r#medicine_classification: Vec<MedicationKnowledgeMedicineClassification>,
    pub r#packaging: Option<MedicationKnowledgePackaging>,
    pub r#drug_characteristic: Vec<MedicationKnowledgeDrugCharacteristic>,
    pub r#contraindication: Vec<Box<super::super::types::Reference>>,
    pub r#regulatory: Vec<MedicationKnowledgeRegulatory>,
    pub r#kinetics: Vec<MedicationKnowledgeKinetics>,
}
impl serde::ser::Serialize for MedicationKnowledge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationKnowledge")?;
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
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#manufacturer.as_ref() {
            state.serialize_entry("manufacturer", some)?;
        }
        if let Some(some) = self.r#dose_form.as_ref() {
            state.serialize_entry("doseForm", some)?;
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        if !self.r#synonym.is_empty() {
            let values: Vec<_> = self.r#synonym.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("synonym", &values)?;
            }
            let requires_elements = self
                .r#synonym
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#synonym
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
                state.serialize_entry("_synonym", &primitive_elements)?;
            }
        }
        if !self.r#related_medication_knowledge.is_empty() {
            state.serialize_entry(
                "relatedMedicationKnowledge",
                &self.r#related_medication_knowledge,
            )?;
        }
        if !self.r#associated_medication.is_empty() {
            state.serialize_entry("associatedMedication", &self.r#associated_medication)?;
        }
        if !self.r#product_type.is_empty() {
            state.serialize_entry("productType", &self.r#product_type)?;
        }
        if !self.r#monograph.is_empty() {
            state.serialize_entry("monograph", &self.r#monograph)?;
        }
        if !self.r#ingredient.is_empty() {
            state.serialize_entry("ingredient", &self.r#ingredient)?;
        }
        if let Some(some) = self.r#preparation_instruction.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preparationInstruction", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preparationInstruction", &primitive_element)?;
            }
        }
        if !self.r#intended_route.is_empty() {
            state.serialize_entry("intendedRoute", &self.r#intended_route)?;
        }
        if !self.r#cost.is_empty() {
            state.serialize_entry("cost", &self.r#cost)?;
        }
        if !self.r#monitoring_program.is_empty() {
            state.serialize_entry("monitoringProgram", &self.r#monitoring_program)?;
        }
        if !self.r#administration_guidelines.is_empty() {
            state.serialize_entry(
                "administrationGuidelines",
                &self.r#administration_guidelines,
            )?;
        }
        if !self.r#medicine_classification.is_empty() {
            state.serialize_entry("medicineClassification", &self.r#medicine_classification)?;
        }
        if let Some(some) = self.r#packaging.as_ref() {
            state.serialize_entry("packaging", some)?;
        }
        if !self.r#drug_characteristic.is_empty() {
            state.serialize_entry("drugCharacteristic", &self.r#drug_characteristic)?;
        }
        if !self.r#contraindication.is_empty() {
            state.serialize_entry("contraindication", &self.r#contraindication)?;
        }
        if !self.r#regulatory.is_empty() {
            state.serialize_entry("regulatory", &self.r#regulatory)?;
        }
        if !self.r#kinetics.is_empty() {
            state.serialize_entry("kinetics", &self.r#kinetics)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationKnowledge {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "doseForm")]
            DoseForm,
            #[serde(rename = "amount")]
            Amount,
            #[serde(rename = "synonym")]
            Synonym,
            #[serde(rename = "_synonym")]
            SynonymPrimitiveElement,
            #[serde(rename = "relatedMedicationKnowledge")]
            RelatedMedicationKnowledge,
            #[serde(rename = "associatedMedication")]
            AssociatedMedication,
            #[serde(rename = "productType")]
            ProductType,
            #[serde(rename = "monograph")]
            Monograph,
            #[serde(rename = "ingredient")]
            Ingredient,
            #[serde(rename = "preparationInstruction")]
            PreparationInstruction,
            #[serde(rename = "_preparationInstruction")]
            PreparationInstructionPrimitiveElement,
            #[serde(rename = "intendedRoute")]
            IntendedRoute,
            #[serde(rename = "cost")]
            Cost,
            #[serde(rename = "monitoringProgram")]
            MonitoringProgram,
            #[serde(rename = "administrationGuidelines")]
            AdministrationGuidelines,
            #[serde(rename = "medicineClassification")]
            MedicineClassification,
            #[serde(rename = "packaging")]
            Packaging,
            #[serde(rename = "drugCharacteristic")]
            DrugCharacteristic,
            #[serde(rename = "contraindication")]
            Contraindication,
            #[serde(rename = "regulatory")]
            Regulatory,
            #[serde(rename = "kinetics")]
            Kinetics,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationKnowledge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledge")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicationKnowledge, V::Error>
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
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#manufacturer: Option<Box<super::super::types::Reference>> = None;
                let mut r#dose_form: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::Quantity>> = None;
                let mut r#synonym: Option<Vec<super::super::types::String>> = None;
                let mut r#related_medication_knowledge: Option<
                    Vec<MedicationKnowledgeRelatedMedicationKnowledge>,
                > = None;
                let mut r#associated_medication: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#product_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#monograph: Option<Vec<MedicationKnowledgeMonograph>> = None;
                let mut r#ingredient: Option<Vec<MedicationKnowledgeIngredient>> = None;
                let mut r#preparation_instruction: Option<super::super::types::Markdown> = None;
                let mut r#intended_route: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#cost: Option<Vec<MedicationKnowledgeCost>> = None;
                let mut r#monitoring_program: Option<Vec<MedicationKnowledgeMonitoringProgram>> =
                    None;
                let mut r#administration_guidelines: Option<
                    Vec<MedicationKnowledgeAdministrationGuidelines>,
                > = None;
                let mut r#medicine_classification: Option<
                    Vec<MedicationKnowledgeMedicineClassification>,
                > = None;
                let mut r#packaging: Option<MedicationKnowledgePackaging> = None;
                let mut r#drug_characteristic: Option<Vec<MedicationKnowledgeDrugCharacteristic>> =
                    None;
                let mut r#contraindication: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#regulatory: Option<Vec<MedicationKnowledgeRegulatory>> = None;
                let mut r#kinetics: Option<Vec<MedicationKnowledgeKinetics>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicationKnowledge" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicationKnowledge",
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
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
                        Field::Manufacturer => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(map_access.next_value()?);
                        }
                        Field::DoseForm => {
                            if r#dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseForm"));
                            }
                            r#dose_form = Some(map_access.next_value()?);
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        Field::Synonym => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#synonym.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("synonym"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::SynonymPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#synonym.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_synonym"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::RelatedMedicationKnowledge => {
                            if r#related_medication_knowledge.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "relatedMedicationKnowledge",
                                ));
                            }
                            r#related_medication_knowledge = Some(map_access.next_value()?);
                        }
                        Field::AssociatedMedication => {
                            if r#associated_medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "associatedMedication",
                                ));
                            }
                            r#associated_medication = Some(map_access.next_value()?);
                        }
                        Field::ProductType => {
                            if r#product_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("productType"));
                            }
                            r#product_type = Some(map_access.next_value()?);
                        }
                        Field::Monograph => {
                            if r#monograph.is_some() {
                                return Err(serde::de::Error::duplicate_field("monograph"));
                            }
                            r#monograph = Some(map_access.next_value()?);
                        }
                        Field::Ingredient => {
                            if r#ingredient.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingredient"));
                            }
                            r#ingredient = Some(map_access.next_value()?);
                        }
                        Field::PreparationInstruction => {
                            let some = r#preparation_instruction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "preparationInstruction",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PreparationInstructionPrimitiveElement => {
                            let some = r#preparation_instruction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_preparationInstruction",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::IntendedRoute => {
                            if r#intended_route.is_some() {
                                return Err(serde::de::Error::duplicate_field("intendedRoute"));
                            }
                            r#intended_route = Some(map_access.next_value()?);
                        }
                        Field::Cost => {
                            if r#cost.is_some() {
                                return Err(serde::de::Error::duplicate_field("cost"));
                            }
                            r#cost = Some(map_access.next_value()?);
                        }
                        Field::MonitoringProgram => {
                            if r#monitoring_program.is_some() {
                                return Err(serde::de::Error::duplicate_field("monitoringProgram"));
                            }
                            r#monitoring_program = Some(map_access.next_value()?);
                        }
                        Field::AdministrationGuidelines => {
                            if r#administration_guidelines.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "administrationGuidelines",
                                ));
                            }
                            r#administration_guidelines = Some(map_access.next_value()?);
                        }
                        Field::MedicineClassification => {
                            if r#medicine_classification.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicineClassification",
                                ));
                            }
                            r#medicine_classification = Some(map_access.next_value()?);
                        }
                        Field::Packaging => {
                            if r#packaging.is_some() {
                                return Err(serde::de::Error::duplicate_field("packaging"));
                            }
                            r#packaging = Some(map_access.next_value()?);
                        }
                        Field::DrugCharacteristic => {
                            if r#drug_characteristic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "drugCharacteristic",
                                ));
                            }
                            r#drug_characteristic = Some(map_access.next_value()?);
                        }
                        Field::Contraindication => {
                            if r#contraindication.is_some() {
                                return Err(serde::de::Error::duplicate_field("contraindication"));
                            }
                            r#contraindication = Some(map_access.next_value()?);
                        }
                        Field::Regulatory => {
                            if r#regulatory.is_some() {
                                return Err(serde::de::Error::duplicate_field("regulatory"));
                            }
                            r#regulatory = Some(map_access.next_value()?);
                        }
                        Field::Kinetics => {
                            if r#kinetics.is_some() {
                                return Err(serde::de::Error::duplicate_field("kinetics"));
                            }
                            r#kinetics = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationKnowledge {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code,
                    r#status,
                    r#manufacturer,
                    r#dose_form,
                    r#amount,
                    r#synonym: r#synonym.unwrap_or(vec![]),
                    r#related_medication_knowledge: r#related_medication_knowledge
                        .unwrap_or(vec![]),
                    r#associated_medication: r#associated_medication.unwrap_or(vec![]),
                    r#product_type: r#product_type.unwrap_or(vec![]),
                    r#monograph: r#monograph.unwrap_or(vec![]),
                    r#ingredient: r#ingredient.unwrap_or(vec![]),
                    r#preparation_instruction,
                    r#intended_route: r#intended_route.unwrap_or(vec![]),
                    r#cost: r#cost.unwrap_or(vec![]),
                    r#monitoring_program: r#monitoring_program.unwrap_or(vec![]),
                    r#administration_guidelines: r#administration_guidelines.unwrap_or(vec![]),
                    r#medicine_classification: r#medicine_classification.unwrap_or(vec![]),
                    r#packaging,
                    r#drug_characteristic: r#drug_characteristic.unwrap_or(vec![]),
                    r#contraindication: r#contraindication.unwrap_or(vec![]),
                    r#regulatory: r#regulatory.unwrap_or(vec![]),
                    r#kinetics: r#kinetics.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
