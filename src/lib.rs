#![allow(unused_imports)]
use std::ops::{Add,Sub,Mul,Div,Rem,BitOr,BitAnd,BitXor,Index,IndexMut,AddAssign,SubAssign,MulAssign,DivAssign,RemAssign,BitOrAssign,BitAndAssign,BitXorAssign};
use std::cmp::PartialEq;

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


// Didn't want to impl anything here but
// it seems it's essential to impl the
// operators otherwise users can't

impl<B,A:PartialEq<B>> PartialEq<Vec3<B>> for Vec3<A>
	where A:Copy,B:Copy
{
	fn eq(&self,b:&Vec3<B>)->bool{
		self.x==b.x && self.y==b.y && self.z==b.z
	}
}
macro_rules! impl_operator_per_elem {
	($({$trait:ident,$opfn:ident},)*)=>{
		//implement the operator for value types
		$(impl<B:Copy, T:Copy+$trait<B>> $trait<Vec3<B>> for Vec3<T>
			where <T as $trait<B> >::Output : Copy
		{
			type Output=Vec3< 
				<T as $trait<B>>::Output
			>;
			fn $opfn(self,other:Vec3<B>)->Self::Output{
				Vec3{
					x: self.x.$opfn(other.x),
					y: self.y.$opfn(other.y),
					z: self.z.$opfn(other.z),
				}
			}
		})*
		//implement the operator for reference types
		$(impl<'a,'b,B:Copy, T:Copy+$trait<B>> $trait<&'b Vec3<B>> for &'a Vec3<T>
			where <T as $trait<B> >::Output : Copy
		{
			type Output=Vec3< 
				<T as $trait<B>>::Output
			>;
			fn $opfn(self,other:&'b Vec3<B>)->Self::Output{
				Vec3{
					x: self.x.$opfn(other.x),
					y: self.y.$opfn(other.y),
					z: self.z.$opfn(other.z),
				}
			}
		})*
	}
}

macro_rules! impl_assign_operator_per_elem {
	($({$trait:ident,$opfn:ident},)*)=>{
		//implement the operator for value types
		$(impl<B:Copy, T:Copy+$trait<B>> $trait<Vec3<B>> for Vec3<T>
		{
			fn $opfn<'a>(&'a mut self,other:Vec3<B>){
				self.x.$opfn(other.x);
				self.y.$opfn(other.y);
				self.z.$opfn(other.z);
			}
		})*
		// todo - how does it work for &mut self
		//implement the operator for reference types
	}
}

impl_operator_per_elem![{Add,add},{Sub,sub},{Mul,mul},{Div,div},{BitAnd,bitand},{BitOr,bitor},];
impl_assign_operator_per_elem![{AddAssign,add_assign},{SubAssign,sub_assign},{MulAssign,mul_assign},{DivAssign,div_assign},{BitAndAssign,bitand_assign},{BitOrAssign,bitor_assign},{BitXorAssign,bitxor_assign},];
