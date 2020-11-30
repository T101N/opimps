# opimps

**opimps** simplifies operator overloading for Rust so that it can be written in such a way that is similar to C++, but without the unnecessary duplication of code.

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

The code generates the following below behind the scenes that we'd otherwise have to implement by hand if we wanted to allow combinations for owned and borrowed data.


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

Notice that in the generated code, there are 4 implementations to represent all possible use cases when adding the number of cars in `Garages`, and the body of the function is essentially the same in all those cases. This is possible due to Rust's ability to automatically determine the level of propagation required to access members of a `structure`, unlike C++ where we need to be specific and use a combination of the dot operator or arrow operator depending on if the input is a referenced object or not. 

We can now use the operator for referenced (`borrowed`) and un-referenced (`owned`) data in any order.

```rust
let garage_a = Garage { number_of_cars: 4 };
let garage_b = Garage { number_of_cars: 9 };

let total = garage_a + garage_b;
let total = &garage_a + garage_b;
let total = garage_a + &garage_b;
let total = &garage_a + &garage_b;
```

**[NOTE!]** *Keep in mind of Rust's hidden* `move` *semantics and that the code won't compile if we tried all of the* `total` *assignments at the same time. Non-referenced data are moved out of the scope once it's called, and will no longer be available in the scope it was originally created*.

For those familiar with C++11 and above, you can read more from [here](https://en.cppreference.com/w/cpp/utility/move).

Official information on how Rust's ownership of data can be found [here](https://doc.rust-lang.org/beta/rust-by-example/scope/move.html)
