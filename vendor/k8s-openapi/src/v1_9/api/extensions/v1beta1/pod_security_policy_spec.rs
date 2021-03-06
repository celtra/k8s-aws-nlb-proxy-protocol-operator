// Generated from definition io.k8s.api.extensions.v1beta1.PodSecurityPolicySpec

/// Pod Security Policy Spec defines the policy enforced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodSecurityPolicySpec {
    /// AllowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true.
    pub allow_privilege_escalation: Option<bool>,

    /// AllowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both AllowedCapabilities and RequiredDropCapabilities.
    pub allowed_capabilities: Option<Vec<String>>,

    /// AllowedFlexVolumes is a whitelist of allowed Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the "Volumes" field.
    pub allowed_flex_volumes: Option<Vec<crate::api::extensions::v1beta1::AllowedFlexVolume>>,

    /// is a white list of allowed host paths. Empty indicates that all host paths may be used.
    pub allowed_host_paths: Option<Vec<crate::api::extensions::v1beta1::AllowedHostPath>>,

    /// DefaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capability in both DefaultAddCapabilities and RequiredDropCapabilities. Capabilities added here are implicitly allowed, and need not be included in the AllowedCapabilities list.
    pub default_add_capabilities: Option<Vec<String>>,

    /// DefaultAllowPrivilegeEscalation controls the default setting for whether a process can gain more privileges than its parent process.
    pub default_allow_privilege_escalation: Option<bool>,

    /// FSGroup is the strategy that will dictate what fs group is used by the SecurityContext.
    pub fs_group: crate::api::extensions::v1beta1::FSGroupStrategyOptions,

    /// hostIPC determines if the policy allows the use of HostIPC in the pod spec.
    pub host_ipc: Option<bool>,

    /// hostNetwork determines if the policy allows the use of HostNetwork in the pod spec.
    pub host_network: Option<bool>,

    /// hostPID determines if the policy allows the use of HostPID in the pod spec.
    pub host_pid: Option<bool>,

    /// hostPorts determines which host port ranges are allowed to be exposed.
    pub host_ports: Option<Vec<crate::api::extensions::v1beta1::HostPortRange>>,

    /// privileged determines if a pod can request to be run as privileged.
    pub privileged: Option<bool>,

    /// ReadOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to.
    pub read_only_root_filesystem: Option<bool>,

    /// RequiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added.
    pub required_drop_capabilities: Option<Vec<String>>,

    /// runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set.
    pub run_as_user: crate::api::extensions::v1beta1::RunAsUserStrategyOptions,

    /// seLinux is the strategy that will dictate the allowable labels that may be set.
    pub se_linux: crate::api::extensions::v1beta1::SELinuxStrategyOptions,

    /// SupplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.
    pub supplemental_groups: crate::api::extensions::v1beta1::SupplementalGroupsStrategyOptions,

    /// volumes is a white list of allowed volume plugins.  Empty indicates that all plugins may be used.
    pub volumes: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for PodSecurityPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_privilege_escalation,
            Key_allowed_capabilities,
            Key_allowed_flex_volumes,
            Key_allowed_host_paths,
            Key_default_add_capabilities,
            Key_default_allow_privilege_escalation,
            Key_fs_group,
            Key_host_ipc,
            Key_host_network,
            Key_host_pid,
            Key_host_ports,
            Key_privileged,
            Key_read_only_root_filesystem,
            Key_required_drop_capabilities,
            Key_run_as_user,
            Key_se_linux,
            Key_supplemental_groups,
            Key_volumes,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "allowPrivilegeEscalation" => Field::Key_allow_privilege_escalation,
                            "allowedCapabilities" => Field::Key_allowed_capabilities,
                            "allowedFlexVolumes" => Field::Key_allowed_flex_volumes,
                            "allowedHostPaths" => Field::Key_allowed_host_paths,
                            "defaultAddCapabilities" => Field::Key_default_add_capabilities,
                            "defaultAllowPrivilegeEscalation" => Field::Key_default_allow_privilege_escalation,
                            "fsGroup" => Field::Key_fs_group,
                            "hostIPC" => Field::Key_host_ipc,
                            "hostNetwork" => Field::Key_host_network,
                            "hostPID" => Field::Key_host_pid,
                            "hostPorts" => Field::Key_host_ports,
                            "privileged" => Field::Key_privileged,
                            "readOnlyRootFilesystem" => Field::Key_read_only_root_filesystem,
                            "requiredDropCapabilities" => Field::Key_required_drop_capabilities,
                            "runAsUser" => Field::Key_run_as_user,
                            "seLinux" => Field::Key_se_linux,
                            "supplementalGroups" => Field::Key_supplemental_groups,
                            "volumes" => Field::Key_volumes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PodSecurityPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PodSecurityPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_allow_privilege_escalation: Option<bool> = None;
                let mut value_allowed_capabilities: Option<Vec<String>> = None;
                let mut value_allowed_flex_volumes: Option<Vec<crate::api::extensions::v1beta1::AllowedFlexVolume>> = None;
                let mut value_allowed_host_paths: Option<Vec<crate::api::extensions::v1beta1::AllowedHostPath>> = None;
                let mut value_default_add_capabilities: Option<Vec<String>> = None;
                let mut value_default_allow_privilege_escalation: Option<bool> = None;
                let mut value_fs_group: Option<crate::api::extensions::v1beta1::FSGroupStrategyOptions> = None;
                let mut value_host_ipc: Option<bool> = None;
                let mut value_host_network: Option<bool> = None;
                let mut value_host_pid: Option<bool> = None;
                let mut value_host_ports: Option<Vec<crate::api::extensions::v1beta1::HostPortRange>> = None;
                let mut value_privileged: Option<bool> = None;
                let mut value_read_only_root_filesystem: Option<bool> = None;
                let mut value_required_drop_capabilities: Option<Vec<String>> = None;
                let mut value_run_as_user: Option<crate::api::extensions::v1beta1::RunAsUserStrategyOptions> = None;
                let mut value_se_linux: Option<crate::api::extensions::v1beta1::SELinuxStrategyOptions> = None;
                let mut value_supplemental_groups: Option<crate::api::extensions::v1beta1::SupplementalGroupsStrategyOptions> = None;
                let mut value_volumes: Option<Vec<String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_privilege_escalation => value_allow_privilege_escalation = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_capabilities => value_allowed_capabilities = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_flex_volumes => value_allowed_flex_volumes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_allowed_host_paths => value_allowed_host_paths = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_add_capabilities => value_default_add_capabilities = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_default_allow_privilege_escalation => value_default_allow_privilege_escalation = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_fs_group => value_fs_group = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_host_ipc => value_host_ipc = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_network => value_host_network = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_pid => value_host_pid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_host_ports => value_host_ports = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_privileged => value_privileged = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_read_only_root_filesystem => value_read_only_root_filesystem = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_required_drop_capabilities => value_required_drop_capabilities = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_run_as_user => value_run_as_user = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_se_linux => value_se_linux = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_supplemental_groups => value_supplemental_groups = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_volumes => value_volumes = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PodSecurityPolicySpec {
                    allow_privilege_escalation: value_allow_privilege_escalation,
                    allowed_capabilities: value_allowed_capabilities,
                    allowed_flex_volumes: value_allowed_flex_volumes,
                    allowed_host_paths: value_allowed_host_paths,
                    default_add_capabilities: value_default_add_capabilities,
                    default_allow_privilege_escalation: value_default_allow_privilege_escalation,
                    fs_group: value_fs_group.ok_or_else(|| serde::de::Error::missing_field("fsGroup"))?,
                    host_ipc: value_host_ipc,
                    host_network: value_host_network,
                    host_pid: value_host_pid,
                    host_ports: value_host_ports,
                    privileged: value_privileged,
                    read_only_root_filesystem: value_read_only_root_filesystem,
                    required_drop_capabilities: value_required_drop_capabilities,
                    run_as_user: value_run_as_user.ok_or_else(|| serde::de::Error::missing_field("runAsUser"))?,
                    se_linux: value_se_linux.ok_or_else(|| serde::de::Error::missing_field("seLinux"))?,
                    supplemental_groups: value_supplemental_groups.ok_or_else(|| serde::de::Error::missing_field("supplementalGroups"))?,
                    volumes: value_volumes,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodSecurityPolicySpec",
            &[
                "allowPrivilegeEscalation",
                "allowedCapabilities",
                "allowedFlexVolumes",
                "allowedHostPaths",
                "defaultAddCapabilities",
                "defaultAllowPrivilegeEscalation",
                "fsGroup",
                "hostIPC",
                "hostNetwork",
                "hostPID",
                "hostPorts",
                "privileged",
                "readOnlyRootFilesystem",
                "requiredDropCapabilities",
                "runAsUser",
                "seLinux",
                "supplementalGroups",
                "volumes",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PodSecurityPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodSecurityPolicySpec",
            4 +
            self.allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.allowed_capabilities.as_ref().map_or(0, |_| 1) +
            self.allowed_flex_volumes.as_ref().map_or(0, |_| 1) +
            self.allowed_host_paths.as_ref().map_or(0, |_| 1) +
            self.default_add_capabilities.as_ref().map_or(0, |_| 1) +
            self.default_allow_privilege_escalation.as_ref().map_or(0, |_| 1) +
            self.host_ipc.as_ref().map_or(0, |_| 1) +
            self.host_network.as_ref().map_or(0, |_| 1) +
            self.host_pid.as_ref().map_or(0, |_| 1) +
            self.host_ports.as_ref().map_or(0, |_| 1) +
            self.privileged.as_ref().map_or(0, |_| 1) +
            self.read_only_root_filesystem.as_ref().map_or(0, |_| 1) +
            self.required_drop_capabilities.as_ref().map_or(0, |_| 1) +
            self.volumes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allow_privilege_escalation {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowPrivilegeEscalation", value)?;
        }
        if let Some(value) = &self.allowed_capabilities {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowedCapabilities", value)?;
        }
        if let Some(value) = &self.allowed_flex_volumes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowedFlexVolumes", value)?;
        }
        if let Some(value) = &self.allowed_host_paths {
            serde::ser::SerializeStruct::serialize_field(&mut state, "allowedHostPaths", value)?;
        }
        if let Some(value) = &self.default_add_capabilities {
            serde::ser::SerializeStruct::serialize_field(&mut state, "defaultAddCapabilities", value)?;
        }
        if let Some(value) = &self.default_allow_privilege_escalation {
            serde::ser::SerializeStruct::serialize_field(&mut state, "defaultAllowPrivilegeEscalation", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "fsGroup", &self.fs_group)?;
        if let Some(value) = &self.host_ipc {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostIPC", value)?;
        }
        if let Some(value) = &self.host_network {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostNetwork", value)?;
        }
        if let Some(value) = &self.host_pid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostPID", value)?;
        }
        if let Some(value) = &self.host_ports {
            serde::ser::SerializeStruct::serialize_field(&mut state, "hostPorts", value)?;
        }
        if let Some(value) = &self.privileged {
            serde::ser::SerializeStruct::serialize_field(&mut state, "privileged", value)?;
        }
        if let Some(value) = &self.read_only_root_filesystem {
            serde::ser::SerializeStruct::serialize_field(&mut state, "readOnlyRootFilesystem", value)?;
        }
        if let Some(value) = &self.required_drop_capabilities {
            serde::ser::SerializeStruct::serialize_field(&mut state, "requiredDropCapabilities", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "runAsUser", &self.run_as_user)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "seLinux", &self.se_linux)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "supplementalGroups", &self.supplemental_groups)?;
        if let Some(value) = &self.volumes {
            serde::ser::SerializeStruct::serialize_field(&mut state, "volumes", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
