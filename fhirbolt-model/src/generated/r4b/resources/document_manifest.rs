// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "Related identifiers or resources associated with the DocumentManifest."]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentManifestRelated {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Related identifier to this DocumentManifest.  For example, Order numbers, accession numbers, XDW workflow numbers."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Related Resource to this DocumentManifest. For example, Order, ServiceRequest,  Procedure, EligibilityRequest, etc."]
    pub r#ref: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for DocumentManifestRelated {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#ref: Default::default(),
        }
    }
}
#[doc = "A collection of documents compiled for a purpose together with metadata that applies to the collection."]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentManifest {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
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
    #[doc = "A single identifier that uniquely identifies this manifest. Principally used to refer to the manifest in non-FHIR contexts."]
    pub r#master_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Other identifiers associated with the document manifest, including version independent  identifiers."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of this document manifest."]
    pub r#status: super::super::types::Code,
    #[doc = "The code specifying the type of clinical activity that resulted in placing the associated content into the DocumentManifest."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Who or what the set of documents is about. The documents can be about a person, (patient or healthcare practitioner), a device (i.e. machine) or even a group of subjects (such as a document about a herd of farm animals, or a set of patients that share a common exposure). If the documents cross more than one subject, then more than one subject is allowed here (unusual use case)."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "When the document manifest was created for submission to the server (not necessarily the same thing as the actual resource last modified time, since it may be modified, replicated, etc.)."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "Identifies who is the author of the manifest. Manifest author is not necessarly the author of the references included."]
    pub r#author: Vec<super::super::types::Reference>,
    #[doc = "A patient, practitioner, or organization for which this set of documents is intended."]
    pub r#recipient: Vec<super::super::types::Reference>,
    #[doc = "Identifies the source system, application, or software that produced the document manifest."]
    pub r#source: Option<super::super::types::Uri>,
    #[doc = "Human-readable description of the source document. This is sometimes known as the \"title\"."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The list of Resources that consist of the parts of this manifest."]
    pub r#content: Vec<super::super::types::Reference>,
    #[doc = "Related identifiers or resources associated with the DocumentManifest."]
    pub r#related: Vec<DocumentManifestRelated>,
}
#[allow(clippy::derivable_impls)]
impl Default for DocumentManifest {
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
            r#master_identifier: Default::default(),
            r#identifier: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Default::default(),
            r#subject: Default::default(),
            r#created: Default::default(),
            r#author: Default::default(),
            r#recipient: Default::default(),
            r#source: Default::default(),
            r#description: Default::default(),
            r#content: Default::default(),
            r#related: Default::default(),
        }
    }
}
