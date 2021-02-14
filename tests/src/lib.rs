#[cfg(test)]
mod tests {
    struct Dummy<T>(T);
    #[test]
    fn test_generics_impl_ops() {
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
    fn test_generics_impl_uni_ops() {
        use core::ops::Neg;

        #[opimps::impl_uni_ops(Neg)]
        fn neg<T: Neg<Output = T> + Copy>(self: Dummy<T>) -> Dummy<T> {
            Dummy(-self.0)
        }

        let a = -Dummy(4.0);        
        assert_eq!(-4.0, a.0);
    }
}
