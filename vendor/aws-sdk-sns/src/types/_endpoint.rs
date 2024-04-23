// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The endpoint for mobile app and device.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Endpoint {
    /// <p>The <code>EndpointArn</code> for mobile app and device.</p>
    pub endpoint_arn: ::std::option::Option<::std::string::String>,
    /// <p>Attributes for endpoint.</p>
    pub attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl Endpoint {
    /// <p>The <code>EndpointArn</code> for mobile app and device.</p>
    pub fn endpoint_arn(&self) -> ::std::option::Option<&str> {
        self.endpoint_arn.as_deref()
    }
    /// <p>Attributes for endpoint.</p>
    pub fn attributes(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.attributes.as_ref()
    }
}
impl Endpoint {
    /// Creates a new builder-style object to manufacture [`Endpoint`](crate::types::Endpoint).
    pub fn builder() -> crate::types::builders::EndpointBuilder {
        crate::types::builders::EndpointBuilder::default()
    }
}

/// A builder for [`Endpoint`](crate::types::Endpoint).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EndpointBuilder {
    pub(crate) endpoint_arn: ::std::option::Option<::std::string::String>,
    pub(crate) attributes: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl EndpointBuilder {
    /// <p>The <code>EndpointArn</code> for mobile app and device.</p>
    pub fn endpoint_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>EndpointArn</code> for mobile app and device.</p>
    pub fn set_endpoint_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint_arn = input;
        self
    }
    /// <p>The <code>EndpointArn</code> for mobile app and device.</p>
    pub fn get_endpoint_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.endpoint_arn
    }
    /// Adds a key-value pair to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>Attributes for endpoint.</p>
    pub fn attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.attributes.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.attributes = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Attributes for endpoint.</p>
    pub fn set_attributes(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.attributes = input;
        self
    }
    /// <p>Attributes for endpoint.</p>
    pub fn get_attributes(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.attributes
    }
    /// Consumes the builder and constructs a [`Endpoint`](crate::types::Endpoint).
    pub fn build(self) -> crate::types::Endpoint {
        crate::types::Endpoint {
            endpoint_arn: self.endpoint_arn,
            attributes: self.attributes,
        }
    }
}
