use num_traits::Float;

use crate::{Line, Point2};

impl<P: Point2> Line<P> {
    /// Checks whether the two line segments intersect, where and whether the intersection happens within the given line
    /// segments (or outside).

    /// Return values (Vec2, a, b) have to be interpreted as follows:
    /// - Vec2 is the intersection point
    /// - a scales (self.dst - self.src) to the intersection
    /// - b scales (o.dst - o.src) to the intersection
    /// - Line2D intersections only happen if a and b are within [0.0, 1.0]. Otherwise, either line (or both) get cut
    ///   outside of its segment.
    #[inline(always)]
    pub fn intersection(&self, o: &Self, tolerance: P::S) -> Line2DIntersection<P> {
        let r = self.to_dst();
        let s = o.to_dst();
        let det = r.wedge(s);

        // Lines are parallel or either line segment has len = 0 (but len = 0 is forbidden)
        // Parallel and/or collinear, potential overlaps.

        // Parallel?
        let tolerance_relaxed = tolerance * <P::S>::from(10.0);
        if det.abs() <= tolerance_relaxed {
            let cp = (o.src() - self.src()).wedge(r);

            if cp.abs() > tolerance_relaxed {
                // Parallel and !collinear -> no intersection
                return Line2DIntersection::ParallelNonCollinear;
            }

            // Scalars to compare
            // NOTE: We cannot use the unnormed scalar here as self might be very short, leading to rounding errors.
            let ssn_src = <P::S>::from(0.0);
            let ssn_dst = self.scalar_of_normed(self.dst());
            let mut osn_src = self.scalar_of_normed(self.project(o.src()));
            let mut osn_dst = self.scalar_of_normed(self.project(o.dst()));
            if osn_src > osn_dst {
                std::mem::swap(&mut osn_src, &mut osn_dst);
            }

            if osn_src > ssn_dst + tolerance || osn_dst < ssn_src - tolerance {
                return Line2DIntersection::CollinearDisjoint;
            } else if (osn_src - ssn_dst).abs() <= tolerance {
                return Line2DIntersection::CollinearTouch(self.dst());
            } else if (osn_dst - ssn_src).abs() <= tolerance {
                return Line2DIntersection::CollinearTouch(self.src());
            } else {
                // Use unnormed scalars to calculate the overlapping segment
                let overlap_src = r * <P::S>::from(0.0).max(osn_src / ssn_dst) + self.src();
                let overlap_dst = r * <P::S>::from(1.0).min(osn_dst / ssn_dst) + self.src();

                // We only return a line if a given threshold is met
                if !overlap_src.abs_diff_eq(overlap_dst, tolerance) {
                    return Line2DIntersection::CollinearOverlap(Line([overlap_src, overlap_dst]));
                }
            }
        }

        // Simple intersections
        // t for self, u for other
        let t = (o.src() - self.src()).wedge(s) / det;
        let u = (o.src() - self.src()).wedge(r) / det;
        Line2DIntersection::Simple(
            r * t + self.src(),
            Line2DIntersectionKind::new(t, tolerance),
            Line2DIntersectionKind::new(u, tolerance),
        )
    }
}

/// Kind of a line segment intersection for one Line
/// Resembles the position of a line intersection regarding a line segment.
/// Scalar * (dst - src) = Pos of intersection.
#[derive(Clone, Copy, Debug)]
pub enum Line2DIntersectionKind<P: Point2> {
    Inside(P::S),
    OutsideSrc(P::S),
    OutsideDst(P::S),
}
impl<P: Point2> Line2DIntersectionKind<P> {
    #[inline(always)]
    fn new(scalar: P::S, tolerance: P::S) -> Self {
        if scalar < -tolerance {
            Self::OutsideSrc(scalar)
        } else if scalar > <P::S>::from(1.0) + tolerance {
            Self::OutsideDst(scalar)
        } else {
            Self::Inside(scalar)
        }
    }

    #[inline(always)]
    pub fn is_endpoint(&self, tolerance: P::S) -> bool {
        let scalar = self.scalar();
        scalar.abs() <= tolerance || (scalar - <P::S>::from(1.0)).abs() <= tolerance
    }

    #[inline(always)]
    pub fn scalar(&self) -> P::S {
        match self {
            Self::Inside(s) | Self::OutsideSrc(s) | Self::OutsideDst(s) => *s,
        }
    }

    /// Returns true if the intersection touches the line segment (including endpoints).
    #[inline(always)]
    pub fn touches_linesegment(&self) -> bool {
        matches!(self, Self::Inside(_))
    }

    #[inline(always)]
    pub fn is_true_intersection(&self) -> bool {
        match self {
            &Self::Inside(s) => s > <P::S>::from(0.0) && s < <P::S>::from(1.0),
            _ => false,
        }
    }
}

/// Returns the result of a Line2D intersection check.
#[derive(Clone, Copy, Debug)]
pub enum Line2DIntersection<P: Point2> {
    /// The given Line2D Segments overlap: parallel, collinear and share a given
    Simple(P, Line2DIntersectionKind<P>, Line2DIntersectionKind<P>),
    /// Lines are collinear and share the given Line.
    CollinearOverlap(Line<P>),
    /// Lines are collinear but do not touch.
    CollinearDisjoint,
    /// Collinear Lines that touch at the given Position.
    CollinearTouch(P),
    /// Line2D segments are Parallel && !collinear
    ParallelNonCollinear,
}
impl<P: Point2> Line2DIntersection<P> {
    /// Returns true if both line segments have a true intersection.
    #[inline(always)]
    pub fn intersect(&self) -> bool {
        match self {
            Self::Simple(_, a, b) => a.touches_linesegment() && b.touches_linesegment(),
            Self::CollinearOverlap(_) => true,
            Self::CollinearTouch(_) => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_true_intersection(&self) -> bool {
        match self {
            Self::Simple(_, a, b) => a.is_true_intersection() && b.is_true_intersection(),
            _ => false,
        }
    }

    #[inline(always)]
    pub fn intersect_exclude_endpoints(&self, tolerance: P::S) -> bool {
        match self {
            Self::Simple(
                _,
                Line2DIntersectionKind::Inside(a),
                Line2DIntersectionKind::Inside(b),
            ) => {
                // TODO We cannot use these scalars here as they depend on the length of the linesegments. For very short ones,
                // this leads to unpredictable precision errors. See #[inline(always)] pub fn intersection on how to solve it:
                // Line2DIntersection should contain the normed scalars instead? Requires very careful refactoring. this.
                *a > tolerance
                    && *a < <P::S>::from(1.0) - tolerance
                    && *b > tolerance
                    && *b < <P::S>::from(1.0) - tolerance
            }
            _ => false,
        }
    }

    /// Returns the position of the true line intersection - if any.
    /// If the lines overlap, returns an arbitrary point of the overlap.
    #[inline(always)]
    pub fn pos(&self) -> Option<P> {
        match self {
            Self::Simple(p, a, b) => {
                (a.touches_linesegment() && b.touches_linesegment()).then_some(*p)
            }

            Self::CollinearOverlap(ls) => Some(ls.center()),
            Self::CollinearTouch(p) => Some(*p),
            _ => None,
        }
    }
}
