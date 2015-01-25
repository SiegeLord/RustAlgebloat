# RustAlgebloat

[![Build Status](https://travis-ci.org/SiegeLord/RustAlgebloat.png)](https://travis-ci.org/SiegeLord/RustAlgebloat)

Do you love template bloat? Do you think waiting for your program to compile is 
awesome? Do you think complicated error messages are the best thing since 
sliced bread? If so, this linear algebra library is just for you!

## Documentation

See [here](http://siegelord.github.io/RustAlgebloat/doc/algebloat/index.html)

## Example

Some basic operations and row access (no allocations except during the initial
matrix creation!):

~~~rust
let m = &mat![1.0, 2.0, 3.0;
              4.0, 5.0, 6.0;
              7.0, 8.0, 9.0];
println!("m =\n{}", m);
let t1 = m.t();
println!("t1 =\n{}", t1);
let r = m.row(0) + t1.row(0);
println!("r =\n{}", r);

let m2 = stack![m.view( ..2, ..2), m.view( ..2, 1..);
                m.view(1..,  ..2), m.view(1..,  1..)];
println!("m2 =\n{}", m2);
~~~

Output:

~~~
m =
⎡1 2 3⎤
⎢4 5 6⎥
⎣7 8 9⎦
t1 =
⎡1 4 7⎤
⎢2 5 8⎥
⎣3 6 9⎦
r =
[2 6 10]
m2 =
⎡1 2 2 3⎤
⎢4 5 5 6⎥
⎢4 5 5 6⎥
⎣7 8 8 9⎦
~~~

## Features

* *WIP!* This is very much incomplete... stay tuned!
* Expression templates (well, more like expression traits since this is Rust) 
assure all the caveats above while in principle providing allocation-free speed 
(only available with optimizations turned on)!
* Matrices
	* Elementwise operations (right-hand side can be a scalar)
		* Binary operators (`*/-+`)
		* Unary negation
		* Binary functions
			* `pow`
			* `atan2`
			* `hypot`
		* Unary functions
			* Trigonometry/exponential functions
			* `ceil`
			* `floor`
			* `ln`
			* `log10`
			* `sqrt`
		* Reductions
			* `min`
			* `max`
	* Row and column access
	* Views
	* Multiplication
	* Reshaping
	* Stacking
	* Flat views (view matrix as a vector)
		* Element access
		* Slicing

## Building

### Via Cargo:

Main library:
```
[dependencies.algebloat]

git = "https://github.com/SiegeLord/RustAlgebloat.git"
```

Useful macros:
```
[dependencies.algebloat_macros]

git = "https://github.com/SiegeLord/RustAlgebloat.git"
```

Examples:

* algebloat_examples

### Via CMake 2.8:

~~~
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=<your_prefix_goes_here>
make -j
make install
~~~
