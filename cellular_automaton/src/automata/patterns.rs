pub type Pattern5 = [[i8; 5]; 5];

pub const BLANK_PATTERN: Pattern5 = [
    [-1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1],
    [-1, -1, -1, -1, -1],
];

pub const CUTTING_CORNERS_1: Pattern5 = [
    [-1, -1, -1, -1, -1],
    [-1, -1,  1, -1, -1],
    [-1,  7,  7,  1, -1],
    [-1,  7,  7, -1, -1],
    [-1, -1, -1, -1, -1],
];

pub const CUTTING_CORNERS_2: Pattern5 = [
    [-1, -1, -1, -1, -1],
    [-1, -1,  1, -1, -1],
    [-1,  7,  7,  0, -1],
    [-1,  7,  7, -1, -1],
    [-1, -1, -1, -1, -1],
];

pub const CUTTING_CORNERS_3: Pattern5 = [
    [-1, -1, -1, -1, -1],
    [-1, -1,  0, -1, -1],
    [-1,  7,  7,  0, -1],
    [-1,  7,  7, -1, -1],
    [-1, -1, -1, -1, -1],
];

pub const ALL_PATTERNS: &[Pattern5] = &[
    CUTTING_CORNERS_1,
    CUTTING_CORNERS_2,
    CUTTING_CORNERS_3,
];