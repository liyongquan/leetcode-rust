/*
给定一个整数类型的数组 nums，请编写一个能够返回数组 “中心索引” 的方法。

我们是这样定义数组 中心索引 的：数组中心索引的左侧所有元素相加的和等于右侧所有元素相加的和。

如果数组不存在中心索引，那么我们应该返回 -1。如果数组有多个中心索引，那么我们应该返回最靠近左边的那一个。

 

示例 1：

输入：
nums = [1, 7, 3, 6, 5, 6]
输出：3
解释：
索引 3 (nums[3] = 6) 的左侧数之和 (1 + 7 + 3 = 11)，与右侧数之和 (5 + 6 = 11) 相等。
同时, 3 也是第一个符合要求的中心索引。
示例 2：

输入：
nums = [1, 2, 3]
输出：-1
解释：
数组中不存在满足此条件的中心索引。
 

说明：

nums 的长度范围为 [0, 10000]。
任何一个 nums[i] 将会是一个范围在 [-1000, 1000]的整数。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-pivot-index
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
先统计数组的总和sum，左边的总和为l_sum,右边的总和为r_sum=sum-l_sum

每次移动指针相当于l_sum+=nums[i]，找到第一个相当的元素即可

时间复杂度O(n)，空间复杂度O(1)
 */
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    let mut sum = 0;
    for x in nums.iter() {
        sum += x;
    }
    let mut l_sum = 0;
    let mut r_sum = sum;
    for i in 0..nums.len() {
        if i > 0 {
            l_sum += nums[i - 1];
        }
        r_sum -= nums[i];
        if l_sum == r_sum {
            return i as i32;
        }
    }
    return -1;
}

#[test]
fn test() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    let res = pivot_index(nums);
    println!("{}", res);
    assert_eq!(3, res);
}

#[test]
fn test2() {
    let nums = vec![-1, -1, -1, -1, -1, 0];
    let res = pivot_index(nums);
    println!("{}", res);
    assert_eq!(2, res);
}