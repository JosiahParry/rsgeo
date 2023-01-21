// taken from https://github.com/geopolars/geopolars/blob/master/geopolars/src/spatial_index.rs#L208

use geo::geometry::{Point, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon, Line};
use geo::BoundingRect;
use rstar::{AABB, RTreeObject};

#[derive(Debug)]
pub enum NodeEnvelope {
    Point([f64; 2]),
    BBox([[f64; 2]; 2]),
}

impl From<Point> for NodeEnvelope {
    fn from(point: Point<f64>) -> Self {
        NodeEnvelope::Point([point.x(), point.y()])
    }
}

impl From<Polygon> for NodeEnvelope {
    fn from(polygon: Polygon<f64>) -> Self {
        let envelope = polygon.bounding_rect().unwrap();
        NodeEnvelope::BBox([
            [envelope.min().x, envelope.min().y],
            [envelope.max().x, envelope.max().y],
        ])
    }
}


// implement a from &Polygon, this is bad because it clones
impl From<&Polygon> for NodeEnvelope {
    fn from(polygon: &Polygon<f64>) -> Self {
        let envelope = polygon.clone().bounding_rect().unwrap();
        NodeEnvelope::BBox([
            [envelope.min().x, envelope.min().y],
            [envelope.max().x, envelope.max().y],
        ])
    }
}

impl From<MultiPolygon> for NodeEnvelope {
    fn from(multi_polygon: MultiPolygon<f64>) -> Self {
        let envelope = multi_polygon.bounding_rect().unwrap();
        NodeEnvelope::BBox([
            [envelope.min().x, envelope.min().y],
            [envelope.max().x, envelope.max().y],
        ])
    }
}

impl From<MultiPoint<f64>> for NodeEnvelope {
    fn from(multi_point: MultiPoint<f64>) -> Self {
        let envelope = multi_point.bounding_rect().unwrap();
        NodeEnvelope::BBox([
            [envelope.min().x, envelope.min().y],
            [envelope.max().x, envelope.max().y],
        ])
    }
}

impl From<LineString<f64>> for NodeEnvelope {
    fn from(line: LineString<f64>) -> Self {
        let envelope = line.bounding_rect().unwrap();
        NodeEnvelope::BBox([
            [envelope.min().x, envelope.min().y],
            [envelope.max().x, envelope.max().y],
        ])
    }
}

impl From<MultiLineString<f64>> for NodeEnvelope {
    fn from(multi_line: MultiLineString<f64>) -> Self {
        let envelope = multi_line.bounding_rect().unwrap();
        NodeEnvelope::BBox([
            [envelope.min().x, envelope.min().y],
            [envelope.max().x, envelope.max().y],
        ])
    }
}

impl From<Line<f64>> for NodeEnvelope {
    fn from(line: Line<f64>) -> Self {
        let envelope = line.bounding_rect();
        NodeEnvelope::BBox([
            [envelope.min().x, envelope.min().y],
            [envelope.max().x, envelope.max().y],
        ])
    }
}

#[derive(Debug)]
pub struct TreeNode {
    pub index: usize,
    pub envelope: NodeEnvelope,
}

impl RTreeObject for TreeNode {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        match self.envelope {
            NodeEnvelope::Point(point) => AABB::from_point(point),
            NodeEnvelope::BBox(bbox) => AABB::from_corners(bbox[0], bbox[1]),
        }
    }
}
