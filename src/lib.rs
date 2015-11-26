use std::str::FromStr;

pub trait TryFrom<T>: Sized {
    type Err;
    fn try_from(T) -> Result<Self, Self::Err>;
}

pub trait TryInto<T>: Sized {
    type Err;
    fn try_into(self) -> Result<T, Self::Err>;
}

impl<T, U> TryInto<U> for T where U: TryFrom<T> {
    type Err = U::Err;
    fn try_into(self) -> Result<U, U::Err> {
        U::try_from(self)
    }
}

impl<'a, T> TryFrom<&'a str> for T where T: FromStr {
    type Err = T::Err;
    fn try_from(string: &'a str) -> Result<Self, Self::Err> {
        T::from_str(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num;

    #[test]
    fn should_have_try_from_impl_for_from_str() {
        let result = u32::try_from("3");
        assert_eq!(result.unwrap(), 3)
    }

    #[test]
    fn should_have_try_into_impl_for_from_str() {
        let result: Result<u32, num::ParseIntError> = "3".try_into();
        assert_eq!(result.unwrap(), 3)
    }
}
