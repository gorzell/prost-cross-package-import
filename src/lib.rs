mod test {
    mod v1 {
        include!(concat!(env!("OUT_DIR"), "/test.main.v1.rs"));
    }

    mod other {
        mod v1 {
            include!(concat!(env!("OUT_DIR"), "/test.other.v1.rs"));
        }
    }

    mod third {
        mod v1 {
            include!(concat!(env!("OUT_DIR"), "/test.third.v1.rs"));
        }
    }
}
