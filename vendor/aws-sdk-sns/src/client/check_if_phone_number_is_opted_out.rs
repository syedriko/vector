// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CheckIfPhoneNumberIsOptedOut`](crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`phone_number(impl Into<String>)`](crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutFluentBuilder::phone_number) / [`set_phone_number(Option<String>)`](crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutFluentBuilder::set_phone_number):<br>required: **true**<br><p>The phone number for which you want to check the opt out status.</p><br>
    /// - On success, responds with [`CheckIfPhoneNumberIsOptedOutOutput`](crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutOutput) with field(s):
    ///   - [`is_opted_out(bool)`](crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutOutput::is_opted_out): <p>Indicates whether the phone number is opted out:</p>  <ul>   <li> <p> <code>true</code> – The phone number is opted out, meaning you cannot publish SMS messages to it.</p> </li>   <li> <p> <code>false</code> – The phone number is opted in, meaning you can publish SMS messages to it.</p> </li>  </ul>
    /// - On failure, responds with [`SdkError<CheckIfPhoneNumberIsOptedOutError>`](crate::operation::check_if_phone_number_is_opted_out::CheckIfPhoneNumberIsOptedOutError)
    pub fn check_if_phone_number_is_opted_out(
        &self,
    ) -> crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutFluentBuilder {
        crate::operation::check_if_phone_number_is_opted_out::builders::CheckIfPhoneNumberIsOptedOutFluentBuilder::new(self.handle.clone())
    }
}
