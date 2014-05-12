//~ // This file is released into Public Domain.
//~ #![feature(globs, macro_rules, phase)]
//~ 
//~ #[phase(syntax)]
//~ extern crate algebloat_macros;
//~ extern crate algebloat;
//~ use algebloat::*;
//~ 
fn main()
{
	//~ let a = vec!(1.0, 2.0, 3.0);
	//~ a.set(0, 10.0);
	//~ let b = vec!(1.0, 2.0, 3.0);
	//~ 
	//~ let d = (&a + &b - &b).slice(1, 3).slice(0, 1).to_vec();
	//~ let sa = a.slice(1, 3);
	//~ sa.set(0, 10.0);
	//~ let sb1 = b.slice(1, 3);
	//~ let sb2 = b.slice(2, 3);
	//~ 
	//~ let e = (a.slice(1, 3) + b.slice(1, 3)).to_vec();
	//~ println!("Vectors");
	//~ println!("{}", a);
	//~ println!("{}", sb1);
	//~ println!("{}", sb2);
	//~ println!("{}", d);
	//~ println!("{}", e);
	//~ 
	//~ let m = mat!(1.0, 2.0, 3.0;
	             //~ 4.0, 5.0, 6.0;
	             //~ 7.0, 8.0, 9.0);
	//~ let t1 = m.t();
	//~ let t2 = t1.t();
	//~ println!("Matrix");
	//~ println!("{}", m);
	//~ println!("r {}\n", m.row(0));
	//~ println!("{}", t1);
	//~ println!("r {}\n", t1.row(0));
	//~ println!("{}", t2);
	//~ println!("r {}", t2.row(0));
//~ 
	//~ let m = mat!(1.0, 2.0, 3.0;
				 //~ 4.0, 5.0, 6.0;
				 //~ 7.0, 8.0, 9.0);
	//~ println!("m =\n{}", m);
	//~ let t1 = m.t();
	//~ println!("t1 =\n{}", t1);
	//~ let r = m.row(0) + t1.row(0);
	//~ println!("r =\n{}", r);
}
