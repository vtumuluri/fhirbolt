// Generated on 2023-04-16 by fhirbolt-codegen v0.2.0
#[doc = "Timing or duration information, that may be associated with use with the indicated condition e.g. Adult patients suffering from myocardial infarction (from a few days until less than 35 days), ischaemic stroke (from 7 days until less than 6 months)."]
#[derive(Debug, Clone, PartialEq)]
pub enum ClinicalUseDefinitionIndicationDuration {
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for ClinicalUseDefinitionIndicationDuration {
    fn default() -> ClinicalUseDefinitionIndicationDuration {
        ClinicalUseDefinitionIndicationDuration::Invalid
    }
}
#[doc = "The specific medication, food or laboratory test that interacts."]
#[derive(Debug, Clone, PartialEq)]
pub enum ClinicalUseDefinitionInteractionInteractantItem {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for ClinicalUseDefinitionInteractionInteractantItem {
    fn default() -> ClinicalUseDefinitionInteractionInteractantItem {
        ClinicalUseDefinitionInteractionInteractantItem::Invalid
    }
}
#[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the contraindication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionContraindicationOtherTherapy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship between the medicinal product indication or contraindication and another therapy."]
    pub r#relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
    pub r#therapy: Box<super::super::types::CodeableReference>,
}
#[doc = "Specifics for when this is a contraindication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionContraindication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation that is being documented as contraindicating against this item."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The status of the disease or symptom for the contraindication, for example \"chronic\" or \"metastatic\"."]
    pub r#disease_status: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "A comorbidity (concurrent condition) or coinfection."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The indication which this is a contraidication for."]
    pub r#indication: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the contraindication."]
    pub r#other_therapy: Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
}
#[doc = "Specifics for when this is an indication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionIndication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation that is being documented as an indicaton for this item."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The status of the disease or symptom for the indication, for example \"chronic\" or \"metastatic\"."]
    pub r#disease_status: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "A comorbidity (concurrent condition) or coinfection as part of the indication."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "The intended effect, aim or strategy to be achieved."]
    pub r#intended_effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Timing or duration information, that may be associated with use with the indicated condition e.g. Adult patients suffering from myocardial infarction (from a few days until less than 35 days), ischaemic stroke (from 7 days until less than 6 months)."]
    pub r#duration: Option<ClinicalUseDefinitionIndicationDuration>,
    #[doc = "An unwanted side effect or negative outcome that may happen if you use the drug (or other subject of this resource) for this indication."]
    pub r#undesirable_effect: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
    pub r#other_therapy: Vec<ClinicalUseDefinitionContraindicationOtherTherapy>,
}
#[doc = "The specific medication, food, substance or laboratory test that interacts."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionInteractionInteractant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific medication, food or laboratory test that interacts."]
    pub r#item: ClinicalUseDefinitionInteractionInteractantItem,
}
#[doc = "Specifics for when this is an interaction."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionInteraction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific medication, food, substance or laboratory test that interacts."]
    pub r#interactant: Vec<ClinicalUseDefinitionInteractionInteractant>,
    #[doc = "The type of the interaction e.g. drug-drug interaction, drug-food interaction, drug-lab test interaction."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The effect of the interaction, for example \"reduced gastric absorption of primary medication\"."]
    pub r#effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The incidence of the interaction, e.g. theoretical, observed."]
    pub r#incidence: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Actions for managing the interaction."]
    pub r#management: Vec<Box<super::super::types::CodeableConcept>>,
}
#[doc = "Describe the possible undesirable effects (negative outcomes) from the use of the medicinal product as treatment."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionUndesirableEffect {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The situation in which the undesirable effect may manifest."]
    pub r#symptom_condition_effect: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "High level classification of the effect."]
    pub r#classification: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How often the effect is seen."]
    pub r#frequency_of_occurrence: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "A critical piece of information about environmental, health or physical risks or hazards that serve as caution to the user. For example 'Do not operate heavy machinery', 'May cause drowsiness', or 'Get medical advice/attention if you feel unwell'."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinitionWarning {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A textual definition of this warning, with formatting."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A coded or unformatted textual definition of this warning."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
#[doc = "A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClinicalUseDefinition {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business identifier for this issue."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "indication | contraindication | interaction | undesirable-effect | warning."]
    pub r#type: super::super::types::Code,
    #[doc = "A categorisation of the issue, primarily for dividing warnings into subject heading areas such as \"Pregnancy and Lactation\", \"Overdose\", \"Effects on Ability to Drive and Use Machines\"."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The medication or procedure for which this is an indication."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "Whether this is a current issue or one that has been retired etc."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifics for when this is a contraindication."]
    pub r#contraindication: Option<ClinicalUseDefinitionContraindication>,
    #[doc = "Specifics for when this is an indication."]
    pub r#indication: Option<ClinicalUseDefinitionIndication>,
    #[doc = "Specifics for when this is an interaction."]
    pub r#interaction: Option<ClinicalUseDefinitionInteraction>,
    #[doc = "The population group to which this applies."]
    pub r#population: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describe the possible undesirable effects (negative outcomes) from the use of the medicinal product as treatment."]
    pub r#undesirable_effect: Option<ClinicalUseDefinitionUndesirableEffect>,
    #[doc = "A critical piece of information about environmental, health or physical risks or hazards that serve as caution to the user. For example 'Do not operate heavy machinery', 'May cause drowsiness', or 'Get medical advice/attention if you feel unwell'."]
    pub r#warning: Option<ClinicalUseDefinitionWarning>,
}
