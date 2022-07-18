use modular_bitfield::{
    bitfield,
    specifiers::{B16, B4},
    BitfieldSpecifier,
};

/// `SYSCTRL0`
#[bitfield]
pub struct SystemControlRegister0 {
    /// Reserved
    #[skip]
    __: B16,
    /// After a wake-up from Deep Sleep, this will be `true` if the SCLK was changed to RCH from XTH.
    /// This happens because when the chip goes into Deep Sleep, both RCH and XTH are disabled.
    /// When the chip wakes back up, only RCH can start up immediately, but if XTH is needed as the SCLK,
    /// then you will need to start up XTH and wait for clock stability before switching to it, which
    /// may take several clock cycles.
    ///
    /// After reset is `TODO`.
    pub wakeup_by_rch: bool,
    /// Reserved
    #[skip]
    __: B4,
    /// Divider for the Advanced Peripheral Bus (APB) clock
    ///
    /// After a reset is `TODO`.
    pub pclk_selection: PclkSelection,
    /// Divider for the MCU clock
    ///
    /// After a reset is `TODO`.
    pub hclk_selection: HclkSelection,
    /// Divider for the system clock
    ///
    /// After a reset is `TODO`.
    pub sclk_selection: SclkSelection,
    /// Enables the external low-speed crystal (XTL).
    ///
    /// If this is set to `true`, then P14 and P15 need to be set as analog ports.
    ///
    /// After a reset is `TODO`.
    pub external_low_speed_crystal_enable: bool,
    /// Enables the internal low-speed clock (RCL).
    ///
    /// After a reset is `TODO`.
    pub internal_low_speed_clock_enable: bool,
    /// Enables the external high-speed crystal (XTH).
    ///
    /// When entering Deep Sleep, XTH is automatically turned off.
    /// [`wakeup_by_rch`] will be `true` when the core wakes up.
    ///
    /// After a reset is `TODO`.
    pub external_high_speed_crystal_enable: bool,
    /// Enables the internal high-speed clock (RCH).
    ///
    /// When entering Deep Sleep, RCH is automatically turned off.
    ///
    /// After a reset is `TODO`.
    pub internal_high_speed_clock_enable: bool,
}

#[derive(BitfieldSpecifier)]
pub enum PclkSelection {
    HclkDiv1,
    HclkDiv2,
    HclkDiv4,
    HclkDiv8,
}

#[derive(BitfieldSpecifier)]
pub enum HclkSelection {
    SclkDiv1,
    SclkDiv2,
    SclkDiv4,
    SclkDiv8,
    SclkDiv16,
    SclkDiv32,
    SclkDiv64,
    SclkDiv128,
}

#[derive(BitfieldSpecifier)]
pub enum SclkSelection {
    /// `RCH`
    InternalHighSpeedClock,
    /// `XTH`
    ExternalHighSpeedCrystal,
    /// `RCL`
    InternalLowSpeedClock,
    /// `XTL`
    ExternalLowSpeedCrystal,
}
