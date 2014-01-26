# RustAlgebloat

[![Build Status](https://travis-ci.org/SiegeLord/RustAlgebloat.png)](https://travis-ci.org/SiegeLord/RustAlgebloat)

Do you love template bloat? Do you think waiting for your program to compile is 
awesome? Do you think complicated error messages are the best thing since 
sliced bread? If so, this linear algebra library is just for you!

## Documentation

http://www.rust-ci.org/SiegeLord/RustAlgebloat/doc/algebloat/

## Example

Some basic operations and row access (no allocations except in `Matrix::new`!):

~~~rust
let m = Matrix::new([&[1.0, 2.0, 3.0],
                     &[4.0, 5.0, 6.0],
                     &[7.0, 8.0, 9.0]]);
println!("m =\n{}", m);
let t1 = m.t();
println!("t1 =\n{}", t1);
let r = m.row(0) + t1.row(0);
println!("r =\n{}", r);
~~~

Output:

~~~
m =
│1 2 3│
│4 5 6│
│7 8 9│
t1 =
│1 4 7│
│2 5 8│
│3 6 9│
r =
[2 6 10]
~~~

## Features

* *WIP!* This is very much incomplete... stay tuned!
* Expression templates (well, more like expression traits since this is Rust) 
assure all the caveats above while in principle providing allocation-free speed 
(only available with optimizations turned on)!
* Slices and views, no linear algebra library should come without them.
* And more... (in the future).

## Building

~~~
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=<your_prefix_goes_here>
make -j
make install
~~~
