mod set1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_test1() {
        let s = String::from("9e9938f1ba8724cd8b41e4892911789df2f1fe88516819b0e7136f033dd6146040ba7bfd981bb7310231793abb38d4f1a7ffb88e76c6ad785b5eb7b9996f7003bdff88d581835ac86083f2cf0a522b1decfcba17c749f9fe62c76057199d81aef10a578e");
        assert_eq!(set1::chal1::hex_to_64(s),"npk48bqHJM2LQeSJKRF4nfLx/ohRaBmw5xNvAz3WFGBAunv9mBu3MQIxeTq7ONTxp/+4jnbGrXhbXre5mW9wA73/iNWBg1rIYIPyzwpSKx3s/LoXx0n5/mLHYFcZnYGu8QpXjg==" );
    }

}
