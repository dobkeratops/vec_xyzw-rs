#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug,Clone)]
pub struct Vec1<X>{pub x:X}

#[derive(Debug,Clone)]
pub struct Vec2<X,Y=X>{pub x:X,pub y:Y}

#[derive(Debug,Clone)]
pub struct Vec3<X,Y=X,Z=Y>{pub x:X,pub y:Y,pub z:Z}

#[derive(Debug,Clone)]
pub struct Vec4<X,Y=X,Z=Y,W=Z>{pub x:X,pub y:Y,pub z:Z,pub w:W}

