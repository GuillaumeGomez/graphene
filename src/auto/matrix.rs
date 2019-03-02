// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_2", feature = "dox"))]
use Box;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use Euler;
use Point;
use Point3D;
use Quad;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use Quaternion;
#[cfg(any(feature = "v1_4", feature = "dox"))]
use Ray;
use Rect;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use Sphere;
use Vec3;
use Vec4;
use ffi;
use glib::translate::*;
use gobject_ffi;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Matrix(Boxed<ffi::graphene_matrix_t>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::graphene_matrix_get_type(), ptr as *mut _) as *mut ffi::graphene_matrix_t,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::graphene_matrix_get_type(), ptr as *mut _),
        get_type => || ffi::graphene_matrix_get_type(),
    }
}

impl Matrix {
    pub fn alloc() -> Matrix {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::graphene_matrix_alloc())
        }
    }

    pub fn determinant(&self) -> f32 {
        unsafe {
            ffi::graphene_matrix_determinant(self.to_glib_none().0)
        }
    }

    pub fn get_row(&self, index_: u32) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_matrix_get_row(self.to_glib_none().0, index_, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn get_value(&self, row: u32, col: u32) -> f32 {
        unsafe {
            ffi::graphene_matrix_get_value(self.to_glib_none().0, row, col)
        }
    }

    pub fn get_x_scale(&self) -> f32 {
        unsafe {
            ffi::graphene_matrix_get_x_scale(self.to_glib_none().0)
        }
    }

    pub fn get_y_scale(&self) -> f32 {
        unsafe {
            ffi::graphene_matrix_get_y_scale(self.to_glib_none().0)
        }
    }

    pub fn get_z_scale(&self) -> f32 {
        unsafe {
            ffi::graphene_matrix_get_z_scale(self.to_glib_none().0)
        }
    }

    pub fn init_from_2d(&mut self, xx: f64, yx: f64, xy: f64, yy: f64, x_0: f64, y_0: f64) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_from_2d(self.to_glib_none_mut().0, xx, yx, xy, yy, x_0, y_0))
        }
    }

    //pub fn init_from_float(&mut self, v: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 16) -> Option<Matrix> {
    //    unsafe { TODO: call ffi::graphene_matrix_init_from_float() }
    //}

    pub fn init_from_matrix(&mut self, src: &Matrix) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_from_matrix(self.to_glib_none_mut().0, src.to_glib_none().0))
        }
    }

    pub fn init_from_vec4(&mut self, v0: &Vec4, v1: &Vec4, v2: &Vec4, v3: &Vec4) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_from_vec4(self.to_glib_none_mut().0, v0.to_glib_none().0, v1.to_glib_none().0, v2.to_glib_none().0, v3.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn init_frustum(&mut self, left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_frustum(self.to_glib_none_mut().0, left, right, bottom, top, z_near, z_far))
        }
    }

    pub fn init_identity(&mut self) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_identity(self.to_glib_none_mut().0))
        }
    }

    pub fn init_look_at(&mut self, eye: &Vec3, center: &Vec3, up: &Vec3) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_look_at(self.to_glib_none_mut().0, eye.to_glib_none().0, center.to_glib_none().0, up.to_glib_none().0))
        }
    }

    pub fn init_ortho(&mut self, left: f32, right: f32, top: f32, bottom: f32, z_near: f32, z_far: f32) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_ortho(self.to_glib_none_mut().0, left, right, top, bottom, z_near, z_far))
        }
    }

    pub fn init_perspective(&mut self, fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_perspective(self.to_glib_none_mut().0, fovy, aspect, z_near, z_far))
        }
    }

    pub fn init_rotate(&mut self, angle: f32, axis: &Vec3) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_rotate(self.to_glib_none_mut().0, angle, axis.to_glib_none().0))
        }
    }

    pub fn init_scale(&mut self, x: f32, y: f32, z: f32) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_scale(self.to_glib_none_mut().0, x, y, z))
        }
    }

    pub fn init_skew(&mut self, x_skew: f32, y_skew: f32) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_skew(self.to_glib_none_mut().0, x_skew, y_skew))
        }
    }

    pub fn init_translate(&mut self, p: &Point3D) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::graphene_matrix_init_translate(self.to_glib_none_mut().0, p.to_glib_none().0))
        }
    }

    pub fn interpolate(&self, b: &Matrix, factor: f64) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_interpolate(self.to_glib_none().0, b.to_glib_none().0, factor, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn inverse(&self) -> Option<Matrix> {
        unsafe {
            let mut res = Matrix::uninitialized();
            let ret = from_glib(ffi::graphene_matrix_inverse(self.to_glib_none().0, res.to_glib_none_mut().0));
            if ret { Some(res) } else { None }
        }
    }

    pub fn is_2d(&self) -> bool {
        unsafe {
            from_glib(ffi::graphene_matrix_is_2d(self.to_glib_none().0))
        }
    }

    pub fn is_backface_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::graphene_matrix_is_backface_visible(self.to_glib_none().0))
        }
    }

    pub fn is_identity(&self) -> bool {
        unsafe {
            from_glib(ffi::graphene_matrix_is_identity(self.to_glib_none().0))
        }
    }

    pub fn is_singular(&self) -> bool {
        unsafe {
            from_glib(ffi::graphene_matrix_is_singular(self.to_glib_none().0))
        }
    }

    pub fn multiply(&self, b: &Matrix) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_multiply(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn normalize(&self) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn perspective(&self, depth: f32) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_perspective(self.to_glib_none().0, depth, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn print(&self) {
        unsafe {
            ffi::graphene_matrix_print(self.to_glib_none().0);
        }
    }

    pub fn project_point(&self, p: &Point) -> Point {
        unsafe {
            let mut res = Point::uninitialized();
            ffi::graphene_matrix_project_point(self.to_glib_none().0, p.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn project_rect(&self, r: &Rect) -> Quad {
        unsafe {
            let mut res = Quad::uninitialized();
            ffi::graphene_matrix_project_rect(self.to_glib_none().0, r.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn project_rect_bounds(&self, r: &Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            ffi::graphene_matrix_project_rect_bounds(self.to_glib_none().0, r.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn rotate(&mut self, angle: f32, axis: &Vec3) {
        unsafe {
            ffi::graphene_matrix_rotate(self.to_glib_none_mut().0, angle, axis.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn rotate_euler(&mut self, e: &Euler) {
        unsafe {
            ffi::graphene_matrix_rotate_euler(self.to_glib_none_mut().0, e.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn rotate_quaternion(&mut self, q: &Quaternion) {
        unsafe {
            ffi::graphene_matrix_rotate_quaternion(self.to_glib_none_mut().0, q.to_glib_none().0);
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        unsafe {
            ffi::graphene_matrix_rotate_x(self.to_glib_none_mut().0, angle);
        }
    }

    pub fn rotate_y(&mut self, angle: f32) {
        unsafe {
            ffi::graphene_matrix_rotate_y(self.to_glib_none_mut().0, angle);
        }
    }

    pub fn rotate_z(&mut self, angle: f32) {
        unsafe {
            ffi::graphene_matrix_rotate_z(self.to_glib_none_mut().0, angle);
        }
    }

    pub fn scale(&mut self, factor_x: f32, factor_y: f32, factor_z: f32) {
        unsafe {
            ffi::graphene_matrix_scale(self.to_glib_none_mut().0, factor_x, factor_y, factor_z);
        }
    }

    pub fn skew_xy(&mut self, factor: f32) {
        unsafe {
            ffi::graphene_matrix_skew_xy(self.to_glib_none_mut().0, factor);
        }
    }

    pub fn skew_xz(&mut self, factor: f32) {
        unsafe {
            ffi::graphene_matrix_skew_xz(self.to_glib_none_mut().0, factor);
        }
    }

    pub fn skew_yz(&mut self, factor: f32) {
        unsafe {
            ffi::graphene_matrix_skew_yz(self.to_glib_none_mut().0, factor);
        }
    }

    pub fn to_2d(&self) -> Option<(f64, f64, f64, f64, f64, f64)> {
        unsafe {
            let mut xx = mem::uninitialized();
            let mut yx = mem::uninitialized();
            let mut xy = mem::uninitialized();
            let mut yy = mem::uninitialized();
            let mut x_0 = mem::uninitialized();
            let mut y_0 = mem::uninitialized();
            let ret = from_glib(ffi::graphene_matrix_to_2d(self.to_glib_none().0, &mut xx, &mut yx, &mut xy, &mut yy, &mut x_0, &mut y_0));
            if ret { Some((xx, yx, xy, yy, x_0, y_0)) } else { None }
        }
    }

    //pub fn to_float(&self, v: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 16) {
    //    unsafe { TODO: call ffi::graphene_matrix_to_float() }
    //}

    pub fn transform_bounds(&self, r: &Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            ffi::graphene_matrix_transform_bounds(self.to_glib_none().0, r.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn transform_box(&self, b: &Box) -> Box {
        unsafe {
            let mut res = Box::uninitialized();
            ffi::graphene_matrix_transform_box(self.to_glib_none().0, b.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn transform_point(&self, p: &Point) -> Point {
        unsafe {
            let mut res = Point::uninitialized();
            ffi::graphene_matrix_transform_point(self.to_glib_none().0, p.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn transform_point3d(&self, p: &Point3D) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_matrix_transform_point3d(self.to_glib_none().0, p.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn transform_ray(&self, r: &Ray) -> Ray {
        unsafe {
            let mut res = Ray::uninitialized();
            ffi::graphene_matrix_transform_ray(self.to_glib_none().0, r.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn transform_rect(&self, r: &Rect) -> Quad {
        unsafe {
            let mut res = Quad::uninitialized();
            ffi::graphene_matrix_transform_rect(self.to_glib_none().0, r.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn transform_sphere(&self, s: &Sphere) -> Sphere {
        unsafe {
            let mut res = Sphere::uninitialized();
            ffi::graphene_matrix_transform_sphere(self.to_glib_none().0, s.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn transform_vec3(&self, v: &Vec3) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            ffi::graphene_matrix_transform_vec3(self.to_glib_none().0, v.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn transform_vec4(&self, v: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_matrix_transform_vec4(self.to_glib_none().0, v.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn translate(&mut self, pos: &Point3D) {
        unsafe {
            ffi::graphene_matrix_translate(self.to_glib_none_mut().0, pos.to_glib_none().0);
        }
    }

    pub fn transpose(&self) -> Matrix {
        unsafe {
            let mut res = Matrix::uninitialized();
            ffi::graphene_matrix_transpose(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn unproject_point3d(&self, modelview: &Matrix, point: &Point3D) -> Point3D {
        unsafe {
            let mut res = Point3D::uninitialized();
            ffi::graphene_matrix_unproject_point3d(self.to_glib_none().0, modelview.to_glib_none().0, point.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn untransform_bounds(&self, r: &Rect, bounds: &Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            ffi::graphene_matrix_untransform_bounds(self.to_glib_none().0, r.to_glib_none().0, bounds.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    pub fn untransform_point(&self, p: &Point, bounds: &Rect) -> Option<Point> {
        unsafe {
            let mut res = Point::uninitialized();
            let ret = from_glib(ffi::graphene_matrix_untransform_point(self.to_glib_none().0, p.to_glib_none().0, bounds.to_glib_none().0, res.to_glib_none_mut().0));
            if ret { Some(res) } else { None }
        }
    }
}
