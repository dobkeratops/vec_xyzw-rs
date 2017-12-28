#![allow(unused_imports)]
#![feature(step_trait)]
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

type DefaultElemType=f32;

#[derive(Debug,Clone,Copy,Default)]
pub struct Vec1<X:Copy+Default=DefaultElemType>{pub x:X}

#[derive(Debug,Clone,Copy,Default)]
pub struct Vec2<X:Copy+Default=DefaultElemType,Y:Copy+Default=X>{pub x:X,pub y:Y}

#[derive(Debug,Clone,Copy,Default)]
pub struct Vec3<X:Copy+Default=DefaultElemType,Y:Copy+Default=X,Z:Copy+Default=Y>{pub x:X,pub y:Y,pub z:Z}

#[derive(Debug,Clone,Copy,Default)]
pub struct Vec4<X:Copy+Default=DefaultElemType,Y:Copy+Default=X,Z:Copy+Default=Y,W:Copy+Default=Z>{pub x:X,pub y:Y,pub z:Z,pub w:W}

// if all parts are homogeneous, impl index (TODO - & iter)
macro_rules! impl_index{($int:ty)=>{
	impl<T:Clone+Copy+Default> Index<$int> for Vec2<T>{
		type Output=T;
		fn index(&self,i:$int)->&T{ match i{
			0=>&self.x, 1=>&self.y, _=>panic!("vec3 index out of range")
		}}
	}
	impl<T:Clone+Copy+Default> IndexMut<$int> for Vec2<T>{
		fn index_mut(&mut self,i:$int)->&mut T{ match i{
			0=>&mut self.x, 1=>&mut self.y, _=>panic!("vec3 index out of range")
		}}
	}
	impl<T:Clone+Copy+Default> Index<$int> for Vec3<T>{
		type Output=T;
		fn index(&self,i:$int)->&T{ match i{
			0=>&self.x, 1=>&self.y, 2=>&self.z, _=>panic!("vec3 index out of range")
		}}
	}
	impl<T:Clone+Copy+Default> IndexMut<$int> for Vec3<T>{
		fn index_mut(&mut self,i:$int)->&mut T{ match i{
			0=>&mut self.x, 1=>&mut self.y, 2=>&mut self.z, _=>panic!("vec3 index out of range")
		}}
	}
	// if all parts are homogeneous, impl index (TODO - & iter)
	impl<T:Clone+Copy+Default> Index<$int> for Vec4<T>{
		type Output=T;
		fn index(&self,i:$int)->&T{ match i{
			0=>&self.x, 1=>&self.y, 2=>&self.z,3=>&self.w, _=>panic!("vec3 index out of range")
		}}
	}
	impl<T:Clone+Copy+Default> IndexMut<$int> for Vec4<T>{
		fn index_mut(&mut self,i:$int)->&mut T{ match i{
			0=>&mut self.x, 1=>&mut self.y, 2=>&mut self.z,3=>&mut self.w, _=>panic!("vec3 index out of range")
		}}
	}
}}
impl_index!(i32);
impl_index!(u32);
impl_index!(i8);
impl_index!(u8);
impl_index!(usize);
impl_index!(isize);


// Didn't want to impl anything here but
// it seems it's essential to impl the
// operators otherwise users can't

impl<B,A:PartialEq<B>> PartialEq<Vec3<B>> for Vec3<A>
	where A:Copy+Default,B:Copy+Default
{
	fn eq(&self,b:&Vec3<B>)->bool{
		self.x==b.x && self.y==b.y && self.z==b.z
	}
}

macro_rules! impl_operator_per_elem {
	($({$trait:ident,$opfn:ident},)*)=>{
		//implement the operator for value types
		$(impl<B:Copy+Default, T:Copy+Default+$trait<B>> $trait<Vec3<B>> for Vec3<T>
			where <T as $trait<B> >::Output : Copy+Default
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
		$(impl<'a,'b,B:Copy+Default, T:Copy+Default+$trait<B>> $trait<&'b Vec3<B>> for &'a Vec3<T>
			where <T as $trait<B> >::Output : Copy+Default
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
		$(impl<B:Copy+Default, T:Copy+Default+$trait<B>> $trait<Vec3<B>> for Vec3<T>
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
