use geo::{BooleanOps, MultiPolygon, Polygon, ConvexHull, GeodesicDistance};
use rstar::{ParentNode, RTree, RTreeNode, RTreeObject};

#[must_use]
pub fn union(polys: Vec<Polygon<f64>>) -> MultiPolygon<f64> {
    let tree = RTree::<Polygon<f64>>::bulk_load(polys);
    bottom_up_fold_reduce(
        &tree,
        || MultiPolygon::<f64>::new(Vec::new()),
        |acc, elem| acc.union(&MultiPolygon::new(vec![elem.clone()])),
        |a, b| a.union(&b),
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
pub fn diameter(poly: &MultiPolygon<f64>) -> f64 {
    let hull = poly.convex_hull();
    // TODO: there is a linear algorithm to get the diameter in O(N) in euclidian space
    let mut diameter = 0.0_f64;
    for i in hull.exterior().points() {
        for j in hull.exterior().points() {
            let distance = i.geodesic_distance(&j);
            if distance > diameter {
                diameter = distance;
            }
        }   
    }
    diameter
}
