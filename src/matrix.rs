use std::vec;
use std::fmt;
use std::io::Writer;
use vector::{VectorGet, write_vec};

pub trait MatrixGet
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32;
	fn get(&self, r: uint, c: uint) -> f32;
}

pub trait MatrixShape
{
	fn ncol(&self) -> uint;
	fn nrow(&self) -> uint;
}

pub trait MatrixTranspose
{
	fn t(self) -> Transposer<Self>;
}

pub trait MatrixRowAccess
{
	unsafe fn unsafe_row(self, r: uint) -> RowAccessor<Self>;
	fn row(self, r: uint) -> RowAccessor<Self>;
}

pub struct Matrix
{
	priv data: ~[f32],
	priv nrow: uint,
	priv ncol: uint
}

impl Matrix
{
	pub fn new(data: &[&[f32]]) -> Matrix
	{
		let nrow = data.len();
		let ncol = data[0].len();
		let mut mat_data = vec::with_capacity(nrow * ncol);
		for &row in data.iter()
		{
			assert!(row.len() == ncol);
			mat_data = vec::append(mat_data, row);
		}
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}
}

impl<'l>
MatrixGet for
&'l Matrix
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32
	{
		*self.data.unsafe_ref(c + r * self.ncol)
	}

	fn get(&self, r: uint, c: uint) -> f32
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		unsafe
		{
			self.unsafe_get(r, c)
		}
	}
}

impl<'l>
MatrixShape for
&'l Matrix
{
	fn ncol(&self) -> uint
	{
		self.nrow
	}

	fn nrow(&self) -> uint
	{
		self.ncol
	}
}

impl<'l>
MatrixTranspose for
&'l Matrix
{
	fn t(self) -> Transposer<&'l Matrix>
	{
		Transposer{ base: self }
	}
}

impl<'l>
MatrixRowAccess for
&'l Matrix
{
	unsafe fn unsafe_row(self, r: uint) -> RowAccessor<&'l Matrix>
	{
		RowAccessor::unsafe_new(self, r)
	}
	
	fn row(self, r: uint) -> RowAccessor<&'l Matrix>
	{
		RowAccessor::new(self, r)
	}
}

pub struct Transposer<T>
{
	base: T
}

impl<'l,
     T: MatrixGet>
MatrixGet for
Transposer<T>
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32
	{
		self.base.unsafe_get(c, r)
	}

	fn get(&self, r: uint, c: uint) -> f32
	{
		self.base.get(c, r)
	}
}

impl<'l,
     T: MatrixShape>
MatrixShape for
Transposer<T>
{
	fn ncol(&self) -> uint
	{
		self.base.nrow()
	}

	fn nrow(&self) -> uint
	{
		self.base.ncol()
	}
}

impl<'l,
     T>
MatrixTranspose for
Transposer<T>
{
	fn t(self) -> Transposer<Transposer<T>>
	{
		Transposer{ base: self }
	}
}

impl<'l,
     T: MatrixShape>
MatrixRowAccess for
Transposer<T>
{
	unsafe fn unsafe_row(self, r: uint) -> RowAccessor<Transposer<T>>
	{
		RowAccessor::unsafe_new(self, r)
	}
	
	fn row(self, r: uint) -> RowAccessor<Transposer<T>>
	{
		RowAccessor::new(self, r)
	}
}

pub struct RowAccessor<T>
{
	base: T,
	row: uint
}

impl<T: MatrixShape>
RowAccessor<T>
{
	pub unsafe fn unsafe_new(base: T, row: uint) -> RowAccessor<T>
	{
		RowAccessor{ base: base, row: row }
	}
	
	pub fn new(base: T, row: uint) -> RowAccessor<T>
	{
		assert!(row < base.nrow());
		RowAccessor{ base: base, row: row }
	}
}

impl<'l,
     T: MatrixGet + MatrixShape>
VectorGet for
RowAccessor<T>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		self.base.unsafe_get(self.row, idx)
	}

	fn get(&self, idx: uint) -> f32
	{
		assert!(idx < self.base.ncol());
		unsafe
		{
			self.base.unsafe_get(self.row, idx)
		}
	}
}

impl<T: MatrixShape>
Container for
RowAccessor<T>
{
	fn len(&self) -> uint
	{
		self.base.ncol()
	}
}

impl<T: MatrixShape + MatrixGet>
fmt::Default for
RowAccessor<T>
{
	fn fmt(v: &RowAccessor<T>, buf: &mut fmt::Formatter)
	{
		write_vec(buf.buf, v);
	}
}

impl<T: Clone>
Clone for
RowAccessor<T>
{
	fn clone(&self) -> RowAccessor<T>
	{
		RowAccessor{ base: self.base.clone(), row: self.row }
	}
}

pub fn write_mat<T: MatrixGet + MatrixShape>(w: &mut Writer, a: &T)
{
	for r in range(0, a.nrow())
	{
		let mut first = true;
		write!(w, "│");
		for c in range(0, a.ncol())
		{
			if !first
			{
				write!(w, " ");
			}
			first = false;
			unsafe
			{
				write!(w, "{}", a.unsafe_get(r, c));
			}
		}
		write!(w, "│");
		if r + 1 < a.nrow()
		{
			writeln!(w, "");
		}
	}
}

impl
fmt::Default for
Matrix
{
	fn fmt(v: &Matrix, buf: &mut fmt::Formatter)
	{
		write_mat(buf.buf, &v);
	}
}

impl<T: MatrixGet + MatrixShape>
fmt::Default for
Transposer<T>
{
	fn fmt(v: &Transposer<T>, buf: &mut fmt::Formatter)
	{
		write_mat(buf.buf, v);
	}
} 

impl<T: Clone>
Clone for
Transposer<T>
{
	fn clone(&self) -> Transposer<T>
	{
		Transposer{ base: self.base.clone() }
	}
}
