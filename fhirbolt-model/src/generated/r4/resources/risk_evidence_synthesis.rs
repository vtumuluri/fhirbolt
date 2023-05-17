// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A description of the size of the sample involved in the synthesis."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskEvidenceSynthesisSampleSize {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Human-readable summary of sample size."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Number of studies included in this evidence synthesis."]
    pub r#number_of_studies: Option<super::super::types::Integer>,
    #[doc = "Number of participants included in this evidence synthesis."]
    pub r#number_of_participants: Option<super::super::types::Integer>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskEvidenceSynthesisSampleSize {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#number_of_studies: Default::default(),
            r#number_of_participants: Default::default(),
        }
    }
}
#[doc = "A description of the precision of the estimate for the effect."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskEvidenceSynthesisRiskEstimatePrecisionEstimate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Examples include confidence interval and interquartile range."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Use 95 for a 95% confidence interval."]
    pub r#level: Option<super::super::types::Decimal>,
    #[doc = "Lower bound of confidence interval."]
    pub r#from: Option<super::super::types::Decimal>,
    #[doc = "Upper bound of confidence interval."]
    pub r#to: Option<super::super::types::Decimal>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskEvidenceSynthesisRiskEstimatePrecisionEstimate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#level: Default::default(),
            r#from: Default::default(),
            r#to: Default::default(),
        }
    }
}
#[doc = "The estimated risk of the outcome."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskEvidenceSynthesisRiskEstimate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Human-readable summary of risk estimate."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Examples include proportion and mean."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The point estimate of the risk estimate."]
    pub r#value: Option<super::super::types::Decimal>,
    #[doc = "Specifies the UCUM unit for the outcome."]
    pub r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The sample size for the group that was measured for this risk estimate."]
    pub r#denominator_count: Option<super::super::types::Integer>,
    #[doc = "The number of group members with the outcome of interest."]
    pub r#numerator_count: Option<super::super::types::Integer>,
    #[doc = "A description of the precision of the estimate for the effect."]
    pub r#precision_estimate: Vec<RiskEvidenceSynthesisRiskEstimatePrecisionEstimate>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskEvidenceSynthesisRiskEstimate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#type: Default::default(),
            r#value: Default::default(),
            r#unit_of_measure: Default::default(),
            r#denominator_count: Default::default(),
            r#numerator_count: Default::default(),
            r#precision_estimate: Default::default(),
        }
    }
}
#[doc = "A description of a component of the overall certainty."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskEvidenceSynthesisCertaintyCertaintySubcomponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Type of subcomponent of certainty rating."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A rating of a subcomponent of rating certainty."]
    pub r#rating: Vec<super::super::types::CodeableConcept>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskEvidenceSynthesisCertaintyCertaintySubcomponent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#rating: Default::default(),
            r#note: Default::default(),
        }
    }
}
#[doc = "A description of the certainty of the risk estimate."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskEvidenceSynthesisCertainty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A rating of the certainty of the effect estimate."]
    pub r#rating: Vec<super::super::types::CodeableConcept>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "A description of a component of the overall certainty."]
    pub r#certainty_subcomponent: Vec<RiskEvidenceSynthesisCertaintyCertaintySubcomponent>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskEvidenceSynthesisCertainty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#rating: Default::default(),
            r#note: Default::default(),
            r#certainty_subcomponent: Default::default(),
        }
    }
}
#[doc = "The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a population plus exposure state where the risk estimate is derived from a combination of research studies."]
#[derive(Debug, Clone, PartialEq)]
pub struct RiskEvidenceSynthesis {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this risk evidence synthesis when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this risk evidence synthesis is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the risk evidence synthesis is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this risk evidence synthesis when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the risk evidence synthesis when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the risk evidence synthesis author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the risk evidence synthesis. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the risk evidence synthesis."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this risk evidence synthesis. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The date  (and optionally time) when the risk evidence synthesis was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the risk evidence synthesis changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the risk evidence synthesis."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the risk evidence synthesis from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A human-readable string to clarify or explain concepts about the resource."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate risk evidence synthesis instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the risk evidence synthesis is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "A copyright statement relating to the risk evidence synthesis and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the risk evidence synthesis."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the risk evidence synthesis content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the RiskEvidenceSynthesis. Topics provide a high-level categorization grouping types of EffectEvidenceSynthesiss that can be useful for filtering and searching."]
    pub r#topic: Vec<super::super::types::CodeableConcept>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "Related artifacts such as additional documentation, justification, or bibliographic references."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "Type of synthesis eg meta-analysis."]
    pub r#synthesis_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of study eg randomized trial."]
    pub r#study_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to a EvidenceVariable resource that defines the population for the research."]
    pub r#population: Box<super::super::types::Reference>,
    #[doc = "A reference to a EvidenceVariable resource that defines the exposure for the research."]
    pub r#exposure: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to a EvidenceVariable resomece that defines the outcome for the research."]
    pub r#outcome: Box<super::super::types::Reference>,
    #[doc = "A description of the size of the sample involved in the synthesis."]
    pub r#sample_size: Option<RiskEvidenceSynthesisSampleSize>,
    #[doc = "The estimated risk of the outcome."]
    pub r#risk_estimate: Option<RiskEvidenceSynthesisRiskEstimate>,
    #[doc = "A description of the certainty of the risk estimate."]
    pub r#certainty: Vec<RiskEvidenceSynthesisCertainty>,
}
#[allow(clippy::derivable_impls)]
impl Default for RiskEvidenceSynthesis {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#url: Default::default(),
            r#identifier: Default::default(),
            r#version: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#note: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#copyright: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#topic: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#related_artifact: Default::default(),
            r#synthesis_type: Default::default(),
            r#study_type: Default::default(),
            r#population: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#exposure: Default::default(),
            r#outcome: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#sample_size: Default::default(),
            r#risk_estimate: Default::default(),
            r#certainty: Default::default(),
        }
    }
}
