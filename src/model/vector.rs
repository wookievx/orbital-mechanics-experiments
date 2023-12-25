use std::ops;

#[derive(Debug, Clone)]
pub struct Vector2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> ops::Add<Vector2d<T>> for Vector2d<T>
where
    T: ops::Add<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn add(self, rhs: Vector2d<T>) -> Self::Output {
        let Vector2d { x, y } = self;
        let Vector2d { x: rhs_x, y: rhs_y } = rhs;
        Vector2d {
            x: x + rhs_x,
            y: y + rhs_y,
        }
    }
}

impl<T> ops::Sub<Vector2d<T>> for Vector2d<T>
where
    T: ops::Sub<T, Output = T>,
{
    type Output = Vector2d<T>;

    fn sub(self, rhs: Vector2d<T>) -> Self::Output {
        let Vector2d { x, y } = self;
        let Vector2d { x: rhs_x, y: rhs_y } = rhs;
        Vector2d {
            x: x - rhs_x,
            y: y - rhs_y,
        }
    }
}

impl<T> ops::Mul<Vector2d<T>> for Vector2d<T>
where
    T: ops::Mul<T, Output = T> + ops::Add<T, Output = T>,
{
    type Output = T;

    fn mul(self, rhs: Vector2d<T>) -> Self::Output {
        let Vector2d { x, y } = self;
        let Vector2d { x: rhs_x, y: rhs_y } = rhs;
        x * rhs_x + y * rhs_y
    }
}

impl<T> ops::Div<T> for Vector2d<T>
where
    T: ops::Div<T, Output = T> + Clone,
{
    type Output = Vector2d<T>;

    fn div(self, rhs: T) -> Self::Output {
        let Vector2d { x, y } = self;
        Vector2d {
            x: x / rhs.clone(),
            y: y / rhs,
        }
    }
}
