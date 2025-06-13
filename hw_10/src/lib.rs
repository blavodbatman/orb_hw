#[allow(dead_code)]
fn merge_sort(input: &[u64]) -> Vec<u64> {
    if input.is_empty() || input.len() == 1 {
        return input.to_vec();
    }

    let l_p = merge_sort(&input[..(input.len() / 2)]);
    let r_p = merge_sort(&input[(input.len() / 2)..]);

    let mut l = 0;
    let mut p = 0;
    let mut res = Vec::<u64>::with_capacity(input.len());

    res.append(
        &mut std::iter::from_fn(|| {
            if l < l_p.len() && p < r_p.len() {
                if l_p[l] <= r_p[p] {
                    l += 1;
                    Some(l_p[l - 1])
                } else {
                    p += 1;
                    Some(r_p[p - 1])
                }
            } else {
                None
            }
        })
        .collect::<Vec<u64>>(),
    );

    res.append(
        &mut (l..l_p.len())
            .map(|i| l_p[i])
            .chain((p..r_p.len()).map(|i| r_p[i]))
            .collect(),
    );

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(merge_sort(&[]), vec![]);
        assert_eq!(merge_sort(&[1]), vec![1]);
        assert_eq!(merge_sort(&[8, 3, 15, 1]), vec![1, 3, 8, 15]);
        assert_eq!(merge_sort(&[8, 3, 15, 1, 7]), vec![1, 3, 7, 8, 15]);
    }
}
