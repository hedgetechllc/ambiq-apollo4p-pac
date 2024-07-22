#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    _reserved2: [u8; 0x08],
    globen: Globen,
    _reserved3: [u8; 0x4c],
    inten: Inten,
    intstat: Intstat,
    intclr: Intclr,
    intset: Intset,
    _reserved7: [u8; 0x10],
    outcfg0: Outcfg0,
    outcfg1: Outcfg1,
    outcfg2: Outcfg2,
    outcfg3: Outcfg3,
    outcfg4: Outcfg4,
    outcfg5: Outcfg5,
    outcfg6: Outcfg6,
    outcfg7: Outcfg7,
    outcfg8: Outcfg8,
    outcfg9: Outcfg9,
    outcfg10: Outcfg10,
    outcfg11: Outcfg11,
    outcfg12: Outcfg12,
    outcfg13: Outcfg13,
    outcfg14: Outcfg14,
    outcfg15: Outcfg15,
    outcfg16: Outcfg16,
    outcfg17: Outcfg17,
    outcfg18: Outcfg18,
    outcfg19: Outcfg19,
    outcfg20: Outcfg20,
    outcfg21: Outcfg21,
    outcfg22: Outcfg22,
    outcfg23: Outcfg23,
    outcfg24: Outcfg24,
    outcfg25: Outcfg25,
    outcfg26: Outcfg26,
    outcfg27: Outcfg27,
    outcfg28: Outcfg28,
    outcfg29: Outcfg29,
    outcfg30: Outcfg30,
    outcfg31: Outcfg31,
    _reserved39: [u8; 0x0100],
    ctrl0: Ctrl0,
    timer0: Timer0,
    tmr0cmp0: Tmr0cmp0,
    tmr0cmp1: Tmr0cmp1,
    mode0: Mode0,
    tmr0lmtval: Tmr0lmtval,
    _reserved45: [u8; 0x08],
    ctrl1: Ctrl1,
    timer1: Timer1,
    tmr1cmp0: Tmr1cmp0,
    tmr1cmp1: Tmr1cmp1,
    mode1: Mode1,
    tmr1lmtval: Tmr1lmtval,
    _reserved51: [u8; 0x08],
    ctrl2: Ctrl2,
    timer2: Timer2,
    tmr2cmp0: Tmr2cmp0,
    tmr2cmp1: Tmr2cmp1,
    mode2: Mode2,
    tmr2lmtval: Tmr2lmtval,
    _reserved57: [u8; 0x08],
    ctrl3: Ctrl3,
    timer3: Timer3,
    tmr3cmp0: Tmr3cmp0,
    tmr3cmp1: Tmr3cmp1,
    mode3: Mode3,
    tmr3lmtval: Tmr3lmtval,
    _reserved63: [u8; 0x08],
    ctrl4: Ctrl4,
    timer4: Timer4,
    tmr4cmp0: Tmr4cmp0,
    tmr4cmp1: Tmr4cmp1,
    mode4: Mode4,
    tmr4lmtval: Tmr4lmtval,
    _reserved69: [u8; 0x08],
    ctrl5: Ctrl5,
    timer5: Timer5,
    tmr5cmp0: Tmr5cmp0,
    tmr5cmp1: Tmr5cmp1,
    mode5: Mode5,
    tmr5lmtval: Tmr5lmtval,
    _reserved75: [u8; 0x08],
    ctrl6: Ctrl6,
    timer6: Timer6,
    tmr6cmp0: Tmr6cmp0,
    tmr6cmp1: Tmr6cmp1,
    mode6: Mode6,
    tmr6lmtval: Tmr6lmtval,
    _reserved81: [u8; 0x08],
    ctrl7: Ctrl7,
    timer7: Timer7,
    tmr7cmp0: Tmr7cmp0,
    tmr7cmp1: Tmr7cmp1,
    mode7: Mode7,
    tmr7lmtval: Tmr7lmtval,
    _reserved87: [u8; 0x08],
    ctrl8: Ctrl8,
    timer8: Timer8,
    tmr8cmp0: Tmr8cmp0,
    tmr8cmp1: Tmr8cmp1,
    mode8: Mode8,
    tmr8lmtval: Tmr8lmtval,
    _reserved93: [u8; 0x08],
    ctrl9: Ctrl9,
    timer9: Timer9,
    tmr9cmp0: Tmr9cmp0,
    tmr9cmp1: Tmr9cmp1,
    mode9: Mode9,
    tmr9lmtval: Tmr9lmtval,
    _reserved99: [u8; 0x08],
    ctrl10: Ctrl10,
    timer10: Timer10,
    tmr10cmp0: Tmr10cmp0,
    tmr10cmp1: Tmr10cmp1,
    mode10: Mode10,
    tmr10lmtval: Tmr10lmtval,
    _reserved105: [u8; 0x08],
    ctrl11: Ctrl11,
    timer11: Timer11,
    tmr11cmp0: Tmr11cmp0,
    tmr11cmp1: Tmr11cmp1,
    mode11: Mode11,
    tmr11lmtval: Tmr11lmtval,
    _reserved111: [u8; 0x08],
    ctrl12: Ctrl12,
    timer12: Timer12,
    tmr12cmp0: Tmr12cmp0,
    tmr12cmp1: Tmr12cmp1,
    mode12: Mode12,
    tmr12lmtval: Tmr12lmtval,
    _reserved117: [u8; 0x08],
    ctrl13: Ctrl13,
    timer13: Timer13,
    tmr13cmp0: Tmr13cmp0,
    tmr13cmp1: Tmr13cmp1,
    mode13: Mode13,
    tmr13lmtval: Tmr13lmtval,
    _reserved123: [u8; 0x08],
    ctrl14: Ctrl14,
    timer14: Timer14,
    tmr14cmp0: Tmr14cmp0,
    tmr14cmp1: Tmr14cmp1,
    mode14: Mode14,
    tmr14lmtval: Tmr14lmtval,
    _reserved129: [u8; 0x08],
    ctrl15: Ctrl15,
    timer15: Timer15,
    tmr15cmp0: Tmr15cmp0,
    tmr15cmp1: Tmr15cmp1,
    mode15: Mode15,
    tmr15lmtval: Tmr15lmtval,
    timerspares: Timerspares,
}
impl RegisterBlock {
    #[doc = "0x00 - General Timer Controls"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - General Timer status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Alternate enables for all TIMERs."]
    #[inline(always)]
    pub const fn globen(&self) -> &Globen {
        &self.globen
    }
    #[doc = "0x60 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x64 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x68 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x6c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0x80 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg0(&self) -> &Outcfg0 {
        &self.outcfg0
    }
    #[doc = "0x84 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg1(&self) -> &Outcfg1 {
        &self.outcfg1
    }
    #[doc = "0x88 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg2(&self) -> &Outcfg2 {
        &self.outcfg2
    }
    #[doc = "0x8c - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg3(&self) -> &Outcfg3 {
        &self.outcfg3
    }
    #[doc = "0x90 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg4(&self) -> &Outcfg4 {
        &self.outcfg4
    }
    #[doc = "0x94 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg5(&self) -> &Outcfg5 {
        &self.outcfg5
    }
    #[doc = "0x98 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg6(&self) -> &Outcfg6 {
        &self.outcfg6
    }
    #[doc = "0x9c - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg7(&self) -> &Outcfg7 {
        &self.outcfg7
    }
    #[doc = "0xa0 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg8(&self) -> &Outcfg8 {
        &self.outcfg8
    }
    #[doc = "0xa4 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg9(&self) -> &Outcfg9 {
        &self.outcfg9
    }
    #[doc = "0xa8 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg10(&self) -> &Outcfg10 {
        &self.outcfg10
    }
    #[doc = "0xac - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg11(&self) -> &Outcfg11 {
        &self.outcfg11
    }
    #[doc = "0xb0 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg12(&self) -> &Outcfg12 {
        &self.outcfg12
    }
    #[doc = "0xb4 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg13(&self) -> &Outcfg13 {
        &self.outcfg13
    }
    #[doc = "0xb8 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg14(&self) -> &Outcfg14 {
        &self.outcfg14
    }
    #[doc = "0xbc - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg15(&self) -> &Outcfg15 {
        &self.outcfg15
    }
    #[doc = "0xc0 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg16(&self) -> &Outcfg16 {
        &self.outcfg16
    }
    #[doc = "0xc4 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg17(&self) -> &Outcfg17 {
        &self.outcfg17
    }
    #[doc = "0xc8 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg18(&self) -> &Outcfg18 {
        &self.outcfg18
    }
    #[doc = "0xcc - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg19(&self) -> &Outcfg19 {
        &self.outcfg19
    }
    #[doc = "0xd0 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg20(&self) -> &Outcfg20 {
        &self.outcfg20
    }
    #[doc = "0xd4 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg21(&self) -> &Outcfg21 {
        &self.outcfg21
    }
    #[doc = "0xd8 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg22(&self) -> &Outcfg22 {
        &self.outcfg22
    }
    #[doc = "0xdc - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg23(&self) -> &Outcfg23 {
        &self.outcfg23
    }
    #[doc = "0xe0 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg24(&self) -> &Outcfg24 {
        &self.outcfg24
    }
    #[doc = "0xe4 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg25(&self) -> &Outcfg25 {
        &self.outcfg25
    }
    #[doc = "0xe8 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg26(&self) -> &Outcfg26 {
        &self.outcfg26
    }
    #[doc = "0xec - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg27(&self) -> &Outcfg27 {
        &self.outcfg27
    }
    #[doc = "0xf0 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg28(&self) -> &Outcfg28 {
        &self.outcfg28
    }
    #[doc = "0xf4 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg29(&self) -> &Outcfg29 {
        &self.outcfg29
    }
    #[doc = "0xf8 - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg30(&self) -> &Outcfg30 {
        &self.outcfg30
    }
    #[doc = "0xfc - Pad output configuration 0."]
    #[inline(always)]
    pub const fn outcfg31(&self) -> &Outcfg31 {
        &self.outcfg31
    }
    #[doc = "0x200 - This includes the Control bit fields for timer 0."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x204 - This register holds the running time or event count for timer 0."]
    #[inline(always)]
    pub const fn timer0(&self) -> &Timer0 {
        &self.timer0
    }
    #[doc = "0x208 - This contains the Compare limits for timer 0. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr0cmp0(&self) -> &Tmr0cmp0 {
        &self.tmr0cmp0
    }
    #[doc = "0x20c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr0cmp1(&self) -> &Tmr0cmp1 {
        &self.tmr0cmp1
    }
    #[doc = "0x210 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode0(&self) -> &Mode0 {
        &self.mode0
    }
    #[doc = "0x214 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr0lmtval(&self) -> &Tmr0lmtval {
        &self.tmr0lmtval
    }
    #[doc = "0x220 - This includes the Control bit fields for timer 1."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x224 - This register holds the running time or event count for timer 1."]
    #[inline(always)]
    pub const fn timer1(&self) -> &Timer1 {
        &self.timer1
    }
    #[doc = "0x228 - This contains the Compare limits for timer 1. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr1cmp0(&self) -> &Tmr1cmp0 {
        &self.tmr1cmp0
    }
    #[doc = "0x22c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr1cmp1(&self) -> &Tmr1cmp1 {
        &self.tmr1cmp1
    }
    #[doc = "0x230 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode1(&self) -> &Mode1 {
        &self.mode1
    }
    #[doc = "0x234 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr1lmtval(&self) -> &Tmr1lmtval {
        &self.tmr1lmtval
    }
    #[doc = "0x240 - This includes the Control bit fields for timer 2."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x244 - This register holds the running time or event count for timer 2."]
    #[inline(always)]
    pub const fn timer2(&self) -> &Timer2 {
        &self.timer2
    }
    #[doc = "0x248 - This contains the Compare limits for timer 2. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr2cmp0(&self) -> &Tmr2cmp0 {
        &self.tmr2cmp0
    }
    #[doc = "0x24c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr2cmp1(&self) -> &Tmr2cmp1 {
        &self.tmr2cmp1
    }
    #[doc = "0x250 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode2(&self) -> &Mode2 {
        &self.mode2
    }
    #[doc = "0x254 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr2lmtval(&self) -> &Tmr2lmtval {
        &self.tmr2lmtval
    }
    #[doc = "0x260 - This includes the Control bit fields for timer 3."]
    #[inline(always)]
    pub const fn ctrl3(&self) -> &Ctrl3 {
        &self.ctrl3
    }
    #[doc = "0x264 - This register holds the running time or event count for timer 3."]
    #[inline(always)]
    pub const fn timer3(&self) -> &Timer3 {
        &self.timer3
    }
    #[doc = "0x268 - This contains the Compare limits for timer 3. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr3cmp0(&self) -> &Tmr3cmp0 {
        &self.tmr3cmp0
    }
    #[doc = "0x26c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr3cmp1(&self) -> &Tmr3cmp1 {
        &self.tmr3cmp1
    }
    #[doc = "0x270 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode3(&self) -> &Mode3 {
        &self.mode3
    }
    #[doc = "0x274 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr3lmtval(&self) -> &Tmr3lmtval {
        &self.tmr3lmtval
    }
    #[doc = "0x280 - This includes the Control bit fields for timer 4."]
    #[inline(always)]
    pub const fn ctrl4(&self) -> &Ctrl4 {
        &self.ctrl4
    }
    #[doc = "0x284 - This register holds the running time or event count for timer 4."]
    #[inline(always)]
    pub const fn timer4(&self) -> &Timer4 {
        &self.timer4
    }
    #[doc = "0x288 - This contains the Compare limits for timer 4. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr4cmp0(&self) -> &Tmr4cmp0 {
        &self.tmr4cmp0
    }
    #[doc = "0x28c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr4cmp1(&self) -> &Tmr4cmp1 {
        &self.tmr4cmp1
    }
    #[doc = "0x290 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode4(&self) -> &Mode4 {
        &self.mode4
    }
    #[doc = "0x294 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr4lmtval(&self) -> &Tmr4lmtval {
        &self.tmr4lmtval
    }
    #[doc = "0x2a0 - This includes the Control bit fields for timer 5."]
    #[inline(always)]
    pub const fn ctrl5(&self) -> &Ctrl5 {
        &self.ctrl5
    }
    #[doc = "0x2a4 - This register holds the running time or event count for timer 5."]
    #[inline(always)]
    pub const fn timer5(&self) -> &Timer5 {
        &self.timer5
    }
    #[doc = "0x2a8 - This contains the Compare limits for timer 5. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr5cmp0(&self) -> &Tmr5cmp0 {
        &self.tmr5cmp0
    }
    #[doc = "0x2ac - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr5cmp1(&self) -> &Tmr5cmp1 {
        &self.tmr5cmp1
    }
    #[doc = "0x2b0 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode5(&self) -> &Mode5 {
        &self.mode5
    }
    #[doc = "0x2b4 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr5lmtval(&self) -> &Tmr5lmtval {
        &self.tmr5lmtval
    }
    #[doc = "0x2c0 - This includes the Control bit fields for timer 6."]
    #[inline(always)]
    pub const fn ctrl6(&self) -> &Ctrl6 {
        &self.ctrl6
    }
    #[doc = "0x2c4 - This register holds the running time or event count for timer 6."]
    #[inline(always)]
    pub const fn timer6(&self) -> &Timer6 {
        &self.timer6
    }
    #[doc = "0x2c8 - This contains the Compare limits for timer 6. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr6cmp0(&self) -> &Tmr6cmp0 {
        &self.tmr6cmp0
    }
    #[doc = "0x2cc - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr6cmp1(&self) -> &Tmr6cmp1 {
        &self.tmr6cmp1
    }
    #[doc = "0x2d0 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode6(&self) -> &Mode6 {
        &self.mode6
    }
    #[doc = "0x2d4 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr6lmtval(&self) -> &Tmr6lmtval {
        &self.tmr6lmtval
    }
    #[doc = "0x2e0 - This includes the Control bit fields for timer 7."]
    #[inline(always)]
    pub const fn ctrl7(&self) -> &Ctrl7 {
        &self.ctrl7
    }
    #[doc = "0x2e4 - This register holds the running time or event count for timer 7."]
    #[inline(always)]
    pub const fn timer7(&self) -> &Timer7 {
        &self.timer7
    }
    #[doc = "0x2e8 - This contains the Compare limits for timer 7. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr7cmp0(&self) -> &Tmr7cmp0 {
        &self.tmr7cmp0
    }
    #[doc = "0x2ec - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr7cmp1(&self) -> &Tmr7cmp1 {
        &self.tmr7cmp1
    }
    #[doc = "0x2f0 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode7(&self) -> &Mode7 {
        &self.mode7
    }
    #[doc = "0x2f4 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr7lmtval(&self) -> &Tmr7lmtval {
        &self.tmr7lmtval
    }
    #[doc = "0x300 - This includes the Control bit fields for timer 8."]
    #[inline(always)]
    pub const fn ctrl8(&self) -> &Ctrl8 {
        &self.ctrl8
    }
    #[doc = "0x304 - This register holds the running time or event count for timer 8."]
    #[inline(always)]
    pub const fn timer8(&self) -> &Timer8 {
        &self.timer8
    }
    #[doc = "0x308 - This contains the Compare limits for timer 8. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr8cmp0(&self) -> &Tmr8cmp0 {
        &self.tmr8cmp0
    }
    #[doc = "0x30c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr8cmp1(&self) -> &Tmr8cmp1 {
        &self.tmr8cmp1
    }
    #[doc = "0x310 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode8(&self) -> &Mode8 {
        &self.mode8
    }
    #[doc = "0x314 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr8lmtval(&self) -> &Tmr8lmtval {
        &self.tmr8lmtval
    }
    #[doc = "0x320 - This includes the Control bit fields for timer 9."]
    #[inline(always)]
    pub const fn ctrl9(&self) -> &Ctrl9 {
        &self.ctrl9
    }
    #[doc = "0x324 - This register holds the running time or event count for timer 9."]
    #[inline(always)]
    pub const fn timer9(&self) -> &Timer9 {
        &self.timer9
    }
    #[doc = "0x328 - This contains the Compare limits for timer 9. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr9cmp0(&self) -> &Tmr9cmp0 {
        &self.tmr9cmp0
    }
    #[doc = "0x32c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr9cmp1(&self) -> &Tmr9cmp1 {
        &self.tmr9cmp1
    }
    #[doc = "0x330 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode9(&self) -> &Mode9 {
        &self.mode9
    }
    #[doc = "0x334 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr9lmtval(&self) -> &Tmr9lmtval {
        &self.tmr9lmtval
    }
    #[doc = "0x340 - This includes the Control bit fields for timer 10."]
    #[inline(always)]
    pub const fn ctrl10(&self) -> &Ctrl10 {
        &self.ctrl10
    }
    #[doc = "0x344 - This register holds the running time or event count for timer 10."]
    #[inline(always)]
    pub const fn timer10(&self) -> &Timer10 {
        &self.timer10
    }
    #[doc = "0x348 - This contains the Compare limits for timer 10. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr10cmp0(&self) -> &Tmr10cmp0 {
        &self.tmr10cmp0
    }
    #[doc = "0x34c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr10cmp1(&self) -> &Tmr10cmp1 {
        &self.tmr10cmp1
    }
    #[doc = "0x350 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode10(&self) -> &Mode10 {
        &self.mode10
    }
    #[doc = "0x354 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr10lmtval(&self) -> &Tmr10lmtval {
        &self.tmr10lmtval
    }
    #[doc = "0x360 - This includes the Control bit fields for timer 11."]
    #[inline(always)]
    pub const fn ctrl11(&self) -> &Ctrl11 {
        &self.ctrl11
    }
    #[doc = "0x364 - This register holds the running time or event count for timer 11."]
    #[inline(always)]
    pub const fn timer11(&self) -> &Timer11 {
        &self.timer11
    }
    #[doc = "0x368 - This contains the Compare limits for timer 11. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr11cmp0(&self) -> &Tmr11cmp0 {
        &self.tmr11cmp0
    }
    #[doc = "0x36c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr11cmp1(&self) -> &Tmr11cmp1 {
        &self.tmr11cmp1
    }
    #[doc = "0x370 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode11(&self) -> &Mode11 {
        &self.mode11
    }
    #[doc = "0x374 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr11lmtval(&self) -> &Tmr11lmtval {
        &self.tmr11lmtval
    }
    #[doc = "0x380 - This includes the Control bit fields for timer 12."]
    #[inline(always)]
    pub const fn ctrl12(&self) -> &Ctrl12 {
        &self.ctrl12
    }
    #[doc = "0x384 - This register holds the running time or event count for timer 12."]
    #[inline(always)]
    pub const fn timer12(&self) -> &Timer12 {
        &self.timer12
    }
    #[doc = "0x388 - This contains the Compare limits for timer 12. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr12cmp0(&self) -> &Tmr12cmp0 {
        &self.tmr12cmp0
    }
    #[doc = "0x38c - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr12cmp1(&self) -> &Tmr12cmp1 {
        &self.tmr12cmp1
    }
    #[doc = "0x390 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode12(&self) -> &Mode12 {
        &self.mode12
    }
    #[doc = "0x394 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr12lmtval(&self) -> &Tmr12lmtval {
        &self.tmr12lmtval
    }
    #[doc = "0x3a0 - This includes the Control bit fields for timer 13."]
    #[inline(always)]
    pub const fn ctrl13(&self) -> &Ctrl13 {
        &self.ctrl13
    }
    #[doc = "0x3a4 - This register holds the running time or event count for timer 13."]
    #[inline(always)]
    pub const fn timer13(&self) -> &Timer13 {
        &self.timer13
    }
    #[doc = "0x3a8 - This contains the Compare limits for timer 13. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr13cmp0(&self) -> &Tmr13cmp0 {
        &self.tmr13cmp0
    }
    #[doc = "0x3ac - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr13cmp1(&self) -> &Tmr13cmp1 {
        &self.tmr13cmp1
    }
    #[doc = "0x3b0 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode13(&self) -> &Mode13 {
        &self.mode13
    }
    #[doc = "0x3b4 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr13lmtval(&self) -> &Tmr13lmtval {
        &self.tmr13lmtval
    }
    #[doc = "0x3c0 - This includes the Control bit fields for timer 14."]
    #[inline(always)]
    pub const fn ctrl14(&self) -> &Ctrl14 {
        &self.ctrl14
    }
    #[doc = "0x3c4 - This register holds the running time or event count for timer 14."]
    #[inline(always)]
    pub const fn timer14(&self) -> &Timer14 {
        &self.timer14
    }
    #[doc = "0x3c8 - This contains the Compare limits for timer 14. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr14cmp0(&self) -> &Tmr14cmp0 {
        &self.tmr14cmp0
    }
    #[doc = "0x3cc - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr14cmp1(&self) -> &Tmr14cmp1 {
        &self.tmr14cmp1
    }
    #[doc = "0x3d0 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode14(&self) -> &Mode14 {
        &self.mode14
    }
    #[doc = "0x3d4 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr14lmtval(&self) -> &Tmr14lmtval {
        &self.tmr14lmtval
    }
    #[doc = "0x3e0 - This includes the Control bit fields for timer 15."]
    #[inline(always)]
    pub const fn ctrl15(&self) -> &Ctrl15 {
        &self.ctrl15
    }
    #[doc = "0x3e4 - This register holds the running time or event count for timer 15."]
    #[inline(always)]
    pub const fn timer15(&self) -> &Timer15 {
        &self.timer15
    }
    #[doc = "0x3e8 - This contains the Compare limits for timer 15. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
    #[inline(always)]
    pub const fn tmr15cmp0(&self) -> &Tmr15cmp0 {
        &self.tmr15cmp0
    }
    #[doc = "0x3ec - This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
    #[inline(always)]
    pub const fn tmr15cmp1(&self) -> &Tmr15cmp1 {
        &self.tmr15cmp1
    }
    #[doc = "0x3f0 - The mode register contains optional mode controls for the timer"]
    #[inline(always)]
    pub const fn mode15(&self) -> &Mode15 {
        &self.mode15
    }
    #[doc = "0x3f4 - This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
    #[inline(always)]
    pub const fn tmr15lmtval(&self) -> &Tmr15lmtval {
        &self.tmr15lmtval
    }
    #[doc = "0x3f8 - Timer Spare Regs"]
    #[inline(always)]
    pub const fn timerspares(&self) -> &Timerspares {
        &self.timerspares
    }
}
#[doc = "CTRL (rw) register accessor: General Timer Controls\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "General Timer Controls"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: General Timer status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "General Timer status"]
pub mod status;
#[doc = "GLOBEN (rw) register accessor: Alternate enables for all TIMERs.\n\nYou can [`read`](crate::Reg::read) this register and get [`globen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`globen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globen`]
module"]
#[doc(alias = "GLOBEN")]
pub type Globen = crate::Reg<globen::GlobenSpec>;
#[doc = "Alternate enables for all TIMERs."]
pub mod globen;
#[doc = "INTEN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod inten;
#[doc = "INTSTAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod intstat;
#[doc = "INTCLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod intclr;
#[doc = "INTSET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`intset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`]
module"]
#[doc(alias = "INTSET")]
pub type Intset = crate::Reg<intset::IntsetSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod intset;
#[doc = "OUTCFG0 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg0`]
module"]
#[doc(alias = "OUTCFG0")]
pub type Outcfg0 = crate::Reg<outcfg0::Outcfg0Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg0;
#[doc = "OUTCFG1 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg1`]
module"]
#[doc(alias = "OUTCFG1")]
pub type Outcfg1 = crate::Reg<outcfg1::Outcfg1Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg1;
#[doc = "OUTCFG2 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg2`]
module"]
#[doc(alias = "OUTCFG2")]
pub type Outcfg2 = crate::Reg<outcfg2::Outcfg2Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg2;
#[doc = "OUTCFG3 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg3`]
module"]
#[doc(alias = "OUTCFG3")]
pub type Outcfg3 = crate::Reg<outcfg3::Outcfg3Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg3;
#[doc = "OUTCFG4 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg4`]
module"]
#[doc(alias = "OUTCFG4")]
pub type Outcfg4 = crate::Reg<outcfg4::Outcfg4Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg4;
#[doc = "OUTCFG5 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg5`]
module"]
#[doc(alias = "OUTCFG5")]
pub type Outcfg5 = crate::Reg<outcfg5::Outcfg5Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg5;
#[doc = "OUTCFG6 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg6`]
module"]
#[doc(alias = "OUTCFG6")]
pub type Outcfg6 = crate::Reg<outcfg6::Outcfg6Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg6;
#[doc = "OUTCFG7 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg7`]
module"]
#[doc(alias = "OUTCFG7")]
pub type Outcfg7 = crate::Reg<outcfg7::Outcfg7Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg7;
#[doc = "OUTCFG8 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg8`]
module"]
#[doc(alias = "OUTCFG8")]
pub type Outcfg8 = crate::Reg<outcfg8::Outcfg8Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg8;
#[doc = "OUTCFG9 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg9`]
module"]
#[doc(alias = "OUTCFG9")]
pub type Outcfg9 = crate::Reg<outcfg9::Outcfg9Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg9;
#[doc = "OUTCFG10 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg10`]
module"]
#[doc(alias = "OUTCFG10")]
pub type Outcfg10 = crate::Reg<outcfg10::Outcfg10Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg10;
#[doc = "OUTCFG11 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg11`]
module"]
#[doc(alias = "OUTCFG11")]
pub type Outcfg11 = crate::Reg<outcfg11::Outcfg11Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg11;
#[doc = "OUTCFG12 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg12`]
module"]
#[doc(alias = "OUTCFG12")]
pub type Outcfg12 = crate::Reg<outcfg12::Outcfg12Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg12;
#[doc = "OUTCFG13 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg13`]
module"]
#[doc(alias = "OUTCFG13")]
pub type Outcfg13 = crate::Reg<outcfg13::Outcfg13Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg13;
#[doc = "OUTCFG14 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg14`]
module"]
#[doc(alias = "OUTCFG14")]
pub type Outcfg14 = crate::Reg<outcfg14::Outcfg14Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg14;
#[doc = "OUTCFG15 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg15`]
module"]
#[doc(alias = "OUTCFG15")]
pub type Outcfg15 = crate::Reg<outcfg15::Outcfg15Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg15;
#[doc = "OUTCFG16 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg16`]
module"]
#[doc(alias = "OUTCFG16")]
pub type Outcfg16 = crate::Reg<outcfg16::Outcfg16Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg16;
#[doc = "OUTCFG17 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg17`]
module"]
#[doc(alias = "OUTCFG17")]
pub type Outcfg17 = crate::Reg<outcfg17::Outcfg17Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg17;
#[doc = "OUTCFG18 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg18`]
module"]
#[doc(alias = "OUTCFG18")]
pub type Outcfg18 = crate::Reg<outcfg18::Outcfg18Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg18;
#[doc = "OUTCFG19 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg19`]
module"]
#[doc(alias = "OUTCFG19")]
pub type Outcfg19 = crate::Reg<outcfg19::Outcfg19Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg19;
#[doc = "OUTCFG20 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg20`]
module"]
#[doc(alias = "OUTCFG20")]
pub type Outcfg20 = crate::Reg<outcfg20::Outcfg20Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg20;
#[doc = "OUTCFG21 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg21`]
module"]
#[doc(alias = "OUTCFG21")]
pub type Outcfg21 = crate::Reg<outcfg21::Outcfg21Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg21;
#[doc = "OUTCFG22 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg22`]
module"]
#[doc(alias = "OUTCFG22")]
pub type Outcfg22 = crate::Reg<outcfg22::Outcfg22Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg22;
#[doc = "OUTCFG23 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg23`]
module"]
#[doc(alias = "OUTCFG23")]
pub type Outcfg23 = crate::Reg<outcfg23::Outcfg23Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg23;
#[doc = "OUTCFG24 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg24`]
module"]
#[doc(alias = "OUTCFG24")]
pub type Outcfg24 = crate::Reg<outcfg24::Outcfg24Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg24;
#[doc = "OUTCFG25 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg25`]
module"]
#[doc(alias = "OUTCFG25")]
pub type Outcfg25 = crate::Reg<outcfg25::Outcfg25Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg25;
#[doc = "OUTCFG26 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg26`]
module"]
#[doc(alias = "OUTCFG26")]
pub type Outcfg26 = crate::Reg<outcfg26::Outcfg26Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg26;
#[doc = "OUTCFG27 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg27`]
module"]
#[doc(alias = "OUTCFG27")]
pub type Outcfg27 = crate::Reg<outcfg27::Outcfg27Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg27;
#[doc = "OUTCFG28 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg28`]
module"]
#[doc(alias = "OUTCFG28")]
pub type Outcfg28 = crate::Reg<outcfg28::Outcfg28Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg28;
#[doc = "OUTCFG29 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg29`]
module"]
#[doc(alias = "OUTCFG29")]
pub type Outcfg29 = crate::Reg<outcfg29::Outcfg29Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg29;
#[doc = "OUTCFG30 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg30`]
module"]
#[doc(alias = "OUTCFG30")]
pub type Outcfg30 = crate::Reg<outcfg30::Outcfg30Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg30;
#[doc = "OUTCFG31 (rw) register accessor: Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcfg31`]
module"]
#[doc(alias = "OUTCFG31")]
pub type Outcfg31 = crate::Reg<outcfg31::Outcfg31Spec>;
#[doc = "Pad output configuration 0."]
pub mod outcfg31;
#[doc = "CTRL0 (rw) register accessor: This includes the Control bit fields for timer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "This includes the Control bit fields for timer 0."]
pub mod ctrl0;
#[doc = "TIMER0 (rw) register accessor: This register holds the running time or event count for timer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0`]
module"]
#[doc(alias = "TIMER0")]
pub type Timer0 = crate::Reg<timer0::Timer0Spec>;
#[doc = "This register holds the running time or event count for timer 0."]
pub mod timer0;
#[doc = "TMR0CMP0 (rw) register accessor: This contains the Compare limits for timer 0. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr0cmp0`]
module"]
#[doc(alias = "TMR0CMP0")]
pub type Tmr0cmp0 = crate::Reg<tmr0cmp0::Tmr0cmp0Spec>;
#[doc = "This contains the Compare limits for timer 0. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr0cmp0;
#[doc = "TMR0CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr0cmp1`]
module"]
#[doc(alias = "TMR0CMP1")]
pub type Tmr0cmp1 = crate::Reg<tmr0cmp1::Tmr0cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr0cmp1;
#[doc = "MODE0 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode0`]
module"]
#[doc(alias = "MODE0")]
pub type Mode0 = crate::Reg<mode0::Mode0Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode0;
#[doc = "TMR0LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr0lmtval`]
module"]
#[doc(alias = "TMR0LMTVAL")]
pub type Tmr0lmtval = crate::Reg<tmr0lmtval::Tmr0lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr0lmtval;
#[doc = "CTRL1 (rw) register accessor: This includes the Control bit fields for timer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "This includes the Control bit fields for timer 1."]
pub mod ctrl1;
#[doc = "TIMER1 (rw) register accessor: This register holds the running time or event count for timer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1`]
module"]
#[doc(alias = "TIMER1")]
pub type Timer1 = crate::Reg<timer1::Timer1Spec>;
#[doc = "This register holds the running time or event count for timer 1."]
pub mod timer1;
#[doc = "TMR1CMP0 (rw) register accessor: This contains the Compare limits for timer 1. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr1cmp0`]
module"]
#[doc(alias = "TMR1CMP0")]
pub type Tmr1cmp0 = crate::Reg<tmr1cmp0::Tmr1cmp0Spec>;
#[doc = "This contains the Compare limits for timer 1. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr1cmp0;
#[doc = "TMR1CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr1cmp1`]
module"]
#[doc(alias = "TMR1CMP1")]
pub type Tmr1cmp1 = crate::Reg<tmr1cmp1::Tmr1cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr1cmp1;
#[doc = "MODE1 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode1`]
module"]
#[doc(alias = "MODE1")]
pub type Mode1 = crate::Reg<mode1::Mode1Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode1;
#[doc = "TMR1LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr1lmtval`]
module"]
#[doc(alias = "TMR1LMTVAL")]
pub type Tmr1lmtval = crate::Reg<tmr1lmtval::Tmr1lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr1lmtval;
#[doc = "CTRL2 (rw) register accessor: This includes the Control bit fields for timer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "This includes the Control bit fields for timer 2."]
pub mod ctrl2;
#[doc = "TIMER2 (rw) register accessor: This register holds the running time or event count for timer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2`]
module"]
#[doc(alias = "TIMER2")]
pub type Timer2 = crate::Reg<timer2::Timer2Spec>;
#[doc = "This register holds the running time or event count for timer 2."]
pub mod timer2;
#[doc = "TMR2CMP0 (rw) register accessor: This contains the Compare limits for timer 2. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2cmp0`]
module"]
#[doc(alias = "TMR2CMP0")]
pub type Tmr2cmp0 = crate::Reg<tmr2cmp0::Tmr2cmp0Spec>;
#[doc = "This contains the Compare limits for timer 2. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr2cmp0;
#[doc = "TMR2CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2cmp1`]
module"]
#[doc(alias = "TMR2CMP1")]
pub type Tmr2cmp1 = crate::Reg<tmr2cmp1::Tmr2cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr2cmp1;
#[doc = "MODE2 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode2`]
module"]
#[doc(alias = "MODE2")]
pub type Mode2 = crate::Reg<mode2::Mode2Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode2;
#[doc = "TMR2LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2lmtval`]
module"]
#[doc(alias = "TMR2LMTVAL")]
pub type Tmr2lmtval = crate::Reg<tmr2lmtval::Tmr2lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr2lmtval;
#[doc = "CTRL3 (rw) register accessor: This includes the Control bit fields for timer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl3`]
module"]
#[doc(alias = "CTRL3")]
pub type Ctrl3 = crate::Reg<ctrl3::Ctrl3Spec>;
#[doc = "This includes the Control bit fields for timer 3."]
pub mod ctrl3;
#[doc = "TIMER3 (rw) register accessor: This register holds the running time or event count for timer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3`]
module"]
#[doc(alias = "TIMER3")]
pub type Timer3 = crate::Reg<timer3::Timer3Spec>;
#[doc = "This register holds the running time or event count for timer 3."]
pub mod timer3;
#[doc = "TMR3CMP0 (rw) register accessor: This contains the Compare limits for timer 3. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr3cmp0`]
module"]
#[doc(alias = "TMR3CMP0")]
pub type Tmr3cmp0 = crate::Reg<tmr3cmp0::Tmr3cmp0Spec>;
#[doc = "This contains the Compare limits for timer 3. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr3cmp0;
#[doc = "TMR3CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr3cmp1`]
module"]
#[doc(alias = "TMR3CMP1")]
pub type Tmr3cmp1 = crate::Reg<tmr3cmp1::Tmr3cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr3cmp1;
#[doc = "MODE3 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode3`]
module"]
#[doc(alias = "MODE3")]
pub type Mode3 = crate::Reg<mode3::Mode3Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode3;
#[doc = "TMR3LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr3lmtval`]
module"]
#[doc(alias = "TMR3LMTVAL")]
pub type Tmr3lmtval = crate::Reg<tmr3lmtval::Tmr3lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr3lmtval;
#[doc = "CTRL4 (rw) register accessor: This includes the Control bit fields for timer 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl4`]
module"]
#[doc(alias = "CTRL4")]
pub type Ctrl4 = crate::Reg<ctrl4::Ctrl4Spec>;
#[doc = "This includes the Control bit fields for timer 4."]
pub mod ctrl4;
#[doc = "TIMER4 (rw) register accessor: This register holds the running time or event count for timer 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4`]
module"]
#[doc(alias = "TIMER4")]
pub type Timer4 = crate::Reg<timer4::Timer4Spec>;
#[doc = "This register holds the running time or event count for timer 4."]
pub mod timer4;
#[doc = "TMR4CMP0 (rw) register accessor: This contains the Compare limits for timer 4. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr4cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr4cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr4cmp0`]
module"]
#[doc(alias = "TMR4CMP0")]
pub type Tmr4cmp0 = crate::Reg<tmr4cmp0::Tmr4cmp0Spec>;
#[doc = "This contains the Compare limits for timer 4. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr4cmp0;
#[doc = "TMR4CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr4cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr4cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr4cmp1`]
module"]
#[doc(alias = "TMR4CMP1")]
pub type Tmr4cmp1 = crate::Reg<tmr4cmp1::Tmr4cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr4cmp1;
#[doc = "MODE4 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode4`]
module"]
#[doc(alias = "MODE4")]
pub type Mode4 = crate::Reg<mode4::Mode4Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode4;
#[doc = "TMR4LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr4lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr4lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr4lmtval`]
module"]
#[doc(alias = "TMR4LMTVAL")]
pub type Tmr4lmtval = crate::Reg<tmr4lmtval::Tmr4lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr4lmtval;
#[doc = "CTRL5 (rw) register accessor: This includes the Control bit fields for timer 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl5`]
module"]
#[doc(alias = "CTRL5")]
pub type Ctrl5 = crate::Reg<ctrl5::Ctrl5Spec>;
#[doc = "This includes the Control bit fields for timer 5."]
pub mod ctrl5;
#[doc = "TIMER5 (rw) register accessor: This register holds the running time or event count for timer 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer5`]
module"]
#[doc(alias = "TIMER5")]
pub type Timer5 = crate::Reg<timer5::Timer5Spec>;
#[doc = "This register holds the running time or event count for timer 5."]
pub mod timer5;
#[doc = "TMR5CMP0 (rw) register accessor: This contains the Compare limits for timer 5. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr5cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr5cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr5cmp0`]
module"]
#[doc(alias = "TMR5CMP0")]
pub type Tmr5cmp0 = crate::Reg<tmr5cmp0::Tmr5cmp0Spec>;
#[doc = "This contains the Compare limits for timer 5. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr5cmp0;
#[doc = "TMR5CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr5cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr5cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr5cmp1`]
module"]
#[doc(alias = "TMR5CMP1")]
pub type Tmr5cmp1 = crate::Reg<tmr5cmp1::Tmr5cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr5cmp1;
#[doc = "MODE5 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode5`]
module"]
#[doc(alias = "MODE5")]
pub type Mode5 = crate::Reg<mode5::Mode5Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode5;
#[doc = "TMR5LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr5lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr5lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr5lmtval`]
module"]
#[doc(alias = "TMR5LMTVAL")]
pub type Tmr5lmtval = crate::Reg<tmr5lmtval::Tmr5lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr5lmtval;
#[doc = "CTRL6 (rw) register accessor: This includes the Control bit fields for timer 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl6`]
module"]
#[doc(alias = "CTRL6")]
pub type Ctrl6 = crate::Reg<ctrl6::Ctrl6Spec>;
#[doc = "This includes the Control bit fields for timer 6."]
pub mod ctrl6;
#[doc = "TIMER6 (rw) register accessor: This register holds the running time or event count for timer 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer6`]
module"]
#[doc(alias = "TIMER6")]
pub type Timer6 = crate::Reg<timer6::Timer6Spec>;
#[doc = "This register holds the running time or event count for timer 6."]
pub mod timer6;
#[doc = "TMR6CMP0 (rw) register accessor: This contains the Compare limits for timer 6. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr6cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr6cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr6cmp0`]
module"]
#[doc(alias = "TMR6CMP0")]
pub type Tmr6cmp0 = crate::Reg<tmr6cmp0::Tmr6cmp0Spec>;
#[doc = "This contains the Compare limits for timer 6. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr6cmp0;
#[doc = "TMR6CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr6cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr6cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr6cmp1`]
module"]
#[doc(alias = "TMR6CMP1")]
pub type Tmr6cmp1 = crate::Reg<tmr6cmp1::Tmr6cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr6cmp1;
#[doc = "MODE6 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode6`]
module"]
#[doc(alias = "MODE6")]
pub type Mode6 = crate::Reg<mode6::Mode6Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode6;
#[doc = "TMR6LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr6lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr6lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr6lmtval`]
module"]
#[doc(alias = "TMR6LMTVAL")]
pub type Tmr6lmtval = crate::Reg<tmr6lmtval::Tmr6lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr6lmtval;
#[doc = "CTRL7 (rw) register accessor: This includes the Control bit fields for timer 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl7`]
module"]
#[doc(alias = "CTRL7")]
pub type Ctrl7 = crate::Reg<ctrl7::Ctrl7Spec>;
#[doc = "This includes the Control bit fields for timer 7."]
pub mod ctrl7;
#[doc = "TIMER7 (rw) register accessor: This register holds the running time or event count for timer 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer7`]
module"]
#[doc(alias = "TIMER7")]
pub type Timer7 = crate::Reg<timer7::Timer7Spec>;
#[doc = "This register holds the running time or event count for timer 7."]
pub mod timer7;
#[doc = "TMR7CMP0 (rw) register accessor: This contains the Compare limits for timer 7. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr7cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr7cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr7cmp0`]
module"]
#[doc(alias = "TMR7CMP0")]
pub type Tmr7cmp0 = crate::Reg<tmr7cmp0::Tmr7cmp0Spec>;
#[doc = "This contains the Compare limits for timer 7. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr7cmp0;
#[doc = "TMR7CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr7cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr7cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr7cmp1`]
module"]
#[doc(alias = "TMR7CMP1")]
pub type Tmr7cmp1 = crate::Reg<tmr7cmp1::Tmr7cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr7cmp1;
#[doc = "MODE7 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode7`]
module"]
#[doc(alias = "MODE7")]
pub type Mode7 = crate::Reg<mode7::Mode7Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode7;
#[doc = "TMR7LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr7lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr7lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr7lmtval`]
module"]
#[doc(alias = "TMR7LMTVAL")]
pub type Tmr7lmtval = crate::Reg<tmr7lmtval::Tmr7lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr7lmtval;
#[doc = "CTRL8 (rw) register accessor: This includes the Control bit fields for timer 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl8`]
module"]
#[doc(alias = "CTRL8")]
pub type Ctrl8 = crate::Reg<ctrl8::Ctrl8Spec>;
#[doc = "This includes the Control bit fields for timer 8."]
pub mod ctrl8;
#[doc = "TIMER8 (rw) register accessor: This register holds the running time or event count for timer 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer8`]
module"]
#[doc(alias = "TIMER8")]
pub type Timer8 = crate::Reg<timer8::Timer8Spec>;
#[doc = "This register holds the running time or event count for timer 8."]
pub mod timer8;
#[doc = "TMR8CMP0 (rw) register accessor: This contains the Compare limits for timer 8. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr8cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr8cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr8cmp0`]
module"]
#[doc(alias = "TMR8CMP0")]
pub type Tmr8cmp0 = crate::Reg<tmr8cmp0::Tmr8cmp0Spec>;
#[doc = "This contains the Compare limits for timer 8. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr8cmp0;
#[doc = "TMR8CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr8cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr8cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr8cmp1`]
module"]
#[doc(alias = "TMR8CMP1")]
pub type Tmr8cmp1 = crate::Reg<tmr8cmp1::Tmr8cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr8cmp1;
#[doc = "MODE8 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode8`]
module"]
#[doc(alias = "MODE8")]
pub type Mode8 = crate::Reg<mode8::Mode8Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode8;
#[doc = "TMR8LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr8lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr8lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr8lmtval`]
module"]
#[doc(alias = "TMR8LMTVAL")]
pub type Tmr8lmtval = crate::Reg<tmr8lmtval::Tmr8lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr8lmtval;
#[doc = "CTRL9 (rw) register accessor: This includes the Control bit fields for timer 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl9`]
module"]
#[doc(alias = "CTRL9")]
pub type Ctrl9 = crate::Reg<ctrl9::Ctrl9Spec>;
#[doc = "This includes the Control bit fields for timer 9."]
pub mod ctrl9;
#[doc = "TIMER9 (rw) register accessor: This register holds the running time or event count for timer 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer9`]
module"]
#[doc(alias = "TIMER9")]
pub type Timer9 = crate::Reg<timer9::Timer9Spec>;
#[doc = "This register holds the running time or event count for timer 9."]
pub mod timer9;
#[doc = "TMR9CMP0 (rw) register accessor: This contains the Compare limits for timer 9. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr9cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr9cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr9cmp0`]
module"]
#[doc(alias = "TMR9CMP0")]
pub type Tmr9cmp0 = crate::Reg<tmr9cmp0::Tmr9cmp0Spec>;
#[doc = "This contains the Compare limits for timer 9. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr9cmp0;
#[doc = "TMR9CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr9cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr9cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr9cmp1`]
module"]
#[doc(alias = "TMR9CMP1")]
pub type Tmr9cmp1 = crate::Reg<tmr9cmp1::Tmr9cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr9cmp1;
#[doc = "MODE9 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode9`]
module"]
#[doc(alias = "MODE9")]
pub type Mode9 = crate::Reg<mode9::Mode9Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode9;
#[doc = "TMR9LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr9lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr9lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr9lmtval`]
module"]
#[doc(alias = "TMR9LMTVAL")]
pub type Tmr9lmtval = crate::Reg<tmr9lmtval::Tmr9lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr9lmtval;
#[doc = "CTRL10 (rw) register accessor: This includes the Control bit fields for timer 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl10`]
module"]
#[doc(alias = "CTRL10")]
pub type Ctrl10 = crate::Reg<ctrl10::Ctrl10Spec>;
#[doc = "This includes the Control bit fields for timer 10."]
pub mod ctrl10;
#[doc = "TIMER10 (rw) register accessor: This register holds the running time or event count for timer 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer10`]
module"]
#[doc(alias = "TIMER10")]
pub type Timer10 = crate::Reg<timer10::Timer10Spec>;
#[doc = "This register holds the running time or event count for timer 10."]
pub mod timer10;
#[doc = "TMR10CMP0 (rw) register accessor: This contains the Compare limits for timer 10. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr10cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr10cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr10cmp0`]
module"]
#[doc(alias = "TMR10CMP0")]
pub type Tmr10cmp0 = crate::Reg<tmr10cmp0::Tmr10cmp0Spec>;
#[doc = "This contains the Compare limits for timer 10. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr10cmp0;
#[doc = "TMR10CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr10cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr10cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr10cmp1`]
module"]
#[doc(alias = "TMR10CMP1")]
pub type Tmr10cmp1 = crate::Reg<tmr10cmp1::Tmr10cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr10cmp1;
#[doc = "MODE10 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode10`]
module"]
#[doc(alias = "MODE10")]
pub type Mode10 = crate::Reg<mode10::Mode10Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode10;
#[doc = "TMR10LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr10lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr10lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr10lmtval`]
module"]
#[doc(alias = "TMR10LMTVAL")]
pub type Tmr10lmtval = crate::Reg<tmr10lmtval::Tmr10lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr10lmtval;
#[doc = "CTRL11 (rw) register accessor: This includes the Control bit fields for timer 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl11`]
module"]
#[doc(alias = "CTRL11")]
pub type Ctrl11 = crate::Reg<ctrl11::Ctrl11Spec>;
#[doc = "This includes the Control bit fields for timer 11."]
pub mod ctrl11;
#[doc = "TIMER11 (rw) register accessor: This register holds the running time or event count for timer 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer11`]
module"]
#[doc(alias = "TIMER11")]
pub type Timer11 = crate::Reg<timer11::Timer11Spec>;
#[doc = "This register holds the running time or event count for timer 11."]
pub mod timer11;
#[doc = "TMR11CMP0 (rw) register accessor: This contains the Compare limits for timer 11. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr11cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr11cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr11cmp0`]
module"]
#[doc(alias = "TMR11CMP0")]
pub type Tmr11cmp0 = crate::Reg<tmr11cmp0::Tmr11cmp0Spec>;
#[doc = "This contains the Compare limits for timer 11. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr11cmp0;
#[doc = "TMR11CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr11cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr11cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr11cmp1`]
module"]
#[doc(alias = "TMR11CMP1")]
pub type Tmr11cmp1 = crate::Reg<tmr11cmp1::Tmr11cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr11cmp1;
#[doc = "MODE11 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode11`]
module"]
#[doc(alias = "MODE11")]
pub type Mode11 = crate::Reg<mode11::Mode11Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode11;
#[doc = "TMR11LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr11lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr11lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr11lmtval`]
module"]
#[doc(alias = "TMR11LMTVAL")]
pub type Tmr11lmtval = crate::Reg<tmr11lmtval::Tmr11lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr11lmtval;
#[doc = "CTRL12 (rw) register accessor: This includes the Control bit fields for timer 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl12`]
module"]
#[doc(alias = "CTRL12")]
pub type Ctrl12 = crate::Reg<ctrl12::Ctrl12Spec>;
#[doc = "This includes the Control bit fields for timer 12."]
pub mod ctrl12;
#[doc = "TIMER12 (rw) register accessor: This register holds the running time or event count for timer 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer12`]
module"]
#[doc(alias = "TIMER12")]
pub type Timer12 = crate::Reg<timer12::Timer12Spec>;
#[doc = "This register holds the running time or event count for timer 12."]
pub mod timer12;
#[doc = "TMR12CMP0 (rw) register accessor: This contains the Compare limits for timer 12. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr12cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr12cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr12cmp0`]
module"]
#[doc(alias = "TMR12CMP0")]
pub type Tmr12cmp0 = crate::Reg<tmr12cmp0::Tmr12cmp0Spec>;
#[doc = "This contains the Compare limits for timer 12. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr12cmp0;
#[doc = "TMR12CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr12cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr12cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr12cmp1`]
module"]
#[doc(alias = "TMR12CMP1")]
pub type Tmr12cmp1 = crate::Reg<tmr12cmp1::Tmr12cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr12cmp1;
#[doc = "MODE12 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode12`]
module"]
#[doc(alias = "MODE12")]
pub type Mode12 = crate::Reg<mode12::Mode12Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode12;
#[doc = "TMR12LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr12lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr12lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr12lmtval`]
module"]
#[doc(alias = "TMR12LMTVAL")]
pub type Tmr12lmtval = crate::Reg<tmr12lmtval::Tmr12lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr12lmtval;
#[doc = "CTRL13 (rw) register accessor: This includes the Control bit fields for timer 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl13`]
module"]
#[doc(alias = "CTRL13")]
pub type Ctrl13 = crate::Reg<ctrl13::Ctrl13Spec>;
#[doc = "This includes the Control bit fields for timer 13."]
pub mod ctrl13;
#[doc = "TIMER13 (rw) register accessor: This register holds the running time or event count for timer 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer13`]
module"]
#[doc(alias = "TIMER13")]
pub type Timer13 = crate::Reg<timer13::Timer13Spec>;
#[doc = "This register holds the running time or event count for timer 13."]
pub mod timer13;
#[doc = "TMR13CMP0 (rw) register accessor: This contains the Compare limits for timer 13. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr13cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr13cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr13cmp0`]
module"]
#[doc(alias = "TMR13CMP0")]
pub type Tmr13cmp0 = crate::Reg<tmr13cmp0::Tmr13cmp0Spec>;
#[doc = "This contains the Compare limits for timer 13. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr13cmp0;
#[doc = "TMR13CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr13cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr13cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr13cmp1`]
module"]
#[doc(alias = "TMR13CMP1")]
pub type Tmr13cmp1 = crate::Reg<tmr13cmp1::Tmr13cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr13cmp1;
#[doc = "MODE13 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode13`]
module"]
#[doc(alias = "MODE13")]
pub type Mode13 = crate::Reg<mode13::Mode13Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode13;
#[doc = "TMR13LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr13lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr13lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr13lmtval`]
module"]
#[doc(alias = "TMR13LMTVAL")]
pub type Tmr13lmtval = crate::Reg<tmr13lmtval::Tmr13lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr13lmtval;
#[doc = "CTRL14 (rw) register accessor: This includes the Control bit fields for timer 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl14`]
module"]
#[doc(alias = "CTRL14")]
pub type Ctrl14 = crate::Reg<ctrl14::Ctrl14Spec>;
#[doc = "This includes the Control bit fields for timer 14."]
pub mod ctrl14;
#[doc = "TIMER14 (rw) register accessor: This register holds the running time or event count for timer 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer14`]
module"]
#[doc(alias = "TIMER14")]
pub type Timer14 = crate::Reg<timer14::Timer14Spec>;
#[doc = "This register holds the running time or event count for timer 14."]
pub mod timer14;
#[doc = "TMR14CMP0 (rw) register accessor: This contains the Compare limits for timer 14. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr14cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr14cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr14cmp0`]
module"]
#[doc(alias = "TMR14CMP0")]
pub type Tmr14cmp0 = crate::Reg<tmr14cmp0::Tmr14cmp0Spec>;
#[doc = "This contains the Compare limits for timer 14. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr14cmp0;
#[doc = "TMR14CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr14cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr14cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr14cmp1`]
module"]
#[doc(alias = "TMR14CMP1")]
pub type Tmr14cmp1 = crate::Reg<tmr14cmp1::Tmr14cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr14cmp1;
#[doc = "MODE14 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode14`]
module"]
#[doc(alias = "MODE14")]
pub type Mode14 = crate::Reg<mode14::Mode14Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode14;
#[doc = "TMR14LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr14lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr14lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr14lmtval`]
module"]
#[doc(alias = "TMR14LMTVAL")]
pub type Tmr14lmtval = crate::Reg<tmr14lmtval::Tmr14lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr14lmtval;
#[doc = "CTRL15 (rw) register accessor: This includes the Control bit fields for timer 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl15`]
module"]
#[doc(alias = "CTRL15")]
pub type Ctrl15 = crate::Reg<ctrl15::Ctrl15Spec>;
#[doc = "This includes the Control bit fields for timer 15."]
pub mod ctrl15;
#[doc = "TIMER15 (rw) register accessor: This register holds the running time or event count for timer 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer15`]
module"]
#[doc(alias = "TIMER15")]
pub type Timer15 = crate::Reg<timer15::Timer15Spec>;
#[doc = "This register holds the running time or event count for timer 15."]
pub mod timer15;
#[doc = "TMR15CMP0 (rw) register accessor: This contains the Compare limits for timer 15. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr15cmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr15cmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr15cmp0`]
module"]
#[doc(alias = "TMR15CMP0")]
pub type Tmr15cmp0 = crate::Reg<tmr15cmp0::Tmr15cmp0Spec>;
#[doc = "This contains the Compare limits for timer 15. This is the primary comparator that can be used to mark the END of a timer cycle (and thus restart the timer for repeat modes)"]
pub mod tmr15cmp0;
#[doc = "TMR15CMP1 (rw) register accessor: This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr15cmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr15cmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr15cmp1`]
module"]
#[doc(alias = "TMR15CMP1")]
pub type Tmr15cmp1 = crate::Reg<tmr15cmp1::Tmr15cmp1Spec>;
#[doc = "This comparator is used as a secondary compare count for modes that generate pulses. For MEASURE mode indicates the low phase sample count."]
pub mod tmr15cmp1;
#[doc = "MODE15 (rw) register accessor: The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode15`]
module"]
#[doc(alias = "MODE15")]
pub type Mode15 = crate::Reg<mode15::Mode15Spec>;
#[doc = "The mode register contains optional mode controls for the timer"]
pub mod mode15;
#[doc = "TMR15LMTVAL (rw) register accessor: This is an internal counter in the hardware that counts down from TMR_LMT to 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr15lmtval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr15lmtval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr15lmtval`]
module"]
#[doc(alias = "TMR15LMTVAL")]
pub type Tmr15lmtval = crate::Reg<tmr15lmtval::Tmr15lmtvalSpec>;
#[doc = "This is an internal counter in the hardware that counts down from TMR_LMT to 1"]
pub mod tmr15lmtval;
#[doc = "TIMERSPARES (rw) register accessor: Timer Spare Regs\n\nYou can [`read`](crate::Reg::read) this register and get [`timerspares::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerspares::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerspares`]
module"]
#[doc(alias = "TIMERSPARES")]
pub type Timerspares = crate::Reg<timerspares::TimersparesSpec>;
#[doc = "Timer Spare Regs"]
pub mod timerspares;
