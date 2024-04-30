// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListBucketsOutput {
    /// <p>The list of buckets owned by the requester.</p>
    pub buckets: ::std::option::Option<::std::vec::Vec<crate::types::Bucket>>,
    /// <p>The owner of the buckets listed.</p>
    pub owner: ::std::option::Option<crate::types::Owner>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl ListBucketsOutput {
    /// <p>The list of buckets owned by the requester.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.buckets.is_none()`.
    pub fn buckets(&self) -> &[crate::types::Bucket] {
        self.buckets.as_deref().unwrap_or_default()
    }
    /// <p>The owner of the buckets listed.</p>
    pub fn owner(&self) -> ::std::option::Option<&crate::types::Owner> {
        self.owner.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for ListBucketsOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListBucketsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListBucketsOutput {
    /// Creates a new builder-style object to manufacture [`ListBucketsOutput`](crate::operation::list_buckets::ListBucketsOutput).
    pub fn builder() -> crate::operation::list_buckets::builders::ListBucketsOutputBuilder {
        crate::operation::list_buckets::builders::ListBucketsOutputBuilder::default()
    }
}

/// A builder for [`ListBucketsOutput`](crate::operation::list_buckets::ListBucketsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListBucketsOutputBuilder {
    pub(crate) buckets: ::std::option::Option<::std::vec::Vec<crate::types::Bucket>>,
    pub(crate) owner: ::std::option::Option<crate::types::Owner>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl ListBucketsOutputBuilder {
    /// Appends an item to `buckets`.
    ///
    /// To override the contents of this collection use [`set_buckets`](Self::set_buckets).
    ///
    /// <p>The list of buckets owned by the requester.</p>
    pub fn buckets(mut self, input: crate::types::Bucket) -> Self {
        let mut v = self.buckets.unwrap_or_default();
        v.push(input);
        self.buckets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of buckets owned by the requester.</p>
    pub fn set_buckets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Bucket>>) -> Self {
        self.buckets = input;
        self
    }
    /// <p>The list of buckets owned by the requester.</p>
    pub fn get_buckets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Bucket>> {
        &self.buckets
    }
    /// <p>The owner of the buckets listed.</p>
    pub fn owner(mut self, input: crate::types::Owner) -> Self {
        self.owner = ::std::option::Option::Some(input);
        self
    }
    /// <p>The owner of the buckets listed.</p>
    pub fn set_owner(mut self, input: ::std::option::Option<crate::types::Owner>) -> Self {
        self.owner = input;
        self
    }
    /// <p>The owner of the buckets listed.</p>
    pub fn get_owner(&self) -> &::std::option::Option<crate::types::Owner> {
        &self.owner
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListBucketsOutput`](crate::operation::list_buckets::ListBucketsOutput).
    pub fn build(self) -> crate::operation::list_buckets::ListBucketsOutput {
        crate::operation::list_buckets::ListBucketsOutput {
            buckets: self.buckets,
            owner: self.owner,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
