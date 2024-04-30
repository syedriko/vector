// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Limits for given InstanceType and for each of it's role. <br><br> Limits contains following <code> <code>StorageTypes,</code> </code> <code> <code>InstanceLimits</code> </code> and <code> <code>AdditionalLimits</code> </code> </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Limits {
    /// <p>StorageType represents the list of storage related types and attributes that are available for given InstanceType. </p>
    pub storage_types: ::std::option::Option<::std::vec::Vec<crate::types::StorageType>>,
    /// <p>InstanceLimits represents the list of instance related attributes that are available for given InstanceType. </p>
    pub instance_limits: ::std::option::Option<crate::types::InstanceLimits>,
    /// <p> List of additional limits that are specific to a given InstanceType and for each of it's <code> <code>InstanceRole</code> </code> . </p>
    pub additional_limits: ::std::option::Option<::std::vec::Vec<crate::types::AdditionalLimit>>,
}
impl Limits {
    /// <p>StorageType represents the list of storage related types and attributes that are available for given InstanceType. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.storage_types.is_none()`.
    pub fn storage_types(&self) -> &[crate::types::StorageType] {
        self.storage_types.as_deref().unwrap_or_default()
    }
    /// <p>InstanceLimits represents the list of instance related attributes that are available for given InstanceType. </p>
    pub fn instance_limits(&self) -> ::std::option::Option<&crate::types::InstanceLimits> {
        self.instance_limits.as_ref()
    }
    /// <p> List of additional limits that are specific to a given InstanceType and for each of it's <code> <code>InstanceRole</code> </code> . </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.additional_limits.is_none()`.
    pub fn additional_limits(&self) -> &[crate::types::AdditionalLimit] {
        self.additional_limits.as_deref().unwrap_or_default()
    }
}
impl Limits {
    /// Creates a new builder-style object to manufacture [`Limits`](crate::types::Limits).
    pub fn builder() -> crate::types::builders::LimitsBuilder {
        crate::types::builders::LimitsBuilder::default()
    }
}

/// A builder for [`Limits`](crate::types::Limits).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LimitsBuilder {
    pub(crate) storage_types: ::std::option::Option<::std::vec::Vec<crate::types::StorageType>>,
    pub(crate) instance_limits: ::std::option::Option<crate::types::InstanceLimits>,
    pub(crate) additional_limits: ::std::option::Option<::std::vec::Vec<crate::types::AdditionalLimit>>,
}
impl LimitsBuilder {
    /// Appends an item to `storage_types`.
    ///
    /// To override the contents of this collection use [`set_storage_types`](Self::set_storage_types).
    ///
    /// <p>StorageType represents the list of storage related types and attributes that are available for given InstanceType. </p>
    pub fn storage_types(mut self, input: crate::types::StorageType) -> Self {
        let mut v = self.storage_types.unwrap_or_default();
        v.push(input);
        self.storage_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>StorageType represents the list of storage related types and attributes that are available for given InstanceType. </p>
    pub fn set_storage_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::StorageType>>) -> Self {
        self.storage_types = input;
        self
    }
    /// <p>StorageType represents the list of storage related types and attributes that are available for given InstanceType. </p>
    pub fn get_storage_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::StorageType>> {
        &self.storage_types
    }
    /// <p>InstanceLimits represents the list of instance related attributes that are available for given InstanceType. </p>
    pub fn instance_limits(mut self, input: crate::types::InstanceLimits) -> Self {
        self.instance_limits = ::std::option::Option::Some(input);
        self
    }
    /// <p>InstanceLimits represents the list of instance related attributes that are available for given InstanceType. </p>
    pub fn set_instance_limits(mut self, input: ::std::option::Option<crate::types::InstanceLimits>) -> Self {
        self.instance_limits = input;
        self
    }
    /// <p>InstanceLimits represents the list of instance related attributes that are available for given InstanceType. </p>
    pub fn get_instance_limits(&self) -> &::std::option::Option<crate::types::InstanceLimits> {
        &self.instance_limits
    }
    /// Appends an item to `additional_limits`.
    ///
    /// To override the contents of this collection use [`set_additional_limits`](Self::set_additional_limits).
    ///
    /// <p> List of additional limits that are specific to a given InstanceType and for each of it's <code> <code>InstanceRole</code> </code> . </p>
    pub fn additional_limits(mut self, input: crate::types::AdditionalLimit) -> Self {
        let mut v = self.additional_limits.unwrap_or_default();
        v.push(input);
        self.additional_limits = ::std::option::Option::Some(v);
        self
    }
    /// <p> List of additional limits that are specific to a given InstanceType and for each of it's <code> <code>InstanceRole</code> </code> . </p>
    pub fn set_additional_limits(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AdditionalLimit>>) -> Self {
        self.additional_limits = input;
        self
    }
    /// <p> List of additional limits that are specific to a given InstanceType and for each of it's <code> <code>InstanceRole</code> </code> . </p>
    pub fn get_additional_limits(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AdditionalLimit>> {
        &self.additional_limits
    }
    /// Consumes the builder and constructs a [`Limits`](crate::types::Limits).
    pub fn build(self) -> crate::types::Limits {
        crate::types::Limits {
            storage_types: self.storage_types,
            instance_limits: self.instance_limits,
            additional_limits: self.additional_limits,
        }
    }
}
