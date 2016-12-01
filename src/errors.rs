
use maidsafe_utilities::serialisation::SerialisationError;

use rustc_serialize::base64::FromBase64Error;
use std::string::FromUtf8Error;

quick_error! {
    /// Serialisation error.
    #[derive(Debug)]
    pub enum ProtocolError {
        /// Error during serialisation (encoding).
        SerialisationError(err: SerialisationError) {
            description("Serialisation error")
            display("Serialisation error: {}", err)
            cause(err)
            from()
        }
        FromBase64Error(err: FromBase64Error) {
            description("FromBase64Error error")
            display("FromBase64Error error: {}", err)
            cause(err)
            from()
        }

        IncompleteUri(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }

        Malformatted(err: FromUtf8Error){
            description("FromUtf8Error error")
            display("FromUtf8Error error: {}", err)
            cause(err)
            from()
        }
    }
}
