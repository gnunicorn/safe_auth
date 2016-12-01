
use maidsafe_utilities::serialisation::{serialise, deserialise};
use rustc_serialize::{Decodable, Encodable};
use errors::ProtocolError;


/// Handy function to get the content from a 
pub fn parse_from_uri<T>(uri: String) -> Result<(String, String, T), ProtocolError>
    where T: Decodable
{
	let mut parts = uri.split(":");
	let protocol = try!(parts.next().ok_or(ProtocolError::IncompleteUri("missing protocol")));
	let action = try!(parts.next().ok_or(ProtocolError::IncompleteUri("missing action")));
	let app_id = try!(if protocol.starts_with("safe-") {
					let (_, id) = protocol.split_at(5);
					// FIXME: must be base64
					Ok(id)
				} else {
					parts.next().ok_or(ProtocolError::IncompleteUri("missing app id"))
				});
	let payload = try!(parts.next().ok_or(ProtocolError::IncompleteUri("missing payload")));
	let decoded = try!(deserialise(&payload.as_bytes()));
	Ok((app_id.to_string(), action.to_string(), decoded))
}

/// TODO: docs
pub fn make_to_uri<T>(action: String, data: &T, app_id: Option<String>) -> Result<String, ProtocolError>
    where T: Encodable
{
	serialise(&data).and_then(|encoded| {
		let payload = String::from_utf8(encoded).unwrap();
		Ok(match app_id {
			Some(ref app_id) => format!("safe-{}:{}:{}", app_id, action, payload), // this should be base64
			None => format!("safeauth:{}:{}", action, payload)
		})
	}).map_err(From::from)
}




#[cfg(test)]
mod tests {
	use super::*;


	#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
	struct Test {}
    #[test]
    fn simple_back_and_forth() {
        let data = (1u64..8).collect::<Vec<_>>();
    	let uri = make_to_uri("auth".to_string(), &data , Some("net.maidsafe.example.test".to_string())).unwrap();
    	let (app_id, action, payload) : (String, String, Vec<u64>) = parse_from_uri(uri).unwrap();

    	assert_eq!(action, "auth");
    	assert_eq!(payload, data );
    	assert_eq!(app_id, "net.maidsafe.example.test");
    }
}