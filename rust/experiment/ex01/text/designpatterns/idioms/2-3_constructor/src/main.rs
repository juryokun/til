#![allow(unused)]
fn main() {
    /// Time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let s = Second::new(42);
    /// assert_eq!(42, s.value());
    /// ```
    pub struct Second {
        value: u64,
    }

    impl Second {
        // Constructs a new instance of [`Second`].
        // Note this is an associated function - no self.
        pub fn new(value: u64) -> Self {
            Self { value }
        }

        /// Returns the value in seconds.
        pub fn value(&self) -> u64 {
            self.value
        }
    }

    /// Time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let s = Second::default();
    /// assert_eq!(0, s.value());
    /// ```
    pub struct Third {
        value: u64,
    }

    impl Third {
        /// Returns the value in seconds.
        pub fn value(&self) -> u64 {
            self.value
        }
    }

    impl Default for Third {
        fn default() -> Self {
            Self { value: 0 }
        }
    }

    /// Time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let s = Second::default();
    /// assert_eq!(0, s.value());
    /// ```
    #[derive(Default)]
    pub struct Fourth {
        value: u64,
    }

    impl Fourth {
        /// Returns the value in seconds.
        pub fn value(&self) -> u64 {
            self.value
        }
    }

    let s = Second::new(42);
    assert_eq!(42, s.value());

    let t = Third::default();
    assert_eq!(0, t.value());

    let f = Fourth::default();
    assert_eq!(0, f.value());
}
