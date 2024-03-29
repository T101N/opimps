#[cfg(test)]
mod tests {
    struct Dummy<T>(T);
    #[test]
    fn test_impl_ops_generics() {
        use core::ops::Add;

        #[opimps::impl_ops(Add)]
        fn add<T: Add<Output = T> + Copy>(self: Dummy<T>, rhs: Dummy<T>) -> Dummy<T> {
            Dummy(self.0 + rhs.0)
        }

        let a = Dummy(4.0);
        let res = &a + &a;
        
        assert_eq!(4.0, a.0);
        assert_eq!(8.0, res.0);
    }

    #[test]
    fn test_impl_ops_generics_with_where_clause() {
        use core::ops::Sub;

        #[opimps::impl_ops(Sub)]
        fn sub<T>(self: Dummy<T>, rhs: Dummy<T>) -> Dummy<T> where T: Sub<Output = T> + Copy {
            Dummy(self.0 - rhs.0)
        }

        let a = Dummy(6.0);
        let b = Dummy(2.0);
        let res = &a - &b;
        
        assert_eq!(6.0, a.0);
        assert_eq!(2.0, b.0);
        assert_eq!(4.0, res.0);
    }

    #[test]
    fn test_generics_impl_uni_ops() {
        use core::ops::Neg;
        
        #[opimps::impl_uni_ops(Neg)]
        fn neg<T: Neg<Output = T> + Copy>(self: Dummy<T>) -> Dummy<T> {
            Dummy(-self.0)
        }
        
        let a = -Dummy(4.0);        
        assert_eq!(-4.0, a.0);
    }

    #[test]
    fn test_generics_with_lifetime() {
        use std::ops::Add;

        struct Num<'a, T> {
            val: &'a mut T
        }

        #[opimps::impl_ops(Add)]
        fn add<'a, T>(self: Num<'a, T>, rhs: Num<'a, T>) -> T where T: Add<Output = T> + Copy {
            *self.val + *rhs.val
        }

        let mut a = 5;
        let mut b = 7;

        let a_num = Num { val: &mut a };
        let b_num = Num { val: &mut b };

        let result = a_num + b_num;

        assert_eq!(12, result);
    }

    #[test]
    fn test_generics_with_different_rhs() {
        use std::ops::Add;

        struct Num<'a, T> {
            val: &'a mut T
        }

        #[opimps::impl_ops(Add)]
        fn add<'a>(self: Num<'a, i64>, rhs: i64) -> i64 {
            *self.val + rhs
        }

        let mut a = 5;
        let b = 7;

        let a_num = Num { val: &mut a };

        let result = a_num + b;

        assert_eq!(12, result);
    }

    #[test]
    fn doc_test_generics() {
        use opimps::impl_ops;
        use std::ops::Add;

        struct Num<T>(pub T);

        #[impl_ops(Add)]
        fn add<T>(self: Num<T>, rhs: Num<T>) -> Num<T> where T: Add<Output = T> + Copy {
            Num(self.0 + rhs.0)
        }


        let a = Num(2.0);
        let b = Num(3.0);
        let res = a + b;
        assert_eq!(5.0, res.0);
    }
}
