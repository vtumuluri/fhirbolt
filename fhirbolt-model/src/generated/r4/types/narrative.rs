// Generated on 2023-04-13 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Narrative {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The status of the narrative - whether it's entirely generated (from just the defined data or the extensions too), or whether a human authored it and it may contain additional data."]
    pub r#status: super::super::types::Code,
    #[doc = "The actual narrative content, a stripped down version of XHTML."]
    pub r#div: super::super::types::Xhtml,
}
