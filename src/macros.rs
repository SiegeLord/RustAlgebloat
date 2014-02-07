#[macro_export]
macro_rules! vec
{
	( $($e: expr),+) =>
	{
		Vector::new([$(
				($e) as f32,
		)+])
	}
}

#[macro_export]
macro_rules! mat
{
	( $($($e: expr),+);+ ) =>
	{
		Matrix::new([$(
			&[$(
				($e) as f32,
			)+],
		)+])
	}
}
