use macroquad::prelude::Vec2;

const EPS: f32 = 1e-14;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl Line {
    pub fn circle_intersections(&self, mx: f32, my: f32, r: f32, segment: bool) -> Vec<Vec2> {
        let mut intersections: Vec<Vec2> = Vec::new();

        let x0 = mx;
        let y0 = my;
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let x2 = self.p2.x;
        let y2 = self.p2.y;

        let ca = y2 - y1;
        let cb = x1 - x2;
        let cc = x2 * y1 - x1 * y2;

        let a = ca.powi(2) + cb.powi(2);
        let mut b = 0.0;
        let mut c = 0.0;
        let mut bnz = true;

        if cb.abs() >= EPS {
            b = 2.0 * (ca * cc + ca * cb * y0 - cb.powi(2) * x0);
            c = cc.powi(2) + 2.0 * cb * cc * y0
                - cb.powi(2) * (r.powi(2) - x0.powi(2) - y0.powi(2));
        } else {
            b = 2.0 * (cb * cc + ca * cb * x0 - ca.powi(2) * y0);
            c = cc.powi(2) + 2.0 * ca * cc * x0
                - ca.powi(2) * (r.powi(2) - x0.powi(2) - y0.powi(2));
            bnz = false;
        }
        let mut d = b.powi(2) - 4.0 * a * c;
        if d < 0.0 {
            return intersections;
        }

        fn within(x: f32, y: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
            let d1 = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt(); // distance between end-Vec2s
            let d2 = ((x - x1).powi(2) + (y - y1).powi(2)).sqrt(); // distance from Vec2 to one end
            let d3 = ((x2 - x).powi(2) + (y2 - y).powi(2)).sqrt(); // distance from Vec2 to other end
            let delta = d1 - d2 - d3;
            return delta.abs() < EPS;
        }

        fn fx(x: f32, ca: f32, cb: f32, cc: f32) -> f32 {
            -(ca * x + cc) / cb
        }

        fn fy(y: f32, ca: f32, cb: f32, cc: f32) -> f32 {
            -(cb * y + cc) / ca
        }

        fn rxy(
            x: f32,
            y: f32,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            segment: bool,
            intersections: &mut Vec<Vec2>,
        ) {
            if !segment || within(x, y, x1, y1, x2, y2) {
                let vec_2 = Vec2::new(x, y);
                intersections.push(vec_2);
            }
        }

        if d == 0.0 {
            if bnz {
                let x = -b / (2.0 * a);
                let y = fx(x, ca, cb, cc);
                rxy(x, y, x1, y1, x2, y2, segment, &mut intersections);
            } else {
                let y = -b / (2.0 * a);
                let x = fy(y, ca, cb, cc);
                rxy(x, y, x1, y1, x2, y2, segment, &mut intersections);
            }
        } else {
            d = d.sqrt();
            if bnz {
                let x = (-b + d) / (2.0 * a);
                let y = fx(x, ca, cb, cc);
                rxy(x, y, x1, y1, x2, y2, segment, &mut intersections);
                let x = (-b - d) / (2.0 * a);
                let y = fx(x, ca, cb, cc);
                rxy(x, y, x1, y1, x2, y2, segment, &mut intersections);
            } else {
                let y = (-b + d) / (2.0 * a);
                let x = fy(y, ca, cb, cc);
                rxy(x, y, x1, y1, x2, y2, segment, &mut intersections);
                let y = (-b - d) / (2.0 * a);
                let x = fy(y, ca, cb, cc);
                rxy(x, y, x1, y1, x2, y2, segment, &mut intersections);
            }
        }

        intersections.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        intersections
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_circle_line_intersections() {
        let mut p1 = Vec2::new(-10.0, 11.0);
        let mut p2 = Vec2::new(10.0, -9.0);
        let mut line = Line { p1, p2 };

        let result1 = line.circle_intersections(3.0, -5.0, 3.0, false);
        assert_eq!(result1.len(), 2);
        assert_approx_eq!(result1[0].x, 3.0);
        assert_approx_eq!(result1[0].y, -2.0);
        assert_approx_eq!(result1[1].x, 6.0);
        assert_approx_eq!(result1[1].y, -5.0);

        p1 = Vec2::new(-10.0, 11.0);
        p2 = Vec2::new(-11.0, -12.0);
        line = Line { p1, p2 };
        let result2 = line.circle_intersections(3.0, -5.0, 3.0, true);
        assert_eq!(result2.len(), 0);

        p1 = Vec2::new(3.0, -2.0);
        p2 = Vec2::new(7.0, -2.0);
        line = Line { p1, p2 };
        let result3 = line.circle_intersections(3.0, -5.0, 3.0, true);
        assert_eq!(result3.len(), 1);
        assert_approx_eq!(result3[0].x, 3.0);
        assert_approx_eq!(result3[0].y, -2.0);

        p1 = Vec2::new(0.0, -3.0);
        p2 = Vec2::new(0.0, 6.0);
        line = Line { p1, p2 };
        let result4 = line.circle_intersections(0.0, 0.0, 4.0, false);
        assert_eq!(result4.len(), 2);
        assert_approx_eq!(result4[0].x, 0.0);
        assert_approx_eq!(result4[1].x, 0.0);

        let result5 = line.circle_intersections(0.0, 0.0, 4.0, true);
        assert_eq!(result5.len(), 1);

        p1 = Vec2::new(6.0, 3.0);
        p2 = Vec2::new(10.0, 7.0);
        line = Line { p1, p2 };
        let result6 = line.circle_intersections(4.0, 2.0, 5.0, false);
        assert_eq!(result6.len(), 2);
        assert_approx_eq!(result6[0].x, 1.0);
        assert_approx_eq!(result6[0].y, -2.0);
        assert_approx_eq!(result6[1].x, 8.0);
        assert_approx_eq!(result6[1].y, 5.0);

        p1 = Vec2::new(7.0, 4.0);
        p2 = Vec2::new(11.0, 8.0);
        line = Line { p1, p2 };
        let result7 = line.circle_intersections(4.0, 2.0, 5.0, true);
        assert_eq!(result7.len(), 1);
        assert_approx_eq!(result7[0].x, 8.0);
        assert_approx_eq!(result7[0].y, 5.0);
    }
}