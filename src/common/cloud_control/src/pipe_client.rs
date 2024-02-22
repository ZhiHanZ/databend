// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use databend_common_exception::Result;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::Request;

use crate::client_config::ClientConfig;
use crate::pb::{AlterPipeRequest, AlterPipeResponse, CreatePipeRequest, CreatePipeResponse, DropPipeRequest, DropPipeResponse, GetPipeRequest, GetPipeResponse, ListPipeRequest, ListPipeResponse, SuspendPipeRequest, SuspendPipeResponse};
use crate::pb::pipe_v2_service_client::PipeV2ServiceClient;

const PIPE_CLIENT_VERSION: &str = "v1";
const PIPE_CLIENT_VERSION_NAME: &str = "PIPE_CLIENT_VERSION";

pub struct PipeClient {
    pub pipe_client: PipeV2ServiceClient<Channel>,
}

// add necessary metadata and client request setup for auditing and tracing purpose
pub fn make_request<T>(t: T, config: ClientConfig) -> Request<T> {
    let mut request = Request::new(t);
    request.set_timeout(config.get_timeout());
    let metadata = request.metadata_mut();
    let config_meta = config.get_metadata().clone();
    for (k, v) in config_meta {
        let key = k
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            .unwrap();
        metadata.insert(key, v.parse().unwrap());
    }
    metadata.insert(
        PIPE_CLIENT_VERSION_NAME
            .to_string()
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            .unwrap(),
        PIPE_CLIENT_VERSION.to_string().parse().unwrap(),
    );
    request
}

impl PipeClient {
    // TODO: add auth interceptor
    pub async fn new(endpoint: Endpoint) -> Result<Arc<PipeClient>> {
        let channel = endpoint.connect_lazy();
        let pipe_client = PipeV2ServiceClient::new(channel);
        Ok(Arc::new(PipeClient { pipe_client: pipe_client }))
    }

    // TODO: richer error handling on Pipe Error
    pub async fn create_pipe(&self, req: Request<CreatePipeRequest>) -> Result<CreatePipeResponse> {
        let mut client = self.pipe_client.clone();
        let resp = client.create_pipe(req).await?;
        Ok(resp.into_inner())
    }

    // TODO: richer error handling on Pipe Error
    pub async fn get_pipe(
        &self,
        req: Request<GetPipeRequest>,
    ) -> Result<GetPipeResponse> {
        let mut client = self.pipe_client.clone();
        let resp = client.get_pipe(req).await?;
        Ok(resp.into_inner())
    }

    // TODO: richer error handling on Pipe Error
    pub async fn drop_pipe(&self, req: Request<DropPipeRequest>) -> Result<DropPipeResponse> {
        let mut client = self.pipe_client.clone();
        let resp = client.drop_pipe(req).await?;
        Ok(resp.into_inner())
    }

    // TODO: richer error handling on Pipe Error
    pub async fn alter_pipe(&self, req: Request<AlterPipeRequest>) -> Result<AlterPipeResponse> {
        let mut client = self.pipe_client.clone();
        let resp = client.alter_pipe(req).await?;
        Ok(resp.into_inner())
    }

    // TODO: richer error handling on Pipe Error
    pub async fn list_pipes(&self, req: Request<ListPipeRequest>) -> Result<ListPipeResponse> {
        let mut client = self.pipe_client.clone();
        let resp = client.list_pipe(req).await?;
        Ok(resp.into_inner())
    }
    // TODO: richer error handling on Pipe Error
    pub async fn suspend_pipe(&self, req: Request<SuspendPipeRequest>) -> Result<SuspendPipeResponse> {
        let mut client = self.pipe_client.clone();
        let resp = client.suspend_pipe(req).await?;
        Ok(resp.into_inner())
    }

    // TODO list pipe tasks, insert pipe
}
