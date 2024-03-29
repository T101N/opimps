# opimps

[![Version](https://img.shields.io/crates/v/opimps)](https://crates.io/crates/opimps)
[![Downloads](https://img.shields.io/crates/d/opimps)](https://crates.io/crates/opimps)
[![Issues](https://img.shields.io/github/issues/t101n/opimps)](https://github.com/T101N/opimps/issues)

**opimps** simplifies operator overloading for Rust so that it can be written in such a way that is similar to C++, but without the unnecessary duplication of code.

- [opimps](#opimps)
  - [Summary](#summary)
- [Usage](#usage)
  - [impl\_op](#impl_op)
  - [impl\_ops](#impl_ops)
  - [impl\_ops\_lprim and impl\_ops\_rprim](#impl_ops_lprim-and-impl_ops_rprim)
    - [impl\_ops\_lprim](#impl_ops_lprim)
    - [impl\_ops\_rprim](#impl_ops_rprim)
  - [impl\_uni\_op](#impl_uni_op)
  - [impl\_uni\_ops](#impl_uni_ops)
  - [impl\_op\_assign](#impl_op_assign)
  - [Generics](#generics)
- [A Realistic Example](#a-realistic-example)

## Summary
When overloading operators in Rust, we can run into design issues on whether the data should be `borrowed` or `owned`. For a good number of cases, we don't care about it and it should be up to the caller of the operator to decide what is appropriate to use.

In the example below, we overload the binary operator `+` so that it totals the cars in the two garages.

Imagine we had a garage that stores a number of cars.

```rust ignore
struct Garage {
    number_of_cars: u64
}
```
With `opimps`, we can overload operators so that we can do things like adding the number of cars between two garages.

```rust ignore
use core::ops::Add;

#[opimps::impl_ops(Add)]
fn add(self: Garage, rhs: Garage) -> u64 {
    self.number_of_cars + rhs.number_of_cars
}
```

The code generates the following code behind the scenes that we'd otherwise have to implement by hand if we wanted to allow combinations for owned and borrowed data.


```rust ignore
use core::ops::Add;

struct Garage {
    number_of_cars: u64
}

impl Add for Garage {
    type Output = u64;
    fn add(self, rhs: Garage) j-> Self::Output {
        self.number_of_cars + rhs.number_of_cars
    }
}

impl Add for &Garage {
    type Output = u64;
    fn add(self, rhs: Garage) j-> Self::Output {
        self.number_of_cars + rhs.number_of_cars
    }
}

impl Add<&Garage> for Garage {
    type Output = u64;
    fn add(self, rhs: &Garage) j-> Self::Output {
        self.number_of_cars + rhs.number_of_cars
    }
}

impl Add<&Garage> for &Garage {
    type Output = u64;
    fn add(self, rhs: &Garage) j-> Self::Output {
        self.number_of_cars + rhs.number_of_cars
    }
}

```

Notice that in the generated code, there are 4 implementations to represent all possible use cases when adding the number of cars in `Garages`, and the body of the function is essentially the same in all those cases. This is possible due to Rust's ability to automatically determine the level of propagation required to access members of a `structure`, unlike C++ where we need to be specific and use a combination of the dereferencing, dot operators and/or arrow operators depending on if the input is a referenced object or not. 

We can now use the operator for either `borrowed` and/or `owned` data in any order.

```rust ignore
let garage_a = Garage { number_of_cars: 4 };
let garage_b = Garage { number_of_cars: 9 };

let total = garage_a + garage_b;
let total = &garage_a + garage_b;
let total = garage_a + &garage_b;
let total = &garage_a + &garage_b;
```

> **\[NOTE!\]** *Keep in mind of Rust's hidden* `move` *semantics and that the code won't compile if we tried all of the* `total` *assignments at the same time. Non-referenced data are moved out of the scope once it's called, and will no longer be available in the scope it was originally created*.

> Official information on Rust's ownership of data can be found [here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), and [here](https://doc.rust-lang.org/beta/rust-by-example/scope/move.html).

> For those familiar with C++11 and above, you can read more from [here](https://en.cppreference.com/w/cpp/utility/move).


# Usage

## impl_op
In the summary, we introduced `impl_ops` which is a macro that generates code for borrowed and owned data. `impl_op` (notice the missing 's' at the end) is a way to overload operators the normal way without generating variations for borrowed data.

```rust ignore
#[opimps::impl_op(Add)]
fn add(self: Garage, rhs: Garage) -> u64 {
    self.number_of_cars + rhs.number_of_cars
}
```

This generates a 1:1 implementation as follows.

```rust ignore
impl Add for Garage {
    type Output = u64;
    fn add(self, rhs: Garage) -> u64 {
        self.number_of_cars + rhs.number_of_cars
    }
}
```

This means that we can only do the following and nothing more.

```rust ignore
let garage_a = Garage { number_of_cars: 4 };
let garage_b = Garage { number_of_cars: 9 };

let total = garage_a + garage_b;

assert_eq!(13, total);

/* Neither of the three lines below will work! */
// let total = &garage_a + garage_b;
// let total = garage_a + &garage_b;
// let total = &garage_a + &garage_b;
```
This by itself isn't very useful compared to `impl_ops` that we demonstrated in the example from the summary, but it allows us a way to fine-tune implementations based on our own design choices.

If we wanted to overload the operator where only the **left** side of the operator is a borrowed type, then we could implement it as follows.

```rust ignore
#[opimps::impl_op(Add)]
fn add(self: &Garage, rhs: Garage) -> u64 {
    self.number_of_cars + rhs.number_of_cars
}
```

This generates the following.

```rust ignore
impl Add for &Garage {
    type Output = u64;
    fn add(self, rhs: Garage) -> u64 {
        self.number_of_cars + rhs.number_of_cars
    }
}
```

We can now do `&garage_a + garage_b`.

```rust ignore
let garage_a = Garage { number_of_cars: 4 };
let garage_b = Garage { number_of_cars: 9 };

let total = &garage_a + garage_b;

assert_eq!(13, total);

/* Neither of the three lines below will work! */
// let total = garage_a + garage_b;
// let total = garage_a + &garage_b;
// let total = &garage_a + &garage_b;
```

Likewise, we can do other combinations of borrowed data with `impl_op` or even use different types.

```rust ignore
// borrowed right hand side
fn add(self: Garage, rhs: &Garage);

// borrowed both sides
fn add(self: &Garage, rhs: &Garage)

// Using a different type so that we can do something like `garage_a + 2`
fn add(self: Garage, rhs: u64)
```

## impl_ops
`impl_ops` uses `impl_op` under the hood to generate implementations of binary operators for combinations of borrowed and owned data.

```rust ignore
use core::ops::Mul;

struct A;
struct B;
struct C;

#[opimps::impl_ops(Mul)]
fn mul(self: A, rhs: B) -> C { ... }

```
The above would generate the following.

```rust ignore
impl Mul<B> for A { type Output = C; ... }
impl Mul<B> for &A { type Output = C; ... }
impl Mul<&B> for A { type Output = C; ... }
impl Mul<&B> for &A { type Output = C; ... }
```

## impl_ops_lprim and impl_ops_rprim
There are cases where we want to generate code for borrowed data but one of the elements are a primitive. This can and will cause issues if we were to use `impl_ops`. As such, `impl_ops_lprim` and `impl_ops_rprim` were created to work around such issues; representing left side primitive and right side primitive respectively.


### impl_ops_lprim
```rust ignore
#[opimps::impl_ops_lprim]
fn add(self: u64, rhs: Garage) -> u64 {
    ...
}
```

### impl_ops_rprim
```rust ignore
#[opimps::impl_ops_lprim]
fn add(self: Garage, rhs: u64) -> u64 {
    ...
}
```

## impl_uni_op
While `impl_op` implement for binary operators, `impl_uni_op` implements for unary operators.

```rust ignore
struct Person {
    has_cars: bool
}

#[opimps::impl_uni_op(core::ops::Not)]
fn not(self: Person) -> Person {
    Person { has_cars: !self.has_cars }
}
```

## impl_uni_ops
Much like how `impl_ops` generates implementations for borrowed and owned data for binary operators, `impl_uni_ops` generates implementations for borrowed and owned data for unary operators. Under the hood, the implementation of `impl_uni_ops` uses `impl_uni_op`.

Given the following `struct`:
```rust ignore
struct Person {
    has_cars: bool
}
```

Implementing the unary operator `!` for `Person` could be done like:

```rust ignore
use core::ops::Not;

#[opimps::impl_uni_ops(Not)]
fn not(self: Person) -> Person {
    Person { has_cars: !self.has_cars }
}
```

We should now be capable of doing the following:
```rust ignore
let a = Person { has_cars: true };

let res = !(&a);
let res = !a;
```

## impl_op_assign
We can implement assignment-based operators like `+=`, `*=`, `-=`.

```rust ignore
pub struct TestObj {
    pub val: i32
}

#[opimps::impl_ops_assign(std::ops::AddAssign)]
fn add_assign(self: TestObj, rhs: TestObj) {
   self.val += rhs.val;
}

let mut a = TestObj { val: 4 };
let b = TestObj { val: 7 };

a += b;
assert_eq!(11, a.val);

let mut a = TestObj { val: 4 };
let b = TestObj { val: 7 };
a += &b;

assert_eq!(11, a.val);
assert_eq!(7, b.val);
```

## Generics
We can use generics for `impl_ops` and `impl_uni_ops` much like how we use generics for standard functions.

```rust ignore
use std::ops::Add;

pub struct Num<T>(pub T);

/// ```
/// use opimps::impl_ops;
/// use mycrate::Num;
/// 
/// let a = Num(2.0);
/// let b = Num(3.0);
/// 
/// let res = a + b;
/// assert_eq!(5.0, res.0);
/// ```
#[opimps::impl_ops(Add)]
fn add<T>(self: Num<T>, rhs: Num<T>) -> Num<T> where T: Add<Output = T> + Copy {
    Num(self.0 + rhs.0)
}

```

# A Realistic Example
We've only shown useless examples so far, but that was because these were simplified so that it's easier to look at once you know how it works. The following is an example that makes use of [`SIMD`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#!=undefined) instructions for `x86_64` architecture, to compute quaternion multiplications. While it isn't the complete source code, this is just a snippet of how `opimps` is being used to implement a mathematical library.

```rust ignore
// No explanation of quaternions will be provided here since it involves a lot of theory. You only need to know that it's used to perform 3D rotations while avoiding the issues of gimbal locking that occurs from performing rotations using euler angles.

/// ```
/// use noname::v32::quat::Quat;
/// let l = Quat::<f32>::new(7.0, 1.0, 9.0, 4.0);
/// let mut r = Quat::<f32>::new(9.0, 4.0, 8.0, 2.0);
///
/// let res = &l * &r;
/// r.j = 5.0; r.k = 7.0;
/// 
/// let r = Quat::<f32>::new(9.0, 5.0, 7.0, 2.0);
/// 
/// let res = Quat::from(res);
///
/// assert_eq!(  22.0, res.i);
/// assert_eq!(  43.0, res.j);
/// assert_eq!(  69.0, res.k);
/// assert_eq!(-131.0, res.s);
/// 
/// let res = l * r;
/// let res = Quat::from(res);
/// 
/// assert_eq!(  12.0, res.i);
/// assert_eq!(  54.0, res.j);
/// assert_eq!(  72.0, res.k);
/// assert_eq!(-123.0, res.s);
/// ```
#[opimps::impl_ops(Mul)]
fn mul(self: Quat<i32>, rhs: Quat<i32>) -> Computable {
    let l = self.as_slice();
    let r = rhs.as_slice();

    let s = (&self.s * &rhs.s) - (&self).dot(rhs.clone());

    let v1 = Computable::from(l);
    let v2 = Computable::from(r);

    let s1 = Computable::all((&self).s);
    let s2 = Computable::all((&rhs).s);

    let s1v2 = s1 * v2;
    let s2v1 = s2 * v1;

    let v1xv2 = self.cross(rhs);
    
    let mut res = s1v2 + s2v1 + v1xv2;
    
    unsafe { crate::insert_i32!(res, s, 3) };
    return res;
}

```