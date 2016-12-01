
use maidsafe_utilities::serialisation::{serialise, deserialise};
use rustc_serialize::{Decodable, Encodable};
use errors::ProtocolError;

use rustc_serialize::base64::{ToBase64, FromBase64, URL_SAFE};

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
        }
        .and_then(|id| id.from_base64().map_err(From::from))
        .and_then(|id| String::from_utf8(id).map_err(From::from)));
    let payload = try!(parts.next()
        .ok_or(ProtocolError::IncompleteUri("missing payload"))
        .and_then(|payload| payload.from_base64().map_err(From::from))
        .and_then(|payload| deserialise(&payload).map_err(From::from)));
    Ok((app_id.to_string(), action.to_string(), payload))
}

/// TODO: docs
pub fn make_to_uri<T>(app_id: String,
                      action: String,
                      data: &T,
                      app_id_is_receiver: bool)
                      -> Result<String, ProtocolError>
    where T: Encodable
{
    serialise(&data)
        .and_then(|encoded| {
            let payload = encoded.to_base64(URL_SAFE);
            let encoded_app_id = app_id.into_bytes().to_base64(URL_SAFE);

            Ok(if app_id_is_receiver {
                format!("safe-{}:{}:{}", encoded_app_id, action, payload)
            } else {
                format!("safeauth:{}:{}:{}", action, encoded_app_id, payload)
            })
        })
        .map_err(From::from)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
    struct TestStruct {
        payload: String,
    }
    #[test]
    fn simple_back_and_forth() {
        let data = (1u64..8).collect::<Vec<_>>();
        let uri = make_to_uri("net.maidsafe.example.test".to_string(),
                              "auth".to_string(),
                              &data,
                              false)
            .unwrap();
        println!("{:}", uri);
        let (app_id, action, payload): (String, String, Vec<u64>) = parse_from_uri(uri).unwrap();

        assert_eq!(action, "auth");
        assert_eq!(payload, data);
        assert_eq!(app_id, "net.maidsafe.example.test");
    }

    #[test]
    fn sending_back() {
        let data = (1u64..8).collect::<Vec<_>>();
        let uri = make_to_uri("net.maidsafe.example.test".to_string(),
                              "register".to_string(),
                              &data,
                              true)
            .unwrap();
        println!("{:}", uri);
        let (app_id, action, payload): (String, String, Vec<u64>) = parse_from_uri(uri).unwrap();

        assert_eq!(action, "register");
        assert_eq!(payload, data);
        assert_eq!(app_id, "net.maidsafe.example.test");
    }


    #[test]
    fn complicated_back_and_forth() {
        let data = TestStruct { payload: "simple Data".to_string() };
        let uri = make_to_uri("net:maidsafe".to_string(),
                              "call-to-action".to_string(),
                              &data,
                              false)
            .unwrap();
        println!("{:}", uri);
        let (app_id, action, payload): (String, String, TestStruct) = parse_from_uri(uri).unwrap();

        assert_eq!(action, "call-to-action");
        assert_eq!(payload, data);
        assert_eq!(app_id, "net:maidsafe");
    }
}
