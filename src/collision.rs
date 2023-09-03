use crate::{ffi, math::{Rectangle, Vector2}};

/// Check collision between two rectangles
pub fn check_collision_rects(rec1: Rectangle, rec2: Rectangle) -> bool {
    unsafe { ffi::CheckCollisionRecs(rec1.into(), rec2.into()) }
}

/// Check collision between two circles
pub fn check_collision_circles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32) -> bool {
    unsafe { ffi::CheckCollisionCircles(center1.into(), radius1, center2.into(), radius2) }
}

/// Check collision between circle and rectangle
pub fn check_collision_circle_rect(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    unsafe { ffi::CheckCollisionCircleRec(center.into(), radius, rec.into()) }
}

/// Check if point is inside rectangle
pub fn check_point_inside_rect(point: Vector2, rec: Rectangle) -> bool {
    unsafe { ffi::CheckCollisionPointRec(point.into(), rec.into()) }
}

/// Check if point is inside circle
pub fn check_point_inside_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point.into(), center.into(), radius) }
}

/// Check if point is inside a triangle
pub fn check_point_inside_triangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2) -> bool {
    unsafe { ffi::CheckCollisionPointTriangle(point.into(), p1.into(), p2.into(), p3.into()) }
}

/// Check if point is within a polygon described by array of vertices
pub fn check_point_inside_polygon(point: Vector2, points: &[Vector2]) -> bool {
    unsafe { ffi::CheckCollisionPointPoly(point.into(), points.as_ptr() as *mut _, points.len() as _) }
}

/// Check the collision between two lines defined by two points each, returns collision point
pub fn check_collision_lines(start_pos1: Vector2, end_pos1: Vector2, start_pos2: Vector2, end_pos2: Vector2) -> Option<Vector2> {
    let mut coll_pt: ffi::Vector2 = ffi::Vector2 { x: 0., y: 0. };

    if unsafe { ffi::CheckCollisionLines(start_pos1.into(), end_pos1.into(), start_pos2.into(), end_pos2.into(), &mut coll_pt as *mut _) } {
        Some(coll_pt.into())
    } else {
        None
    }
}

/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
pub fn check_collision_point_line(point: Vector2, p1: Vector2, p2: Vector2, threshold: u32) -> bool {
    unsafe { ffi::CheckCollisionPointLine(point.into(), p1.into(), p2.into(), threshold as _) }
}

/// Get collision rectangle for two rectangles collision
pub fn get_collision_rect(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
    unsafe { ffi::GetCollisionRec(rec1.into(), rec2.into()).into() }
}
