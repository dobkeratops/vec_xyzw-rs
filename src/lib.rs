#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug,Clone)]
pub struct Vec1<X>{x:X}

#[derive(Debug,Clone)]
pub struct Vec2<X,Y=X>{x:X,y:Y}

#[derive(Debug,Clone)]
pub struct Vec3<X,Y=X,Z=Y>{x:X,y:Y,z:Z}

#[derive(Debug,Clone)]
pub struct Vec4<X,Y=X,Z=Y,W=Z>{x:X,y:Y,z:Z,w:W}

