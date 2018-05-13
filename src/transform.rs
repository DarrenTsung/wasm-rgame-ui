use cgmath::Vector2;

pub struct Transform {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
    pub pivot: Vector2<f32>,
}

impl Transform {
    pub fn contains(&self, point: Vector2<f32>) -> bool {
        let bottom_left = self.bottom_left();
        bottom_left.x <= point.x && point.x <= bottom_left.x + self.size.x &&
            bottom_left.y <= point.y && point.y <= bottom_left.y + self.size.y
    }

    pub fn bottom_left(&self) -> Vector2<f32> {
        Vector2 {
            x: self.pos.x - (self.pivot.x * self.size.x),
            y: self.pos.y - (self.pivot.y * self.size.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_left_works() {
        let transform = Transform {
            pos: Vector2 { x: 2.0, y: 1.0, },
            size: Vector2 { x: 5.5, y: 6.5, },
            pivot: Vector2 { x: 0.5, y: 0.5, },
        };

        assert_eq!(transform.bottom_left(), Vector2 { x: -0.75, y: -2.25, });

        let transform2 = Transform {
            pos: Vector2 { x: 3.0, y: 5.0, },
            size: Vector2 { x: 5.5, y: 6.5, },
            pivot: Vector2 { x: 0.0, y: 0.0, },
        };

        assert_eq!(transform2.bottom_left(), Vector2 { x: 3.0, y: 5.0, });
    }

    #[test]
    fn contains_works() {
        let transform = Transform {
            pos: Vector2 { x: 2.0, y: 1.0, },
            size: Vector2 { x: 5.5, y: 6.5, },
            pivot: Vector2 { x: 0.5, y: 0.5, },
        };

        assert_eq!(transform.bottom_left(), Vector2 { x: -0.75, y: -2.25, });
        // top_right -> x: 4.75, y: 4.25

        assert_eq!(transform.contains(Vector2 { x: -0.76, y: -2.25, }), false);
        assert_eq!(transform.contains(Vector2 { x: -0.75, y: -2.25, }), true);
        assert_eq!(transform.contains(Vector2 { x: 0.0, y: 0.0, }), true);
        assert_eq!(transform.contains(Vector2 { x: 2.0, y: 1.0, }), true);
        assert_eq!(transform.contains(Vector2 { x: 4.75, y: 0.0, }), true);
        assert_eq!(transform.contains(Vector2 { x: 5.0, y: 0.0, }), false);
    }
}
