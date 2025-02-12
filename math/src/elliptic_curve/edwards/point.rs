use crate::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        point::ProjectivePoint,
        traits::{EllipticCurveError, FromAffine, IsEllipticCurve},
    },
    field::element::FieldElement,
};

use super::traits::IsEdwards;

#[derive(Clone, Debug)]
pub struct EdwardsProjectivePoint<E: IsEllipticCurve>(ProjectivePoint<E>);

impl<E: IsEllipticCurve> EdwardsProjectivePoint<E> {
    /// Creates an elliptic curve point giving the projective [x: y: z] coordinates.
    pub fn new(value: [FieldElement<E::BaseField>; 3]) -> Self {
        Self(ProjectivePoint::new(value))
    }

    /// Returns the `x` coordinate of the point.
    pub fn x(&self) -> &FieldElement<E::BaseField> {
        self.0.x()
    }

    /// Returns the `y` coordinate of the point.
    pub fn y(&self) -> &FieldElement<E::BaseField> {
        self.0.y()
    }

    /// Returns the `z` coordinate of the point.
    pub fn z(&self) -> &FieldElement<E::BaseField> {
        self.0.z()
    }

    /// Returns a tuple [x, y, z] with the coordinates of the point.
    pub fn coordinates(&self) -> &[FieldElement<E::BaseField>; 3] {
        self.0.coordinates()
    }

    /// Creates the same point in affine coordinates. That is,
    /// returns [x / z: y / z: 1] where `self` is [x: y: z].
    /// Panics if `self` is the point at infinity.
    pub fn to_affine(&self) -> Self {
        Self(self.0.to_affine())
    }

    /// Returns the additive inverse of the projective point `p`
    pub fn neg(&self) -> Self {
        let [px, py, pz] = self.coordinates();
        Self::new([-px, py.clone(), pz.clone()])
    }
}

impl<E: IsEllipticCurve> PartialEq for EdwardsProjectivePoint<E> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E: IsEdwards> FromAffine<E::BaseField> for EdwardsProjectivePoint<E> {
    fn from_affine(
        x: FieldElement<E::BaseField>,
        y: FieldElement<E::BaseField>,
    ) -> Result<Self, crate::elliptic_curve::traits::EllipticCurveError> {
        let coordinates = [x, y, FieldElement::one()];
        if E::defining_equation(&coordinates) != FieldElement::zero() {
            Err(EllipticCurveError::InvalidPoint)
        } else {
            Ok(EdwardsProjectivePoint::new(coordinates))
        }
    }
}

impl<E: IsEllipticCurve> Eq for EdwardsProjectivePoint<E> {}

impl<E: IsEdwards> IsGroup for EdwardsProjectivePoint<E> {
    /// The point at infinity.
    fn neutral_element() -> Self {
        Self::new([
            FieldElement::zero(),
            FieldElement::one(),
            FieldElement::one(),
        ])
    }

    /// Computes the addition of `self` and `other`.
    /// Taken from "Moonmath" (Eq 5.38, page 97)
    fn operate_with(&self, other: &Self) -> Self {
        // TODO: Remove repeated operations
        let [x1, y1, _] = self.to_affine().coordinates().clone();
        let [x2, y2, _] = other.to_affine().coordinates().clone();

        let num_s1 = &x1 * &y2 + &y1 * &x2;
        let den_s1 = FieldElement::one() + E::d() * &x1 * &x2 * &y1 * &y2;

        let num_s2 = &y1 * &y2 - E::a() * &x1 * &x2;
        let den_s2 = FieldElement::one() - E::d() * x1 * x2 * y1 * y2;

        Self::new([num_s1 / den_s1, num_s2 / den_s2, FieldElement::one()])
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cyclic_group::IsGroup,
        elliptic_curve::{
            edwards::{curves::tiny_jub_jub::TinyJubJubEdwards, point::EdwardsProjectivePoint},
            traits::{EllipticCurveError, IsEllipticCurve},
        },
        field::element::FieldElement,
    };

    fn create_point(x: u64, y: u64) -> EdwardsProjectivePoint<TinyJubJubEdwards> {
        TinyJubJubEdwards::create_point_from_affine(FieldElement::from(x), FieldElement::from(y))
            .unwrap()
    }

    #[test]
    fn create_valid_point_works() {
        let p = TinyJubJubEdwards::create_point_from_affine(
            FieldElement::from(5),
            FieldElement::from(5),
        )
        .unwrap();
        assert_eq!(p.x(), &FieldElement::from(5));
        assert_eq!(p.y(), &FieldElement::from(5));
        assert_eq!(p.z(), &FieldElement::from(1));
    }

    #[test]
    fn create_invalid_point_returns_invalid_point_error() {
        let result = TinyJubJubEdwards::create_point_from_affine(
            FieldElement::from(5),
            FieldElement::from(4),
        );
        assert_eq!(result.unwrap_err(), EllipticCurveError::InvalidPoint);
    }

    #[test]
    fn operate_with_works_for_points_in_tiny_jub_jub() {
        let p = EdwardsProjectivePoint::<TinyJubJubEdwards>::new([
            FieldElement::from(5),
            FieldElement::from(5),
            FieldElement::from(1),
        ]);
        let q = EdwardsProjectivePoint::<TinyJubJubEdwards>::new([
            FieldElement::from(8),
            FieldElement::from(5),
            FieldElement::from(1),
        ]);
        let expected = EdwardsProjectivePoint::<TinyJubJubEdwards>::new([
            FieldElement::from(0),
            FieldElement::from(1),
            FieldElement::from(1),
        ]);
        assert_eq!(p.operate_with(&q), expected);
    }

    #[test]
    fn test_negation_in_edwards() {
        let a = create_point(5, 5);
        let b = create_point(13 - 5, 5);

        assert_eq!(a.neg(), b);
        assert!(a.operate_with(&b).is_neutral_element());
    }

    #[test]
    fn operate_with_works_and_cycles_in_tiny_jub_jub() {
        let g = create_point(12, 11);
        assert_eq!(g.operate_with_self(0), create_point(0, 1));
        assert_eq!(g.operate_with_self(1), create_point(12, 11));
        assert_eq!(g.operate_with_self(2), create_point(8, 5));
        assert_eq!(g.operate_with_self(3), create_point(11, 6));
        assert_eq!(g.operate_with_self(4), create_point(6, 9));
        assert_eq!(g.operate_with_self(5), create_point(10, 0));
        assert_eq!(g.operate_with_self(6), create_point(6, 4));
        assert_eq!(g.operate_with_self(7), create_point(11, 7));
        assert_eq!(g.operate_with_self(8), create_point(8, 8));
        assert_eq!(g.operate_with_self(9), create_point(12, 2));
        assert_eq!(g.operate_with_self(10), create_point(0, 12));
        assert_eq!(g.operate_with_self(11), create_point(1, 2));
        assert_eq!(g.operate_with_self(12), create_point(5, 8));
        assert_eq!(g.operate_with_self(13), create_point(2, 7));
        assert_eq!(g.operate_with_self(14), create_point(7, 4));
        assert_eq!(g.operate_with_self(15), create_point(3, 0));
        assert_eq!(g.operate_with_self(16), create_point(7, 9));
        assert_eq!(g.operate_with_self(17), create_point(2, 6));
        assert_eq!(g.operate_with_self(18), create_point(5, 5));
        assert_eq!(g.operate_with_self(19), create_point(1, 11));
        assert_eq!(g.operate_with_self(20), create_point(0, 1));
    }
}
