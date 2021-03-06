//有两种特殊字符。第一种字符可以用一比特0来表示。第二种字符可以用两比特(10 或 11)来表示。
//
// 现给一个由若干比特组成的字符串。问最后一个字符是否必定为一个一比特字符。给定的字符串总是由0结束。
//
// 示例 1:
//
// 输入:
// bits = [1, 0, 0]
// 输出: True
// 解释:
// 唯一的编码方式是一个两比特字符和一个一比特字符。所以最后一个字符是一比特字符。
// 示例 2:
//
// 输入:
// bits = [1, 1, 1, 0]
// 输出: False
// 解释:
// 唯一的编码方式是两比特字符和两比特字符。所以最后一个字符不是一比特字符。
// 注意:
//
// 1 <= len(bits) <= 1000.
// bits[i] 总是0 或 1.
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/1-bit-and-2-bit-characters
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。



///当前字符是1，则必然为两字符数字。否则为单字符数字
pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut idx = 0;
    let mut res = false;
    while idx < bits.len() {
        if bits[idx] == 1 {
            idx += 2;
            res = false;
        } else {
            idx += 1;
            res = true;
        }
    }
    //可以代替return res;这个写法
    res
}

#[test]
fn test() {
    let character = is_one_bit_character(vec![1, 0, 0]);
    println!("{}", character);
    assert!(character);
}


