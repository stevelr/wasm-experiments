use bindings::handler::{Error, Handler, Request, Response};
use bindings::strings;

use std::str;

struct Component;

impl Handler for Component {
    fn execute(req: Request) -> Result<Response, Error> {
        // The content should be plain text
        let content_type = req
            .headers
            .iter()
            .find(|(k, _)| k == b"content-type")
            .map(|(_, v)| v)
            .ok_or(Error::BadRequest)?;

        if content_type != b"text/plain" {
            return Err(Error::BadRequest);
        }

        // We assume the body is UTF-8 encoded
        let body = str::from_utf8(&req.body).map_err(|_| Error::BadRequest)?;

        // use upstream concat service
        let greeting = strings::concat("you said: ", &body);

        // Echo the body back in the response
        Ok(Response {
            headers: vec![(b"content-type".to_vec(), b"text/plain".to_vec())],
            body: greeting.into_bytes(),
        })
    }
}

bindings::export!(Component);
