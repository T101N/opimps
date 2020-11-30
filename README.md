# opimps

**opimps** simplifies operator overloading for Rust so that it can be written in such a way that is similar to C++, but without the unnecessary duplication of code.

- [Usage](#usage)

    * [impl_op](#impl_op)
    * [impl_ops](#impl_ops)
    * [impl_ops_lprim](#impl_ops_lprim)
    * [impl_ops_rprim](#impl_ops_rprim)
    * [impl_uni_op](#impl_uni_op)
    * [impl_uni_ops](#impl_uni_ops)
- [A Realistic Example](#a-realistic-example)

## Summary
When overloading operators in Rust, we can run into design issues on whether the data should be `borrowed` or `owned`. For a good number of cases, we don't care about it and it should be up to the caller of the operator to decide what is appropriate to use.

In the example below, we overload the binary operator `+` so that it totals the cars in the two garages.

Imagine we had a garage that stores a number of cars.

```rust
struct Garage {
    number_of_cars: u64
}
```
With `opimps`, we can overload operators so that we can do things like adding the number of cars between two garages.

```rust
use core::ops::Add;

#[opimps::impl_ops(Add)]
fn add(self: Garage, rhs: Garage) -> u64 {
    self.number_of_cars + rhs.number_of_cars
}
```

The code generates the following code behind the scenes that we'd otherwise have to implement by hand if we wanted to allow combinations for owned and borrowed data.


```rust
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

```rust
let garage_a = Garage { number_of_cars: 4 };
let garage_b = Garage { number_of_cars: 9 };

let total = garage_a + garage_b;
let total = &garage_a + garage_b;
let total = garage_a + &garage_b;
let total = &garage_a + &garage_b;
```

> **[NOTE!]** *Keep in mind of Rust's hidden* `move` *semantics and that the code won't compile if we tried all of the* `total` *assignments at the same time. Non-referenced data are moved out of the scope once it's called, and will no longer be available in the scope it was originally created*.

For those familiar with C++11 and above, you can read more from [here](https://en.cppreference.com/w/cpp/utility/move).

Official information on Rust's ownership of data can be found [here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), and [here](https://doc.rust-lang.org/beta/rust-by-example/scope/move.html).

# Usage

## impl_op
In the summary, we introduced `impl_ops` which is a macro that generates code for borrowed and owned data. `impl_op` (notice the missing 's' at the end) is a way to overload operators the normal way without generating variations for borrowed data.

```rust
#[opimps::impl_op(Add)]
fn add(self: Garage, rhs: Garage) -> u64 {
    self.number_of_cars + rhs.number_of_cars
}
```

This generates a 1:1 implementation as follows.

```rust
impl Add for Garage {
    type Output = u64;
    fn add(self, rhs: Garage) -> u64 {
        self.number_of_cars + rhs.number_of_cars
    }
}
```

This means that we can only do the following and nothing more.

```rust
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

```rust
#[opimps::impl_op(Add)]
fn add(self: &Garage, rhs: Garage) -> u64 {
    self.number_of_cars + rhs.number_of_cars
}
```

This generates the following.

```rust
impl Add for &Garage {
    type Output = u64;
    fn add(self, rhs: Garage) -> u64 {
        self.number_of_cars + rhs.number_of_cars
    }
}
```

We can now do `&garage_a + garage_b`.

```rust
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

```rust
// borrowed right hand side
fn add(self: Garage, rhs: &Garage);

// borrowed both sides
fn add(self: &Garage, rhs: &Garage)

// Using a different type so that we can do something like `garage_a + 2`
fn add(self: Garage, rhs: i64)
```

## impl_ops
> todo

## impl_ops_lprim and impl_ops_rprim
There are cases where we want to generate code for borrowed data but one of the elements are a primitive. This can and will cause issues if we were to use `impl_ops`. As such, `impl_ops_lprim` and `impl_ops_rprim` were created to work around such issues; representing left side primitvie and right side primitive respectively.


### impl_ops_lprim
```rust
#[opimps::impl_ops_lprim]
fn add(self: i64, rhs: Garage) -> i64 {
    ...
}
```

### impl_ops_rprim
```rust
#[opimps::impl_ops_lprim]
fn add(self: Garage, rhs: i64) -> i64 {
    ...
}
```

## impl_uni_op
While `impl_op` implement for binary operators, `impl_uni_op` implements for unary operators.

```rust
struct Person {
    has_cars: bool
}

#[opimps::impl_uni_op(core::ops::Not)]
fn not(self: Person) -> Person {
    Person { has_cars: !self.has_cars }
}
```

## impl_uni_ops
> todo

# A Realistic Example
We've only shown useless examples so far, but that was because these were simplified so that it's easier to look at once you know how it works. The following is an example that makes use of [`SIMD`](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#!=undefined) instructions for `x86_64` architecture, to compute quaternion multiplications. While it isn't the complete source code, this is just a snippet of how `opimps` is being used to implement a mathematical library.

```rust
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