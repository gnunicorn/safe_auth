
use maidsafe_utilities::serialisation::SerialisationError;

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

        IncompleteUri(descr: &'static str) {
            description(descr)
            display("Error {}", descr)
        }
    }
}