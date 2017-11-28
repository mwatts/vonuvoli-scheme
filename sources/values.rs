

use super::contexts::exports::*;
use super::errors::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
use super::runtime::exports::*;

use std::char;
use std::cmp;
use std::fmt;
use std::hash;
use std::ops;
use std::ptr;
use std::str;




pub mod exports {
	
	pub use super::{Value, ValueBox, ValueVec, ValueClass};
	pub use super::{Boolean, BooleanBox, BooleanVec};
	pub use super::{NumberInteger, NumberIntegerBox, NumberIntegerVec};
	pub use super::{NumberReal, NumberRealBox, NumberRealVec};
	pub use super::{Character, CharacterBox, CharacterVec};
	pub use super::{Symbol, SymbolBox, SymbolVec};
	pub use super::{String, StringBox, StringVec};
	pub use super::{Bytes, BytesBox, BytesVec};
	pub use super::{Pair, PairBox, PairVec};
	pub use super::{Array, ArrayBox, ArrayVec};
	
	pub use super::{boolean, number_i64, number_f64, character};
	pub use super::{symbol_new, symbol_clone_str, symbol_clone_characters};
	pub use super::{string_new, string_clone_str, string_clone_characters};
	pub use super::{bytes_new, bytes_clone_slice};
	pub use super::{array_new, array_clone_slice};
	pub use super::{pair_new};
	
	pub use super::{ValueMeta1, ValueMeta2, VALUE_META_1, VALUE_META_2};
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ValueClass {
	
	Null,
	Void,
	Undefined,
	
	Boolean,
	NumberInteger,
	NumberReal,
	Character,
	
	Symbol,
	String,
	Bytes,
	
	Pair,
	Array,
	
	Error,
	
	Lambda,
	ProcedurePrimitive,
	SyntaxPrimitive,
	
	Context,
	Binding,
	
}




#[ derive (Clone, Eq, PartialEq, Hash) ]
pub enum Value {
	
	Null ( ValueMeta1, ValueMeta2 ),
	Void ( ValueMeta1, ValueMeta2 ),
	Undefined ( ValueMeta1, ValueMeta2 ),
	
	Boolean ( ValueMeta1, Boolean, ValueMeta2 ),
	NumberInteger ( ValueMeta1, NumberInteger, ValueMeta2 ),
	NumberReal ( ValueMeta1, NumberReal, ValueMeta2 ),
	Character ( ValueMeta1, Character, ValueMeta2 ),
	
	Symbol ( ValueMeta1, Symbol, ValueMeta2 ),
	String ( ValueMeta1, String, ValueMeta2 ),
	Bytes ( ValueMeta1, Bytes, ValueMeta2 ),
	
	Pair ( ValueMeta1, Pair, ValueMeta2 ),
	Array ( ValueMeta1, Array, ValueMeta2 ),
	
	Error ( ValueMeta1, Error, ValueMeta2 ),
	
	Lambda ( ValueMeta1, Lambda, ValueMeta2 ),
	ProcedurePrimitive ( ValueMeta1, ProcedurePrimitive, ValueMeta2 ),
	SyntaxPrimitive ( ValueMeta1, SyntaxPrimitive, ValueMeta2, ),
	
	Context ( ValueMeta1, Context, ValueMeta2 ),
	Binding ( ValueMeta1, Binding, ValueMeta2 ),
	
}


pub type ValueBox = StdBox<Value>;
pub type ValueVec = StdVec<Value>;


#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct ValueMeta1 ( u8, u8, u8 );
pub const VALUE_META_1 : ValueMeta1 = ValueMeta1 (0, 0, 0);

#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
pub struct ValueMeta2 ( u8, u8, u8, u8 );
pub const VALUE_META_2 : ValueMeta2 = ValueMeta2 (0, 0, 0, 0);


impl Value {
	
	pub fn class (&self) -> (ValueClass) {
		match *self {
			
			Value::Null (_, _) => ValueClass::Null,
			Value::Void (_, _) => ValueClass::Void,
			Value::Undefined (_, _) => ValueClass::Undefined,
			
			Value::Boolean (_, _, _) => ValueClass::Boolean,
			Value::NumberInteger (_, _, _) => ValueClass::NumberInteger,
			Value::NumberReal (_, _, _) => ValueClass::NumberReal,
			Value::Character (_, _, _) => ValueClass::Character,
			
			Value::Symbol (_, _, _) => ValueClass::Symbol,
			Value::String (_, _, _) => ValueClass::String,
			Value::Bytes (_, _, _) => ValueClass::Bytes,
			
			Value::Pair (_, _, _) => ValueClass::Pair,
			Value::Array (_, _, _) => ValueClass::Array,
			
			Value::Error (_, _, _) => ValueClass::Error,
			
			Value::Lambda (_, _, _) => ValueClass::Lambda,
			Value::ProcedurePrimitive (_, _, _) => ValueClass::ProcedurePrimitive,
			Value::SyntaxPrimitive (_, _, _) => ValueClass::SyntaxPrimitive,
			
			Value::Context (_, _, _) => ValueClass::Context,
			Value::Binding (_, _, _) => ValueClass::Binding,
			
		}
	}
	
	pub fn is (&self, class : ValueClass) -> (bool) {
		self.class () == class
	}
	
	
	pub fn is_self (&self, other : &Value) -> (bool) {
		ptr::eq (self, other)
	}
	
	
}




impl fmt::Display for Value {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			
			Value::Null (_, _) => formatter.write_str ("#null"),
			Value::Void (_, _) => formatter.write_str ("#void"),
			Value::Undefined (_, _) => formatter.write_str ("#undefined"),
			
			Value::Boolean (_, ref value, _) => value.fmt (formatter),
			Value::NumberInteger (_, ref value, _) => value.fmt (formatter),
			Value::NumberReal (_, ref value, _) => value.fmt (formatter),
			Value::Character (_, ref value, _) => value.fmt (formatter),
			
			Value::Symbol (_, ref value, _) => value.fmt (formatter),
			Value::String (_, ref value, _) => value.fmt (formatter),
			Value::Bytes (_, ref value, _) => value.fmt (formatter),
			
			Value::Pair (_, ref value, _) => value.fmt (formatter),
			Value::Array (_, ref value, _) => value.fmt (formatter),
			
			Value::Error (_, ref value, _) => value.fmt (formatter),
			
			Value::Lambda (_, ref value, _) => value.fmt (formatter),
			
			Value::ProcedurePrimitive (_, ref value, _) => write! (formatter, "#<procedure:{:?}>", value),
			Value::SyntaxPrimitive (_, ref value, _) => write! (formatter, "#<syntax:{:?}>", value),
			
			Value::Context (_, ref value, _) => value.fmt (formatter),
			Value::Binding (_, ref value, _) => value.fmt (formatter),
			
		}
	}
}


impl fmt::Debug for Value {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match *self {
			
			Value::Null (_, _) => formatter.debug_struct ("Null") .finish (),
			Value::Void (_, _) => formatter.debug_struct ("Undefined") .finish (),
			Value::Undefined (_, _) => formatter.debug_struct ("Undefined") .finish (),
			
			Value::Boolean (_, ref value, _) => value.fmt (formatter),
			Value::NumberInteger (_, ref value, _) => value.fmt (formatter),
			Value::NumberReal (_, ref value, _) => value.fmt (formatter),
			Value::Character (_, ref value, _) => value.fmt (formatter),
			
			Value::Symbol (_, ref value, _) => value.fmt (formatter),
			Value::String (_, ref value, _) => value.fmt (formatter),
			Value::Bytes (_, ref value, _) => value.fmt (formatter),
			
			Value::Pair (_, ref value, _) => value.fmt (formatter),
			Value::Array (_, ref value, _) => value.fmt (formatter),
			
			Value::Error (_, ref value, _) => value.fmt (formatter),
			
			Value::Lambda (_, ref value, _) => value.fmt (formatter),
			
			Value::ProcedurePrimitive (_, ref value, _) => value.fmt (formatter),
			Value::SyntaxPrimitive (_, ref value, _) => value.fmt (formatter),
			
			Value::Context (_, ref value, _) => value.fmt (formatter),
			Value::Binding (_, ref value, _) => value.fmt (formatter),
			
		}
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Boolean ( pub bool );


pub type BooleanBox = StdBox<Boolean>;
pub type BooleanVec = StdVec<Boolean>;


impl Boolean {
	
	pub fn not (&self) -> (Boolean) {
		(!self.0) .into ()
	}
	
	pub fn and (&self, other : &Boolean) -> (Boolean) {
		(self.0 && other.0) .into ()
	}
	
	pub fn or (&self, other : &Boolean) -> (Boolean) {
		(self.0 || other.0) .into ()
	}
	
	pub fn xor (&self, other : &Boolean) -> (Boolean) {
		(self.0 ^ other.0) .into ()
	}
	
	pub fn nand (&self, other : &Boolean) -> (Boolean) {
		self.and (other) .not ()
	}
	
	pub fn nor (&self, other : &Boolean) -> (Boolean) {
		self.or (other) .not ()
	}
	
	pub fn nxor (&self, other : &Boolean) -> (Boolean) {
		self.xor (other) .not ()
	}
}


impl ops::Not for Boolean {
	type Output = Boolean;
	fn not (self) -> (Boolean) {
		Boolean::not (&self)
	}
}


impl fmt::Display for Boolean {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		match self.0 {
			true =>
				formatter.write_str ("#true"),
			false =>
				formatter.write_str ("#false"),
		}
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct NumberInteger ( pub i64 );


pub type NumberIntegerBox = StdBox<NumberInteger>;
pub type NumberIntegerVec = StdVec<NumberInteger>;


macro_rules! NumberInteger_fn_try_to_signed_integer {
	($export : ident, $type : ty) => (
		pub fn $export (&self) -> (Outcome<$type>) {
			let value = self.0;
			if ::std::mem::size_of::<i64> () <= ::std::mem::size_of::<$type> () {
				succeed! (value as $type);
			}
			let min = <$type>::min_value () as i64;
			if value < min {
				fail! (0xe1deffc5);
			}
			let max = <$type>::max_value () as i64;
			if value > max {
				fail! (0x3d9c7881);
			}
			succeed! (value as $type);
		}
	);
}


macro_rules! NumberInteger_fn_try_to_unsigned_integer {
	($export : ident, $type : ty) => (
		pub fn $export (&self) -> (Outcome<$type>) {
			let value = self.0;
			if value < 0 {
				fail! (0xe4d76587);
			}
			if ::std::mem::size_of::<i64> () <= ::std::mem::size_of::<$type> () {
				succeed! (value as $type);
			}
			let max = <$type>::max_value () as i64;
			if value > max {
				fail! (0x4212bbb7);
			}
			succeed! (value as $type);
		}
	);
}


macro_rules! NumberInteger_fn_predicate {
	($delegate : ident) => (
		NumberInteger_fn_predicate! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self) -> (bool) {
			<i64>::$delegate (self.0)
		}
	);
}


macro_rules! NumberInteger_fn_delegate_1 {
	($delegate : ident) => (
		NumberInteger_fn_delegate_1! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self) -> (NumberInteger) {
			<i64>::$delegate (self.0) .into ()
		}
	);
}

macro_rules! NumberInteger_fn_delegate_2 {
	($delegate : ident) => (
		NumberInteger_fn_delegate_2! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self, other : &NumberInteger) -> (NumberInteger) {
			<i64>::$delegate (self.0, other.0) .into ()
		}
	);
}


macro_rules! NumberInteger_fn_delegate_1_real {
	($delegate : ident) => (
		NumberInteger_fn_delegate_1_real! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self) -> (NumberReal) {
			<f64>::$delegate (self.0 as f64) .into ()
		}
	);
}

macro_rules! NumberInteger_fn_delegate_2_real {
	($delegate : ident) => (
		NumberInteger_fn_delegate_2_real! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self, other : &NumberReal) -> (NumberReal) {
			<f64>::$delegate (self.0 as f64, other.0) .into ()
		}
	);
}


impl NumberInteger {
	
	
	NumberInteger_fn_try_to_signed_integer! (try_to_i8, i8);
	NumberInteger_fn_try_to_signed_integer! (try_to_i16, i16);
	NumberInteger_fn_try_to_signed_integer! (try_to_i32, i32);
	NumberInteger_fn_try_to_signed_integer! (try_to_i64, i64);
	NumberInteger_fn_try_to_signed_integer! (try_to_isize, isize);
	
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u8, u8);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u16, u16);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u32, u32);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_u64, u64);
	NumberInteger_fn_try_to_unsigned_integer! (try_to_usize, usize);
	
	pub fn try_to_char (&self) -> (Outcome<char>) {
		let value = try! (self.try_to_u32 ());
		if let Some (value) = char::from_u32 (value) {
			succeed! (value);
		} else {
			fail! (0x36d5ef86);
		}
	}
	
	
	pub fn neg (&self) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_neg (self.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0xd93d04db);
		}
	}
	
	pub fn abs (&self) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_abs (self.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0x997daa2a);
		}
	}
	
	pub fn add (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_add (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0xd61736b6);
		}
	}
	
	pub fn sub (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_sub (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0x1e036be9);
		}
	}
	
	pub fn mul (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_mul (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0x32c5b516);
		}
	}
	
	pub fn div (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_div (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0xce26bc76);
		}
	}
	
	pub fn rem (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		if let Some (outcome) = <i64>::checked_rem (self.0, other.0) {
			succeed! (outcome.into ());
		} else {
			fail! (0xce26bc76);
		}
	}
	
	pub fn pow (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xdcca20dd);
		}
		succeed! (<i64>::pow (self.0, other as u32) .into ());
	}
	
	
	pub fn is_zero (&self) -> (bool) {
		self.0 == 0
	}
	
	pub fn is_even (&self) -> (bool) {
		(self.0 & 1) == 0
	}
	
	pub fn is_odd (&self) -> (bool) {
		(self.0 & 1) != 0
	}
	
	
	pub fn bitnot (&self) -> (NumberInteger) {
		(!self.0) .into ()
	}
	
	pub fn bitand (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 & other.0) .into ()
	}
	
	pub fn bitor (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 | other.0) .into ()
	}
	
	pub fn bitxor (&self, other : &NumberInteger) -> (NumberInteger) {
		(self.0 ^ other.0) .into ()
	}
	
	pub fn bitnand (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitand (other) .bitnot ()
	}
	
	pub fn bitnor (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitor (other) .bitnot ()
	}
	
	pub fn bitnxor (&self, other : &NumberInteger) -> (NumberInteger) {
		self.bitxor (other) .bitnot ()
	}
	
	
	pub fn shl (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xb84272a0);
		}
		if let Some (outcome) = <i64>::checked_shl (self.0, other as u32) {
			succeed! (outcome.into ());
		} else {
			fail! (0x734e69d8);
		}
	}
	
	pub fn shr (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0x26d90f55);
		}
		if let Some (outcome) = <i64>::checked_shr (self.0, other as u32) {
			succeed! (outcome.into ());
		} else {
			fail! (0xc3bb81a9);
		}
	}
	
	pub fn rotate_left (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0xe2038e82);
		}
		succeed! ((<i64>::rotate_left (self.0, other as u32)) .into ());
	}
	
	pub fn rotate_right (&self, other : &NumberInteger) -> (Outcome<NumberInteger>) {
		let other = other.0;
		if (other < 0) || (other > (<u32>::max_value () as i64)) {
			fail! (0x1d868231);
		}
		succeed! ((<i64>::rotate_right (self.0, other as u32)) .into ());
	}
	
	
	NumberInteger_fn_delegate_1! (wrapping_neg);
	NumberInteger_fn_delegate_1! (wrapping_abs);
	NumberInteger_fn_delegate_2! (wrapping_add);
	NumberInteger_fn_delegate_2! (wrapping_sub);
	NumberInteger_fn_delegate_2! (wrapping_mul);
	NumberInteger_fn_delegate_2! (wrapping_div);
	NumberInteger_fn_delegate_2! (wrapping_rem);
	
	NumberInteger_fn_delegate_2! (saturating_add);
	NumberInteger_fn_delegate_2! (saturating_sub);
	NumberInteger_fn_delegate_2! (saturating_mul);
	
	NumberInteger_fn_delegate_1! (signum);
	
	NumberInteger_fn_predicate! (is_positive);
	NumberInteger_fn_predicate! (is_negative);
	
	NumberInteger_fn_delegate_2! (min);
	NumberInteger_fn_delegate_2! (max);
	
	NumberInteger_fn_delegate_1! (count_ones);
	NumberInteger_fn_delegate_1! (count_zeros);
	NumberInteger_fn_delegate_1! (leading_zeros);
	NumberInteger_fn_delegate_1! (trailing_zeros);
	NumberInteger_fn_delegate_1! (swap_bytes);
	NumberInteger_fn_delegate_1! (from_be);
	NumberInteger_fn_delegate_1! (from_le);
	NumberInteger_fn_delegate_1! (to_be);
	NumberInteger_fn_delegate_1! (to_le);
	
	NumberInteger_fn_delegate_1_real! (recip);
	NumberInteger_fn_delegate_1_real! (sqrt);
	NumberInteger_fn_delegate_1_real! (cbrt);
	
	NumberInteger_fn_delegate_2_real! (power, powf);
	NumberInteger_fn_delegate_2_real! (log);
	
	NumberInteger_fn_delegate_1_real! (exp);
	NumberInteger_fn_delegate_1_real! (exp2);
	NumberInteger_fn_delegate_1_real! (exp_m1);
	NumberInteger_fn_delegate_1_real! (ln);
	NumberInteger_fn_delegate_1_real! (log2);
	NumberInteger_fn_delegate_1_real! (log10);
	NumberInteger_fn_delegate_1_real! (ln_1p);
	
	NumberInteger_fn_delegate_1_real! (sin);
	NumberInteger_fn_delegate_1_real! (cos);
	NumberInteger_fn_delegate_1_real! (tan);
	NumberInteger_fn_delegate_1_real! (asin);
	NumberInteger_fn_delegate_1_real! (acos);
	NumberInteger_fn_delegate_1_real! (atan);
	
	NumberInteger_fn_delegate_1_real! (sinh);
	NumberInteger_fn_delegate_1_real! (cosh);
	NumberInteger_fn_delegate_1_real! (tanh);
	NumberInteger_fn_delegate_1_real! (asinh);
	NumberInteger_fn_delegate_1_real! (acosh);
	NumberInteger_fn_delegate_1_real! (atanh);
	
	NumberInteger_fn_delegate_2_real! (hypot);
	NumberInteger_fn_delegate_2_real! (atan2);
	
	NumberInteger_fn_delegate_1_real! (to_degrees);
	NumberInteger_fn_delegate_1_real! (to_radians);
	
}


impl ops::Neg for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn neg (self) -> (Outcome<NumberInteger>) {
		NumberInteger::neg (&self)
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Add<NumberIntegerInto> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn add (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::add (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Sub<NumberIntegerInto> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn sub (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::sub (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Mul<NumberIntegerInto> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn mul (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::mul (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Div<NumberIntegerInto> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn div (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::div (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Rem<NumberIntegerInto> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn rem (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::rem (&self, &other.into ())
	}
}


impl ops::Not for NumberInteger {
	type Output = NumberInteger;
	fn not (self) -> (NumberInteger) {
		NumberInteger::bitnot (&self)
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::BitAnd<NumberIntegerInto> for NumberInteger {
	type Output = NumberInteger;
	fn bitand (self, other : NumberIntegerInto) -> (NumberInteger) {
		NumberInteger::bitand (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::BitOr<NumberIntegerInto> for NumberInteger {
	type Output = NumberInteger;
	fn bitor (self, other : NumberIntegerInto) -> (NumberInteger) {
		NumberInteger::bitor (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::BitXor<NumberIntegerInto> for NumberInteger {
	type Output = NumberInteger;
	fn bitxor (self, other : NumberIntegerInto) -> (NumberInteger) {
		NumberInteger::bitxor (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Shl<NumberIntegerInto> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn shl (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::shl (&self, &other.into ())
	}
}

impl <NumberIntegerInto : StdInto<NumberInteger>> ops::Shr<NumberIntegerInto> for NumberInteger {
	type Output = Outcome<NumberInteger>;
	fn shr (self, other : NumberIntegerInto) -> (Outcome<NumberInteger>) {
		NumberInteger::shr (&self, &other.into ())
	}
}


impl fmt::Display for NumberInteger {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{}", self.0)
	}
}




#[ derive (Copy, Clone, Debug) ]
pub struct NumberReal ( pub f64 );


pub type NumberRealBox = StdBox<NumberReal>;
pub type NumberRealVec = StdVec<NumberReal>;


macro_rules! NumberReal_fn_predicate {
	($delegate : ident) => (
		NumberReal_fn_predicate! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self) -> (bool) {
			<f64>::$delegate (self.0)
		}
	);
}


macro_rules! NumberReal_fn_delegate_1 {
	($delegate : ident) => (
		NumberReal_fn_delegate_1! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self) -> (NumberReal) {
			<f64>::$delegate (self.0) .into ()
		}
	);
}

macro_rules! NumberReal_fn_delegate_2 {
	($delegate : ident) => (
		NumberReal_fn_delegate_2! ($delegate, $delegate);
	);
	($export : ident, $delegate : ident) => (
		pub fn $export (&self, other : &NumberReal) -> (NumberReal) {
			<f64>::$delegate (self.0, other.0) .into ()
		}
	);
}


impl NumberReal {
	
	
	pub fn neg (&self) -> (NumberReal) {
		(-self.0) .into ()
	}
	
	pub fn add (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 + other.0) .into ()
	}
	
	pub fn sub (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 - other.0) .into ()
	}
	
	pub fn mul (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 * other.0) .into ()
	}
	
	pub fn div (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 / other.0) .into ()
	}
	
	pub fn rem (&self, other : &NumberReal) -> (NumberReal) {
		(self.0 % other.0) .into ()
	}
	
	
	pub fn is_zero (&self) -> (bool) {
		self.0 == 0.0
	}
	
	pub fn is_even (&self) -> (bool) {
		(self.0 % 2.0) == 0.0
	}
	
	pub fn is_odd (&self) -> (bool) {
		(self.0 % 2.0) != 0.0
	}
	
	
	NumberReal_fn_delegate_1! (abs);
	NumberReal_fn_delegate_1! (signum);
	
	NumberReal_fn_predicate! (is_finite);
	NumberReal_fn_predicate! (is_infinite);
	NumberReal_fn_predicate! (is_nan);
	NumberReal_fn_predicate! (is_positive, is_sign_positive);
	NumberReal_fn_predicate! (is_negative, is_sign_negative);
	
	NumberReal_fn_delegate_2! (min);
	NumberReal_fn_delegate_2! (max);
	
	NumberReal_fn_delegate_1! (floor);
	NumberReal_fn_delegate_1! (ceil);
	NumberReal_fn_delegate_1! (round);
	NumberReal_fn_delegate_1! (trunc);
	NumberReal_fn_delegate_1! (fract);
	
	NumberReal_fn_delegate_1! (recip);
	NumberReal_fn_delegate_1! (sqrt);
	NumberReal_fn_delegate_1! (cbrt);
	
	NumberReal_fn_delegate_2! (power, powf);
	NumberReal_fn_delegate_2! (log);
	
	NumberReal_fn_delegate_1! (exp);
	NumberReal_fn_delegate_1! (exp2);
	NumberReal_fn_delegate_1! (exp_m1);
	NumberReal_fn_delegate_1! (ln);
	NumberReal_fn_delegate_1! (log2);
	NumberReal_fn_delegate_1! (log10);
	NumberReal_fn_delegate_1! (ln_1p);
	
	NumberReal_fn_delegate_1! (sin);
	NumberReal_fn_delegate_1! (cos);
	NumberReal_fn_delegate_1! (tan);
	NumberReal_fn_delegate_1! (asin);
	NumberReal_fn_delegate_1! (acos);
	NumberReal_fn_delegate_1! (atan);
	
	NumberReal_fn_delegate_1! (sinh);
	NumberReal_fn_delegate_1! (cosh);
	NumberReal_fn_delegate_1! (tanh);
	NumberReal_fn_delegate_1! (asinh);
	NumberReal_fn_delegate_1! (acosh);
	NumberReal_fn_delegate_1! (atanh);
	
	NumberReal_fn_delegate_2! (hypot);
	NumberReal_fn_delegate_2! (atan2);
	
	NumberReal_fn_delegate_1! (to_degrees);
	NumberReal_fn_delegate_1! (to_radians);
	
}


impl ops::Neg for NumberReal {
	type Output = NumberReal;
	fn neg (self) -> (NumberReal) {
		NumberReal::neg (&self)
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Add<NumberRealInto> for NumberReal {
	type Output = NumberReal;
	fn add (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::add (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Sub<NumberRealInto> for NumberReal {
	type Output = NumberReal;
	fn sub (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::sub (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Mul<NumberRealInto> for NumberReal {
	type Output = NumberReal;
	fn mul (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::mul (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Div<NumberRealInto> for NumberReal {
	type Output = NumberReal;
	fn div (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::div (&self, &other.into ())
	}
}

impl <NumberRealInto : StdInto<NumberReal>> ops::Rem<NumberRealInto> for NumberReal {
	type Output = NumberReal;
	fn rem (self, other : NumberRealInto) -> (NumberReal) {
		NumberReal::rem (&self, &other.into ())
	}
}


impl cmp::Eq for NumberReal {}

impl cmp::PartialEq for NumberReal {
	fn eq (&self, other : &NumberReal) -> (bool) {
		self.0.to_bits () == other.0.to_bits ()
	}
}

impl cmp::Ord for NumberReal {
	fn cmp (&self, other : &NumberReal) -> (cmp::Ordering) {
		if let Some (ordering) = self.0.partial_cmp (&other.0) {
			ordering
		} else {
			// FIXME:  Correctly handle this!
			cmp::Ordering::Equal
		}
	}
}

impl cmp::PartialOrd for NumberReal {
	fn partial_cmp (&self, other : &NumberReal) -> (Option<cmp::Ordering>) {
		self.0.partial_cmp (&other.0)
	}
}

impl hash::Hash for NumberReal {
	fn hash<Hasher : hash::Hasher> (&self, hasher : &mut Hasher) -> () {
		hasher.write_u64 (self.0.to_bits ());
	}
}

impl fmt::Display for NumberReal {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		write! (formatter, "{}", self.0)
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Character ( pub char );


pub type CharacterBox = StdBox<Character>;
pub type CharacterVec = StdVec<Character>;


impl fmt::Display for Character {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		let character = self.0;
		match character {
			'!' ... '~' => {
				try! (formatter.write_str ("#\\"));
				try! (formatter.write_char (character));
			},
			_ =>
				try! (write! (formatter, "#\\x{:02x}", character as u32)),
		}
		succeed! (());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Symbol ( StdRc<StdString> );


pub type SymbolBox = StdBox<Symbol>;
pub type SymbolVec = StdVec<Symbol>;


impl Symbol {
	
	pub fn string_as_str (&self) -> (&str) {
		self.0.as_ref () .as_str ()
	}
	
	pub fn string_ref (&self) -> (&StdString) {
		&self.0.as_ref ()
	}
	
	pub fn string_clone (&self) -> (StdString) {
		self.0.as_ref () .clone ()
	}
	
	pub fn string_is_empty (&self) -> (bool) {
		self.string_ref () .is_empty ()
	}
	
	pub fn string_is_not_empty (&self) -> (bool) {
		!self.string_ref () .is_empty ()
	}
	
	pub fn string_eq (&self, other : &str) -> (bool) {
		self.string_ref () .eq (other)
	}
	
	pub fn string_utf8_bytes_count (&self) -> (usize) {
		self.string_ref () .len ()
	}
	
	pub fn string_chars (&self) -> (str::Chars) {
		self.string_ref () .chars ()
	}
	
	pub fn string_chars_count_compute (&self) -> (usize) {
		self.string_chars () .count ()
	}
	
	pub fn string_char_at_compute (&self, index : usize) -> (Option<char>) {
		self.string_chars () .nth (index)
	}
	
}


impl fmt::Display for Symbol {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		if self.0.is_empty () {
			try! (formatter.write_str ("||"));
		} else {
			use std::fmt::Write;
			try! (formatter.write_char ('|'));
			for character in self.0.chars () {
				match character {
					'|' | '\\' => {
						try! (formatter.write_char ('\\'));
						try! (formatter.write_char (character));
					},
					' ' ... '~' =>
						try! (formatter.write_char (character)),
					_ =>
						try! (write! (formatter, "#\\x{:02x};", character as u32)),
				}
			}
			try! (formatter.write_char ('|'));
		}
		succeed! (());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
// FIXME:  Add immutability flag!
pub struct String ( StdRc<StdString> );


pub type StringBox = StdBox<String>;
pub type StringVec = StdVec<String>;


impl String {
	
	pub fn string_as_str (&self) -> (&str) {
		self.0.as_ref () .as_str ()
	}
	
	pub fn string_ref (&self) -> (&StdString) {
		&self.0.as_ref ()
	}
	
	pub fn string_clone (&self) -> (StdString) {
		self.0.as_ref () .clone ()
	}
	
	pub fn string_is_empty (&self) -> (bool) {
		self.string_ref () .is_empty ()
	}
	
	pub fn string_is_not_empty (&self) -> (bool) {
		!self.string_ref () .is_empty ()
	}
	
	pub fn string_eq (&self, other : &str) -> (bool) {
		self.string_ref () .eq (other)
	}
	
	pub fn string_utf8_bytes_count (&self) -> (usize) {
		self.string_ref () .len ()
	}
	
	pub fn string_chars (&self) -> (str::Chars) {
		self.string_ref () .chars ()
	}
	
	pub fn string_chars_count_compute (&self) -> (usize) {
		self.string_chars () .count ()
	}
	
	pub fn string_char_at_compute (&self, index : usize) -> (Option<char>) {
		self.string_chars () .nth (index)
	}
	
}


impl fmt::Display for String {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_char ('"'));
		for character in self.0.chars () {
			match character {
				'"' | '\\' => {
					try! (formatter.write_char ('\\'));
					try! (formatter.write_char (character));
				},
				' ' ... '~' =>
					try! (formatter.write_char (character)),
				_ =>
					try! (write! (formatter, "#\\x{:02x};", character as u32)),
			}
		}
		try! (formatter.write_char ('"'));
		succeed! (());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
// FIXME:  Add immutability flag!
pub struct Bytes ( StdRc<StdVec<u8>> );


pub type BytesBox = StdBox<Bytes>;
pub type BytesVec = StdVec<Bytes>;


impl Bytes {
	
	pub fn values_as_slice (&self) -> (&[u8]) {
		self.0.as_ref () .as_slice ()
	}
	
	pub fn values_ref (&self) -> (&StdVec<u8>) {
		self.0.as_ref ()
	}
	
	pub fn values_clone (&self) -> (StdVec<u8>) {
		self.0.as_ref () .clone ()
	}
	
	pub fn values_is_empty (&self) -> (bool) {
		self.values_ref () .is_empty ()
	}
	
	pub fn values_is_not_empty (&self) -> (bool) {
		!self.values_ref () .is_empty ()
	}
	
	pub fn values_length (&self) -> (usize) {
		self.values_ref () .len ()
	}
	
}



impl fmt::Display for Bytes {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_str ("#u8("));
		let mut is_first = true;
		for byte in self.0.iter () {
			if !is_first {
				try! (formatter.write_char (' '));
			} else {
				is_first = false;
			}
			try! (write! (formatter, "{}", byte));
		}
		try! (formatter.write_char (')'));
		succeed! (());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
// FIXME:  Add immutability flag!
pub struct Pair ( StdRc<(Value, Value)> );


pub type PairBox = StdBox<Pair>;
pub type PairVec = StdVec<Pair>;


impl Pair {
	
	
	pub fn left (&self) -> (&Value) {
		&(self.0).0
	}
	
	pub fn right (&self) -> (&Value) {
		&(self.0).1
	}
	
	pub fn left_and_right (&self) -> (&Value, &Value) {
		(&(self.0).0, &(self.0).1)
	}
	
	
	pub fn is_self (&self, other : &Pair) -> (bool) {
		ptr::eq (self, other)
	}
	
}


impl fmt::Display for Pair {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_char ('('));
		let mut cursor = self;
		loop {
			let (left, right) = cursor.left_and_right ();
			try! (left.fmt (formatter));
			match *right {
				Value::Null (_, _) =>
					break,
				Value::Pair (_, ref right, _) => {
					try! (formatter.write_char (' '));
					cursor = right;
				},
				_ => {
					try! (formatter.write_char (' '));
					try! (formatter.write_char ('.'));
					try! (formatter.write_char (' '));
					try! (right.fmt (formatter));
					break;
				},
			}
			if self.is_self (cursor) {
				try! (formatter.write_char ('.'));
				try! (formatter.write_char (' '));
				try! (formatter.write_str ("#cyclic"));
				break;
			}
		}
		try! (formatter.write_char (')'));
		succeed! (());
	}
}




#[ derive (Clone, Debug, Eq, PartialEq, Hash) ]
// FIXME:  Add immutability flag!
pub struct Array ( StdRc<StdVec<Value>> );


pub type ArrayBox = StdBox<Array>;
pub type ArrayVec = StdVec<Array>;


impl Array {
	
	pub fn values_as_slice (&self) -> (&[Value]) {
		self.0.as_ref () .as_slice ()
	}
	
	pub fn values_ref (&self) -> (&StdVec<Value>) {
		self.0.as_ref ()
	}
	
	pub fn values_clone (&self) -> (StdVec<Value>) {
		self.0.as_ref () .clone ()
	}
	
	pub fn values_is_empty (&self) -> (bool) {
		self.values_ref () .is_empty ()
	}
	
	pub fn values_is_not_empty (&self) -> (bool) {
		!self.values_ref () .is_empty ()
	}
	
	pub fn values_length (&self) -> (usize) {
		self.values_ref () .len ()
	}
	
}


impl fmt::Display for Array {
	fn fmt (&self, formatter : &mut fmt::Formatter) -> (fmt::Result) {
		use std::fmt::Write;
		try! (formatter.write_str ("#("));
		let mut is_first = true;
		for element in self.0.iter () {
			if !is_first {
				try! (formatter.write_char (' '));
			} else {
				is_first = false;
			}
			try! (element.fmt (formatter));
		}
		try! (formatter.write_char (')'));
		succeed! (());
	}
}




pub fn boolean (value : bool) -> (Boolean) {
	Boolean (value)
}

pub fn number_i64 (value : i64) -> (NumberInteger) {
	NumberInteger (value)
}

pub fn number_f64 (value : f64) -> (NumberReal) {
	NumberReal (value)
}

pub fn character (value : char) -> (Character) {
	Character (value)
}




pub fn symbol_new (value : StdString) -> (Symbol) {
	Symbol (StdRc::new (value))
}

pub fn string_new (value : StdString) -> (String) {
	String (StdRc::new (value))
}




pub fn symbol_clone_str (value : &str) -> (Symbol) {
	symbol_new (StdString::from (value))
}

pub fn string_clone_str (value : &str) -> (String) {
	string_new (StdString::from (value))
}




pub fn symbol_clone_characters (characters : &[char]) -> (Symbol) {
	symbol_new (characters_clone (characters))
}

pub fn string_clone_characters (characters : &[char]) -> (String) {
	string_new (characters_clone (characters))
}

fn characters_clone (characters : &[char]) -> (StdString) {
	let mut value = StdString::with_capacity (characters.len ());
	for character in characters {
		value.push (*character);
	}
	StdString::from (value)
}




pub fn bytes_new (values : Vec<u8>) -> (Bytes) {
	Bytes (StdRc::new (values))
}

pub fn bytes_clone_slice (values : &[u8]) -> (Bytes) {
	bytes_new (values.to_vec ())
}




pub fn array_new (values : ValueVec) -> (Array) {
	Array (StdRc::new (values))
}

pub fn array_clone_slice (values : &[Value]) -> (Array) {
	array_new (values.to_vec ())
}




pub fn pair_new (left : Value, right : Value) -> (Pair) {
	Pair (StdRc::new ((left, right)))
}

