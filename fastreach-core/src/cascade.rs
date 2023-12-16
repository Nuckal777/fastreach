use geo::{
    BoundingRect, ConvexHull, CoordsIter, GeoFloat, HaversineDistance, Intersects, MultiPolygon,
    Polygon,
};
use geo_clipper::Clipper;
use num_traits::FromPrimitive;
use rstar::{ParentNode, RTree, RTreeNode, RTreeObject};

use crate::graph::TimedNode;

const SCALE_FACTOR: f32 = 2000.0;

#[must_use]
pub fn union_polys<T: GeoFloat>(polys: Vec<Polygon<T>>) -> MultiPolygon<T> {
    let tree = RTree::<Polygon<T>>::bulk_load(polys);
    bottom_up_fold_reduce(
        &tree,
        || MultiPolygon::<T>::new(Vec::new()),
        |acc, elem| {
            if acc.exterior_coords_iter().count() == 0 {
                return MultiPolygon::new(vec![elem.clone()]);
            }
            if acc
                .bounding_rect()
                .unwrap()
                .intersects(&elem.bounding_rect().unwrap())
            {
                acc.union(
                    &MultiPolygon::new(vec![elem.clone()]),
                    num_traits::cast(SCALE_FACTOR).unwrap(),
                )
            } else {
                let mut polys = acc.0;
                polys.push(elem.clone());
                MultiPolygon::new(polys)
            }
        },
        |a, b| {
            if a.exterior_coords_iter().count() == 0 {
                return b;
            }
            if b.exterior_coords_iter().count() == 0 {
                return a;
            }
            if a.bounding_rect()
                .unwrap()
                .intersects(&b.bounding_rect().unwrap())
            {
                a.union(&b, num_traits::cast(SCALE_FACTOR).unwrap())
            } else {
                let mut polys = a.0;
                polys.extend(b.0);
                MultiPolygon::new(polys)
            }
        },
    )
}

#[must_use]
pub fn union(tree: &RTree<&TimedNode<'_, '_>>) -> MultiPolygon<f32> {
    bottom_up_fold_reduce(
        tree,
        || MultiPolygon::<f32>::new(Vec::new()),
        |acc, elem| {
            acc.union(
                &MultiPolygon::new(vec![elem.to_poly()]),
                num_traits::cast(SCALE_FACTOR).unwrap(),
            )
        },
        |a, b| a.union(&b, num_traits::cast(SCALE_FACTOR).unwrap()),
    )
}

// https://github.com/georust/rstar/issues/80#issuecomment-988615807
fn bottom_up_fold_reduce<T, S, I, F, R>(
    tree: &RTree<T>,
    mut init: I,
    mut fold: F,
    mut reduce: R,
) -> S
where
    T: RTreeObject,
    I: FnMut() -> S,
    F: FnMut(S, &T) -> S,
    R: FnMut(S, S) -> S,
{
    fn inner<T, S, I, F, R>(parent: &ParentNode<T>, init: &mut I, fold: &mut F, reduce: &mut R) -> S
    where
        T: RTreeObject,
        I: FnMut() -> S,
        F: FnMut(S, &T) -> S,
        R: FnMut(S, S) -> S,
    {
        parent
            .children()
            .iter()
            .fold(init(), |accum, child| match child {
                RTreeNode::Leaf(value) => fold(accum, value),
                RTreeNode::Parent(parent) => {
                    let value = inner(parent, init, fold, reduce);

                    reduce(accum, value)
                }
            })
    }

    inner(tree.root(), &mut init, &mut fold, &mut reduce)
}

#[must_use]
pub fn diameter<T: GeoFloat + FromPrimitive>(poly: &MultiPolygon<T>) -> T {
    let hull = poly.convex_hull();
    // TODO: there is a linear algorithm to get the diameter in O(N) in euclidian space
    let mut diameter = T::zero();
    for i in hull.exterior().points() {
        for j in hull.exterior().points() {
            let distance = i.haversine_distance(&j);
            if distance > diameter {
                diameter = distance;
            }
        }
    }
    diameter
}
