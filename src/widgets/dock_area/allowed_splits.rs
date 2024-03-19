/// What directions can this dock be split in?
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AllowedSplits(u8);

impl Default for AllowedSplits {
    fn default() -> Self {
        AllowedSplits::ALL
    }
}

impl std::ops::BitAnd for AllowedSplits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for AllowedSplits {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self::from_u8(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for AllowedSplits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for AllowedSplits {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self::from_u8(self.0 | rhs.0)
    }
}

impl AllowedSplits {
    /// Allow splits in any direction (horizontal and vertical).
    pub const ALL: Self = Self(0b1111);

    /// Allow splitting to the left
    pub const LEFT: Self = Self(0b1000);

    /// Allow splitting to the right
    pub const RIGHT: Self = Self(0b0100);

    /// Allow splits in horizontal directions.
    pub const LEFT_RIGHT: Self = Self(0b1100);

    /// Allow splitting up
    pub const TOP: Self = Self(0b0010);

    /// Allow splitting down
    pub const BOTTOM: Self = Self(0b0001);

    /// Allow splits in vertical directions.
    pub const TOP_BOTTOM: Self = Self(0b0011);

    /// Don't allow splits at all.
    pub const NONE: Self = Self(0b0000);

    /// Create allowed splits from a u8, panics if an invalid value is given.
    #[inline(always)]
    fn from_u8(u8: u8) -> Self {
        if u8 > 0b1111 {
            panic!("Provided an invalid value for allowed splits: {u8:0x}");
        }
        Self(u8)
    }

    /// Are we allowed to split above?
    #[inline(always)]
    pub const fn top(&self) -> bool {
        self.0 & Self::TOP.0 != 0
    }

    /// Are we allowed to split below?
    #[inline(always)]
    pub const fn bottom(&self) -> bool {
        self.0 & Self::BOTTOM.0 != 0
    }

    /// Are we allowed to split vertically?
    #[inline(always)]
    pub const fn top_or_bottom(&self) -> bool {
        self.0 & Self::TOP_BOTTOM.0 != 0
    }

    /// Are we allowed to split left?
    #[inline(always)]
    pub const fn left(&self) -> bool {
        self.0 & Self::LEFT.0 != 0
    }

    /// Are we allowed to split right?
    #[inline(always)]
    pub const fn right(&self) -> bool {
        self.0 & Self::RIGHT.0 != 0
    }

    /// Are we allowed to split horizontally?
    #[inline(always)]
    pub const fn left_or_right(&self) -> bool {
        self.0 & Self::LEFT_RIGHT.0 != 0
    }

    /// Are all splits disallowed?
    #[inline(always)]
    pub const fn none(&self) -> bool {
        self.0 == Self::NONE.0
    }

    /// Are all splits allowed?
    #[inline(always)]
    pub const fn all(&self) -> bool {
        self.0 == Self::ALL.0
    }

    #[inline(always)]
    pub(crate) const fn allowed(&self, tree_split: &crate::tree::Split) -> bool {
        match tree_split {
            crate::tree::Split::Left => self.left(),
            crate::tree::Split::Right => self.right(),
            crate::tree::Split::Above => self.top(),
            crate::tree::Split::Below => self.bottom(),
        }
    }
}
