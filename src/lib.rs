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

pub type DefaultElemType=f32;

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

/// Conversions for vector types.
pub mod conversions {
	use super::{Vec1,Vec2,Vec3,Vec4};
	impl IsNot<Vec3<f32>> for Vec3<f64>{}
	impl IsNot<Vec3<f64>> for Vec3<f32>{}
	impl IsNot<Vec2<f32>> for Vec2<f64>{}
	impl IsNot<Vec2<f64>> for Vec2<f32>{}
	impl IsNot<Vec4<f32>> for Vec4<f64>{}
	impl IsNot<Vec4<f64>> for Vec4<f32>{}

	/// conversion to and from tuple types
/*
// it wont allow that, only 'into'
	impl<X:Clone> From<Vec1<X>> for (X,){
		fn from(v:&Vec1<X>)->Self{(v.x.clone(),)}
	}
	impl<X:Clone,Y:Clone> From<Vec2<X,Y>> for (X,Y){
		fn from(v:&Vec2<X,Y>)->Self{(v.x.clone(),v.y.clone())}
	}
	impl<X:Clone,Y:Clone,Z:Clone> From<Vec3<X,Y,Z>> for (X,Y,Z){
		fn from(v:&Vec3<X,Y,Z>)->Self{(v.x.clone(),v.y.clone(),v.z.clone())}
	}
	impl<X:Clone,Y:Clone,Z:Clone,W:Clone> From<Vec4<X,Y,Z,W>> for (X,Y,Z,W){
		fn from(v:&Vec4<X,Y,Z,W>)->Self{(v.x.clone(),v.y.clone(),v.z.clone(),v.w.clone())}
	}
*/

	impl<X:Copy+Default,Y:Copy+Default> Into< (X,Y) > for Vec2<X,Y>  {
		fn into(self)->(X,Y){ (self.x.clone(),self.y.clone()) }
	}
	impl<X:Copy+Default,Y:Copy+Default,Z:Copy+Default> Into< (X,Y,Z) > for Vec3<X,Y,Z>  {
		fn into(self)->(X,Y,Z){ (self.x.clone(),self.y.clone(),self.z.clone()) }
	}
	impl<X:Copy+Default,Y:Copy+Default,Z:Copy+Default,W:Copy+Default> Into< (X,Y,Z,W) > for Vec4<X,Y,Z,W>  {
		fn into(self)->(X,Y,Z,W){ (self.x.clone(),self.y.clone(),self.z.clone(),self.w.clone()) }
	}

	impl<X:Copy+Default,Y:Copy+Default> From< (X,Y) > for Vec2<X,Y>  {
		fn from(src:(X,Y))->Self { Vec2{x:src.0 .clone(),y:src.1 .clone()} }
	}

	impl<X:Copy+Default,Y:Copy+Default,Z:Copy+Default> From< (X,Y,Z) > for Vec3<X,Y,Z>  {
		fn from(src:(X,Y,Z))->Self { Vec3{x:src.0 .clone(),y:src.1 .clone(),z:src.2 .clone()} }
	}
	impl<X:Copy+Default,Y:Copy+Default,Z:Copy+Default,W:Copy+Default> From< (X,Y,Z,W) > for Vec4<X,Y,Z,W>  {
		fn from(src:(X,Y,Z,W))->Self { Vec4{x:src.0 .clone(),y:src.1 .clone(),z:src.2 .clone(),w:src.3 .clone()} }
	}

	/// conversion to & from array types
	impl<T:Copy+Default> Into< [T;2] > for Vec2<T>  {
		fn into(self)->[T;2]{ [self.x.clone(),self.y.clone()] }
	}
	impl<T:Copy+Default> Into< [T;3] > for Vec3<T>  {
		fn into(self)->[T;3]{ [self.x.clone(),self.y.clone(),self.z.clone()] }
	}
	impl<T:Copy+Default> Into< [T;4] > for Vec4<T>  {
		fn into(self)->[T;4]{ [self.x.clone(),self.y.clone(),self.z.clone(),self.w.clone()] }
	}
	impl<T:Copy+Default> From< [T;2] > for Vec2<T>  {
		fn from(src:[T;2])->Self { Vec2{x:src[0] .clone(),y:src[1] .clone()} }
	}

	impl<T:Copy+Default> From< [T;3] > for Vec3<T>  {
		fn from(src:[T;3])->Self { Vec3{x:src[0] .clone(),y:src[1] .clone(),z:src[2] .clone()} }
	}
	impl<T:Copy+Default> From< [T;4] > for Vec4<T>  {
		fn from(src:[T;4])->Self { Vec4{x:src[0] .clone(),y:src[1] .clone(),z:src[2] .clone(),w:src[3] .clone()} }
	}


	/// helper trait to hack restriction, to avoid clash between generic componentwise conversions and the 'reflecti
	pub trait IsNot<B>{}
	impl IsNot<f32> for f64{}
	impl IsNot<f64> for f32{}
	impl IsNot<i32> for f32{}
	impl IsNot<f32> for i32{}
	impl IsNot<i8> for f32{}
	impl IsNot<f32> for i8{}
	impl IsNot<u8> for f32{}
	impl IsNot<f32> for u8{}

	// TODO could roll a macro for other types
	/// Generic componentwise conversions to and from VecN<f32>
	impl<B:Copy+Default> From<Vec2<B>> for Vec2<f32> where f32:From<B>, B:IsNot<f32> {
		fn from(b:Vec2<B>)->Self {
			Vec2::<f32>{ x: b.x.into(), y: b.y.into() }		
		}
	}
	impl<B:Copy+Default> From<Vec2<f32>> for Vec2<B> where B:From<f32>, f32:IsNot<B> {
		fn from(b:Vec2<f32>)->Self {
			Vec2::<B>{ x: b.x.into(), y: b.y.into() }		
		}
	}

	impl<B:Copy+Default> From<Vec3<B>> for Vec3<f32> where f32:From<B>, B:IsNot<f32> {
		fn from(b:Vec3<B>)->Self {
			Vec3::<f32>{ x: b.x.into(), y: b.y.into(), z: b.z.into() }		
		}
	}
	impl<B:Copy+Default> From<Vec3<f32>> for Vec3<B> where B:From<f32>, f32:IsNot<B> {
		fn from(b:Vec3<f32>)->Self {
			Vec3::<B>{ x: b.x.into(), y: b.y.into(), z: b.z.into() }		
		}
	}

	impl<B:Copy+Default> From<Vec4<B>> for Vec4<f32> where f32:From<B>, B:IsNot<f32> {
		fn from(b:Vec4<B>)->Self {
			Vec4::<f32>{ x: b.x.into(), y: b.y.into(), z: b.z.into(), w: b.w.into() }		
		}
	}
	impl<B:Copy+Default> From<Vec4<f32>> for Vec4<B> where B:From<f32>, f32:IsNot<B> {
		fn from(b:Vec4<f32>)->Self {
			Vec4::<B>{ x: b.x.into(), y: b.y.into(), z: b.z.into(), w: b.w.into() }		
		}
	}

}
pub use conversions::*;

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
