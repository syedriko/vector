// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`VerifySMSSandboxPhoneNumber`](crate::operation::verify_sms_sandbox_phone_number::builders::VerifySMSSandboxPhoneNumberFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`phone_number(impl Into<String>)`](crate::operation::verify_sms_sandbox_phone_number::builders::VerifySMSSandboxPhoneNumberFluentBuilder::phone_number) / [`set_phone_number(Option<String>)`](crate::operation::verify_sms_sandbox_phone_number::builders::VerifySMSSandboxPhoneNumberFluentBuilder::set_phone_number):<br>required: **true**<br><p>The destination phone number to verify.</p><br>
    ///   - [`one_time_password(impl Into<String>)`](crate::operation::verify_sms_sandbox_phone_number::builders::VerifySMSSandboxPhoneNumberFluentBuilder::one_time_password) / [`set_one_time_password(Option<String>)`](crate::operation::verify_sms_sandbox_phone_number::builders::VerifySMSSandboxPhoneNumberFluentBuilder::set_one_time_password):<br>required: **true**<br><p>The OTP sent to the destination number from the <code>CreateSMSSandBoxPhoneNumber</code> call.</p><br>
    /// - On success, responds with [`VerifySmsSandboxPhoneNumberOutput`](crate::operation::verify_sms_sandbox_phone_number::VerifySmsSandboxPhoneNumberOutput)
    /// - On failure, responds with [`SdkError<VerifySMSSandboxPhoneNumberError>`](crate::operation::verify_sms_sandbox_phone_number::VerifySMSSandboxPhoneNumberError)
    pub fn verify_sms_sandbox_phone_number(
        &self,
    ) -> crate::operation::verify_sms_sandbox_phone_number::builders::VerifySMSSandboxPhoneNumberFluentBuilder {
        crate::operation::verify_sms_sandbox_phone_number::builders::VerifySMSSandboxPhoneNumberFluentBuilder::new(self.handle.clone())
    }
}
