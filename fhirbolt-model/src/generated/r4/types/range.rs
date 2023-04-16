// Generated on 2023-04-16 by fhirbolt-codegen v0.2.0
#[doc = "Base StructureDefinition for Range Type: A set of ordered Quantities defined by a low and high limit.\n\nNeed to be able to specify ranges of values."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Range {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The low limit. The boundary is inclusive."]
    pub r#low: Option<Box<super::super::types::Quantity>>,
    #[doc = "The high limit. The boundary is inclusive."]
    pub r#high: Option<Box<super::super::types::Quantity>>,
}
