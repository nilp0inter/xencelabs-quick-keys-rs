fn pad_zeroes<const A:usize, const B:usize>(arr:[u8;A])->[u8;B]{
    assert!(B >= A);
    let mut b = [0;B];
    b[..A].copy_from_slice(&arr);
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_doesnt_change_filled_arrays() {
        let result : [u8; 3] = pad_zeroes([1, 2, 3]);
        assert_eq!(result, [1, 2, 3]);
    }
}
