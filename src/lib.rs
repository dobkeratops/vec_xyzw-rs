#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// dammit.. have to commit to 'Copy' types for current sources :(


#[derive(Debug,Clone,Copy)]
pub struct Vec1<X:Copy>{pub x:X}

#[derive(Debug,Clone,Copy)]
pub struct Vec2<X:Copy,Y:Copy=X>{pub x:X,pub y:Y}

#[derive(Debug,Clone,Copy)]
pub struct Vec3<X:Copy,Y:Copy=X,Z:Copy=Y>{pub x:X,pub y:Y,pub z:Z}

#[derive(Debug,Clone,Copy)]
pub struct Vec4<X:Copy,Y:Copy=X,Z:Copy=Y,W:Copy=Z>{pub x:X,pub y:Y,pub z:Z,pub w:W}

