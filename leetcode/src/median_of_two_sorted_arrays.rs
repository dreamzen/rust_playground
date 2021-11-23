struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        if total_len == 0 {
            return 0 as f64;
        }
        return if (total_len) % 2 == 0 {
            let m1 = find_kth_in_sorted_array(
                &nums1,
                0,
                nums1.len(),
                &nums2,
                0,
                nums2.len(),
                total_len / 2,
            ) as f64;
            let m2 = find_kth_in_sorted_array(
                &nums1,
                0,
                nums1.len(),
                &nums2,
                0,
                nums2.len(),
                total_len / 2 + 1,
            ) as f64;
            (m1 + m2) / 2.0
        } else {
            find_kth_in_sorted_array(
                &nums1,
                0,
                nums1.len(),
                &nums2,
                0,
                nums2.len(),
                total_len / 2 + 1,
            ) as f64
        };
    }
}

// k start from 1, [1, 2, 3, 4] with k = 2, then return 2
fn find_kth_in_sorted_array(
    nums1: &Vec<i32>,
    idx1: usize,
    len1: usize,
    nums2: &Vec<i32>,
    idx2: usize,
    len2: usize,
    k: usize,
) -> i32 {
    if len1 == 0 {
        return nums2[idx2 + k - 1];
    }
    if len2 == 0 {
        return nums1[idx1 + k - 1];
    }
    if k == 1 {
        return nums1[idx1].min(nums2[idx2]);
    }
    if len1 < len2 {
        return find_kth_in_sorted_array(nums2, idx2, len2, nums1, idx1, len1, k);
    }

    let ele_in_nums2 = len2.min(k / 2);
    let ele_in_nums1 = k - ele_in_nums2;
    let n1 = nums1[idx1 + ele_in_nums1 - 1];
    let n2 = nums2[idx2 + ele_in_nums2 - 1];
    if n1 == n2 {
        return n1;
    } else if n1 > n2 {
        // drop ele_in_nums2
        return find_kth_in_sorted_array(
            nums1,
            idx1,
            len1,
            nums2,
            idx2 + ele_in_nums2,
            len2 - ele_in_nums2,
            k - ele_in_nums2,
        );
    } else {
        // drop ele_in_nums1
        return find_kth_in_sorted_array(
            nums1,
            idx1 + ele_in_nums1,
            len1 - ele_in_nums1,
            nums2,
            idx2,
            len2,
            k - ele_in_nums1,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn find_kth_panic() {
        assert_eq!(
            2,
            find_kth_in_sorted_array(&vec![1, 3, 5], 0, 3, &vec![2, 6], 0, 2, 0)
        );
    }

    #[test]
    fn find_kth() {
        assert_eq!(
            2,
            find_kth_in_sorted_array(&vec![1, 3, 5], 0, 3, &vec![2, 6], 0, 2, 2)
        );
        assert_eq!(
            3,
            find_kth_in_sorted_array(&vec![1, 3, 5], 0, 3, &vec![2, 6], 0, 2, 3)
        );
        assert_eq!(
            6,
            find_kth_in_sorted_array(&vec![1, 3, 5], 0, 3, &vec![2, 6], 0, 2, 5)
        );
    }

    #[test]
    fn find_median() {
        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 6])
        );
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 6])
        );
        assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![], vec![2]));
        assert_eq!(0.0, Solution::find_median_sorted_arrays(vec![], vec![]));
    }
}
