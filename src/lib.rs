//! 这是一个示例库的文档

/// 添加两个数字
/// 
/// # 示例
/// ```
/// use iot_driver_core::add;
/// 
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

mod driver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}