#![allow(dead_code)]
#![allow(unused_variables)]

use crate::proto::route_guide_server::RouteGuide;
use crate::proto::{Feature, Point, Rectangle, RouteNote, RouteSummary};
use std::sync::Arc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}

impl RouteGuideService {
    pub fn new(features: Vec<Feature>) -> Self {
        Self {
            features: Arc::new(features),
        }
    }
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        for feature in &self.features[..] {
            if feature.location.as_ref() == Some(request.get_ref()) {
                return Ok(Response::new(feature.clone()));
            }
        }

        let status = Status::new(tonic::Code::NotFound, "given point not found");
        Err(status)
    }

    /// Server streaming response type for the ListFeatures method.
    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            for feature in &features[..] {
                if in_range(feature, request.get_ref()) {
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn record_route(
        &self,
        request: Request<Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        let mut stream = request.into_inner();
        let mut summary = RouteSummary::default();
        let mut last_point = None;
        let now = std::time::Instant::now();

        while let Some(point) = stream.next().await {
            let point = point?;
            summary.point_count += 1;

            for feature in &self.features[..] {
                if feature.location.as_ref() == Some(&point) {
                    summary.feature_count += 1;
                }
            }

            // if let Some(last) = last_point {
            //     summary.distance += calc_distance(&last, &point);
            // }

            last_point = Some(point);
        }

        summary.elapsed_time = now.elapsed().as_secs_f32();

        Ok(Response::new(summary))
    }

    /// Server streaming response type for the RouteChat method.
    type RouteChatStream = ReceiverStream<Result<RouteNote, Status>>;

    async fn route_chat(
        &self,
        request: Request<Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        let mut stream = request.into_inner();
        let mut notes = std::collections::HashMap::new();

        while let Some(note) = stream.next().await {
            let note = note?;
            let location = note.location.clone().unwrap();
            let location_notes = notes.entry(location).or_insert(vec![]);
            location_notes.push(note);
        }

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

fn in_range(feature: &Feature, rectangle: &Rectangle) -> bool {
    use std::cmp::{max, min};

    let min_lat = min(
        rectangle.corner_one.as_ref().unwrap().latitude,
        rectangle.corner_two.as_ref().unwrap().latitude,
    );
    let max_lat = max(
        rectangle.corner_one.as_ref().unwrap().latitude,
        rectangle.corner_two.as_ref().unwrap().latitude,
    );
    let min_long = min(
        rectangle.corner_one.as_ref().unwrap().longitude,
        rectangle.corner_two.as_ref().unwrap().longitude,
    );
    let max_long = max(
        rectangle.corner_one.as_ref().unwrap().longitude,
        rectangle.corner_two.as_ref().unwrap().longitude,
    );

    let latitude = feature.location.as_ref().unwrap().latitude;
    let longitude = feature.location.as_ref().unwrap().longitude;

    if min_lat < latitude && latitude < max_lat && min_long < longitude && longitude < max_long {
        return true;
    }

    false
}

#[must_use = "returned value must be used"]
fn calc_distance(point_a: &Point, point_b: &Point) -> u32 {
    let powered_dist = (point_a.latitude - point_b.latitude).pow(2)
        + (point_a.longitude - point_b.longitude).pow(2);

    (powered_dist as f32).sqrt() as u32
}
