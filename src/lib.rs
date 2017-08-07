#![cfg_attr(feature = "nightly", feature(i128_type))]

#[cfg(feature = "extprim")]
extern crate extprim;

#[cfg(feature = "extprim_literals")]
#[macro_use] extern crate extprim_literals;

/// A trait for all Fowler-Noll-Vo hash implementations.
///
/// This matches the `std::hash::Hasher` definition but for multiple hash
/// types.
pub trait FnvHasher {

    /// The type of the hash.
    type Hash;

    /// Completes a round of hashing, producing the output hash generated.
    fn finish(&self) -> Self::Hash;

    /// Writes some data into this Hasher.
    fn write(&mut self, bytes: &[u8]);
}

/// The FNV-0 hash.
///
/// This is deprecated except for computing the FNV offset basis for FNV-1 and
/// FNV-1a hashes.
#[derive(Debug, Default)]
pub struct Fnv0<T>{
    hash: T
}

/// The FNV-1 hash.
#[derive(Debug)]
pub struct Fnv1<T> {
    hash: T
}

/// The FNV-1a hash.
#[derive(Debug)]
pub struct Fnv1a<T> {
    hash: T
}

impl<T : Default> Fnv0<T> {
    /// Creates a new `Fnv0<T>`.
    ///
    /// ```
    /// use lz_fnv::Fnv0;
    ///
    /// let fnv_hasher = Fnv0::<u32>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Fnv0<T> {
    /// Creates a new `Fnv0<T>` with the specified key.
    ///
    /// ```
    /// use lz_fnv::Fnv0;
    ///
    /// let fnv_hasher = Fnv0::with_key(872u32);
    /// ```
    pub fn with_key(key: T) -> Self {
        Self {
            hash: key
        }
    }
}

impl<T> Fnv1<T> {
    /// Creates a new `Fnv1<T>` with the specified key.
    ///
    /// ```
    /// use lz_fnv::Fnv1;
    ///
    /// let fnv_hasher = Fnv1::with_key(872u32);
    /// ```
    pub fn with_key(key: T) -> Self {
        Self {
            hash: key
        }
    }
}

impl<T> Fnv1a<T> {
    /// Creates a new `Fnv1a<T>` with the specified key.
    ///
    /// ```
    /// use lz_fnv::Fnv1a;
    ///
    /// let fnv_hasher = Fnv1a::with_key(872u32);
    /// ```
    pub fn with_key(key: T) -> Self {
        Self {
            hash: key
        }
    }
}

macro_rules! fnv0_impl {
    ($type: ty, $prime: expr, $from_byte: expr) => {
        impl FnvHasher for Fnv0<$type> {
            type Hash = $type;

            fn finish(&self) -> Self::Hash {
                self.hash
            }

            fn write(&mut self, bytes: &[u8]) {
                let mut hash = self.hash;

                for byte in bytes {
                    hash = hash.wrapping_mul($prime);
                    hash ^= ($from_byte)(*byte);
                }

                self.hash = hash;
            }
        }
    }
}

macro_rules! fnv1_impl { 
    ($type: ty, $offset: expr, $prime: expr, $from_byte: expr) => {
        impl Default for Fnv1<$type> {
            fn default() -> Self {
                Self {
                    hash: $offset
                }
            }
        }

        impl Fnv1<$type> {
            /// Creates a new `Fnv1<T>`.
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl FnvHasher for Fnv1<$type> {
            type Hash = $type;

            fn finish(&self) -> Self::Hash {
                self.hash
            }

            fn write(&mut self, bytes: &[u8]) {
                let mut hash = self.hash;

                for byte in bytes {
                    hash = hash.wrapping_mul($prime);
                    hash ^= ($from_byte)(*byte);
                }

                self.hash = hash;
            }
        }        
    }
}

macro_rules! fnv1a_impl { 
    ($type: ty, $offset: expr, $prime: expr, $from_byte: expr) => {
        impl Default for Fnv1a<$type> {
            fn default() -> Self {
                Self {
                    hash: $offset
                }
            }
        }

        impl Fnv1a<$type> {
            /// Creates a new `Fnv1a<T>`.
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl FnvHasher for Fnv1a<$type> {
            type Hash = $type;

            fn finish(&self) -> Self::Hash {
                self.hash
            }

            fn write(&mut self, bytes: &[u8]) {
                let mut hash = self.hash;

                for byte in bytes {
                    hash ^= ($from_byte)(*byte);
                    hash = hash.wrapping_mul($prime);
                }

                self.hash = hash;
            }
        }
        
    }
}

macro_rules! fnv_hasher_impl {
    ($type: ty) => {
        impl ::std::hash::Hasher for $type {
            fn finish(&self) -> u64 {
                ::FnvHasher::finish(self)
            }

            fn write(&mut self, bytes: &[u8]) {
                ::FnvHasher::write(self, bytes);
            }
        }
    }
}
macro_rules! fnv_impl {  
    (u64, $offset: expr, $prime: expr, $from_byte: expr) => {        
        fnv0_impl!(u64, $prime, $from_byte);
        fnv_hasher_impl!(Fnv0<u64>);

        fnv1_impl!(u64, $offset, $prime, $from_byte);
        fnv_hasher_impl!(Fnv1<u64>);

        fnv1a_impl!(u64, $offset, $prime, $from_byte); 
        fnv_hasher_impl!(Fnv1a<u64>);
    };
    ($type: ty, $offset: expr, $prime: expr, $from_byte: expr) => {
        fnv0_impl!($type, $prime, $from_byte);
        fnv1_impl!($type, $offset, $prime, $from_byte);
        fnv1a_impl!($type, $offset, $prime, $from_byte);
    };
}

fnv_impl!(u32, 0x811c9dc5, 0x1000193, |byte| byte as u32);
fnv_impl!(u64, 0xcbf29ce484222325, 0x100000001B3, |byte| byte as u64);

#[cfg(feature = "u128")]
fnv_impl!(extprim::u128::u128, u128!(0x6C62272E07BB014262B821756295C58D), u128!(0x0000000001000000000000000000013B), |byte| extprim::u128::u128::new(byte as u64));

#[cfg(feature = "nightly")]
fnv_impl!(u128, 0x6C62272E07BB014262B821756295C58Du128, 0x0000000001000000000000000000013Bu128, |byte| byte as u128);

#[cfg(test)]
mod tests {
    use {Fnv0, Fnv1a, FnvHasher};

    #[cfg(feature = "u128")]
    use extprim::u128::u128;

    #[test]
    fn fnv0_32_prime_calculation() {
        let mut fnv0 = Fnv0::<u32>::new();

        fnv0.write(b"chongo <Landon Curt Noll> /\\../\\");

        let result = fnv0.finish();

        assert_eq!(result, 0x811c9dc5);
    }

    #[test]
    fn fnv0_64_prime_calculation() {
        let mut fnv0 = Fnv0::<u64>::new();

        fnv0.write(b"chongo <Landon Curt Noll> /\\../\\");

        let result = fnv0.finish();

        assert_eq!(result, 0xcbf29ce484222325);
    }

    #[cfg(feature = "u128")]
    #[test]
    fn empty_hash() {
        let fnv128a = Fnv1a::<u128>::default();

        let hash = fnv128a.finish();

        assert_eq!(hash, u128!(0x6C62272E07BB014262B821756295C58D));
    }

    #[cfg(feature = "u128")]
    #[test]
    fn test_hash() {
        let mut fnv128a = Fnv1a::<u128>::default();
        fnv128a.write(b"foobar");

        let hash = fnv128a.finish();

        assert_eq!(hash, u128!(0x343e1662793c64bf6f0d3597ba446f18));
    }
}
