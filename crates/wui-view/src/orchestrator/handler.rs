use crate::prelude::*;

impl Orchestrator {
    pub(crate) async fn handle_query(&mut self, query: Query) -> Result<()> {
        let request = query.request;
        let response = query.response;

        match request {
            _ => match response {
                Some(response) => response
                    .send(Response::NotImplemented)
                    .map_err(|e| eyre::eyre!("Failed to send response: {:?}", e)),
                None => Ok(()),
            },
        }
    }
}
