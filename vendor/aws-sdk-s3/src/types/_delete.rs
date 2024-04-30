// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the objects to delete.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Delete {
    /// <p>The object to delete.</p> <note>
    /// <p> <b>Directory buckets</b> - For directory buckets, an object that's composed entirely of whitespace characters is not supported by the <code>DeleteObjects</code> API operation. The request will receive a <code>400 Bad Request</code> error and none of the objects in the request will be deleted.</p>
    /// </note>
    pub objects: ::std::vec::Vec<crate::types::ObjectIdentifier>,
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set its value to <code>true</code>.</p>
    pub quiet: ::std::option::Option<bool>,
}
impl Delete {
    /// <p>The object to delete.</p> <note>
    /// <p> <b>Directory buckets</b> - For directory buckets, an object that's composed entirely of whitespace characters is not supported by the <code>DeleteObjects</code> API operation. The request will receive a <code>400 Bad Request</code> error and none of the objects in the request will be deleted.</p>
    /// </note>
    pub fn objects(&self) -> &[crate::types::ObjectIdentifier] {
        use std::ops::Deref;
        self.objects.deref()
    }
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set its value to <code>true</code>.</p>
    pub fn quiet(&self) -> ::std::option::Option<bool> {
        self.quiet
    }
}
impl Delete {
    /// Creates a new builder-style object to manufacture [`Delete`](crate::types::Delete).
    pub fn builder() -> crate::types::builders::DeleteBuilder {
        crate::types::builders::DeleteBuilder::default()
    }
}

/// A builder for [`Delete`](crate::types::Delete).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteBuilder {
    pub(crate) objects: ::std::option::Option<::std::vec::Vec<crate::types::ObjectIdentifier>>,
    pub(crate) quiet: ::std::option::Option<bool>,
}
impl DeleteBuilder {
    /// Appends an item to `objects`.
    ///
    /// To override the contents of this collection use [`set_objects`](Self::set_objects).
    ///
    /// <p>The object to delete.</p> <note>
    /// <p> <b>Directory buckets</b> - For directory buckets, an object that's composed entirely of whitespace characters is not supported by the <code>DeleteObjects</code> API operation. The request will receive a <code>400 Bad Request</code> error and none of the objects in the request will be deleted.</p>
    /// </note>
    pub fn objects(mut self, input: crate::types::ObjectIdentifier) -> Self {
        let mut v = self.objects.unwrap_or_default();
        v.push(input);
        self.objects = ::std::option::Option::Some(v);
        self
    }
    /// <p>The object to delete.</p> <note>
    /// <p> <b>Directory buckets</b> - For directory buckets, an object that's composed entirely of whitespace characters is not supported by the <code>DeleteObjects</code> API operation. The request will receive a <code>400 Bad Request</code> error and none of the objects in the request will be deleted.</p>
    /// </note>
    pub fn set_objects(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ObjectIdentifier>>) -> Self {
        self.objects = input;
        self
    }
    /// <p>The object to delete.</p> <note>
    /// <p> <b>Directory buckets</b> - For directory buckets, an object that's composed entirely of whitespace characters is not supported by the <code>DeleteObjects</code> API operation. The request will receive a <code>400 Bad Request</code> error and none of the objects in the request will be deleted.</p>
    /// </note>
    pub fn get_objects(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ObjectIdentifier>> {
        &self.objects
    }
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set its value to <code>true</code>.</p>
    pub fn quiet(mut self, input: bool) -> Self {
        self.quiet = ::std::option::Option::Some(input);
        self
    }
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set its value to <code>true</code>.</p>
    pub fn set_quiet(mut self, input: ::std::option::Option<bool>) -> Self {
        self.quiet = input;
        self
    }
    /// <p>Element to enable quiet mode for the request. When you add this element, you must set its value to <code>true</code>.</p>
    pub fn get_quiet(&self) -> &::std::option::Option<bool> {
        &self.quiet
    }
    /// Consumes the builder and constructs a [`Delete`](crate::types::Delete).
    /// This method will fail if any of the following fields are not set:
    /// - [`objects`](crate::types::builders::DeleteBuilder::objects)
    pub fn build(self) -> ::std::result::Result<crate::types::Delete, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Delete {
            objects: self.objects.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "objects",
                    "objects was not specified but it is required when building Delete",
                )
            })?,
            quiet: self.quiet,
        })
    }
}
