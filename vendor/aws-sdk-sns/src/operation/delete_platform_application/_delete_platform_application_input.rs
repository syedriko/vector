// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for DeletePlatformApplication action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeletePlatformApplicationInput {
    /// <p>PlatformApplicationArn of platform application object to delete.</p>
    pub platform_application_arn: ::std::option::Option<::std::string::String>,
}
impl DeletePlatformApplicationInput {
    /// <p>PlatformApplicationArn of platform application object to delete.</p>
    pub fn platform_application_arn(&self) -> ::std::option::Option<&str> {
        self.platform_application_arn.as_deref()
    }
}
impl DeletePlatformApplicationInput {
    /// Creates a new builder-style object to manufacture [`DeletePlatformApplicationInput`](crate::operation::delete_platform_application::DeletePlatformApplicationInput).
    pub fn builder() -> crate::operation::delete_platform_application::builders::DeletePlatformApplicationInputBuilder {
        crate::operation::delete_platform_application::builders::DeletePlatformApplicationInputBuilder::default()
    }
}

/// A builder for [`DeletePlatformApplicationInput`](crate::operation::delete_platform_application::DeletePlatformApplicationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeletePlatformApplicationInputBuilder {
    pub(crate) platform_application_arn: ::std::option::Option<::std::string::String>,
}
impl DeletePlatformApplicationInputBuilder {
    /// <p>PlatformApplicationArn of platform application object to delete.</p>
    /// This field is required.
    pub fn platform_application_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.platform_application_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>PlatformApplicationArn of platform application object to delete.</p>
    pub fn set_platform_application_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.platform_application_arn = input;
        self
    }
    /// <p>PlatformApplicationArn of platform application object to delete.</p>
    pub fn get_platform_application_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.platform_application_arn
    }
    /// Consumes the builder and constructs a [`DeletePlatformApplicationInput`](crate::operation::delete_platform_application::DeletePlatformApplicationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_platform_application::DeletePlatformApplicationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_platform_application::DeletePlatformApplicationInput {
            platform_application_arn: self.platform_application_arn,
        })
    }
}
