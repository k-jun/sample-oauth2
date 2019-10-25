pub const TOP_LEVEL: i64 = 1000;
pub const SUBTOP_LEVEL: i64 = 900;
pub const MEMBER_LEVEL: i64 = 500;

pub fn is_top(your_level: i64) -> bool {
    TOP_LEVEL == your_level
}

pub fn is_subtop(your_level: i64) -> bool {
    SUBTOP_LEVEL == your_level
}

pub fn is_top_or_subtop(your_level: i64) -> bool {
    SUBTOP_LEVEL <= your_level
}

pub fn is_member_or_more(your_level: i64) -> bool {
    MEMBER_LEVEL <= your_level
}
