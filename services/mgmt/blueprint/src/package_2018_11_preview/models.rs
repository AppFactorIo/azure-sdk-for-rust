#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub properties: BlueprintProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub kind: artifact::Kind,
}
pub mod artifact {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "template")]
        Template,
        #[serde(rename = "roleAssignment")]
        RoleAssignment,
        #[serde(rename = "policyAssignment")]
        PolicyAssignment,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedBlueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub properties: PublishedBlueprintProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Blueprint>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedBlueprintList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PublishedBlueprint>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Artifact>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
pub mod resource_provider_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedBlueprintProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BlueprintStatus>,
    #[serde(rename = "targetScope", skip_serializing_if = "Option::is_none")]
    pub target_scope: Option<shared_blueprint_properties::TargetScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "resourceGroups", skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<serde_json::Value>,
}
pub mod shared_blueprint_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetScope {
        #[serde(rename = "subscription")]
        Subscription,
        #[serde(rename = "managementGroup")]
        ManagementGroup,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedBlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[serde(rename = "blueprintName", skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[serde(rename = "changeNotes", skip_serializing_if = "Option::is_none")]
    pub change_notes: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    pub template: serde_json::Value,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    pub parameters: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    pub properties: TemplateArtifactProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[serde(rename = "principalIds")]
    pub principal_ids: serde_json::Value,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    pub properties: RoleAssignmentArtifactProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[serde(rename = "policyDefinitionId")]
    pub policy_definition_id: String,
    pub parameters: serde_json::Value,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    pub properties: PolicyAssignmentArtifactProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<SecretValueReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueReference {
    #[serde(rename = "keyVault")]
    pub key_vault: KeyVaultReference,
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(rename = "secretVersion", skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultReference {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[serde(rename = "type")]
    pub type_: parameter_definition::Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(rename = "allowedValues", skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<serde_json::Value>,
}
pub mod parameter_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "array")]
        Array,
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "object")]
        Object,
        #[serde(rename = "secureObject")]
        SecureObject,
        #[serde(rename = "secureString")]
        SecureString,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[serde(rename = "dependsOn", skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinitionMetadata {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "strongType", skip_serializing_if = "Option::is_none")]
    pub strong_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceBase {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactPropertiesBase {
    #[serde(rename = "dependsOn", skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintResourcePropertiesBase {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintResourceStatusBase {
    #[serde(rename = "timeCreated", skip_serializing)]
    pub time_created: Option<String>,
    #[serde(rename = "lastModified", skip_serializing)]
    pub last_modified: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub identity: ManagedServiceIdentity,
    pub properties: AssignmentProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Assignment>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentity {
    #[serde(rename = "principalId", skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[serde(rename = "type")]
    pub type_: managed_service_identity::Type,
    #[serde(rename = "principalId", skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "userAssignedIdentities", skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod managed_service_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
    #[serde(rename = "managedResources", skip_serializing)]
    pub managed_resources: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentLockSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<assignment_lock_settings::Mode>,
    #[serde(rename = "excludedPrincipals", skip_serializing_if = "Vec::is_empty")]
    pub excluded_principals: Vec<String>,
    #[serde(rename = "excludedActions", skip_serializing_if = "Vec::is_empty")]
    pub excluded_actions: Vec<String>,
}
pub mod assignment_lock_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        None,
        AllResourcesReadOnly,
        AllResourcesDoNotDelete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(rename = "blueprintId", skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub parameters: serde_json::Value,
    #[serde(rename = "resourceGroups")]
    pub resource_groups: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssignmentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locks: Option<AssignmentLockSettings>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<assignment_properties::ProvisioningState>,
}
pub mod assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "validating")]
        Validating,
        #[serde(rename = "waiting")]
        Waiting,
        #[serde(rename = "deploying")]
        Deploying,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "locking")]
        Locking,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "deleting")]
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhoIsBlueprintContract {
    #[serde(rename = "objectId", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentOperation {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssignmentOperationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentOperationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AssignmentOperation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentOperationProperties {
    #[serde(rename = "blueprintVersion", skip_serializing_if = "Option::is_none")]
    pub blueprint_version: Option<String>,
    #[serde(rename = "assignmentState", skip_serializing_if = "Option::is_none")]
    pub assignment_state: Option<String>,
    #[serde(rename = "timeCreated", skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "timeStarted", skip_serializing_if = "Option::is_none")]
    pub time_started: Option<String>,
    #[serde(rename = "timeFinished", skip_serializing_if = "Option::is_none")]
    pub time_finished: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deployments: Vec<AssignmentDeploymentJob>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentDeploymentJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "jobId", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobState", skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<AssignmentDeploymentJobResult>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<AssignmentDeploymentJobResult>,
    #[serde(rename = "requestUri", skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentDeploymentJobResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureResourceManagerError>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<AssignmentJobCreatedResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentJobCreatedResource {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceManagerError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", skip_serializing)]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub info: Option<serde_json::Value>,
}