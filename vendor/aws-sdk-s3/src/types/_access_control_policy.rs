// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccessControlPolicy {
    /// <p>A list of grants.</p>
    pub grants: ::std::option::Option<::std::vec::Vec<crate::types::Grant>>,
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub owner: ::std::option::Option<crate::types::Owner>,
}
impl AccessControlPolicy {
    /// <p>A list of grants.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.grants.is_none()`.
    pub fn grants(&self) -> &[crate::types::Grant] {
        self.grants.as_deref().unwrap_or_default()
    }
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub fn owner(&self) -> ::std::option::Option<&crate::types::Owner> {
        self.owner.as_ref()
    }
}
impl AccessControlPolicy {
    /// Creates a new builder-style object to manufacture [`AccessControlPolicy`](crate::types::AccessControlPolicy).
    pub fn builder() -> crate::types::builders::AccessControlPolicyBuilder {
        crate::types::builders::AccessControlPolicyBuilder::default()
    }
}

/// A builder for [`AccessControlPolicy`](crate::types::AccessControlPolicy).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AccessControlPolicyBuilder {
    pub(crate) grants: ::std::option::Option<::std::vec::Vec<crate::types::Grant>>,
    pub(crate) owner: ::std::option::Option<crate::types::Owner>,
}
impl AccessControlPolicyBuilder {
    /// Appends an item to `grants`.
    ///
    /// To override the contents of this collection use [`set_grants`](Self::set_grants).
    ///
    /// <p>A list of grants.</p>
    pub fn grants(mut self, input: crate::types::Grant) -> Self {
        let mut v = self.grants.unwrap_or_default();
        v.push(input);
        self.grants = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of grants.</p>
    pub fn set_grants(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Grant>>) -> Self {
        self.grants = input;
        self
    }
    /// <p>A list of grants.</p>
    pub fn get_grants(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Grant>> {
        &self.grants
    }
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub fn owner(mut self, input: crate::types::Owner) -> Self {
        self.owner = ::std::option::Option::Some(input);
        self
    }
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub fn set_owner(mut self, input: ::std::option::Option<crate::types::Owner>) -> Self {
        self.owner = input;
        self
    }
    /// <p>Container for the bucket owner's display name and ID.</p>
    pub fn get_owner(&self) -> &::std::option::Option<crate::types::Owner> {
        &self.owner
    }
    /// Consumes the builder and constructs a [`AccessControlPolicy`](crate::types::AccessControlPolicy).
    pub fn build(self) -> crate::types::AccessControlPolicy {
        crate::types::AccessControlPolicy {
            grants: self.grants,
            owner: self.owner,
        }
    }
}
