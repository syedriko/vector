// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Status of a daily automated snapshot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SnapshotOptionsStatus {
    /// <p>Specifies the daily snapshot options specified for the Elasticsearch domain.</p>
    pub options: ::std::option::Option<crate::types::SnapshotOptions>,
    /// <p>Specifies the status of a daily automated snapshot.</p>
    pub status: ::std::option::Option<crate::types::OptionStatus>,
}
impl SnapshotOptionsStatus {
    /// <p>Specifies the daily snapshot options specified for the Elasticsearch domain.</p>
    pub fn options(&self) -> ::std::option::Option<&crate::types::SnapshotOptions> {
        self.options.as_ref()
    }
    /// <p>Specifies the status of a daily automated snapshot.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::OptionStatus> {
        self.status.as_ref()
    }
}
impl SnapshotOptionsStatus {
    /// Creates a new builder-style object to manufacture [`SnapshotOptionsStatus`](crate::types::SnapshotOptionsStatus).
    pub fn builder() -> crate::types::builders::SnapshotOptionsStatusBuilder {
        crate::types::builders::SnapshotOptionsStatusBuilder::default()
    }
}

/// A builder for [`SnapshotOptionsStatus`](crate::types::SnapshotOptionsStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SnapshotOptionsStatusBuilder {
    pub(crate) options: ::std::option::Option<crate::types::SnapshotOptions>,
    pub(crate) status: ::std::option::Option<crate::types::OptionStatus>,
}
impl SnapshotOptionsStatusBuilder {
    /// <p>Specifies the daily snapshot options specified for the Elasticsearch domain.</p>
    /// This field is required.
    pub fn options(mut self, input: crate::types::SnapshotOptions) -> Self {
        self.options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the daily snapshot options specified for the Elasticsearch domain.</p>
    pub fn set_options(mut self, input: ::std::option::Option<crate::types::SnapshotOptions>) -> Self {
        self.options = input;
        self
    }
    /// <p>Specifies the daily snapshot options specified for the Elasticsearch domain.</p>
    pub fn get_options(&self) -> &::std::option::Option<crate::types::SnapshotOptions> {
        &self.options
    }
    /// <p>Specifies the status of a daily automated snapshot.</p>
    /// This field is required.
    pub fn status(mut self, input: crate::types::OptionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the status of a daily automated snapshot.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::OptionStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Specifies the status of a daily automated snapshot.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::OptionStatus> {
        &self.status
    }
    /// Consumes the builder and constructs a [`SnapshotOptionsStatus`](crate::types::SnapshotOptionsStatus).
    pub fn build(self) -> crate::types::SnapshotOptionsStatus {
        crate::types::SnapshotOptionsStatus {
            options: self.options,
            status: self.status,
        }
    }
}
