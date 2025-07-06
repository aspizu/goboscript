fn get_block_height(max_nesting: i32) -> i32 {
    8 * max_nesting.max(1) + 40
}
