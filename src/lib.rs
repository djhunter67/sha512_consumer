use std::mem::MaybeUninit;

pub fn hash_sha512(bytes: &[u8]) -> [u8; 64] {
    let mut out: MaybeUninit<[u8; 64]> = MaybeUninit::uninit();

    unsafe {
        c_in_rust::crypto_hash_sha512_tweet(
            out.as_mut_ptr().cast(),
            bytes.as_ptr(),
            bytes.len() as u64,
        );

        out.assume_init()
    }
}

#[cfg(test)]
mod test {
    use rstest::{fixture, rstest};

    use crate::hash_sha512;

    #[fixture]
    fn convert_answer(#[default("")] online_answer: &str) -> [u8; 64] {
        let mut answer: [u8; 64] = [0; 64];

        for i in 0..answer.len() {
            answer[i] = u8::from_str_radix(&online_answer[i * 2..i * 2 + 2], 16).unwrap();
        }

        answer
    }

    #[test]
    fn test_it_hashes() {
        let bytes = b"Hello, world!";

        let the_hash = hash_sha512(bytes);

        assert_eq!(
            the_hash,
            [
                0xc1, 0x52, 0x7c, 0xd8, 0x93, 0xc1, 0x24, 0x77, 0x3d, 0x81, 0x19, 0x11, 0x97, 0xc,
                0x8f, 0xe6, 0xe8, 0x57, 0xd6, 0xdf, 0x5d, 0xc9, 0x22, 0x6b, 0xd8, 0xa1, 0x60, 0x61,
                0x4c, 0xc, 0xd9, 0x63, 0xa4, 0xdd, 0xea, 0x2b, 0x94, 0xbb, 0x7d, 0x36, 0x2, 0x1e,
                0xf9, 0xd8, 0x65, 0xd5, 0xce, 0xa2, 0x94, 0xa8, 0x2d, 0xd4, 0x9a, 0xb, 0xb2, 0x69,
                0xf5, 0x1f, 0x6e, 0x7a, 0x57, 0xf7, 0x94, 0x21,
            ]
        )
    }

    #[rstest]
    fn test_it_hashes_another(
        #[with("d6517f783df80d8ff6981c740347665d56a9a073a83343d4e90c5688460d6f110c5fbb1e1bb5cba964b96c6a138565eeb2aa6a45b3d68401bbb723c8f8dd83a2")]
        convert_answer: [u8; 64],
    ) {
        let bytes = b"another_string";

        let the_hash = hash_sha512(bytes);

        for (i, _) in the_hash.iter().enumerate() {
            assert_eq!(the_hash[i], convert_answer[i]);
        }
    }

    #[rstest]
    fn test_it_hashes_empty_string(
        #[with("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e")]
        convert_answer: [u8; 64],
    ) {
        let bytes = b"";

        let the_hash = hash_sha512(bytes);

        for (i, _) in the_hash.iter().enumerate() {
            assert_eq!(the_hash[i], convert_answer[i]);
        }
    }

    #[rstest]
    fn test_it_hashes_long_string(
        #[with("ee384a55b9c30bb8c700e04ddde25f991d6a5a964c001f311a0aa2b02eb0f8c9e48eb0ec788f1c30d214f97636b05410b1dbd42d7e9c90e999fb4a190e700f4a")]
        convert_answer: [u8; 64],
    ) {
        let bytes = b"long string";

        let the_hash = hash_sha512(bytes);

        for (i, _) in the_hash.iter().enumerate() {
            assert_eq!(the_hash[i], convert_answer[i]);
        }
    }
}
