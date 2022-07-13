// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(tag = "resourceType")]
pub enum Resource {
    AccountCoverage(Box<super::AccountCoverage>),
    ActivityDefinitionParticipant(Box<super::ActivityDefinitionParticipant>),
    AdverseEventSuspectEntityCausality(Box<super::AdverseEventSuspectEntityCausality>),
    AllergyIntoleranceReaction(Box<super::AllergyIntoleranceReaction>),
    AppointmentParticipant(Box<super::AppointmentParticipant>),
    AppointmentResponse(Box<super::AppointmentResponse>),
    AuditEventAgentNetwork(Box<super::AuditEventAgentNetwork>),
    Basic(Box<super::Basic>),
    Binary(Box<super::Binary>),
    BiologicallyDerivedProductCollection(Box<super::BiologicallyDerivedProductCollection>),
    BodyStructure(Box<super::BodyStructure>),
    BundleLink(Box<super::BundleLink>),
    CapabilityStatementSoftware(Box<super::CapabilityStatementSoftware>),
    CarePlanActivityDetail(Box<super::CarePlanActivityDetail>),
    CareTeamParticipant(Box<super::CareTeamParticipant>),
    CatalogEntryRelatedEntry(Box<super::CatalogEntryRelatedEntry>),
    ChargeItemPerformer(Box<super::ChargeItemPerformer>),
    ChargeItemDefinitionApplicability(Box<super::ChargeItemDefinitionApplicability>),
    ClaimRelated(Box<super::ClaimRelated>),
    ClaimResponseItemAdjudication(Box<super::ClaimResponseItemAdjudication>),
    ClinicalImpressionInvestigation(Box<super::ClinicalImpressionInvestigation>),
    CodeSystemFilter(Box<super::CodeSystemFilter>),
    CommunicationPayload(Box<super::CommunicationPayload>),
    CommunicationRequestPayload(Box<super::CommunicationRequestPayload>),
    CompartmentDefinitionResource(Box<super::CompartmentDefinitionResource>),
    CompositionAttester(Box<super::CompositionAttester>),
    ConceptMapGroupElementTargetDependsOn(Box<super::ConceptMapGroupElementTargetDependsOn>),
    ConditionStage(Box<super::ConditionStage>),
    ConsentPolicy(Box<super::ConsentPolicy>),
    ContractContentDefinition(Box<super::ContractContentDefinition>),
    CoverageClass(Box<super::CoverageClass>),
    CoverageEligibilityRequestSupportingInfo(Box<super::CoverageEligibilityRequestSupportingInfo>),
    CoverageEligibilityResponseInsuranceItemBenefit(
        Box<super::CoverageEligibilityResponseInsuranceItemBenefit>,
    ),
    DetectedIssueEvidence(Box<super::DetectedIssueEvidence>),
    DeviceUdiCarrier(Box<super::DeviceUdiCarrier>),
    DeviceDefinitionUdiDeviceIdentifier(Box<super::DeviceDefinitionUdiDeviceIdentifier>),
    DeviceMetricCalibration(Box<super::DeviceMetricCalibration>),
    DeviceRequestParameter(Box<super::DeviceRequestParameter>),
    DeviceUseStatement(Box<super::DeviceUseStatement>),
    DiagnosticReportMedia(Box<super::DiagnosticReportMedia>),
    DocumentManifestRelated(Box<super::DocumentManifestRelated>),
    DocumentReferenceRelatesTo(Box<super::DocumentReferenceRelatesTo>),
    EffectEvidenceSynthesisSampleSize(Box<super::EffectEvidenceSynthesisSampleSize>),
    EncounterStatusHistory(Box<super::EncounterStatusHistory>),
    Endpoint(Box<super::Endpoint>),
    EnrollmentRequest(Box<super::EnrollmentRequest>),
    EnrollmentResponse(Box<super::EnrollmentResponse>),
    EpisodeOfCareStatusHistory(Box<super::EpisodeOfCareStatusHistory>),
    EventDefinition(Box<super::EventDefinition>),
    Evidence(Box<super::Evidence>),
    EvidenceVariableCharacteristic(Box<super::EvidenceVariableCharacteristic>),
    ExampleScenarioActor(Box<super::ExampleScenarioActor>),
    ExplanationOfBenefitRelated(Box<super::ExplanationOfBenefitRelated>),
    FamilyMemberHistoryCondition(Box<super::FamilyMemberHistoryCondition>),
    Flag(Box<super::Flag>),
    GoalTarget(Box<super::GoalTarget>),
    GraphDefinitionLinkTargetCompartment(Box<super::GraphDefinitionLinkTargetCompartment>),
    GroupCharacteristic(Box<super::GroupCharacteristic>),
    GuidanceResponse(Box<super::GuidanceResponse>),
    HealthcareServiceEligibility(Box<super::HealthcareServiceEligibility>),
    ImagingStudySeriesPerformer(Box<super::ImagingStudySeriesPerformer>),
    ImmunizationPerformer(Box<super::ImmunizationPerformer>),
    ImmunizationEvaluation(Box<super::ImmunizationEvaluation>),
    ImmunizationRecommendationRecommendationDateCriterion(
        Box<super::ImmunizationRecommendationRecommendationDateCriterion>,
    ),
    ImplementationGuideDependsOn(Box<super::ImplementationGuideDependsOn>),
    InsurancePlanContact(Box<super::InsurancePlanContact>),
    InvoiceParticipant(Box<super::InvoiceParticipant>),
    Library(Box<super::Library>),
    LinkageItem(Box<super::LinkageItem>),
    ListEntry(Box<super::ListEntry>),
    LocationPosition(Box<super::LocationPosition>),
    MeasureGroupPopulation(Box<super::MeasureGroupPopulation>),
    MeasureReportGroupPopulation(Box<super::MeasureReportGroupPopulation>),
    Media(Box<super::Media>),
    MedicationIngredient(Box<super::MedicationIngredient>),
    MedicationAdministrationPerformer(Box<super::MedicationAdministrationPerformer>),
    MedicationDispensePerformer(Box<super::MedicationDispensePerformer>),
    MedicationKnowledgeRelatedMedicationKnowledge(
        Box<super::MedicationKnowledgeRelatedMedicationKnowledge>,
    ),
    MedicationRequestDispenseRequestInitialFill(
        Box<super::MedicationRequestDispenseRequestInitialFill>,
    ),
    MedicationStatement(Box<super::MedicationStatement>),
    MedicinalProductNameNamePart(Box<super::MedicinalProductNameNamePart>),
    MedicinalProductAuthorizationJurisdictionalAuthorization(
        Box<super::MedicinalProductAuthorizationJurisdictionalAuthorization>,
    ),
    MedicinalProductContraindicationOtherTherapy(
        Box<super::MedicinalProductContraindicationOtherTherapy>,
    ),
    MedicinalProductIndicationOtherTherapy(Box<super::MedicinalProductIndicationOtherTherapy>),
    MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength(
        Box<super::MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength>,
    ),
    MedicinalProductInteractionInteractant(Box<super::MedicinalProductInteractionInteractant>),
    MedicinalProductManufactured(Box<super::MedicinalProductManufactured>),
    MedicinalProductPackagedBatchIdentifier(Box<super::MedicinalProductPackagedBatchIdentifier>),
    MedicinalProductPharmaceuticalCharacteristics(
        Box<super::MedicinalProductPharmaceuticalCharacteristics>,
    ),
    MedicinalProductUndesirableEffect(Box<super::MedicinalProductUndesirableEffect>),
    MessageDefinitionFocus(Box<super::MessageDefinitionFocus>),
    MessageHeaderDestination(Box<super::MessageHeaderDestination>),
    MolecularSequenceReferenceSeq(Box<super::MolecularSequenceReferenceSeq>),
    NamingSystemUniqueId(Box<super::NamingSystemUniqueId>),
    NutritionOrderOralDietNutrient(Box<super::NutritionOrderOralDietNutrient>),
    ObservationReferenceRange(Box<super::ObservationReferenceRange>),
    ObservationDefinitionQuantitativeDetails(Box<super::ObservationDefinitionQuantitativeDetails>),
    OperationDefinitionParameterBinding(Box<super::OperationDefinitionParameterBinding>),
    OperationOutcomeIssue(Box<super::OperationOutcomeIssue>),
    OrganizationContact(Box<super::OrganizationContact>),
    OrganizationAffiliation(Box<super::OrganizationAffiliation>),
    ParametersParameter(Box<super::ParametersParameter>),
    PatientContact(Box<super::PatientContact>),
    PaymentNotice(Box<super::PaymentNotice>),
    PaymentReconciliationDetail(Box<super::PaymentReconciliationDetail>),
    PersonLink(Box<super::PersonLink>),
    PlanDefinitionGoalTarget(Box<super::PlanDefinitionGoalTarget>),
    PractitionerQualification(Box<super::PractitionerQualification>),
    PractitionerRoleAvailableTime(Box<super::PractitionerRoleAvailableTime>),
    ProcedurePerformer(Box<super::ProcedurePerformer>),
    ProvenanceAgent(Box<super::ProvenanceAgent>),
    QuestionnaireItemEnableWhen(Box<super::QuestionnaireItemEnableWhen>),
    QuestionnaireResponseItemAnswer(Box<super::QuestionnaireResponseItemAnswer>),
    RelatedPersonCommunication(Box<super::RelatedPersonCommunication>),
    RequestGroupActionCondition(Box<super::RequestGroupActionCondition>),
    ResearchDefinition(Box<super::ResearchDefinition>),
    ResearchElementDefinitionCharacteristic(Box<super::ResearchElementDefinitionCharacteristic>),
    ResearchStudyArm(Box<super::ResearchStudyArm>),
    ResearchSubject(Box<super::ResearchSubject>),
    RiskAssessmentPrediction(Box<super::RiskAssessmentPrediction>),
    RiskEvidenceSynthesisSampleSize(Box<super::RiskEvidenceSynthesisSampleSize>),
    Schedule(Box<super::Schedule>),
    SearchParameterComponent(Box<super::SearchParameterComponent>),
    ServiceRequest(Box<super::ServiceRequest>),
    Slot(Box<super::Slot>),
    SpecimenCollection(Box<super::SpecimenCollection>),
    SpecimenDefinitionTypeTestedContainerAdditive(
        Box<super::SpecimenDefinitionTypeTestedContainerAdditive>,
    ),
    StructureDefinitionMapping(Box<super::StructureDefinitionMapping>),
    StructureMapStructure(Box<super::StructureMapStructure>),
    SubscriptionChannel(Box<super::SubscriptionChannel>),
    SubstanceInstance(Box<super::SubstanceInstance>),
    SubstanceNucleicAcidSubunitLinkage(Box<super::SubstanceNucleicAcidSubunitLinkage>),
    SubstancePolymerMonomerSetStartingMaterial(
        Box<super::SubstancePolymerMonomerSetStartingMaterial>,
    ),
    SubstanceProteinSubunit(Box<super::SubstanceProteinSubunit>),
    SubstanceReferenceInformationGene(Box<super::SubstanceReferenceInformationGene>),
    SubstanceSourceMaterialFractionDescription(
        Box<super::SubstanceSourceMaterialFractionDescription>,
    ),
    SubstanceSpecificationMoiety(Box<super::SubstanceSpecificationMoiety>),
    SupplyDeliverySuppliedItem(Box<super::SupplyDeliverySuppliedItem>),
    SupplyRequestParameter(Box<super::SupplyRequestParameter>),
    TaskRestriction(Box<super::TaskRestriction>),
    TerminologyCapabilitiesSoftware(Box<super::TerminologyCapabilitiesSoftware>),
    TestReportParticipant(Box<super::TestReportParticipant>),
    TestScriptOrigin(Box<super::TestScriptOrigin>),
    ValueSetComposeIncludeConceptDesignation(Box<super::ValueSetComposeIncludeConceptDesignation>),
    VerificationResultPrimarySource(Box<super::VerificationResultPrimarySource>),
    VisionPrescriptionLensSpecificationPrism(Box<super::VisionPrescriptionLensSpecificationPrism>),
    MetadataResource(Box<super::MetadataResource>),
}
impl Default for Resource {
    fn default() -> Resource {
        unimplemented!()
    }
}
