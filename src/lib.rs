use std::ascii::AsciiExt;

pub trait FQDN {
    fn is_valid_fqdn(&self) -> bool;
}

impl FQDN for str {
    /// Check that the string is a
    /// valid fully qualified domain
    /// name
    ///
    /// The requirements are that every
    /// label must be less than 62 bytes
    /// and the overall string length less
    /// than 255 bytes
    ///
    /// A label is one byte for the bytes
    /// count, 62 for the bytes and a null
    /// terminator
    ///
    /// The entire string is one byte for
    /// the first label. 253 bytes and a
    /// null terminator
    fn is_valid_fqdn(&self) -> bool {
        if !self.is_ascii() {
            return false;
        }
        if 0 < self.len() && self.len() < 253{
            self.split('.').all(|label| label.len() < 62)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use FQDN;
    #[test]
    fn reject_empty_string() {
        assert_eq!(String::from("").is_valid_fqdn(), false);
    }
    #[test]
    fn reject_very_long_label() {
        assert_eq!(String::from("thisisaverylonglabelandshouldnotbeacceptedIreallyhopesthisworks.").is_valid_fqdn(), false);
    }
    #[test]
    fn a_valid_fqdn() {
        assert!(String::from("this.is.a.valid.fqdn").is_valid_fqdn());
    }
    #[test]
    fn a_valid_absolute_fqdn() {
        assert!(String::from("this.is.a.valid.absolute.fqdn.").is_valid_fqdn());
    }
    #[test]
    fn not_ascii() {
        assert_eq!(String::from("ü").is_valid_fqdn(), false);
    }
}
