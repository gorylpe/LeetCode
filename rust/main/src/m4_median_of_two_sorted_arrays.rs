struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let ri = nums1.len() + nums2.len();

        let mut x = &nums1[..];
        let mut y = &nums2[..];
        let mut mid_x = x[x.len() - 1 / 2];
        let mut mid_y = y[y.len() - 1 / 2];

        if mid_x > mid_y {
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut mid_x, &mut mid_y);
        }
        let mut min = mid_x;
        let mut max = mid_y;

        let mut left_counter = 0;
        let mut right_counter = 0;

        loop {
            let mut mid_x = x[x.len() - 1 / 2];
            if mid_x <= max {
                left_counter += x.len() - 1 / 2 + 1;
                x = &x[..x.len() - 1 / 2];
            } else {

            }

            let mut mid_y = y[y.len() - 1 / 2];
            let i_x = (x_l + x_r) / 2;
            let i_y = (y_l + y_r) / 2;
            let mut mid_x = x[i_x];
            let mut mid_y = y[i_y];
            if mid_x < mid_y {
                x_l = i_x;
                y_r = i_y;
            } else {
                x_r = i_x;
                y_l = i_y;
            }
            x = &nums1[x_l..(x_r - x_l + 1)];
            y = &nums2[y_l..(y_r - y_l + 1)];

        }



        let mut i_x = x.len()/2;
        let mut i_y = y.len()/2;
        let mut mid_x = nums1[i_x];
        let mut mid_y = nums2[i_y];

        let mut left_counter = 0;
        loop {
            if mid_y > mid_x {
                left_counter += i_x;
                x = &nums1[i_x..];

            }
        }



        return 0.0;
    }
}

pub fn test() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,3,3], vec![2,5]), 3.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,2,3], vec![5,5]), 3.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,1,1], vec![4,5,5,5]), 4.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1,1], vec![1,5,5,5,5]), 5.0);

    assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
}