mod test {
    pub(crate) mod main {
        pub(crate) mod v1 {
            include!(concat!(env!("OUT_DIR"), "/test.main.v1.rs"));
        }
    }

    pub(crate) mod other {
        pub(crate) mod v1 {
            include!(concat!(env!("OUT_DIR"), "/test.other.v1.rs"));
        }
    }

    pub(crate) mod third {
        pub(crate) mod v1 {
            include!(concat!(env!("OUT_DIR"), "/test.third.v1.rs"));
        }
    }
}
