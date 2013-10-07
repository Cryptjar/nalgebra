//! **nalgebra** prelude.

pub use std::num::{Zero, One};
pub use traits::{
    Absolute,
    AbsoluteRotate,
    AlgebraicVec,
    AlgebraicVecExt,
    Basis,
    Col,
    Cov,
    Cross,
    CrossMatrix,
    Dim,
    Dot,
    FromHomogeneous,
    Indexable,
    Inv,
    Iterable,
    IterableMut,
    LMul,
    Mat,
    MatCast,
    Mean,
    Norm,
    Outer,
    RMul,
    Rotate, Rotation, RotationMatrix, RotationWithTranslation,
    Row,
    ScalarAdd, ScalarSub,
    ToHomogeneous,
    Transform, Transformation,
    Translate, Translation,
    Transpose,
    UniformSphereSample,
    Vec,
    VecCast,
    VecExt
};

pub use structs::{
    DMat, DVec,
    Iso2, Iso3, Iso4,
    Mat1, Mat2, Mat3, Mat4,
    Mat5, Mat6,
    Rot2, Rot3, Rot4,
    Vec0, Vec1, Vec2, Vec3, Vec4, Vec5, Vec6
};

//
//
// Constructors
//
//
/// Create a zero-valued value.
///
/// This is the same as `std::num::Zero::zero()`.
#[inline(always)]
pub fn zero<T: Zero>() -> T {
    Zero::zero()
}

/// Create a one-valued value.
///
/// This is the same as `std::num::One::one()`.
#[inline(always)]
pub fn one<T: One>() -> T {
    One::one()
}

/// Creates a new 1d vector.
///
/// This is the same as `Vec1::new(x)`.
#[inline(always)]
pub fn vec1<N>(x: N) -> Vec1<N> {
    Vec1::new(x)
}

/// Creates a new 2d vector.
///
/// This is the same as `Vec2::new(x, y)`.
#[inline(always)]
pub fn vec2<N>(x: N, y: N) -> Vec2<N> {
    Vec2::new(x, y)
}

/// Creates a new 3d vector.
///
/// This is the same as `Vec3::new(x, y, z)`.
#[inline(always)]
pub fn vec3<N>(x: N, y: N, z: N) -> Vec3<N> {
    Vec3::new(x, y, z)
}

/// Creates a new 4d vector.
///
/// This is the same as `Vec4::new(x, y, z, w)`.
#[inline(always)]
pub fn vec4<N>(x: N, y: N, z: N, w: N) -> Vec4<N> {
    Vec4::new(x, y, z, w)
}

/// Creates a new 1d matrix.
///
/// This is the same as `Mat1::new(...)`.
#[inline(always)]
pub fn mat1<N>(m11: N) -> Mat1<N> {
    Mat1::new(m11)
}

/// Creates a new 2d matrix.
///
/// This is the same as `Mat2::new(...)`.
#[inline(always)]
pub fn mat2<N>(m11: N, m12: N,
               m21: N, m22: N) -> Mat2<N> {
    Mat2::new(
        m11, m12,
        m21, m22)
}

/// Creates a new 3d matrix.
///
/// This is the same as `Mat3::new(...)`.
#[inline(always)]
pub fn mat3<N>(m11: N, m12: N, m13: N,
               m21: N, m22: N, m23: N,
               m31: N, m32: N, m33: N) -> Mat3<N> {
    Mat3::new(
        m11, m12, m13,
        m21, m22, m23,
        m31, m32, m33)
}

/// Creates a new 4d matrix.
///
/// This is the same as `Mat4::new(...)`.
#[inline(always)]
pub fn mat4<N>(m11: N, m12: N, m13: N, m14: N,
               m21: N, m22: N, m23: N, m24: N,
               m31: N, m32: N, m33: N, m34: N,
               m41: N, m42: N, m43: N, m44: N) -> Mat4<N> {
    Mat4::new(
        m11, m12, m13, m14,
        m21, m22, m23, m24,
        m31, m32, m33, m34,
        m41, m42, m43, m44)
}

//
//
// Geometry
//
//

/*
 * Translation<V>
 */

/// Gets the translation applicable by the given object.
///
/// ```rust
/// extern mod nalgebra;
/// use nalgebra::types::{Vec3, Affmat};
/// use nalgebra::na;
///
/// pub main() {
///     let t     = Affmat::new_translation3d(1.0, 1.0, 1.0);
///     let trans = na::translation(t);
///
///     assert!(trans == Vec3::new(1.0, 1.0, 1.0));
/// }
/// ```
#[inline(always)]
pub fn translation<V, M: Translation<V>>(m: &M) -> V {
    m.translation()
}

/// Gets the inverse translation applicable by the given object.
///
/// ```rust
/// extern mod nalgebra;
/// use nalgebra::types::{Vec3, Affmat};
/// use nalgebra::na;
///
/// pub main() {
///     let t      = Affmat::new_translation3d(1.0, 1.0, 1.0);
///     let itrans = na::inv_translation(t);
///
///     assert!(itrans == Vec3::new(-1.0, -1.0, -1.0));
/// }
/// ```
#[inline(always)]
pub fn inv_translation<V, M: Translation<V>>(m: &M) -> V {
    m.inv_translation()
}

/// In-place version of `translated`.
#[inline(always)]
pub fn translate_by<V, M: Translation<V>>(m: &mut M, v: &V) {
    m.translate_by(v)
}

/// Gets a translated copy of the given object.
#[inline(always)]
pub fn translated<V, M: Translation<V>>(m: &M, v: &V) -> M {
    m.translated(v)
}

/// Sets the translation of the given object.
#[inline(always)]
pub fn set_translation<V, M: Translation<V>>(m: &mut M, v: V) {
    m.set_translation(v)
}

/*
 * Translate<V>
 */

/// FIXME
#[inline(always)]
pub fn translate<V, M: Translate<V>>(m: &M, v: &V) -> V {
    m.translate(v)
}

/// FIXME
#[inline(always)]
pub fn inv_translate<V, M: Translate<V>>(m: &M, v: &V) -> V {
    m.inv_translate(v)
}

/*
 * Rotation<V>
 */

/// Gets the rotation applicable by the given object.
///
/// ```rust
/// extern mod nalgebra;
/// use nalgebra::na;
///
/// pub main() {
///     let t = na::rotation3d(1.0, 1.0, 1.0);
///
///     assert!(na::rotation(t) == na::vec3(1.0, 1.0, 1.0));
/// }
/// ```
#[inline(always)]
pub fn rotation<V, M: Rotation<V>>(m: &M) -> V {
    m.rotation()
}


/// Gets the rotation applicable by the given object.
///
/// ```rust
/// extern mod nalgebra;
/// use nalgebra::na;
///
/// pub main() {
///     let t = na::rotation3d(1.0, 1.0, 1.0);
///
///     assert!(na::inv_rotation(t) == na::vec3(-1.0, -1.0, -1.0));
/// }
/// ```
#[inline(always)]
pub fn inv_rotation<V, M: Rotation<V>>(m: &M) -> V {
    m.inv_rotation()
}

/// FIXME
#[inline(always)]
pub fn rotate_by<V, M: Rotation<V>>(m: &mut M, v: &V) {
    m.rotate_by(v)
}

/// FIXME
#[inline(always)]
pub fn rotated<V, M: Rotation<V>>(m: &M, v: &V) -> M {
    m.rotated(v)
}

/// FIXME
#[inline(always)]
pub fn set_rotation<V, M: Rotation<V>>(m: &mut M, v: V) {
    m.set_rotation(v)
}

/*
 * Rotate<V>
 */

/// FIXME
#[inline(always)]
pub fn rotate<V, M: Rotate<V>>(m: &M, v: &V) -> V {
    m.rotate(v)
}

/// FIXME
#[inline(always)]
pub fn inv_rotate<V, M: Rotate<V>>(m: &M, v: &V) -> V {
    m.inv_rotate(v)
}

/*
 * RotationWithTranslation<LV, AV>
 */

/// FIXME
#[inline(always)]
pub fn rotated_wrt_point<LV: Neg<LV>,
                         AV,
                         M: RotationWithTranslation<LV, AV>>(
                         m:      &M,
                         amount: &AV,
                         center: &LV) -> M {
    m.rotated_wrt_point(amount, center)
}

/// FIXME
#[inline(always)]
pub fn rotate_wrt_point<LV: Neg<LV>,
                        AV,
                        M: RotationWithTranslation<LV, AV>>(
                        m:      &mut M,
                        amount: &AV,
                        center: &LV) {
    m.rotate_wrt_point(amount, center)
}

/// FIXME
#[inline(always)]
pub fn rotated_wrt_center<LV: Neg<LV>,
                          AV,
                          M: RotationWithTranslation<LV, AV>>(
                          m:      &M,
                          amount: &AV) -> M {
    m.rotated_wrt_center(amount)
}

/// FIXME
#[inline(always)]
pub fn rotate_wrt_center<LV: Neg<LV>,
                         AV,
                         M: RotationWithTranslation<LV, AV>>(
                         m:      &mut M,
                         amount: &AV) {
    m.rotate_wrt_center(amount)
}

/*
 * RotationMatrix<LV, AV, R>
 */

/// FIXME
#[inline(always)]
pub fn to_rot_mat<LV, AV, M: Mat<LV, LV> + Rotation<AV>, R: RotationMatrix<LV, AV, M>>(r: &R) -> M {
    r.to_rot_mat()
}

/*
 * AbsoluteRotate<V>
 */

/// FIXME
#[inline(always)]
pub fn absolute_rotate<V, M: AbsoluteRotate<V>>(m: &M, v: &V) -> V {
    m.absolute_rotate(v)
}

/*
 * Transformation<V>
 */

/// FIXME
#[inline(always)]
pub fn transformation<V, M: Transformation<V>>(m: &M) -> V {
    m.transformation()
}

/// FIXME
#[inline(always)]
pub fn inv_transformation<V, M: Transformation<V>>(m: &M) -> V {
    m.inv_transformation()
}

/// FIXME
#[inline(always)]
pub fn transform_by<V, M: Transformation<V>>(m: &mut M, v: &V) {
    m.transform_by(v)
}

/// FIXME
#[inline(always)]
pub fn transformed<V, M: Transformation<V>>(m: &M, v: &V) -> M {
    m.transformed(v)
}

/// FIXME
#[inline(always)]
pub fn set_transformation<V, M: Transformation<V>>(m: &mut M, v: V) {
    m.set_transformation(v)
}

/*
 * Transform<V>
 */

/// FIXME
#[inline(always)]
pub fn transform<V, M: Transform<V>>(m: &M, v: &V) -> V {
    m.transform(v)
}

/// FIXME
#[inline(always)]
pub fn inv_transform<V, M: Transform<V>>(m: &M, v: &V) -> V {
    m.inv_transform(v)
}

/*
 * Dot<N>
 */

/// FIXME
#[inline(always)]
pub fn dot<V: Dot<N>, N>(a: &V, b: &V) -> N {
    a.dot(b)
}

/// FIXME
#[inline(always)]
pub fn sub_dot<V: Dot<N>, N>(a: &V, b: &V, c: &V) -> N {
    a.sub_dot(b, c)
}

/*
 * Norm<N>
 */

/// FIXME
#[inline(always)]
pub fn norm<V: Norm<N>, N: Algebraic>(v: &V) -> N {
    v.norm()
}

/// FIXME
#[inline(always)]
pub fn sqnorm<V: Norm<N>, N: Algebraic>(v: &V) -> N {
    v.sqnorm()
}

/// FIXME
#[inline(always)]
pub fn normalized<V: Norm<N>, N: Algebraic>(v: &V) -> V {
    v.normalized()
}

/// FIXME
#[inline(always)]
pub fn normalize<V: Norm<N>, N: Algebraic>(v: &mut V) -> N {
    v.normalize()
}

/*
 * Cross<V>
 */

/// FIXME
#[inline(always)]
pub fn cross<LV: Cross<AV>, AV>(a: &LV, b: &LV) -> AV {
    a.cross(b)
}

/*
 * CrossMatrix<M>
 */

/// FIXME
#[inline(always)]
pub fn cross_matrix<V: CrossMatrix<M>, M>(v: &V) -> M {
    v.cross_matrix()
}

/*
 * ToHomogeneous<U>
 */

/// FIXME
#[inline(always)]
pub fn to_homogeneous<M: ToHomogeneous<Res>, Res>(m: &M) -> Res {
    m.to_homogeneous()
}

/*
 * FromHomogeneous<U>
 */

/// FIXME
#[inline(always)]
pub fn from_homogeneous<M, Res: FromHomogeneous<M>>(m: &M) -> Res {
    FromHomogeneous::from(m)
}

/*
 * UniformSphereSample
 */

/// FIXME
#[inline(always)]
pub fn sample_sphere<V: UniformSphereSample>(f: &fn(V)) {
    UniformSphereSample::sample(f)
}

//
//
// Operations
//
//


/*
 * Absolute<A>
 */

/// FIXME
#[inline(always)]
pub fn absolute<M: Absolute<Res>, Res>(m: &M) -> Res {
    m.absolute()
}

/*
 * Inv
 */

/// FIXME
#[inline(always)]
pub fn inverted<M: Inv>(m: &M) -> Option<M> {
    m.inverted()
}

/// FIXME
#[inline(always)]
pub fn invert<M: Inv>(m: &mut M) -> bool {
    m.invert()
}

/*
 * Transpose
 */

/// FIXME
#[inline(always)]
pub fn transposed<M: Transpose>(m: &M) -> M {
    m.transposed()
}

/// FIXME
#[inline(always)]
pub fn transpose<M: Transpose>(m: &mut M) {
    m.transpose()
}

/*
 * Outer<M>
 */

/// FIXME
#[inline(always)]
pub fn outer<V: Outer<M>, M>(a: &V, b: &V) -> M {
    a.outer(b)
}

/*
 * Cov<M>
 */

/// FIXME
#[inline(always)]
pub fn cov<M: Cov<Res>, Res>(observations: &M) -> Res {
    observations.cov()
}

/*
 * Mean<N>
 */

/// FIXME
#[inline(always)]
pub fn mean<N, M: Mean<N>>(observations: &M) -> N {
    observations.mean()
}

//
//
// Structure
//
//

/*
 * MatCast<M>
 */

/// FIXME
#[inline(always)]
pub fn cast_mat<M: MatCast<Res>, Res>(m: M) -> Res {
    MatCast::from(m)
}

/*
 * VecCast<M>
 */

/// FIXME
#[inline(always)]
pub fn cast_vec<V: VecCast<Res>, Res>(v: V) -> Res {
    VecCast::from(v)
}

/*
 * Basis
 */

/// FIXME
#[inline(always)]
pub fn canonical_basis<V: Basis>(f: &fn(V) -> bool) {
    Basis::canonical_basis(f)
}

/// FIXME
#[inline(always)]
pub fn orthonormal_subspace_basis<V: Basis>(v: &V, f: &fn(V) -> bool) {
    v.orthonormal_subspace_basis(f)
}

/*
 * Row<R>
 */

/*
 * Col<C>
 */

/*
 * Dim
 */

/*
 * Indexable
 */