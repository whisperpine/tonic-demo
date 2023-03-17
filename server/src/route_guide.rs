#![allow(dead_code)]
#![allow(unused_variables)]

use crate::proto::route_guide_server::RouteGuide;
use crate::proto::{Feature, Point, Rectangle, RouteNote, RouteSummary};
use std::sync::Arc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        todo!()
    }

    /// Server streaming response type for the ListFeatures method.
    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        todo!()
    }

    async fn record_route(
        &self,
        request: Request<Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        todo!()
    }

    /// Server streaming response type for the RouteChat method.
    type RouteChatStream = ReceiverStream<Result<RouteNote, Status>>;

    async fn route_chat(
        &self,
        request: Request<Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        todo!()
    }
}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
}

impl Eq for Point {}
