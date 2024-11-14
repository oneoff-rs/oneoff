/// One off Left or Right type
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum OneOff<T> {
    Left(T),
    Right(T),
}

impl<T> OneOff<T> {
    /// Returns the left value if it exists
    ///
    /// # Examples
    ///
    /// ```
    /// use oneoff::OneOff;
    /// let left = OneOff::Left(1);
    /// let right = OneOff::Right(2);
    /// assert_eq!(left.left(), Some(1));
    /// assert_eq!(right.left(), None);
    /// ```
    pub fn left(self) -> Option<T> {
        match self {
            OneOff::Left(t) => Some(t),
            _ => None,
        }
    }

    /// Returns the right value if it exists
    ///
    /// # Examples
    ///
    /// ```
    /// use oneoff::OneOff;
    /// let left = OneOff::Left(1);
    /// let right = OneOff::Right(2);
    /// assert_eq!(left.right(), None);
    /// assert_eq!(right.right(), Some(2));
    /// ```
    pub fn right(self) -> Option<T> {
        match self {
            OneOff::Right(t) => Some(t),
            _ => None,
        }
    }

    /// returns true if the value is a left
    ///
    /// # Examples
    ///
    /// ```
    /// use oneoff::OneOff;
    /// let left = OneOff::Left(1);
    /// let right = OneOff::Right(2);
    /// assert!(left.is_left());
    /// assert!(!right.is_left());
    /// ```
    pub fn is_left(&self) -> bool {
        matches!(self, OneOff::Left(_))
    }

    /// returns true if the value is a right
    ///
    /// # Examples
    /// ```
    /// use oneoff::OneOff;
    /// let left = OneOff::Left(1);
    /// let right = OneOff::Right(2);
    /// assert!(!left.is_right());
    /// assert!(right.is_right());
    /// ```
    pub fn is_right(&self) -> bool {
        matches!(self, OneOff::Right(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oneoff() {
        let left = OneOff::Left(1);
        let right = OneOff::Right(2);

        assert_eq!(left, OneOff::Left(1));
        assert_eq!(right, OneOff::Right(2));

        assert_eq!(left.cmp(&right), std::cmp::Ordering::Less);
        assert_eq!(right.cmp(&left), std::cmp::Ordering::Greater);
    }
}
