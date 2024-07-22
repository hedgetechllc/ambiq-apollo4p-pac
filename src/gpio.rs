#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pincfg0: Pincfg0,
    pincfg1: Pincfg1,
    pincfg2: Pincfg2,
    pincfg3: Pincfg3,
    pincfg4: Pincfg4,
    pincfg5: Pincfg5,
    pincfg6: Pincfg6,
    pincfg7: Pincfg7,
    pincfg8: Pincfg8,
    pincfg9: Pincfg9,
    pincfg10: Pincfg10,
    pincfg11: Pincfg11,
    pincfg12: Pincfg12,
    pincfg13: Pincfg13,
    pincfg14: Pincfg14,
    pincfg15: Pincfg15,
    pincfg16: Pincfg16,
    pincfg17: Pincfg17,
    pincfg18: Pincfg18,
    pincfg19: Pincfg19,
    pincfg20: Pincfg20,
    pincfg21: Pincfg21,
    pincfg22: Pincfg22,
    pincfg23: Pincfg23,
    pincfg24: Pincfg24,
    pincfg25: Pincfg25,
    pincfg26: Pincfg26,
    pincfg27: Pincfg27,
    pincfg28: Pincfg28,
    pincfg29: Pincfg29,
    pincfg30: Pincfg30,
    pincfg31: Pincfg31,
    pincfg32: Pincfg32,
    pincfg33: Pincfg33,
    pincfg34: Pincfg34,
    pincfg35: Pincfg35,
    pincfg36: Pincfg36,
    pincfg37: Pincfg37,
    pincfg38: Pincfg38,
    pincfg39: Pincfg39,
    pincfg40: Pincfg40,
    pincfg41: Pincfg41,
    pincfg42: Pincfg42,
    pincfg43: Pincfg43,
    pincfg44: Pincfg44,
    pincfg45: Pincfg45,
    pincfg46: Pincfg46,
    pincfg47: Pincfg47,
    pincfg48: Pincfg48,
    pincfg49: Pincfg49,
    pincfg50: Pincfg50,
    pincfg51: Pincfg51,
    pincfg52: Pincfg52,
    pincfg53: Pincfg53,
    pincfg54: Pincfg54,
    pincfg55: Pincfg55,
    pincfg56: Pincfg56,
    pincfg57: Pincfg57,
    pincfg58: Pincfg58,
    pincfg59: Pincfg59,
    pincfg60: Pincfg60,
    pincfg61: Pincfg61,
    pincfg62: Pincfg62,
    pincfg63: Pincfg63,
    pincfg64: Pincfg64,
    pincfg65: Pincfg65,
    pincfg66: Pincfg66,
    pincfg67: Pincfg67,
    pincfg68: Pincfg68,
    pincfg69: Pincfg69,
    pincfg70: Pincfg70,
    pincfg71: Pincfg71,
    pincfg72: Pincfg72,
    pincfg73: Pincfg73,
    pincfg74: Pincfg74,
    pincfg75: Pincfg75,
    pincfg76: Pincfg76,
    pincfg77: Pincfg77,
    pincfg78: Pincfg78,
    pincfg79: Pincfg79,
    pincfg80: Pincfg80,
    pincfg81: Pincfg81,
    pincfg82: Pincfg82,
    pincfg83: Pincfg83,
    pincfg84: Pincfg84,
    pincfg85: Pincfg85,
    pincfg86: Pincfg86,
    pincfg87: Pincfg87,
    pincfg88: Pincfg88,
    pincfg89: Pincfg89,
    pincfg90: Pincfg90,
    pincfg91: Pincfg91,
    pincfg92: Pincfg92,
    pincfg93: Pincfg93,
    pincfg94: Pincfg94,
    pincfg95: Pincfg95,
    pincfg96: Pincfg96,
    pincfg97: Pincfg97,
    pincfg98: Pincfg98,
    pincfg99: Pincfg99,
    pincfg100: Pincfg100,
    pincfg101: Pincfg101,
    pincfg102: Pincfg102,
    pincfg103: Pincfg103,
    pincfg104: Pincfg104,
    pincfg105: Pincfg105,
    pincfg106: Pincfg106,
    pincfg107: Pincfg107,
    pincfg108: Pincfg108,
    pincfg109: Pincfg109,
    pincfg110: Pincfg110,
    pincfg111: Pincfg111,
    pincfg112: Pincfg112,
    pincfg113: Pincfg113,
    pincfg114: Pincfg114,
    pincfg115: Pincfg115,
    pincfg116: Pincfg116,
    pincfg117: Pincfg117,
    pincfg118: Pincfg118,
    pincfg119: Pincfg119,
    pincfg120: Pincfg120,
    pincfg121: Pincfg121,
    pincfg122: Pincfg122,
    pincfg123: Pincfg123,
    pincfg124: Pincfg124,
    pincfg125: Pincfg125,
    pincfg126: Pincfg126,
    pincfg127: Pincfg127,
    padkey: Padkey,
    rd0: Rd0,
    rd1: Rd1,
    rd2: Rd2,
    rd3: Rd3,
    wt0: Wt0,
    wt1: Wt1,
    wt2: Wt2,
    wt3: Wt3,
    wts0: Wts0,
    wts1: Wts1,
    wts2: Wts2,
    wts3: Wts3,
    wtc0: Wtc0,
    wtc1: Wtc1,
    wtc2: Wtc2,
    wtc3: Wtc3,
    en0: En0,
    en1: En1,
    en2: En2,
    en3: En3,
    ens0: Ens0,
    ens1: Ens1,
    ens2: Ens2,
    ens3: Ens3,
    enc0: Enc0,
    enc1: Enc1,
    enc2: Enc2,
    enc3: Enc3,
    iom0irq: Iom0irq,
    iom1irq: Iom1irq,
    iom2irq: Iom2irq,
    iom3irq: Iom3irq,
    iom4irq: Iom4irq,
    iom5irq: Iom5irq,
    iom6irq: Iom6irq,
    iom7irq: Iom7irq,
    sdifcdwp: Sdifcdwp,
    obsdata: Obsdata,
    ieobs0: Ieobs0,
    ieobs1: Ieobs1,
    ieobs2: Ieobs2,
    ieobs3: Ieobs3,
    oeobs0: Oeobs0,
    oeobs1: Oeobs1,
    oeobs2: Oeobs2,
    oeobs3: Oeobs3,
    _reserved175: [u8; 0x04],
    mcun0int0en: Mcun0int0en,
    mcun0int0stat: Mcun0int0stat,
    mcun0int0clr: Mcun0int0clr,
    mcun0int0set: Mcun0int0set,
    mcun0int1en: Mcun0int1en,
    mcun0int1stat: Mcun0int1stat,
    mcun0int1clr: Mcun0int1clr,
    mcun0int1set: Mcun0int1set,
    mcun0int2en: Mcun0int2en,
    mcun0int2stat: Mcun0int2stat,
    mcun0int2clr: Mcun0int2clr,
    mcun0int2set: Mcun0int2set,
    mcun0int3en: Mcun0int3en,
    mcun0int3stat: Mcun0int3stat,
    mcun0int3clr: Mcun0int3clr,
    mcun0int3set: Mcun0int3set,
    mcun1int0en: Mcun1int0en,
    mcun1int0stat: Mcun1int0stat,
    mcun1int0clr: Mcun1int0clr,
    mcun1int0set: Mcun1int0set,
    mcun1int1en: Mcun1int1en,
    mcun1int1stat: Mcun1int1stat,
    mcun1int1clr: Mcun1int1clr,
    mcun1int1set: Mcun1int1set,
    mcun1int2en: Mcun1int2en,
    mcun1int2stat: Mcun1int2stat,
    mcun1int2clr: Mcun1int2clr,
    mcun1int2set: Mcun1int2set,
    mcun1int3en: Mcun1int3en,
    mcun1int3stat: Mcun1int3stat,
    mcun1int3clr: Mcun1int3clr,
    mcun1int3set: Mcun1int3set,
    dsp0n0int0en: Dsp0n0int0en,
    dsp0n0int0stat: Dsp0n0int0stat,
    dsp0n0int0clr: Dsp0n0int0clr,
    dsp0n0int0set: Dsp0n0int0set,
    dsp0n0int1en: Dsp0n0int1en,
    dsp0n0int1stat: Dsp0n0int1stat,
    dsp0n0int1clr: Dsp0n0int1clr,
    dsp0n0int1set: Dsp0n0int1set,
    dsp0n0int2en: Dsp0n0int2en,
    dsp0n0int2stat: Dsp0n0int2stat,
    dsp0n0int2clr: Dsp0n0int2clr,
    dsp0n0int2set: Dsp0n0int2set,
    dsp0n0int3en: Dsp0n0int3en,
    dsp0n0int3stat: Dsp0n0int3stat,
    dsp0n0int3clr: Dsp0n0int3clr,
    dsp0n0int3set: Dsp0n0int3set,
    dsp0n1int0en: Dsp0n1int0en,
    dsp0n1int0stat: Dsp0n1int0stat,
    dsp0n1int0clr: Dsp0n1int0clr,
    dsp0n1int0set: Dsp0n1int0set,
    dsp0n1int1en: Dsp0n1int1en,
    dsp0n1int1stat: Dsp0n1int1stat,
    dsp0n1int1clr: Dsp0n1int1clr,
    dsp0n1int1set: Dsp0n1int1set,
    dsp0n1int2en: Dsp0n1int2en,
    dsp0n1int2stat: Dsp0n1int2stat,
    dsp0n1int2clr: Dsp0n1int2clr,
    dsp0n1int2set: Dsp0n1int2set,
    dsp0n1int3en: Dsp0n1int3en,
    dsp0n1int3stat: Dsp0n1int3stat,
    dsp0n1int3clr: Dsp0n1int3clr,
    dsp0n1int3set: Dsp0n1int3set,
    dsp1n0int0en: Dsp1n0int0en,
    dsp1n0int0stat: Dsp1n0int0stat,
    dsp1n0int0clr: Dsp1n0int0clr,
    dsp1n0int0set: Dsp1n0int0set,
    dsp1n0int1en: Dsp1n0int1en,
    dsp1n0int1stat: Dsp1n0int1stat,
    dsp1n0int1clr: Dsp1n0int1clr,
    dsp1n0int1set: Dsp1n0int1set,
    dsp1n0int2en: Dsp1n0int2en,
    dsp1n0int2stat: Dsp1n0int2stat,
    dsp1n0int2clr: Dsp1n0int2clr,
    dsp1n0int2set: Dsp1n0int2set,
    dsp1n0int3en: Dsp1n0int3en,
    dsp1n0int3stat: Dsp1n0int3stat,
    dsp1n0int3clr: Dsp1n0int3clr,
    dsp1n0int3set: Dsp1n0int3set,
    dsp1n1int0en: Dsp1n1int0en,
    dsp1n1int0stat: Dsp1n1int0stat,
    dsp1n1int0clr: Dsp1n1int0clr,
    dsp1n1int0set: Dsp1n1int0set,
    dsp1n1int1en: Dsp1n1int1en,
    dsp1n1int1stat: Dsp1n1int1stat,
    dsp1n1int1clr: Dsp1n1int1clr,
    dsp1n1int1set: Dsp1n1int1set,
    dsp1n1int2en: Dsp1n1int2en,
    dsp1n1int2stat: Dsp1n1int2stat,
    dsp1n1int2clr: Dsp1n1int2clr,
    dsp1n1int2set: Dsp1n1int2set,
    dsp1n1int3en: Dsp1n1int3en,
    dsp1n1int3stat: Dsp1n1int3stat,
    dsp1n1int3clr: Dsp1n1int3clr,
    dsp1n1int3set: Dsp1n1int3set,
}
impl RegisterBlock {
    #[doc = "0x00 - Controls the operation of GPIO pin 0."]
    #[inline(always)]
    pub const fn pincfg0(&self) -> &Pincfg0 {
        &self.pincfg0
    }
    #[doc = "0x04 - Controls the operation of GPIO pin 1."]
    #[inline(always)]
    pub const fn pincfg1(&self) -> &Pincfg1 {
        &self.pincfg1
    }
    #[doc = "0x08 - Controls the operation of GPIO pin 2."]
    #[inline(always)]
    pub const fn pincfg2(&self) -> &Pincfg2 {
        &self.pincfg2
    }
    #[doc = "0x0c - Controls the operation of GPIO pin 3."]
    #[inline(always)]
    pub const fn pincfg3(&self) -> &Pincfg3 {
        &self.pincfg3
    }
    #[doc = "0x10 - Controls the operation of GPIO pin 4."]
    #[inline(always)]
    pub const fn pincfg4(&self) -> &Pincfg4 {
        &self.pincfg4
    }
    #[doc = "0x14 - Controls the operation of GPIO pin 5."]
    #[inline(always)]
    pub const fn pincfg5(&self) -> &Pincfg5 {
        &self.pincfg5
    }
    #[doc = "0x18 - Controls the operation of GPIO pin 6."]
    #[inline(always)]
    pub const fn pincfg6(&self) -> &Pincfg6 {
        &self.pincfg6
    }
    #[doc = "0x1c - Controls the operation of GPIO pin 7."]
    #[inline(always)]
    pub const fn pincfg7(&self) -> &Pincfg7 {
        &self.pincfg7
    }
    #[doc = "0x20 - Controls the operation of GPIO pin 8."]
    #[inline(always)]
    pub const fn pincfg8(&self) -> &Pincfg8 {
        &self.pincfg8
    }
    #[doc = "0x24 - Controls the operation of GPIO pin 9."]
    #[inline(always)]
    pub const fn pincfg9(&self) -> &Pincfg9 {
        &self.pincfg9
    }
    #[doc = "0x28 - Controls the operation of GPIO pin 10."]
    #[inline(always)]
    pub const fn pincfg10(&self) -> &Pincfg10 {
        &self.pincfg10
    }
    #[doc = "0x2c - Controls the operation of GPIO pin 11."]
    #[inline(always)]
    pub const fn pincfg11(&self) -> &Pincfg11 {
        &self.pincfg11
    }
    #[doc = "0x30 - Controls the operation of GPIO pin 12."]
    #[inline(always)]
    pub const fn pincfg12(&self) -> &Pincfg12 {
        &self.pincfg12
    }
    #[doc = "0x34 - Controls the operation of GPIO pin 13."]
    #[inline(always)]
    pub const fn pincfg13(&self) -> &Pincfg13 {
        &self.pincfg13
    }
    #[doc = "0x38 - Controls the operation of GPIO pin 14."]
    #[inline(always)]
    pub const fn pincfg14(&self) -> &Pincfg14 {
        &self.pincfg14
    }
    #[doc = "0x3c - Controls the operation of GPIO pin 15."]
    #[inline(always)]
    pub const fn pincfg15(&self) -> &Pincfg15 {
        &self.pincfg15
    }
    #[doc = "0x40 - Controls the operation of GPIO pin 16."]
    #[inline(always)]
    pub const fn pincfg16(&self) -> &Pincfg16 {
        &self.pincfg16
    }
    #[doc = "0x44 - Controls the operation of GPIO pin 17."]
    #[inline(always)]
    pub const fn pincfg17(&self) -> &Pincfg17 {
        &self.pincfg17
    }
    #[doc = "0x48 - Controls the operation of GPIO pin 18."]
    #[inline(always)]
    pub const fn pincfg18(&self) -> &Pincfg18 {
        &self.pincfg18
    }
    #[doc = "0x4c - Controls the operation of GPIO pin 19."]
    #[inline(always)]
    pub const fn pincfg19(&self) -> &Pincfg19 {
        &self.pincfg19
    }
    #[doc = "0x50 - Controls the operation of GPIO pin 20."]
    #[inline(always)]
    pub const fn pincfg20(&self) -> &Pincfg20 {
        &self.pincfg20
    }
    #[doc = "0x54 - Controls the operation of GPIO pin 21."]
    #[inline(always)]
    pub const fn pincfg21(&self) -> &Pincfg21 {
        &self.pincfg21
    }
    #[doc = "0x58 - Controls the operation of GPIO pin 22."]
    #[inline(always)]
    pub const fn pincfg22(&self) -> &Pincfg22 {
        &self.pincfg22
    }
    #[doc = "0x5c - Controls the operation of GPIO pin 23."]
    #[inline(always)]
    pub const fn pincfg23(&self) -> &Pincfg23 {
        &self.pincfg23
    }
    #[doc = "0x60 - Controls the operation of GPIO pin 24."]
    #[inline(always)]
    pub const fn pincfg24(&self) -> &Pincfg24 {
        &self.pincfg24
    }
    #[doc = "0x64 - Controls the operation of GPIO pin 25."]
    #[inline(always)]
    pub const fn pincfg25(&self) -> &Pincfg25 {
        &self.pincfg25
    }
    #[doc = "0x68 - Controls the operation of GPIO pin 26."]
    #[inline(always)]
    pub const fn pincfg26(&self) -> &Pincfg26 {
        &self.pincfg26
    }
    #[doc = "0x6c - Controls the operation of GPIO pin 27."]
    #[inline(always)]
    pub const fn pincfg27(&self) -> &Pincfg27 {
        &self.pincfg27
    }
    #[doc = "0x70 - Controls the operation of GPIO pin 28."]
    #[inline(always)]
    pub const fn pincfg28(&self) -> &Pincfg28 {
        &self.pincfg28
    }
    #[doc = "0x74 - Controls the operation of GPIO pin 29."]
    #[inline(always)]
    pub const fn pincfg29(&self) -> &Pincfg29 {
        &self.pincfg29
    }
    #[doc = "0x78 - Controls the operation of GPIO pin 30."]
    #[inline(always)]
    pub const fn pincfg30(&self) -> &Pincfg30 {
        &self.pincfg30
    }
    #[doc = "0x7c - Controls the operation of GPIO pin 31."]
    #[inline(always)]
    pub const fn pincfg31(&self) -> &Pincfg31 {
        &self.pincfg31
    }
    #[doc = "0x80 - Controls the operation of GPIO pin 32."]
    #[inline(always)]
    pub const fn pincfg32(&self) -> &Pincfg32 {
        &self.pincfg32
    }
    #[doc = "0x84 - Controls the operation of GPIO pin 33."]
    #[inline(always)]
    pub const fn pincfg33(&self) -> &Pincfg33 {
        &self.pincfg33
    }
    #[doc = "0x88 - Controls the operation of GPIO pin 34."]
    #[inline(always)]
    pub const fn pincfg34(&self) -> &Pincfg34 {
        &self.pincfg34
    }
    #[doc = "0x8c - Controls the operation of GPIO pin 35."]
    #[inline(always)]
    pub const fn pincfg35(&self) -> &Pincfg35 {
        &self.pincfg35
    }
    #[doc = "0x90 - Controls the operation of GPIO pin 36."]
    #[inline(always)]
    pub const fn pincfg36(&self) -> &Pincfg36 {
        &self.pincfg36
    }
    #[doc = "0x94 - Controls the operation of GPIO pin 37."]
    #[inline(always)]
    pub const fn pincfg37(&self) -> &Pincfg37 {
        &self.pincfg37
    }
    #[doc = "0x98 - Controls the operation of GPIO pin 38."]
    #[inline(always)]
    pub const fn pincfg38(&self) -> &Pincfg38 {
        &self.pincfg38
    }
    #[doc = "0x9c - Controls the operation of GPIO pin 39."]
    #[inline(always)]
    pub const fn pincfg39(&self) -> &Pincfg39 {
        &self.pincfg39
    }
    #[doc = "0xa0 - Controls the operation of GPIO pin 40."]
    #[inline(always)]
    pub const fn pincfg40(&self) -> &Pincfg40 {
        &self.pincfg40
    }
    #[doc = "0xa4 - Controls the operation of GPIO pin 41."]
    #[inline(always)]
    pub const fn pincfg41(&self) -> &Pincfg41 {
        &self.pincfg41
    }
    #[doc = "0xa8 - Controls the operation of GPIO pin 42."]
    #[inline(always)]
    pub const fn pincfg42(&self) -> &Pincfg42 {
        &self.pincfg42
    }
    #[doc = "0xac - Controls the operation of GPIO pin 43."]
    #[inline(always)]
    pub const fn pincfg43(&self) -> &Pincfg43 {
        &self.pincfg43
    }
    #[doc = "0xb0 - Controls the operation of GPIO pin 44."]
    #[inline(always)]
    pub const fn pincfg44(&self) -> &Pincfg44 {
        &self.pincfg44
    }
    #[doc = "0xb4 - Controls the operation of GPIO pin 45."]
    #[inline(always)]
    pub const fn pincfg45(&self) -> &Pincfg45 {
        &self.pincfg45
    }
    #[doc = "0xb8 - Controls the operation of GPIO pin 46."]
    #[inline(always)]
    pub const fn pincfg46(&self) -> &Pincfg46 {
        &self.pincfg46
    }
    #[doc = "0xbc - Controls the operation of GPIO pin 47."]
    #[inline(always)]
    pub const fn pincfg47(&self) -> &Pincfg47 {
        &self.pincfg47
    }
    #[doc = "0xc0 - Controls the operation of GPIO pin 48."]
    #[inline(always)]
    pub const fn pincfg48(&self) -> &Pincfg48 {
        &self.pincfg48
    }
    #[doc = "0xc4 - Controls the operation of GPIO pin 49."]
    #[inline(always)]
    pub const fn pincfg49(&self) -> &Pincfg49 {
        &self.pincfg49
    }
    #[doc = "0xc8 - Controls the operation of GPIO pin 50."]
    #[inline(always)]
    pub const fn pincfg50(&self) -> &Pincfg50 {
        &self.pincfg50
    }
    #[doc = "0xcc - Controls the operation of GPIO pin 51."]
    #[inline(always)]
    pub const fn pincfg51(&self) -> &Pincfg51 {
        &self.pincfg51
    }
    #[doc = "0xd0 - Controls the operation of GPIO pin 52."]
    #[inline(always)]
    pub const fn pincfg52(&self) -> &Pincfg52 {
        &self.pincfg52
    }
    #[doc = "0xd4 - Controls the operation of GPIO pin 53."]
    #[inline(always)]
    pub const fn pincfg53(&self) -> &Pincfg53 {
        &self.pincfg53
    }
    #[doc = "0xd8 - Controls the operation of GPIO pin 54."]
    #[inline(always)]
    pub const fn pincfg54(&self) -> &Pincfg54 {
        &self.pincfg54
    }
    #[doc = "0xdc - Controls the operation of GPIO pin 55."]
    #[inline(always)]
    pub const fn pincfg55(&self) -> &Pincfg55 {
        &self.pincfg55
    }
    #[doc = "0xe0 - Controls the operation of GPIO pin 56."]
    #[inline(always)]
    pub const fn pincfg56(&self) -> &Pincfg56 {
        &self.pincfg56
    }
    #[doc = "0xe4 - Controls the operation of GPIO pin 57."]
    #[inline(always)]
    pub const fn pincfg57(&self) -> &Pincfg57 {
        &self.pincfg57
    }
    #[doc = "0xe8 - Controls the operation of GPIO pin 58."]
    #[inline(always)]
    pub const fn pincfg58(&self) -> &Pincfg58 {
        &self.pincfg58
    }
    #[doc = "0xec - Controls the operation of GPIO pin 59."]
    #[inline(always)]
    pub const fn pincfg59(&self) -> &Pincfg59 {
        &self.pincfg59
    }
    #[doc = "0xf0 - Controls the operation of GPIO pin 60."]
    #[inline(always)]
    pub const fn pincfg60(&self) -> &Pincfg60 {
        &self.pincfg60
    }
    #[doc = "0xf4 - Controls the operation of GPIO pin 61."]
    #[inline(always)]
    pub const fn pincfg61(&self) -> &Pincfg61 {
        &self.pincfg61
    }
    #[doc = "0xf8 - Controls the operation of GPIO pin 62."]
    #[inline(always)]
    pub const fn pincfg62(&self) -> &Pincfg62 {
        &self.pincfg62
    }
    #[doc = "0xfc - Controls the operation of GPIO pin 63."]
    #[inline(always)]
    pub const fn pincfg63(&self) -> &Pincfg63 {
        &self.pincfg63
    }
    #[doc = "0x100 - Controls the operation of GPIO pin 64."]
    #[inline(always)]
    pub const fn pincfg64(&self) -> &Pincfg64 {
        &self.pincfg64
    }
    #[doc = "0x104 - Controls the operation of GPIO pin 65."]
    #[inline(always)]
    pub const fn pincfg65(&self) -> &Pincfg65 {
        &self.pincfg65
    }
    #[doc = "0x108 - Controls the operation of GPIO pin 66."]
    #[inline(always)]
    pub const fn pincfg66(&self) -> &Pincfg66 {
        &self.pincfg66
    }
    #[doc = "0x10c - Controls the operation of GPIO pin 67."]
    #[inline(always)]
    pub const fn pincfg67(&self) -> &Pincfg67 {
        &self.pincfg67
    }
    #[doc = "0x110 - Controls the operation of GPIO pin 68."]
    #[inline(always)]
    pub const fn pincfg68(&self) -> &Pincfg68 {
        &self.pincfg68
    }
    #[doc = "0x114 - Controls the operation of GPIO pin 69."]
    #[inline(always)]
    pub const fn pincfg69(&self) -> &Pincfg69 {
        &self.pincfg69
    }
    #[doc = "0x118 - Controls the operation of GPIO pin 70."]
    #[inline(always)]
    pub const fn pincfg70(&self) -> &Pincfg70 {
        &self.pincfg70
    }
    #[doc = "0x11c - Controls the operation of GPIO pin 71."]
    #[inline(always)]
    pub const fn pincfg71(&self) -> &Pincfg71 {
        &self.pincfg71
    }
    #[doc = "0x120 - Controls the operation of GPIO pin 72."]
    #[inline(always)]
    pub const fn pincfg72(&self) -> &Pincfg72 {
        &self.pincfg72
    }
    #[doc = "0x124 - Controls the operation of GPIO pin 73."]
    #[inline(always)]
    pub const fn pincfg73(&self) -> &Pincfg73 {
        &self.pincfg73
    }
    #[doc = "0x128 - Controls the operation of GPIO pin 74."]
    #[inline(always)]
    pub const fn pincfg74(&self) -> &Pincfg74 {
        &self.pincfg74
    }
    #[doc = "0x12c - Controls the operation of GPIO pin 75."]
    #[inline(always)]
    pub const fn pincfg75(&self) -> &Pincfg75 {
        &self.pincfg75
    }
    #[doc = "0x130 - Controls the operation of GPIO pin 76."]
    #[inline(always)]
    pub const fn pincfg76(&self) -> &Pincfg76 {
        &self.pincfg76
    }
    #[doc = "0x134 - Controls the operation of GPIO pin 77."]
    #[inline(always)]
    pub const fn pincfg77(&self) -> &Pincfg77 {
        &self.pincfg77
    }
    #[doc = "0x138 - Controls the operation of GPIO pin 78."]
    #[inline(always)]
    pub const fn pincfg78(&self) -> &Pincfg78 {
        &self.pincfg78
    }
    #[doc = "0x13c - Controls the operation of GPIO pin 79."]
    #[inline(always)]
    pub const fn pincfg79(&self) -> &Pincfg79 {
        &self.pincfg79
    }
    #[doc = "0x140 - Controls the operation of GPIO pin 80."]
    #[inline(always)]
    pub const fn pincfg80(&self) -> &Pincfg80 {
        &self.pincfg80
    }
    #[doc = "0x144 - Controls the operation of GPIO pin 81."]
    #[inline(always)]
    pub const fn pincfg81(&self) -> &Pincfg81 {
        &self.pincfg81
    }
    #[doc = "0x148 - Controls the operation of GPIO pin 82."]
    #[inline(always)]
    pub const fn pincfg82(&self) -> &Pincfg82 {
        &self.pincfg82
    }
    #[doc = "0x14c - Controls the operation of GPIO pin 83."]
    #[inline(always)]
    pub const fn pincfg83(&self) -> &Pincfg83 {
        &self.pincfg83
    }
    #[doc = "0x150 - Controls the operation of GPIO pin 84."]
    #[inline(always)]
    pub const fn pincfg84(&self) -> &Pincfg84 {
        &self.pincfg84
    }
    #[doc = "0x154 - Controls the operation of GPIO pin 85."]
    #[inline(always)]
    pub const fn pincfg85(&self) -> &Pincfg85 {
        &self.pincfg85
    }
    #[doc = "0x158 - Controls the operation of GPIO pin 86."]
    #[inline(always)]
    pub const fn pincfg86(&self) -> &Pincfg86 {
        &self.pincfg86
    }
    #[doc = "0x15c - Controls the operation of GPIO pin 87."]
    #[inline(always)]
    pub const fn pincfg87(&self) -> &Pincfg87 {
        &self.pincfg87
    }
    #[doc = "0x160 - Controls the operation of GPIO pin 88."]
    #[inline(always)]
    pub const fn pincfg88(&self) -> &Pincfg88 {
        &self.pincfg88
    }
    #[doc = "0x164 - Controls the operation of GPIO pin 89."]
    #[inline(always)]
    pub const fn pincfg89(&self) -> &Pincfg89 {
        &self.pincfg89
    }
    #[doc = "0x168 - Controls the operation of GPIO pin 90."]
    #[inline(always)]
    pub const fn pincfg90(&self) -> &Pincfg90 {
        &self.pincfg90
    }
    #[doc = "0x16c - Controls the operation of GPIO pin 91."]
    #[inline(always)]
    pub const fn pincfg91(&self) -> &Pincfg91 {
        &self.pincfg91
    }
    #[doc = "0x170 - Controls the operation of GPIO pin 92."]
    #[inline(always)]
    pub const fn pincfg92(&self) -> &Pincfg92 {
        &self.pincfg92
    }
    #[doc = "0x174 - Controls the operation of GPIO pin 93."]
    #[inline(always)]
    pub const fn pincfg93(&self) -> &Pincfg93 {
        &self.pincfg93
    }
    #[doc = "0x178 - Controls the operation of GPIO pin 94."]
    #[inline(always)]
    pub const fn pincfg94(&self) -> &Pincfg94 {
        &self.pincfg94
    }
    #[doc = "0x17c - Controls the operation of GPIO pin 95."]
    #[inline(always)]
    pub const fn pincfg95(&self) -> &Pincfg95 {
        &self.pincfg95
    }
    #[doc = "0x180 - Controls the operation of GPIO pin 96."]
    #[inline(always)]
    pub const fn pincfg96(&self) -> &Pincfg96 {
        &self.pincfg96
    }
    #[doc = "0x184 - Controls the operation of GPIO pin 97."]
    #[inline(always)]
    pub const fn pincfg97(&self) -> &Pincfg97 {
        &self.pincfg97
    }
    #[doc = "0x188 - Controls the operation of GPIO pin 98."]
    #[inline(always)]
    pub const fn pincfg98(&self) -> &Pincfg98 {
        &self.pincfg98
    }
    #[doc = "0x18c - Controls the operation of GPIO pin 99."]
    #[inline(always)]
    pub const fn pincfg99(&self) -> &Pincfg99 {
        &self.pincfg99
    }
    #[doc = "0x190 - Controls the operation of GPIO pin 100."]
    #[inline(always)]
    pub const fn pincfg100(&self) -> &Pincfg100 {
        &self.pincfg100
    }
    #[doc = "0x194 - Controls the operation of GPIO pin 101."]
    #[inline(always)]
    pub const fn pincfg101(&self) -> &Pincfg101 {
        &self.pincfg101
    }
    #[doc = "0x198 - Controls the operation of GPIO pin 102."]
    #[inline(always)]
    pub const fn pincfg102(&self) -> &Pincfg102 {
        &self.pincfg102
    }
    #[doc = "0x19c - Controls the operation of GPIO pin 103."]
    #[inline(always)]
    pub const fn pincfg103(&self) -> &Pincfg103 {
        &self.pincfg103
    }
    #[doc = "0x1a0 - Controls the operation of GPIO pin 104."]
    #[inline(always)]
    pub const fn pincfg104(&self) -> &Pincfg104 {
        &self.pincfg104
    }
    #[doc = "0x1a4 - Controls the operation of virtual GPIO pin 105."]
    #[inline(always)]
    pub const fn pincfg105(&self) -> &Pincfg105 {
        &self.pincfg105
    }
    #[doc = "0x1a8 - Controls the operation of virtual GPIO pin 106."]
    #[inline(always)]
    pub const fn pincfg106(&self) -> &Pincfg106 {
        &self.pincfg106
    }
    #[doc = "0x1ac - Controls the operation of virtual GPIO pin 107."]
    #[inline(always)]
    pub const fn pincfg107(&self) -> &Pincfg107 {
        &self.pincfg107
    }
    #[doc = "0x1b0 - Controls the operation of virtual GPIO pin 108."]
    #[inline(always)]
    pub const fn pincfg108(&self) -> &Pincfg108 {
        &self.pincfg108
    }
    #[doc = "0x1b4 - Controls the operation of virtual GPIO pin 109."]
    #[inline(always)]
    pub const fn pincfg109(&self) -> &Pincfg109 {
        &self.pincfg109
    }
    #[doc = "0x1b8 - Controls the operation of virtual GPIO pin 110."]
    #[inline(always)]
    pub const fn pincfg110(&self) -> &Pincfg110 {
        &self.pincfg110
    }
    #[doc = "0x1bc - Controls the operation of virtual GPIO pin 111."]
    #[inline(always)]
    pub const fn pincfg111(&self) -> &Pincfg111 {
        &self.pincfg111
    }
    #[doc = "0x1c0 - Controls the operation of virtual GPIO pin 112."]
    #[inline(always)]
    pub const fn pincfg112(&self) -> &Pincfg112 {
        &self.pincfg112
    }
    #[doc = "0x1c4 - Controls the operation of virtual GPIO pin 113."]
    #[inline(always)]
    pub const fn pincfg113(&self) -> &Pincfg113 {
        &self.pincfg113
    }
    #[doc = "0x1c8 - Controls the operation of virtual GPIO pin 114."]
    #[inline(always)]
    pub const fn pincfg114(&self) -> &Pincfg114 {
        &self.pincfg114
    }
    #[doc = "0x1cc - Controls the operation of virtual GPIO pin 115."]
    #[inline(always)]
    pub const fn pincfg115(&self) -> &Pincfg115 {
        &self.pincfg115
    }
    #[doc = "0x1d0 - Controls the operation of virtual GPIO pin 116."]
    #[inline(always)]
    pub const fn pincfg116(&self) -> &Pincfg116 {
        &self.pincfg116
    }
    #[doc = "0x1d4 - Controls the operation of virtual GPIO pin 117."]
    #[inline(always)]
    pub const fn pincfg117(&self) -> &Pincfg117 {
        &self.pincfg117
    }
    #[doc = "0x1d8 - Controls the operation of virtual GPIO pin 118."]
    #[inline(always)]
    pub const fn pincfg118(&self) -> &Pincfg118 {
        &self.pincfg118
    }
    #[doc = "0x1dc - Controls the operation of virtual GPIO pin 119."]
    #[inline(always)]
    pub const fn pincfg119(&self) -> &Pincfg119 {
        &self.pincfg119
    }
    #[doc = "0x1e0 - Controls the operation of virtual GPIO pin 120."]
    #[inline(always)]
    pub const fn pincfg120(&self) -> &Pincfg120 {
        &self.pincfg120
    }
    #[doc = "0x1e4 - Controls the operation of virtual GPIO pin 121."]
    #[inline(always)]
    pub const fn pincfg121(&self) -> &Pincfg121 {
        &self.pincfg121
    }
    #[doc = "0x1e8 - Controls the operation of virtual GPIO pin 122."]
    #[inline(always)]
    pub const fn pincfg122(&self) -> &Pincfg122 {
        &self.pincfg122
    }
    #[doc = "0x1ec - Controls the operation of virtual GPIO pin 123."]
    #[inline(always)]
    pub const fn pincfg123(&self) -> &Pincfg123 {
        &self.pincfg123
    }
    #[doc = "0x1f0 - Controls the operation of virtual GPIO pin 124."]
    #[inline(always)]
    pub const fn pincfg124(&self) -> &Pincfg124 {
        &self.pincfg124
    }
    #[doc = "0x1f4 - Controls the operation of virtual GPIO pin 125."]
    #[inline(always)]
    pub const fn pincfg125(&self) -> &Pincfg125 {
        &self.pincfg125
    }
    #[doc = "0x1f8 - Controls the operation of virtual GPIO pin 126."]
    #[inline(always)]
    pub const fn pincfg126(&self) -> &Pincfg126 {
        &self.pincfg126
    }
    #[doc = "0x1fc - Controls the operation of virtual GPIO pin 127."]
    #[inline(always)]
    pub const fn pincfg127(&self) -> &Pincfg127 {
        &self.pincfg127
    }
    #[doc = "0x200 - Lock state of the PINCFG and GPIO configuration registers. Write a value of 0x73 to unlock write access to the PAD and GPIO."]
    #[inline(always)]
    pub const fn padkey(&self) -> &Padkey {
        &self.padkey
    }
    #[doc = "0x204 - GPIO Input 0 (31-0)"]
    #[inline(always)]
    pub const fn rd0(&self) -> &Rd0 {
        &self.rd0
    }
    #[doc = "0x208 - GPIO Input 1 (63-32)"]
    #[inline(always)]
    pub const fn rd1(&self) -> &Rd1 {
        &self.rd1
    }
    #[doc = "0x20c - GPIO Input 2 (95-64)"]
    #[inline(always)]
    pub const fn rd2(&self) -> &Rd2 {
        &self.rd2
    }
    #[doc = "0x210 - GPIO Input 3 (127-96)"]
    #[inline(always)]
    pub const fn rd3(&self) -> &Rd3 {
        &self.rd3
    }
    #[doc = "0x214 - GPIO Output 0 (31-0)"]
    #[inline(always)]
    pub const fn wt0(&self) -> &Wt0 {
        &self.wt0
    }
    #[doc = "0x218 - GPIO Output 1 (63-32)"]
    #[inline(always)]
    pub const fn wt1(&self) -> &Wt1 {
        &self.wt1
    }
    #[doc = "0x21c - GPIO Output 2 (95-64)"]
    #[inline(always)]
    pub const fn wt2(&self) -> &Wt2 {
        &self.wt2
    }
    #[doc = "0x220 - GPIO Output 3 (127-96)"]
    #[inline(always)]
    pub const fn wt3(&self) -> &Wt3 {
        &self.wt3
    }
    #[doc = "0x224 - GPIO Output Set 0 (31-0)"]
    #[inline(always)]
    pub const fn wts0(&self) -> &Wts0 {
        &self.wts0
    }
    #[doc = "0x228 - GPIO Output Set 1 (63-32)"]
    #[inline(always)]
    pub const fn wts1(&self) -> &Wts1 {
        &self.wts1
    }
    #[doc = "0x22c - GPIO Output Set 2 (95-64)"]
    #[inline(always)]
    pub const fn wts2(&self) -> &Wts2 {
        &self.wts2
    }
    #[doc = "0x230 - GPIO Output Set 3 (127-96)"]
    #[inline(always)]
    pub const fn wts3(&self) -> &Wts3 {
        &self.wts3
    }
    #[doc = "0x234 - GPIO Output Clear 0 (31-0)"]
    #[inline(always)]
    pub const fn wtc0(&self) -> &Wtc0 {
        &self.wtc0
    }
    #[doc = "0x238 - GPIO Output Clear 1 (63-32)"]
    #[inline(always)]
    pub const fn wtc1(&self) -> &Wtc1 {
        &self.wtc1
    }
    #[doc = "0x23c - GPIO Output Clear 2 (95-64)"]
    #[inline(always)]
    pub const fn wtc2(&self) -> &Wtc2 {
        &self.wtc2
    }
    #[doc = "0x240 - GPIO Output Clear 3 (127-96)"]
    #[inline(always)]
    pub const fn wtc3(&self) -> &Wtc3 {
        &self.wtc3
    }
    #[doc = "0x244 - GPIO Enable 0 (31-0)"]
    #[inline(always)]
    pub const fn en0(&self) -> &En0 {
        &self.en0
    }
    #[doc = "0x248 - GPIO Enable 1 (63-32)"]
    #[inline(always)]
    pub const fn en1(&self) -> &En1 {
        &self.en1
    }
    #[doc = "0x24c - GPIO Enable 2 (95-64)"]
    #[inline(always)]
    pub const fn en2(&self) -> &En2 {
        &self.en2
    }
    #[doc = "0x250 - GPIO Enable 3 (127-96)"]
    #[inline(always)]
    pub const fn en3(&self) -> &En3 {
        &self.en3
    }
    #[doc = "0x254 - GPIO Enable Set 0 (31-0)"]
    #[inline(always)]
    pub const fn ens0(&self) -> &Ens0 {
        &self.ens0
    }
    #[doc = "0x258 - GPIO Enable Set 1 (63-32)"]
    #[inline(always)]
    pub const fn ens1(&self) -> &Ens1 {
        &self.ens1
    }
    #[doc = "0x25c - GPIO Enable Set 2 (95-64)"]
    #[inline(always)]
    pub const fn ens2(&self) -> &Ens2 {
        &self.ens2
    }
    #[doc = "0x260 - GPIO Enable Set 3 (127-96)"]
    #[inline(always)]
    pub const fn ens3(&self) -> &Ens3 {
        &self.ens3
    }
    #[doc = "0x264 - GPIO Enable Clear 0 (31-0)"]
    #[inline(always)]
    pub const fn enc0(&self) -> &Enc0 {
        &self.enc0
    }
    #[doc = "0x268 - GPIO Enable Clear 1 (63-32)"]
    #[inline(always)]
    pub const fn enc1(&self) -> &Enc1 {
        &self.enc1
    }
    #[doc = "0x26c - GPIO Enable Clear 2 (95-64)"]
    #[inline(always)]
    pub const fn enc2(&self) -> &Enc2 {
        &self.enc2
    }
    #[doc = "0x270 - GPIO Enable Clear 3 (127-96)"]
    #[inline(always)]
    pub const fn enc3(&self) -> &Enc3 {
        &self.enc3
    }
    #[doc = "0x274 - IOM0 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom0irq(&self) -> &Iom0irq {
        &self.iom0irq
    }
    #[doc = "0x278 - IOM1 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom1irq(&self) -> &Iom1irq {
        &self.iom1irq
    }
    #[doc = "0x27c - IOM2 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom2irq(&self) -> &Iom2irq {
        &self.iom2irq
    }
    #[doc = "0x280 - IOM3 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom3irq(&self) -> &Iom3irq {
        &self.iom3irq
    }
    #[doc = "0x284 - IOM4 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom4irq(&self) -> &Iom4irq {
        &self.iom4irq
    }
    #[doc = "0x288 - IOM5 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom5irq(&self) -> &Iom5irq {
        &self.iom5irq
    }
    #[doc = "0x28c - IOM6 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom6irq(&self) -> &Iom6irq {
        &self.iom6irq
    }
    #[doc = "0x290 - IOM7 IRQ select for flow control."]
    #[inline(always)]
    pub const fn iom7irq(&self) -> &Iom7irq {
        &self.iom7irq
    }
    #[doc = "0x294 - SDIF CD and WP Select."]
    #[inline(always)]
    pub const fn sdifcdwp(&self) -> &Sdifcdwp {
        &self.sdifcdwp
    }
    #[doc = "0x298 - GPIO Observation mode sample"]
    #[inline(always)]
    pub const fn obsdata(&self) -> &Obsdata {
        &self.obsdata
    }
    #[doc = "0x29c - Read only. Reflects the value of the input enable signals for pads 31-0 sent to the pad."]
    #[inline(always)]
    pub const fn ieobs0(&self) -> &Ieobs0 {
        &self.ieobs0
    }
    #[doc = "0x2a0 - Read only. Reflects the value of the input enable signals for pads 63-32 sent to the pad."]
    #[inline(always)]
    pub const fn ieobs1(&self) -> &Ieobs1 {
        &self.ieobs1
    }
    #[doc = "0x2a4 - Read only. Reflects the value of the input enable signals for pads 95-64 sent to the pad."]
    #[inline(always)]
    pub const fn ieobs2(&self) -> &Ieobs2 {
        &self.ieobs2
    }
    #[doc = "0x2a8 - Read only. Reflects the value of the input enable signals for pads 127-96 sent to the pad."]
    #[inline(always)]
    pub const fn ieobs3(&self) -> &Ieobs3 {
        &self.ieobs3
    }
    #[doc = "0x2ac - Read only. Reflects the value of the output enable signals for pads 31-0 sent to the pad."]
    #[inline(always)]
    pub const fn oeobs0(&self) -> &Oeobs0 {
        &self.oeobs0
    }
    #[doc = "0x2b0 - Read only. Reflects the value of the output enable signals for pads 63-32 sent to the pad."]
    #[inline(always)]
    pub const fn oeobs1(&self) -> &Oeobs1 {
        &self.oeobs1
    }
    #[doc = "0x2b4 - Read only. Reflects the value of the output enable signals for pads 95-64 sent to the pad."]
    #[inline(always)]
    pub const fn oeobs2(&self) -> &Oeobs2 {
        &self.oeobs2
    }
    #[doc = "0x2b8 - Read only. Reflects the value of the output enable signals for pads 127-96 sent to the pad."]
    #[inline(always)]
    pub const fn oeobs3(&self) -> &Oeobs3 {
        &self.oeobs3
    }
    #[doc = "0x2c0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun0int0en(&self) -> &Mcun0int0en {
        &self.mcun0int0en
    }
    #[doc = "0x2c4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun0int0stat(&self) -> &Mcun0int0stat {
        &self.mcun0int0stat
    }
    #[doc = "0x2c8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun0int0clr(&self) -> &Mcun0int0clr {
        &self.mcun0int0clr
    }
    #[doc = "0x2cc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun0int0set(&self) -> &Mcun0int0set {
        &self.mcun0int0set
    }
    #[doc = "0x2d0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun0int1en(&self) -> &Mcun0int1en {
        &self.mcun0int1en
    }
    #[doc = "0x2d4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun0int1stat(&self) -> &Mcun0int1stat {
        &self.mcun0int1stat
    }
    #[doc = "0x2d8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun0int1clr(&self) -> &Mcun0int1clr {
        &self.mcun0int1clr
    }
    #[doc = "0x2dc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun0int1set(&self) -> &Mcun0int1set {
        &self.mcun0int1set
    }
    #[doc = "0x2e0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun0int2en(&self) -> &Mcun0int2en {
        &self.mcun0int2en
    }
    #[doc = "0x2e4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun0int2stat(&self) -> &Mcun0int2stat {
        &self.mcun0int2stat
    }
    #[doc = "0x2e8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun0int2clr(&self) -> &Mcun0int2clr {
        &self.mcun0int2clr
    }
    #[doc = "0x2ec - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun0int2set(&self) -> &Mcun0int2set {
        &self.mcun0int2set
    }
    #[doc = "0x2f0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun0int3en(&self) -> &Mcun0int3en {
        &self.mcun0int3en
    }
    #[doc = "0x2f4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun0int3stat(&self) -> &Mcun0int3stat {
        &self.mcun0int3stat
    }
    #[doc = "0x2f8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun0int3clr(&self) -> &Mcun0int3clr {
        &self.mcun0int3clr
    }
    #[doc = "0x2fc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun0int3set(&self) -> &Mcun0int3set {
        &self.mcun0int3set
    }
    #[doc = "0x300 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun1int0en(&self) -> &Mcun1int0en {
        &self.mcun1int0en
    }
    #[doc = "0x304 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun1int0stat(&self) -> &Mcun1int0stat {
        &self.mcun1int0stat
    }
    #[doc = "0x308 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun1int0clr(&self) -> &Mcun1int0clr {
        &self.mcun1int0clr
    }
    #[doc = "0x30c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun1int0set(&self) -> &Mcun1int0set {
        &self.mcun1int0set
    }
    #[doc = "0x310 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun1int1en(&self) -> &Mcun1int1en {
        &self.mcun1int1en
    }
    #[doc = "0x314 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun1int1stat(&self) -> &Mcun1int1stat {
        &self.mcun1int1stat
    }
    #[doc = "0x318 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun1int1clr(&self) -> &Mcun1int1clr {
        &self.mcun1int1clr
    }
    #[doc = "0x31c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun1int1set(&self) -> &Mcun1int1set {
        &self.mcun1int1set
    }
    #[doc = "0x320 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun1int2en(&self) -> &Mcun1int2en {
        &self.mcun1int2en
    }
    #[doc = "0x324 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun1int2stat(&self) -> &Mcun1int2stat {
        &self.mcun1int2stat
    }
    #[doc = "0x328 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun1int2clr(&self) -> &Mcun1int2clr {
        &self.mcun1int2clr
    }
    #[doc = "0x32c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun1int2set(&self) -> &Mcun1int2set {
        &self.mcun1int2set
    }
    #[doc = "0x330 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn mcun1int3en(&self) -> &Mcun1int3en {
        &self.mcun1int3en
    }
    #[doc = "0x334 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn mcun1int3stat(&self) -> &Mcun1int3stat {
        &self.mcun1int3stat
    }
    #[doc = "0x338 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn mcun1int3clr(&self) -> &Mcun1int3clr {
        &self.mcun1int3clr
    }
    #[doc = "0x33c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn mcun1int3set(&self) -> &Mcun1int3set {
        &self.mcun1int3set
    }
    #[doc = "0x340 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int0en(&self) -> &Dsp0n0int0en {
        &self.dsp0n0int0en
    }
    #[doc = "0x344 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int0stat(&self) -> &Dsp0n0int0stat {
        &self.dsp0n0int0stat
    }
    #[doc = "0x348 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n0int0clr(&self) -> &Dsp0n0int0clr {
        &self.dsp0n0int0clr
    }
    #[doc = "0x34c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n0int0set(&self) -> &Dsp0n0int0set {
        &self.dsp0n0int0set
    }
    #[doc = "0x350 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int1en(&self) -> &Dsp0n0int1en {
        &self.dsp0n0int1en
    }
    #[doc = "0x354 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int1stat(&self) -> &Dsp0n0int1stat {
        &self.dsp0n0int1stat
    }
    #[doc = "0x358 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n0int1clr(&self) -> &Dsp0n0int1clr {
        &self.dsp0n0int1clr
    }
    #[doc = "0x35c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n0int1set(&self) -> &Dsp0n0int1set {
        &self.dsp0n0int1set
    }
    #[doc = "0x360 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int2en(&self) -> &Dsp0n0int2en {
        &self.dsp0n0int2en
    }
    #[doc = "0x364 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int2stat(&self) -> &Dsp0n0int2stat {
        &self.dsp0n0int2stat
    }
    #[doc = "0x368 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n0int2clr(&self) -> &Dsp0n0int2clr {
        &self.dsp0n0int2clr
    }
    #[doc = "0x36c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n0int2set(&self) -> &Dsp0n0int2set {
        &self.dsp0n0int2set
    }
    #[doc = "0x370 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int3en(&self) -> &Dsp0n0int3en {
        &self.dsp0n0int3en
    }
    #[doc = "0x374 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n0int3stat(&self) -> &Dsp0n0int3stat {
        &self.dsp0n0int3stat
    }
    #[doc = "0x378 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n0int3clr(&self) -> &Dsp0n0int3clr {
        &self.dsp0n0int3clr
    }
    #[doc = "0x37c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n0int3set(&self) -> &Dsp0n0int3set {
        &self.dsp0n0int3set
    }
    #[doc = "0x380 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int0en(&self) -> &Dsp0n1int0en {
        &self.dsp0n1int0en
    }
    #[doc = "0x384 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int0stat(&self) -> &Dsp0n1int0stat {
        &self.dsp0n1int0stat
    }
    #[doc = "0x388 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n1int0clr(&self) -> &Dsp0n1int0clr {
        &self.dsp0n1int0clr
    }
    #[doc = "0x38c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n1int0set(&self) -> &Dsp0n1int0set {
        &self.dsp0n1int0set
    }
    #[doc = "0x390 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int1en(&self) -> &Dsp0n1int1en {
        &self.dsp0n1int1en
    }
    #[doc = "0x394 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int1stat(&self) -> &Dsp0n1int1stat {
        &self.dsp0n1int1stat
    }
    #[doc = "0x398 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n1int1clr(&self) -> &Dsp0n1int1clr {
        &self.dsp0n1int1clr
    }
    #[doc = "0x39c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n1int1set(&self) -> &Dsp0n1int1set {
        &self.dsp0n1int1set
    }
    #[doc = "0x3a0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int2en(&self) -> &Dsp0n1int2en {
        &self.dsp0n1int2en
    }
    #[doc = "0x3a4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int2stat(&self) -> &Dsp0n1int2stat {
        &self.dsp0n1int2stat
    }
    #[doc = "0x3a8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n1int2clr(&self) -> &Dsp0n1int2clr {
        &self.dsp0n1int2clr
    }
    #[doc = "0x3ac - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n1int2set(&self) -> &Dsp0n1int2set {
        &self.dsp0n1int2set
    }
    #[doc = "0x3b0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int3en(&self) -> &Dsp0n1int3en {
        &self.dsp0n1int3en
    }
    #[doc = "0x3b4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp0n1int3stat(&self) -> &Dsp0n1int3stat {
        &self.dsp0n1int3stat
    }
    #[doc = "0x3b8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp0n1int3clr(&self) -> &Dsp0n1int3clr {
        &self.dsp0n1int3clr
    }
    #[doc = "0x3bc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp0n1int3set(&self) -> &Dsp0n1int3set {
        &self.dsp0n1int3set
    }
    #[doc = "0x3c0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int0en(&self) -> &Dsp1n0int0en {
        &self.dsp1n0int0en
    }
    #[doc = "0x3c4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int0stat(&self) -> &Dsp1n0int0stat {
        &self.dsp1n0int0stat
    }
    #[doc = "0x3c8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n0int0clr(&self) -> &Dsp1n0int0clr {
        &self.dsp1n0int0clr
    }
    #[doc = "0x3cc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n0int0set(&self) -> &Dsp1n0int0set {
        &self.dsp1n0int0set
    }
    #[doc = "0x3d0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int1en(&self) -> &Dsp1n0int1en {
        &self.dsp1n0int1en
    }
    #[doc = "0x3d4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int1stat(&self) -> &Dsp1n0int1stat {
        &self.dsp1n0int1stat
    }
    #[doc = "0x3d8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n0int1clr(&self) -> &Dsp1n0int1clr {
        &self.dsp1n0int1clr
    }
    #[doc = "0x3dc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n0int1set(&self) -> &Dsp1n0int1set {
        &self.dsp1n0int1set
    }
    #[doc = "0x3e0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int2en(&self) -> &Dsp1n0int2en {
        &self.dsp1n0int2en
    }
    #[doc = "0x3e4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int2stat(&self) -> &Dsp1n0int2stat {
        &self.dsp1n0int2stat
    }
    #[doc = "0x3e8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n0int2clr(&self) -> &Dsp1n0int2clr {
        &self.dsp1n0int2clr
    }
    #[doc = "0x3ec - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n0int2set(&self) -> &Dsp1n0int2set {
        &self.dsp1n0int2set
    }
    #[doc = "0x3f0 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int3en(&self) -> &Dsp1n0int3en {
        &self.dsp1n0int3en
    }
    #[doc = "0x3f4 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n0int3stat(&self) -> &Dsp1n0int3stat {
        &self.dsp1n0int3stat
    }
    #[doc = "0x3f8 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n0int3clr(&self) -> &Dsp1n0int3clr {
        &self.dsp1n0int3clr
    }
    #[doc = "0x3fc - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n0int3set(&self) -> &Dsp1n0int3set {
        &self.dsp1n0int3set
    }
    #[doc = "0x400 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int0en(&self) -> &Dsp1n1int0en {
        &self.dsp1n1int0en
    }
    #[doc = "0x404 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int0stat(&self) -> &Dsp1n1int0stat {
        &self.dsp1n1int0stat
    }
    #[doc = "0x408 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n1int0clr(&self) -> &Dsp1n1int0clr {
        &self.dsp1n1int0clr
    }
    #[doc = "0x40c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n1int0set(&self) -> &Dsp1n1int0set {
        &self.dsp1n1int0set
    }
    #[doc = "0x410 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int1en(&self) -> &Dsp1n1int1en {
        &self.dsp1n1int1en
    }
    #[doc = "0x414 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int1stat(&self) -> &Dsp1n1int1stat {
        &self.dsp1n1int1stat
    }
    #[doc = "0x418 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n1int1clr(&self) -> &Dsp1n1int1clr {
        &self.dsp1n1int1clr
    }
    #[doc = "0x41c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n1int1set(&self) -> &Dsp1n1int1set {
        &self.dsp1n1int1set
    }
    #[doc = "0x420 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int2en(&self) -> &Dsp1n1int2en {
        &self.dsp1n1int2en
    }
    #[doc = "0x424 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int2stat(&self) -> &Dsp1n1int2stat {
        &self.dsp1n1int2stat
    }
    #[doc = "0x428 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n1int2clr(&self) -> &Dsp1n1int2clr {
        &self.dsp1n1int2clr
    }
    #[doc = "0x42c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n1int2set(&self) -> &Dsp1n1int2set {
        &self.dsp1n1int2set
    }
    #[doc = "0x430 - Set bits in this register to allow this module to generate the corresponding interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int3en(&self) -> &Dsp1n1int3en {
        &self.dsp1n1int3en
    }
    #[doc = "0x434 - Read bits from this register to discover the cause of a recent interrupt."]
    #[inline(always)]
    pub const fn dsp1n1int3stat(&self) -> &Dsp1n1int3stat {
        &self.dsp1n1int3stat
    }
    #[doc = "0x438 - Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
    #[inline(always)]
    pub const fn dsp1n1int3clr(&self) -> &Dsp1n1int3clr {
        &self.dsp1n1int3clr
    }
    #[doc = "0x43c - Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
    #[inline(always)]
    pub const fn dsp1n1int3set(&self) -> &Dsp1n1int3set {
        &self.dsp1n1int3set
    }
}
#[doc = "PINCFG0 (rw) register accessor: Controls the operation of GPIO pin 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg0`]
module"]
#[doc(alias = "PINCFG0")]
pub type Pincfg0 = crate::Reg<pincfg0::Pincfg0Spec>;
#[doc = "Controls the operation of GPIO pin 0."]
pub mod pincfg0;
#[doc = "PINCFG1 (rw) register accessor: Controls the operation of GPIO pin 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg1`]
module"]
#[doc(alias = "PINCFG1")]
pub type Pincfg1 = crate::Reg<pincfg1::Pincfg1Spec>;
#[doc = "Controls the operation of GPIO pin 1."]
pub mod pincfg1;
#[doc = "PINCFG2 (rw) register accessor: Controls the operation of GPIO pin 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg2`]
module"]
#[doc(alias = "PINCFG2")]
pub type Pincfg2 = crate::Reg<pincfg2::Pincfg2Spec>;
#[doc = "Controls the operation of GPIO pin 2."]
pub mod pincfg2;
#[doc = "PINCFG3 (rw) register accessor: Controls the operation of GPIO pin 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg3`]
module"]
#[doc(alias = "PINCFG3")]
pub type Pincfg3 = crate::Reg<pincfg3::Pincfg3Spec>;
#[doc = "Controls the operation of GPIO pin 3."]
pub mod pincfg3;
#[doc = "PINCFG4 (rw) register accessor: Controls the operation of GPIO pin 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg4`]
module"]
#[doc(alias = "PINCFG4")]
pub type Pincfg4 = crate::Reg<pincfg4::Pincfg4Spec>;
#[doc = "Controls the operation of GPIO pin 4."]
pub mod pincfg4;
#[doc = "PINCFG5 (rw) register accessor: Controls the operation of GPIO pin 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg5`]
module"]
#[doc(alias = "PINCFG5")]
pub type Pincfg5 = crate::Reg<pincfg5::Pincfg5Spec>;
#[doc = "Controls the operation of GPIO pin 5."]
pub mod pincfg5;
#[doc = "PINCFG6 (rw) register accessor: Controls the operation of GPIO pin 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg6`]
module"]
#[doc(alias = "PINCFG6")]
pub type Pincfg6 = crate::Reg<pincfg6::Pincfg6Spec>;
#[doc = "Controls the operation of GPIO pin 6."]
pub mod pincfg6;
#[doc = "PINCFG7 (rw) register accessor: Controls the operation of GPIO pin 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg7`]
module"]
#[doc(alias = "PINCFG7")]
pub type Pincfg7 = crate::Reg<pincfg7::Pincfg7Spec>;
#[doc = "Controls the operation of GPIO pin 7."]
pub mod pincfg7;
#[doc = "PINCFG8 (rw) register accessor: Controls the operation of GPIO pin 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg8`]
module"]
#[doc(alias = "PINCFG8")]
pub type Pincfg8 = crate::Reg<pincfg8::Pincfg8Spec>;
#[doc = "Controls the operation of GPIO pin 8."]
pub mod pincfg8;
#[doc = "PINCFG9 (rw) register accessor: Controls the operation of GPIO pin 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg9`]
module"]
#[doc(alias = "PINCFG9")]
pub type Pincfg9 = crate::Reg<pincfg9::Pincfg9Spec>;
#[doc = "Controls the operation of GPIO pin 9."]
pub mod pincfg9;
#[doc = "PINCFG10 (rw) register accessor: Controls the operation of GPIO pin 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg10`]
module"]
#[doc(alias = "PINCFG10")]
pub type Pincfg10 = crate::Reg<pincfg10::Pincfg10Spec>;
#[doc = "Controls the operation of GPIO pin 10."]
pub mod pincfg10;
#[doc = "PINCFG11 (rw) register accessor: Controls the operation of GPIO pin 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg11`]
module"]
#[doc(alias = "PINCFG11")]
pub type Pincfg11 = crate::Reg<pincfg11::Pincfg11Spec>;
#[doc = "Controls the operation of GPIO pin 11."]
pub mod pincfg11;
#[doc = "PINCFG12 (rw) register accessor: Controls the operation of GPIO pin 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg12`]
module"]
#[doc(alias = "PINCFG12")]
pub type Pincfg12 = crate::Reg<pincfg12::Pincfg12Spec>;
#[doc = "Controls the operation of GPIO pin 12."]
pub mod pincfg12;
#[doc = "PINCFG13 (rw) register accessor: Controls the operation of GPIO pin 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg13`]
module"]
#[doc(alias = "PINCFG13")]
pub type Pincfg13 = crate::Reg<pincfg13::Pincfg13Spec>;
#[doc = "Controls the operation of GPIO pin 13."]
pub mod pincfg13;
#[doc = "PINCFG14 (rw) register accessor: Controls the operation of GPIO pin 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg14`]
module"]
#[doc(alias = "PINCFG14")]
pub type Pincfg14 = crate::Reg<pincfg14::Pincfg14Spec>;
#[doc = "Controls the operation of GPIO pin 14."]
pub mod pincfg14;
#[doc = "PINCFG15 (rw) register accessor: Controls the operation of GPIO pin 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg15`]
module"]
#[doc(alias = "PINCFG15")]
pub type Pincfg15 = crate::Reg<pincfg15::Pincfg15Spec>;
#[doc = "Controls the operation of GPIO pin 15."]
pub mod pincfg15;
#[doc = "PINCFG16 (rw) register accessor: Controls the operation of GPIO pin 16.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg16`]
module"]
#[doc(alias = "PINCFG16")]
pub type Pincfg16 = crate::Reg<pincfg16::Pincfg16Spec>;
#[doc = "Controls the operation of GPIO pin 16."]
pub mod pincfg16;
#[doc = "PINCFG17 (rw) register accessor: Controls the operation of GPIO pin 17.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg17`]
module"]
#[doc(alias = "PINCFG17")]
pub type Pincfg17 = crate::Reg<pincfg17::Pincfg17Spec>;
#[doc = "Controls the operation of GPIO pin 17."]
pub mod pincfg17;
#[doc = "PINCFG18 (rw) register accessor: Controls the operation of GPIO pin 18.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg18`]
module"]
#[doc(alias = "PINCFG18")]
pub type Pincfg18 = crate::Reg<pincfg18::Pincfg18Spec>;
#[doc = "Controls the operation of GPIO pin 18."]
pub mod pincfg18;
#[doc = "PINCFG19 (rw) register accessor: Controls the operation of GPIO pin 19.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg19`]
module"]
#[doc(alias = "PINCFG19")]
pub type Pincfg19 = crate::Reg<pincfg19::Pincfg19Spec>;
#[doc = "Controls the operation of GPIO pin 19."]
pub mod pincfg19;
#[doc = "PINCFG20 (rw) register accessor: Controls the operation of GPIO pin 20.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg20`]
module"]
#[doc(alias = "PINCFG20")]
pub type Pincfg20 = crate::Reg<pincfg20::Pincfg20Spec>;
#[doc = "Controls the operation of GPIO pin 20."]
pub mod pincfg20;
#[doc = "PINCFG21 (rw) register accessor: Controls the operation of GPIO pin 21.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg21`]
module"]
#[doc(alias = "PINCFG21")]
pub type Pincfg21 = crate::Reg<pincfg21::Pincfg21Spec>;
#[doc = "Controls the operation of GPIO pin 21."]
pub mod pincfg21;
#[doc = "PINCFG22 (rw) register accessor: Controls the operation of GPIO pin 22.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg22`]
module"]
#[doc(alias = "PINCFG22")]
pub type Pincfg22 = crate::Reg<pincfg22::Pincfg22Spec>;
#[doc = "Controls the operation of GPIO pin 22."]
pub mod pincfg22;
#[doc = "PINCFG23 (rw) register accessor: Controls the operation of GPIO pin 23.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg23`]
module"]
#[doc(alias = "PINCFG23")]
pub type Pincfg23 = crate::Reg<pincfg23::Pincfg23Spec>;
#[doc = "Controls the operation of GPIO pin 23."]
pub mod pincfg23;
#[doc = "PINCFG24 (rw) register accessor: Controls the operation of GPIO pin 24.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg24`]
module"]
#[doc(alias = "PINCFG24")]
pub type Pincfg24 = crate::Reg<pincfg24::Pincfg24Spec>;
#[doc = "Controls the operation of GPIO pin 24."]
pub mod pincfg24;
#[doc = "PINCFG25 (rw) register accessor: Controls the operation of GPIO pin 25.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg25`]
module"]
#[doc(alias = "PINCFG25")]
pub type Pincfg25 = crate::Reg<pincfg25::Pincfg25Spec>;
#[doc = "Controls the operation of GPIO pin 25."]
pub mod pincfg25;
#[doc = "PINCFG26 (rw) register accessor: Controls the operation of GPIO pin 26.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg26`]
module"]
#[doc(alias = "PINCFG26")]
pub type Pincfg26 = crate::Reg<pincfg26::Pincfg26Spec>;
#[doc = "Controls the operation of GPIO pin 26."]
pub mod pincfg26;
#[doc = "PINCFG27 (rw) register accessor: Controls the operation of GPIO pin 27.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg27`]
module"]
#[doc(alias = "PINCFG27")]
pub type Pincfg27 = crate::Reg<pincfg27::Pincfg27Spec>;
#[doc = "Controls the operation of GPIO pin 27."]
pub mod pincfg27;
#[doc = "PINCFG28 (rw) register accessor: Controls the operation of GPIO pin 28.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg28`]
module"]
#[doc(alias = "PINCFG28")]
pub type Pincfg28 = crate::Reg<pincfg28::Pincfg28Spec>;
#[doc = "Controls the operation of GPIO pin 28."]
pub mod pincfg28;
#[doc = "PINCFG29 (rw) register accessor: Controls the operation of GPIO pin 29.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg29`]
module"]
#[doc(alias = "PINCFG29")]
pub type Pincfg29 = crate::Reg<pincfg29::Pincfg29Spec>;
#[doc = "Controls the operation of GPIO pin 29."]
pub mod pincfg29;
#[doc = "PINCFG30 (rw) register accessor: Controls the operation of GPIO pin 30.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg30`]
module"]
#[doc(alias = "PINCFG30")]
pub type Pincfg30 = crate::Reg<pincfg30::Pincfg30Spec>;
#[doc = "Controls the operation of GPIO pin 30."]
pub mod pincfg30;
#[doc = "PINCFG31 (rw) register accessor: Controls the operation of GPIO pin 31.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg31`]
module"]
#[doc(alias = "PINCFG31")]
pub type Pincfg31 = crate::Reg<pincfg31::Pincfg31Spec>;
#[doc = "Controls the operation of GPIO pin 31."]
pub mod pincfg31;
#[doc = "PINCFG32 (rw) register accessor: Controls the operation of GPIO pin 32.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg32`]
module"]
#[doc(alias = "PINCFG32")]
pub type Pincfg32 = crate::Reg<pincfg32::Pincfg32Spec>;
#[doc = "Controls the operation of GPIO pin 32."]
pub mod pincfg32;
#[doc = "PINCFG33 (rw) register accessor: Controls the operation of GPIO pin 33.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg33`]
module"]
#[doc(alias = "PINCFG33")]
pub type Pincfg33 = crate::Reg<pincfg33::Pincfg33Spec>;
#[doc = "Controls the operation of GPIO pin 33."]
pub mod pincfg33;
#[doc = "PINCFG34 (rw) register accessor: Controls the operation of GPIO pin 34.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg34`]
module"]
#[doc(alias = "PINCFG34")]
pub type Pincfg34 = crate::Reg<pincfg34::Pincfg34Spec>;
#[doc = "Controls the operation of GPIO pin 34."]
pub mod pincfg34;
#[doc = "PINCFG35 (rw) register accessor: Controls the operation of GPIO pin 35.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg35`]
module"]
#[doc(alias = "PINCFG35")]
pub type Pincfg35 = crate::Reg<pincfg35::Pincfg35Spec>;
#[doc = "Controls the operation of GPIO pin 35."]
pub mod pincfg35;
#[doc = "PINCFG36 (rw) register accessor: Controls the operation of GPIO pin 36.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg36`]
module"]
#[doc(alias = "PINCFG36")]
pub type Pincfg36 = crate::Reg<pincfg36::Pincfg36Spec>;
#[doc = "Controls the operation of GPIO pin 36."]
pub mod pincfg36;
#[doc = "PINCFG37 (rw) register accessor: Controls the operation of GPIO pin 37.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg37`]
module"]
#[doc(alias = "PINCFG37")]
pub type Pincfg37 = crate::Reg<pincfg37::Pincfg37Spec>;
#[doc = "Controls the operation of GPIO pin 37."]
pub mod pincfg37;
#[doc = "PINCFG38 (rw) register accessor: Controls the operation of GPIO pin 38.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg38`]
module"]
#[doc(alias = "PINCFG38")]
pub type Pincfg38 = crate::Reg<pincfg38::Pincfg38Spec>;
#[doc = "Controls the operation of GPIO pin 38."]
pub mod pincfg38;
#[doc = "PINCFG39 (rw) register accessor: Controls the operation of GPIO pin 39.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg39`]
module"]
#[doc(alias = "PINCFG39")]
pub type Pincfg39 = crate::Reg<pincfg39::Pincfg39Spec>;
#[doc = "Controls the operation of GPIO pin 39."]
pub mod pincfg39;
#[doc = "PINCFG40 (rw) register accessor: Controls the operation of GPIO pin 40.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg40`]
module"]
#[doc(alias = "PINCFG40")]
pub type Pincfg40 = crate::Reg<pincfg40::Pincfg40Spec>;
#[doc = "Controls the operation of GPIO pin 40."]
pub mod pincfg40;
#[doc = "PINCFG41 (rw) register accessor: Controls the operation of GPIO pin 41.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg41`]
module"]
#[doc(alias = "PINCFG41")]
pub type Pincfg41 = crate::Reg<pincfg41::Pincfg41Spec>;
#[doc = "Controls the operation of GPIO pin 41."]
pub mod pincfg41;
#[doc = "PINCFG42 (rw) register accessor: Controls the operation of GPIO pin 42.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg42`]
module"]
#[doc(alias = "PINCFG42")]
pub type Pincfg42 = crate::Reg<pincfg42::Pincfg42Spec>;
#[doc = "Controls the operation of GPIO pin 42."]
pub mod pincfg42;
#[doc = "PINCFG43 (rw) register accessor: Controls the operation of GPIO pin 43.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg43`]
module"]
#[doc(alias = "PINCFG43")]
pub type Pincfg43 = crate::Reg<pincfg43::Pincfg43Spec>;
#[doc = "Controls the operation of GPIO pin 43."]
pub mod pincfg43;
#[doc = "PINCFG44 (rw) register accessor: Controls the operation of GPIO pin 44.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg44`]
module"]
#[doc(alias = "PINCFG44")]
pub type Pincfg44 = crate::Reg<pincfg44::Pincfg44Spec>;
#[doc = "Controls the operation of GPIO pin 44."]
pub mod pincfg44;
#[doc = "PINCFG45 (rw) register accessor: Controls the operation of GPIO pin 45.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg45`]
module"]
#[doc(alias = "PINCFG45")]
pub type Pincfg45 = crate::Reg<pincfg45::Pincfg45Spec>;
#[doc = "Controls the operation of GPIO pin 45."]
pub mod pincfg45;
#[doc = "PINCFG46 (rw) register accessor: Controls the operation of GPIO pin 46.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg46`]
module"]
#[doc(alias = "PINCFG46")]
pub type Pincfg46 = crate::Reg<pincfg46::Pincfg46Spec>;
#[doc = "Controls the operation of GPIO pin 46."]
pub mod pincfg46;
#[doc = "PINCFG47 (rw) register accessor: Controls the operation of GPIO pin 47.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg47`]
module"]
#[doc(alias = "PINCFG47")]
pub type Pincfg47 = crate::Reg<pincfg47::Pincfg47Spec>;
#[doc = "Controls the operation of GPIO pin 47."]
pub mod pincfg47;
#[doc = "PINCFG48 (rw) register accessor: Controls the operation of GPIO pin 48.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg48`]
module"]
#[doc(alias = "PINCFG48")]
pub type Pincfg48 = crate::Reg<pincfg48::Pincfg48Spec>;
#[doc = "Controls the operation of GPIO pin 48."]
pub mod pincfg48;
#[doc = "PINCFG49 (rw) register accessor: Controls the operation of GPIO pin 49.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg49`]
module"]
#[doc(alias = "PINCFG49")]
pub type Pincfg49 = crate::Reg<pincfg49::Pincfg49Spec>;
#[doc = "Controls the operation of GPIO pin 49."]
pub mod pincfg49;
#[doc = "PINCFG50 (rw) register accessor: Controls the operation of GPIO pin 50.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg50`]
module"]
#[doc(alias = "PINCFG50")]
pub type Pincfg50 = crate::Reg<pincfg50::Pincfg50Spec>;
#[doc = "Controls the operation of GPIO pin 50."]
pub mod pincfg50;
#[doc = "PINCFG51 (rw) register accessor: Controls the operation of GPIO pin 51.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg51`]
module"]
#[doc(alias = "PINCFG51")]
pub type Pincfg51 = crate::Reg<pincfg51::Pincfg51Spec>;
#[doc = "Controls the operation of GPIO pin 51."]
pub mod pincfg51;
#[doc = "PINCFG52 (rw) register accessor: Controls the operation of GPIO pin 52.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg52`]
module"]
#[doc(alias = "PINCFG52")]
pub type Pincfg52 = crate::Reg<pincfg52::Pincfg52Spec>;
#[doc = "Controls the operation of GPIO pin 52."]
pub mod pincfg52;
#[doc = "PINCFG53 (rw) register accessor: Controls the operation of GPIO pin 53.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg53`]
module"]
#[doc(alias = "PINCFG53")]
pub type Pincfg53 = crate::Reg<pincfg53::Pincfg53Spec>;
#[doc = "Controls the operation of GPIO pin 53."]
pub mod pincfg53;
#[doc = "PINCFG54 (rw) register accessor: Controls the operation of GPIO pin 54.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg54`]
module"]
#[doc(alias = "PINCFG54")]
pub type Pincfg54 = crate::Reg<pincfg54::Pincfg54Spec>;
#[doc = "Controls the operation of GPIO pin 54."]
pub mod pincfg54;
#[doc = "PINCFG55 (rw) register accessor: Controls the operation of GPIO pin 55.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg55`]
module"]
#[doc(alias = "PINCFG55")]
pub type Pincfg55 = crate::Reg<pincfg55::Pincfg55Spec>;
#[doc = "Controls the operation of GPIO pin 55."]
pub mod pincfg55;
#[doc = "PINCFG56 (rw) register accessor: Controls the operation of GPIO pin 56.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg56`]
module"]
#[doc(alias = "PINCFG56")]
pub type Pincfg56 = crate::Reg<pincfg56::Pincfg56Spec>;
#[doc = "Controls the operation of GPIO pin 56."]
pub mod pincfg56;
#[doc = "PINCFG57 (rw) register accessor: Controls the operation of GPIO pin 57.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg57`]
module"]
#[doc(alias = "PINCFG57")]
pub type Pincfg57 = crate::Reg<pincfg57::Pincfg57Spec>;
#[doc = "Controls the operation of GPIO pin 57."]
pub mod pincfg57;
#[doc = "PINCFG58 (rw) register accessor: Controls the operation of GPIO pin 58.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg58`]
module"]
#[doc(alias = "PINCFG58")]
pub type Pincfg58 = crate::Reg<pincfg58::Pincfg58Spec>;
#[doc = "Controls the operation of GPIO pin 58."]
pub mod pincfg58;
#[doc = "PINCFG59 (rw) register accessor: Controls the operation of GPIO pin 59.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg59`]
module"]
#[doc(alias = "PINCFG59")]
pub type Pincfg59 = crate::Reg<pincfg59::Pincfg59Spec>;
#[doc = "Controls the operation of GPIO pin 59."]
pub mod pincfg59;
#[doc = "PINCFG60 (rw) register accessor: Controls the operation of GPIO pin 60.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg60`]
module"]
#[doc(alias = "PINCFG60")]
pub type Pincfg60 = crate::Reg<pincfg60::Pincfg60Spec>;
#[doc = "Controls the operation of GPIO pin 60."]
pub mod pincfg60;
#[doc = "PINCFG61 (rw) register accessor: Controls the operation of GPIO pin 61.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg61`]
module"]
#[doc(alias = "PINCFG61")]
pub type Pincfg61 = crate::Reg<pincfg61::Pincfg61Spec>;
#[doc = "Controls the operation of GPIO pin 61."]
pub mod pincfg61;
#[doc = "PINCFG62 (rw) register accessor: Controls the operation of GPIO pin 62.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg62`]
module"]
#[doc(alias = "PINCFG62")]
pub type Pincfg62 = crate::Reg<pincfg62::Pincfg62Spec>;
#[doc = "Controls the operation of GPIO pin 62."]
pub mod pincfg62;
#[doc = "PINCFG63 (rw) register accessor: Controls the operation of GPIO pin 63.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg63`]
module"]
#[doc(alias = "PINCFG63")]
pub type Pincfg63 = crate::Reg<pincfg63::Pincfg63Spec>;
#[doc = "Controls the operation of GPIO pin 63."]
pub mod pincfg63;
#[doc = "PINCFG64 (rw) register accessor: Controls the operation of GPIO pin 64.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg64`]
module"]
#[doc(alias = "PINCFG64")]
pub type Pincfg64 = crate::Reg<pincfg64::Pincfg64Spec>;
#[doc = "Controls the operation of GPIO pin 64."]
pub mod pincfg64;
#[doc = "PINCFG65 (rw) register accessor: Controls the operation of GPIO pin 65.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg65`]
module"]
#[doc(alias = "PINCFG65")]
pub type Pincfg65 = crate::Reg<pincfg65::Pincfg65Spec>;
#[doc = "Controls the operation of GPIO pin 65."]
pub mod pincfg65;
#[doc = "PINCFG66 (rw) register accessor: Controls the operation of GPIO pin 66.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg66`]
module"]
#[doc(alias = "PINCFG66")]
pub type Pincfg66 = crate::Reg<pincfg66::Pincfg66Spec>;
#[doc = "Controls the operation of GPIO pin 66."]
pub mod pincfg66;
#[doc = "PINCFG67 (rw) register accessor: Controls the operation of GPIO pin 67.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg67`]
module"]
#[doc(alias = "PINCFG67")]
pub type Pincfg67 = crate::Reg<pincfg67::Pincfg67Spec>;
#[doc = "Controls the operation of GPIO pin 67."]
pub mod pincfg67;
#[doc = "PINCFG68 (rw) register accessor: Controls the operation of GPIO pin 68.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg68`]
module"]
#[doc(alias = "PINCFG68")]
pub type Pincfg68 = crate::Reg<pincfg68::Pincfg68Spec>;
#[doc = "Controls the operation of GPIO pin 68."]
pub mod pincfg68;
#[doc = "PINCFG69 (rw) register accessor: Controls the operation of GPIO pin 69.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg69`]
module"]
#[doc(alias = "PINCFG69")]
pub type Pincfg69 = crate::Reg<pincfg69::Pincfg69Spec>;
#[doc = "Controls the operation of GPIO pin 69."]
pub mod pincfg69;
#[doc = "PINCFG70 (rw) register accessor: Controls the operation of GPIO pin 70.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg70`]
module"]
#[doc(alias = "PINCFG70")]
pub type Pincfg70 = crate::Reg<pincfg70::Pincfg70Spec>;
#[doc = "Controls the operation of GPIO pin 70."]
pub mod pincfg70;
#[doc = "PINCFG71 (rw) register accessor: Controls the operation of GPIO pin 71.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg71`]
module"]
#[doc(alias = "PINCFG71")]
pub type Pincfg71 = crate::Reg<pincfg71::Pincfg71Spec>;
#[doc = "Controls the operation of GPIO pin 71."]
pub mod pincfg71;
#[doc = "PINCFG72 (rw) register accessor: Controls the operation of GPIO pin 72.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg72`]
module"]
#[doc(alias = "PINCFG72")]
pub type Pincfg72 = crate::Reg<pincfg72::Pincfg72Spec>;
#[doc = "Controls the operation of GPIO pin 72."]
pub mod pincfg72;
#[doc = "PINCFG73 (rw) register accessor: Controls the operation of GPIO pin 73.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg73`]
module"]
#[doc(alias = "PINCFG73")]
pub type Pincfg73 = crate::Reg<pincfg73::Pincfg73Spec>;
#[doc = "Controls the operation of GPIO pin 73."]
pub mod pincfg73;
#[doc = "PINCFG74 (rw) register accessor: Controls the operation of GPIO pin 74.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg74`]
module"]
#[doc(alias = "PINCFG74")]
pub type Pincfg74 = crate::Reg<pincfg74::Pincfg74Spec>;
#[doc = "Controls the operation of GPIO pin 74."]
pub mod pincfg74;
#[doc = "PINCFG75 (rw) register accessor: Controls the operation of GPIO pin 75.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg75`]
module"]
#[doc(alias = "PINCFG75")]
pub type Pincfg75 = crate::Reg<pincfg75::Pincfg75Spec>;
#[doc = "Controls the operation of GPIO pin 75."]
pub mod pincfg75;
#[doc = "PINCFG76 (rw) register accessor: Controls the operation of GPIO pin 76.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg76`]
module"]
#[doc(alias = "PINCFG76")]
pub type Pincfg76 = crate::Reg<pincfg76::Pincfg76Spec>;
#[doc = "Controls the operation of GPIO pin 76."]
pub mod pincfg76;
#[doc = "PINCFG77 (rw) register accessor: Controls the operation of GPIO pin 77.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg77`]
module"]
#[doc(alias = "PINCFG77")]
pub type Pincfg77 = crate::Reg<pincfg77::Pincfg77Spec>;
#[doc = "Controls the operation of GPIO pin 77."]
pub mod pincfg77;
#[doc = "PINCFG78 (rw) register accessor: Controls the operation of GPIO pin 78.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg78`]
module"]
#[doc(alias = "PINCFG78")]
pub type Pincfg78 = crate::Reg<pincfg78::Pincfg78Spec>;
#[doc = "Controls the operation of GPIO pin 78."]
pub mod pincfg78;
#[doc = "PINCFG79 (rw) register accessor: Controls the operation of GPIO pin 79.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg79`]
module"]
#[doc(alias = "PINCFG79")]
pub type Pincfg79 = crate::Reg<pincfg79::Pincfg79Spec>;
#[doc = "Controls the operation of GPIO pin 79."]
pub mod pincfg79;
#[doc = "PINCFG80 (rw) register accessor: Controls the operation of GPIO pin 80.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg80`]
module"]
#[doc(alias = "PINCFG80")]
pub type Pincfg80 = crate::Reg<pincfg80::Pincfg80Spec>;
#[doc = "Controls the operation of GPIO pin 80."]
pub mod pincfg80;
#[doc = "PINCFG81 (rw) register accessor: Controls the operation of GPIO pin 81.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg81`]
module"]
#[doc(alias = "PINCFG81")]
pub type Pincfg81 = crate::Reg<pincfg81::Pincfg81Spec>;
#[doc = "Controls the operation of GPIO pin 81."]
pub mod pincfg81;
#[doc = "PINCFG82 (rw) register accessor: Controls the operation of GPIO pin 82.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg82`]
module"]
#[doc(alias = "PINCFG82")]
pub type Pincfg82 = crate::Reg<pincfg82::Pincfg82Spec>;
#[doc = "Controls the operation of GPIO pin 82."]
pub mod pincfg82;
#[doc = "PINCFG83 (rw) register accessor: Controls the operation of GPIO pin 83.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg83`]
module"]
#[doc(alias = "PINCFG83")]
pub type Pincfg83 = crate::Reg<pincfg83::Pincfg83Spec>;
#[doc = "Controls the operation of GPIO pin 83."]
pub mod pincfg83;
#[doc = "PINCFG84 (rw) register accessor: Controls the operation of GPIO pin 84.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg84`]
module"]
#[doc(alias = "PINCFG84")]
pub type Pincfg84 = crate::Reg<pincfg84::Pincfg84Spec>;
#[doc = "Controls the operation of GPIO pin 84."]
pub mod pincfg84;
#[doc = "PINCFG85 (rw) register accessor: Controls the operation of GPIO pin 85.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg85`]
module"]
#[doc(alias = "PINCFG85")]
pub type Pincfg85 = crate::Reg<pincfg85::Pincfg85Spec>;
#[doc = "Controls the operation of GPIO pin 85."]
pub mod pincfg85;
#[doc = "PINCFG86 (rw) register accessor: Controls the operation of GPIO pin 86.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg86`]
module"]
#[doc(alias = "PINCFG86")]
pub type Pincfg86 = crate::Reg<pincfg86::Pincfg86Spec>;
#[doc = "Controls the operation of GPIO pin 86."]
pub mod pincfg86;
#[doc = "PINCFG87 (rw) register accessor: Controls the operation of GPIO pin 87.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg87`]
module"]
#[doc(alias = "PINCFG87")]
pub type Pincfg87 = crate::Reg<pincfg87::Pincfg87Spec>;
#[doc = "Controls the operation of GPIO pin 87."]
pub mod pincfg87;
#[doc = "PINCFG88 (rw) register accessor: Controls the operation of GPIO pin 88.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg88`]
module"]
#[doc(alias = "PINCFG88")]
pub type Pincfg88 = crate::Reg<pincfg88::Pincfg88Spec>;
#[doc = "Controls the operation of GPIO pin 88."]
pub mod pincfg88;
#[doc = "PINCFG89 (rw) register accessor: Controls the operation of GPIO pin 89.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg89`]
module"]
#[doc(alias = "PINCFG89")]
pub type Pincfg89 = crate::Reg<pincfg89::Pincfg89Spec>;
#[doc = "Controls the operation of GPIO pin 89."]
pub mod pincfg89;
#[doc = "PINCFG90 (rw) register accessor: Controls the operation of GPIO pin 90.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg90`]
module"]
#[doc(alias = "PINCFG90")]
pub type Pincfg90 = crate::Reg<pincfg90::Pincfg90Spec>;
#[doc = "Controls the operation of GPIO pin 90."]
pub mod pincfg90;
#[doc = "PINCFG91 (rw) register accessor: Controls the operation of GPIO pin 91.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg91`]
module"]
#[doc(alias = "PINCFG91")]
pub type Pincfg91 = crate::Reg<pincfg91::Pincfg91Spec>;
#[doc = "Controls the operation of GPIO pin 91."]
pub mod pincfg91;
#[doc = "PINCFG92 (rw) register accessor: Controls the operation of GPIO pin 92.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg92`]
module"]
#[doc(alias = "PINCFG92")]
pub type Pincfg92 = crate::Reg<pincfg92::Pincfg92Spec>;
#[doc = "Controls the operation of GPIO pin 92."]
pub mod pincfg92;
#[doc = "PINCFG93 (rw) register accessor: Controls the operation of GPIO pin 93.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg93`]
module"]
#[doc(alias = "PINCFG93")]
pub type Pincfg93 = crate::Reg<pincfg93::Pincfg93Spec>;
#[doc = "Controls the operation of GPIO pin 93."]
pub mod pincfg93;
#[doc = "PINCFG94 (rw) register accessor: Controls the operation of GPIO pin 94.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg94`]
module"]
#[doc(alias = "PINCFG94")]
pub type Pincfg94 = crate::Reg<pincfg94::Pincfg94Spec>;
#[doc = "Controls the operation of GPIO pin 94."]
pub mod pincfg94;
#[doc = "PINCFG95 (rw) register accessor: Controls the operation of GPIO pin 95.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg95`]
module"]
#[doc(alias = "PINCFG95")]
pub type Pincfg95 = crate::Reg<pincfg95::Pincfg95Spec>;
#[doc = "Controls the operation of GPIO pin 95."]
pub mod pincfg95;
#[doc = "PINCFG96 (rw) register accessor: Controls the operation of GPIO pin 96.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg96`]
module"]
#[doc(alias = "PINCFG96")]
pub type Pincfg96 = crate::Reg<pincfg96::Pincfg96Spec>;
#[doc = "Controls the operation of GPIO pin 96."]
pub mod pincfg96;
#[doc = "PINCFG97 (rw) register accessor: Controls the operation of GPIO pin 97.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg97`]
module"]
#[doc(alias = "PINCFG97")]
pub type Pincfg97 = crate::Reg<pincfg97::Pincfg97Spec>;
#[doc = "Controls the operation of GPIO pin 97."]
pub mod pincfg97;
#[doc = "PINCFG98 (rw) register accessor: Controls the operation of GPIO pin 98.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg98`]
module"]
#[doc(alias = "PINCFG98")]
pub type Pincfg98 = crate::Reg<pincfg98::Pincfg98Spec>;
#[doc = "Controls the operation of GPIO pin 98."]
pub mod pincfg98;
#[doc = "PINCFG99 (rw) register accessor: Controls the operation of GPIO pin 99.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg99`]
module"]
#[doc(alias = "PINCFG99")]
pub type Pincfg99 = crate::Reg<pincfg99::Pincfg99Spec>;
#[doc = "Controls the operation of GPIO pin 99."]
pub mod pincfg99;
#[doc = "PINCFG100 (rw) register accessor: Controls the operation of GPIO pin 100.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg100`]
module"]
#[doc(alias = "PINCFG100")]
pub type Pincfg100 = crate::Reg<pincfg100::Pincfg100Spec>;
#[doc = "Controls the operation of GPIO pin 100."]
pub mod pincfg100;
#[doc = "PINCFG101 (rw) register accessor: Controls the operation of GPIO pin 101.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg101`]
module"]
#[doc(alias = "PINCFG101")]
pub type Pincfg101 = crate::Reg<pincfg101::Pincfg101Spec>;
#[doc = "Controls the operation of GPIO pin 101."]
pub mod pincfg101;
#[doc = "PINCFG102 (rw) register accessor: Controls the operation of GPIO pin 102.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg102`]
module"]
#[doc(alias = "PINCFG102")]
pub type Pincfg102 = crate::Reg<pincfg102::Pincfg102Spec>;
#[doc = "Controls the operation of GPIO pin 102."]
pub mod pincfg102;
#[doc = "PINCFG103 (rw) register accessor: Controls the operation of GPIO pin 103.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg103`]
module"]
#[doc(alias = "PINCFG103")]
pub type Pincfg103 = crate::Reg<pincfg103::Pincfg103Spec>;
#[doc = "Controls the operation of GPIO pin 103."]
pub mod pincfg103;
#[doc = "PINCFG104 (rw) register accessor: Controls the operation of GPIO pin 104.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg104`]
module"]
#[doc(alias = "PINCFG104")]
pub type Pincfg104 = crate::Reg<pincfg104::Pincfg104Spec>;
#[doc = "Controls the operation of GPIO pin 104."]
pub mod pincfg104;
#[doc = "PINCFG105 (rw) register accessor: Controls the operation of virtual GPIO pin 105.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg105`]
module"]
#[doc(alias = "PINCFG105")]
pub type Pincfg105 = crate::Reg<pincfg105::Pincfg105Spec>;
#[doc = "Controls the operation of virtual GPIO pin 105."]
pub mod pincfg105;
#[doc = "PINCFG106 (rw) register accessor: Controls the operation of virtual GPIO pin 106.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg106`]
module"]
#[doc(alias = "PINCFG106")]
pub type Pincfg106 = crate::Reg<pincfg106::Pincfg106Spec>;
#[doc = "Controls the operation of virtual GPIO pin 106."]
pub mod pincfg106;
#[doc = "PINCFG107 (rw) register accessor: Controls the operation of virtual GPIO pin 107.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg107`]
module"]
#[doc(alias = "PINCFG107")]
pub type Pincfg107 = crate::Reg<pincfg107::Pincfg107Spec>;
#[doc = "Controls the operation of virtual GPIO pin 107."]
pub mod pincfg107;
#[doc = "PINCFG108 (rw) register accessor: Controls the operation of virtual GPIO pin 108.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg108`]
module"]
#[doc(alias = "PINCFG108")]
pub type Pincfg108 = crate::Reg<pincfg108::Pincfg108Spec>;
#[doc = "Controls the operation of virtual GPIO pin 108."]
pub mod pincfg108;
#[doc = "PINCFG109 (rw) register accessor: Controls the operation of virtual GPIO pin 109.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg109`]
module"]
#[doc(alias = "PINCFG109")]
pub type Pincfg109 = crate::Reg<pincfg109::Pincfg109Spec>;
#[doc = "Controls the operation of virtual GPIO pin 109."]
pub mod pincfg109;
#[doc = "PINCFG110 (rw) register accessor: Controls the operation of virtual GPIO pin 110.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg110`]
module"]
#[doc(alias = "PINCFG110")]
pub type Pincfg110 = crate::Reg<pincfg110::Pincfg110Spec>;
#[doc = "Controls the operation of virtual GPIO pin 110."]
pub mod pincfg110;
#[doc = "PINCFG111 (rw) register accessor: Controls the operation of virtual GPIO pin 111.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg111`]
module"]
#[doc(alias = "PINCFG111")]
pub type Pincfg111 = crate::Reg<pincfg111::Pincfg111Spec>;
#[doc = "Controls the operation of virtual GPIO pin 111."]
pub mod pincfg111;
#[doc = "PINCFG112 (rw) register accessor: Controls the operation of virtual GPIO pin 112.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg112`]
module"]
#[doc(alias = "PINCFG112")]
pub type Pincfg112 = crate::Reg<pincfg112::Pincfg112Spec>;
#[doc = "Controls the operation of virtual GPIO pin 112."]
pub mod pincfg112;
#[doc = "PINCFG113 (rw) register accessor: Controls the operation of virtual GPIO pin 113.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg113`]
module"]
#[doc(alias = "PINCFG113")]
pub type Pincfg113 = crate::Reg<pincfg113::Pincfg113Spec>;
#[doc = "Controls the operation of virtual GPIO pin 113."]
pub mod pincfg113;
#[doc = "PINCFG114 (rw) register accessor: Controls the operation of virtual GPIO pin 114.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg114`]
module"]
#[doc(alias = "PINCFG114")]
pub type Pincfg114 = crate::Reg<pincfg114::Pincfg114Spec>;
#[doc = "Controls the operation of virtual GPIO pin 114."]
pub mod pincfg114;
#[doc = "PINCFG115 (rw) register accessor: Controls the operation of virtual GPIO pin 115.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg115`]
module"]
#[doc(alias = "PINCFG115")]
pub type Pincfg115 = crate::Reg<pincfg115::Pincfg115Spec>;
#[doc = "Controls the operation of virtual GPIO pin 115."]
pub mod pincfg115;
#[doc = "PINCFG116 (rw) register accessor: Controls the operation of virtual GPIO pin 116.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg116`]
module"]
#[doc(alias = "PINCFG116")]
pub type Pincfg116 = crate::Reg<pincfg116::Pincfg116Spec>;
#[doc = "Controls the operation of virtual GPIO pin 116."]
pub mod pincfg116;
#[doc = "PINCFG117 (rw) register accessor: Controls the operation of virtual GPIO pin 117.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg117`]
module"]
#[doc(alias = "PINCFG117")]
pub type Pincfg117 = crate::Reg<pincfg117::Pincfg117Spec>;
#[doc = "Controls the operation of virtual GPIO pin 117."]
pub mod pincfg117;
#[doc = "PINCFG118 (rw) register accessor: Controls the operation of virtual GPIO pin 118.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg118`]
module"]
#[doc(alias = "PINCFG118")]
pub type Pincfg118 = crate::Reg<pincfg118::Pincfg118Spec>;
#[doc = "Controls the operation of virtual GPIO pin 118."]
pub mod pincfg118;
#[doc = "PINCFG119 (rw) register accessor: Controls the operation of virtual GPIO pin 119.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg119`]
module"]
#[doc(alias = "PINCFG119")]
pub type Pincfg119 = crate::Reg<pincfg119::Pincfg119Spec>;
#[doc = "Controls the operation of virtual GPIO pin 119."]
pub mod pincfg119;
#[doc = "PINCFG120 (rw) register accessor: Controls the operation of virtual GPIO pin 120.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg120`]
module"]
#[doc(alias = "PINCFG120")]
pub type Pincfg120 = crate::Reg<pincfg120::Pincfg120Spec>;
#[doc = "Controls the operation of virtual GPIO pin 120."]
pub mod pincfg120;
#[doc = "PINCFG121 (rw) register accessor: Controls the operation of virtual GPIO pin 121.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg121`]
module"]
#[doc(alias = "PINCFG121")]
pub type Pincfg121 = crate::Reg<pincfg121::Pincfg121Spec>;
#[doc = "Controls the operation of virtual GPIO pin 121."]
pub mod pincfg121;
#[doc = "PINCFG122 (rw) register accessor: Controls the operation of virtual GPIO pin 122.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg122`]
module"]
#[doc(alias = "PINCFG122")]
pub type Pincfg122 = crate::Reg<pincfg122::Pincfg122Spec>;
#[doc = "Controls the operation of virtual GPIO pin 122."]
pub mod pincfg122;
#[doc = "PINCFG123 (rw) register accessor: Controls the operation of virtual GPIO pin 123.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg123`]
module"]
#[doc(alias = "PINCFG123")]
pub type Pincfg123 = crate::Reg<pincfg123::Pincfg123Spec>;
#[doc = "Controls the operation of virtual GPIO pin 123."]
pub mod pincfg123;
#[doc = "PINCFG124 (rw) register accessor: Controls the operation of virtual GPIO pin 124.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg124`]
module"]
#[doc(alias = "PINCFG124")]
pub type Pincfg124 = crate::Reg<pincfg124::Pincfg124Spec>;
#[doc = "Controls the operation of virtual GPIO pin 124."]
pub mod pincfg124;
#[doc = "PINCFG125 (rw) register accessor: Controls the operation of virtual GPIO pin 125.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg125`]
module"]
#[doc(alias = "PINCFG125")]
pub type Pincfg125 = crate::Reg<pincfg125::Pincfg125Spec>;
#[doc = "Controls the operation of virtual GPIO pin 125."]
pub mod pincfg125;
#[doc = "PINCFG126 (rw) register accessor: Controls the operation of virtual GPIO pin 126.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg126`]
module"]
#[doc(alias = "PINCFG126")]
pub type Pincfg126 = crate::Reg<pincfg126::Pincfg126Spec>;
#[doc = "Controls the operation of virtual GPIO pin 126."]
pub mod pincfg126;
#[doc = "PINCFG127 (rw) register accessor: Controls the operation of virtual GPIO pin 127.\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg127`]
module"]
#[doc(alias = "PINCFG127")]
pub type Pincfg127 = crate::Reg<pincfg127::Pincfg127Spec>;
#[doc = "Controls the operation of virtual GPIO pin 127."]
pub mod pincfg127;
#[doc = "PADKEY (rw) register accessor: Lock state of the PINCFG and GPIO configuration registers. Write a value of 0x73 to unlock write access to the PAD and GPIO.\n\nYou can [`read`](crate::Reg::read) this register and get [`padkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padkey`]
module"]
#[doc(alias = "PADKEY")]
pub type Padkey = crate::Reg<padkey::PadkeySpec>;
#[doc = "Lock state of the PINCFG and GPIO configuration registers. Write a value of 0x73 to unlock write access to the PAD and GPIO."]
pub mod padkey;
#[doc = "RD0 (rw) register accessor: GPIO Input 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd0`]
module"]
#[doc(alias = "RD0")]
pub type Rd0 = crate::Reg<rd0::Rd0Spec>;
#[doc = "GPIO Input 0 (31-0)"]
pub mod rd0;
#[doc = "RD1 (rw) register accessor: GPIO Input 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd1`]
module"]
#[doc(alias = "RD1")]
pub type Rd1 = crate::Reg<rd1::Rd1Spec>;
#[doc = "GPIO Input 1 (63-32)"]
pub mod rd1;
#[doc = "RD2 (rw) register accessor: GPIO Input 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd2`]
module"]
#[doc(alias = "RD2")]
pub type Rd2 = crate::Reg<rd2::Rd2Spec>;
#[doc = "GPIO Input 2 (95-64)"]
pub mod rd2;
#[doc = "RD3 (rw) register accessor: GPIO Input 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`rd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd3`]
module"]
#[doc(alias = "RD3")]
pub type Rd3 = crate::Reg<rd3::Rd3Spec>;
#[doc = "GPIO Input 3 (127-96)"]
pub mod rd3;
#[doc = "WT0 (rw) register accessor: GPIO Output 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt0`]
module"]
#[doc(alias = "WT0")]
pub type Wt0 = crate::Reg<wt0::Wt0Spec>;
#[doc = "GPIO Output 0 (31-0)"]
pub mod wt0;
#[doc = "WT1 (rw) register accessor: GPIO Output 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt1`]
module"]
#[doc(alias = "WT1")]
pub type Wt1 = crate::Reg<wt1::Wt1Spec>;
#[doc = "GPIO Output 1 (63-32)"]
pub mod wt1;
#[doc = "WT2 (rw) register accessor: GPIO Output 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt2`]
module"]
#[doc(alias = "WT2")]
pub type Wt2 = crate::Reg<wt2::Wt2Spec>;
#[doc = "GPIO Output 2 (95-64)"]
pub mod wt2;
#[doc = "WT3 (rw) register accessor: GPIO Output 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wt3`]
module"]
#[doc(alias = "WT3")]
pub type Wt3 = crate::Reg<wt3::Wt3Spec>;
#[doc = "GPIO Output 3 (127-96)"]
pub mod wt3;
#[doc = "WTS0 (rw) register accessor: GPIO Output Set 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts0`]
module"]
#[doc(alias = "WTS0")]
pub type Wts0 = crate::Reg<wts0::Wts0Spec>;
#[doc = "GPIO Output Set 0 (31-0)"]
pub mod wts0;
#[doc = "WTS1 (rw) register accessor: GPIO Output Set 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts1`]
module"]
#[doc(alias = "WTS1")]
pub type Wts1 = crate::Reg<wts1::Wts1Spec>;
#[doc = "GPIO Output Set 1 (63-32)"]
pub mod wts1;
#[doc = "WTS2 (rw) register accessor: GPIO Output Set 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts2`]
module"]
#[doc(alias = "WTS2")]
pub type Wts2 = crate::Reg<wts2::Wts2Spec>;
#[doc = "GPIO Output Set 2 (95-64)"]
pub mod wts2;
#[doc = "WTS3 (rw) register accessor: GPIO Output Set 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wts3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wts3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wts3`]
module"]
#[doc(alias = "WTS3")]
pub type Wts3 = crate::Reg<wts3::Wts3Spec>;
#[doc = "GPIO Output Set 3 (127-96)"]
pub mod wts3;
#[doc = "WTC0 (rw) register accessor: GPIO Output Clear 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc0`]
module"]
#[doc(alias = "WTC0")]
pub type Wtc0 = crate::Reg<wtc0::Wtc0Spec>;
#[doc = "GPIO Output Clear 0 (31-0)"]
pub mod wtc0;
#[doc = "WTC1 (rw) register accessor: GPIO Output Clear 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc1`]
module"]
#[doc(alias = "WTC1")]
pub type Wtc1 = crate::Reg<wtc1::Wtc1Spec>;
#[doc = "GPIO Output Clear 1 (63-32)"]
pub mod wtc1;
#[doc = "WTC2 (rw) register accessor: GPIO Output Clear 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc2`]
module"]
#[doc(alias = "WTC2")]
pub type Wtc2 = crate::Reg<wtc2::Wtc2Spec>;
#[doc = "GPIO Output Clear 2 (95-64)"]
pub mod wtc2;
#[doc = "WTC3 (rw) register accessor: GPIO Output Clear 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtc3`]
module"]
#[doc(alias = "WTC3")]
pub type Wtc3 = crate::Reg<wtc3::Wtc3Spec>;
#[doc = "GPIO Output Clear 3 (127-96)"]
pub mod wtc3;
#[doc = "EN0 (rw) register accessor: GPIO Enable 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en0`]
module"]
#[doc(alias = "EN0")]
pub type En0 = crate::Reg<en0::En0Spec>;
#[doc = "GPIO Enable 0 (31-0)"]
pub mod en0;
#[doc = "EN1 (rw) register accessor: GPIO Enable 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en1`]
module"]
#[doc(alias = "EN1")]
pub type En1 = crate::Reg<en1::En1Spec>;
#[doc = "GPIO Enable 1 (63-32)"]
pub mod en1;
#[doc = "EN2 (rw) register accessor: GPIO Enable 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en2`]
module"]
#[doc(alias = "EN2")]
pub type En2 = crate::Reg<en2::En2Spec>;
#[doc = "GPIO Enable 2 (95-64)"]
pub mod en2;
#[doc = "EN3 (rw) register accessor: GPIO Enable 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`en3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en3`]
module"]
#[doc(alias = "EN3")]
pub type En3 = crate::Reg<en3::En3Spec>;
#[doc = "GPIO Enable 3 (127-96)"]
pub mod en3;
#[doc = "ENS0 (rw) register accessor: GPIO Enable Set 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens0`]
module"]
#[doc(alias = "ENS0")]
pub type Ens0 = crate::Reg<ens0::Ens0Spec>;
#[doc = "GPIO Enable Set 0 (31-0)"]
pub mod ens0;
#[doc = "ENS1 (rw) register accessor: GPIO Enable Set 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens1`]
module"]
#[doc(alias = "ENS1")]
pub type Ens1 = crate::Reg<ens1::Ens1Spec>;
#[doc = "GPIO Enable Set 1 (63-32)"]
pub mod ens1;
#[doc = "ENS2 (rw) register accessor: GPIO Enable Set 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens2`]
module"]
#[doc(alias = "ENS2")]
pub type Ens2 = crate::Reg<ens2::Ens2Spec>;
#[doc = "GPIO Enable Set 2 (95-64)"]
pub mod ens2;
#[doc = "ENS3 (rw) register accessor: GPIO Enable Set 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`ens3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ens3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ens3`]
module"]
#[doc(alias = "ENS3")]
pub type Ens3 = crate::Reg<ens3::Ens3Spec>;
#[doc = "GPIO Enable Set 3 (127-96)"]
pub mod ens3;
#[doc = "ENC0 (rw) register accessor: GPIO Enable Clear 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc0`]
module"]
#[doc(alias = "ENC0")]
pub type Enc0 = crate::Reg<enc0::Enc0Spec>;
#[doc = "GPIO Enable Clear 0 (31-0)"]
pub mod enc0;
#[doc = "ENC1 (rw) register accessor: GPIO Enable Clear 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc1`]
module"]
#[doc(alias = "ENC1")]
pub type Enc1 = crate::Reg<enc1::Enc1Spec>;
#[doc = "GPIO Enable Clear 1 (63-32)"]
pub mod enc1;
#[doc = "ENC2 (rw) register accessor: GPIO Enable Clear 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc2`]
module"]
#[doc(alias = "ENC2")]
pub type Enc2 = crate::Reg<enc2::Enc2Spec>;
#[doc = "GPIO Enable Clear 2 (95-64)"]
pub mod enc2;
#[doc = "ENC3 (rw) register accessor: GPIO Enable Clear 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`enc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enc3`]
module"]
#[doc(alias = "ENC3")]
pub type Enc3 = crate::Reg<enc3::Enc3Spec>;
#[doc = "GPIO Enable Clear 3 (127-96)"]
pub mod enc3;
#[doc = "IOM0IRQ (rw) register accessor: IOM0 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom0irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom0irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom0irq`]
module"]
#[doc(alias = "IOM0IRQ")]
pub type Iom0irq = crate::Reg<iom0irq::Iom0irqSpec>;
#[doc = "IOM0 IRQ select for flow control."]
pub mod iom0irq;
#[doc = "IOM1IRQ (rw) register accessor: IOM1 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom1irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom1irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom1irq`]
module"]
#[doc(alias = "IOM1IRQ")]
pub type Iom1irq = crate::Reg<iom1irq::Iom1irqSpec>;
#[doc = "IOM1 IRQ select for flow control."]
pub mod iom1irq;
#[doc = "IOM2IRQ (rw) register accessor: IOM2 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom2irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom2irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom2irq`]
module"]
#[doc(alias = "IOM2IRQ")]
pub type Iom2irq = crate::Reg<iom2irq::Iom2irqSpec>;
#[doc = "IOM2 IRQ select for flow control."]
pub mod iom2irq;
#[doc = "IOM3IRQ (rw) register accessor: IOM3 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom3irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom3irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom3irq`]
module"]
#[doc(alias = "IOM3IRQ")]
pub type Iom3irq = crate::Reg<iom3irq::Iom3irqSpec>;
#[doc = "IOM3 IRQ select for flow control."]
pub mod iom3irq;
#[doc = "IOM4IRQ (rw) register accessor: IOM4 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom4irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom4irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom4irq`]
module"]
#[doc(alias = "IOM4IRQ")]
pub type Iom4irq = crate::Reg<iom4irq::Iom4irqSpec>;
#[doc = "IOM4 IRQ select for flow control."]
pub mod iom4irq;
#[doc = "IOM5IRQ (rw) register accessor: IOM5 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom5irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom5irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom5irq`]
module"]
#[doc(alias = "IOM5IRQ")]
pub type Iom5irq = crate::Reg<iom5irq::Iom5irqSpec>;
#[doc = "IOM5 IRQ select for flow control."]
pub mod iom5irq;
#[doc = "IOM6IRQ (rw) register accessor: IOM6 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom6irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom6irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom6irq`]
module"]
#[doc(alias = "IOM6IRQ")]
pub type Iom6irq = crate::Reg<iom6irq::Iom6irqSpec>;
#[doc = "IOM6 IRQ select for flow control."]
pub mod iom6irq;
#[doc = "IOM7IRQ (rw) register accessor: IOM7 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom7irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom7irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iom7irq`]
module"]
#[doc(alias = "IOM7IRQ")]
pub type Iom7irq = crate::Reg<iom7irq::Iom7irqSpec>;
#[doc = "IOM7 IRQ select for flow control."]
pub mod iom7irq;
#[doc = "SDIFCDWP (rw) register accessor: SDIF CD and WP Select.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdifcdwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdifcdwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdifcdwp`]
module"]
#[doc(alias = "SDIFCDWP")]
pub type Sdifcdwp = crate::Reg<sdifcdwp::SdifcdwpSpec>;
#[doc = "SDIF CD and WP Select."]
pub mod sdifcdwp;
#[doc = "OBSDATA (rw) register accessor: GPIO Observation mode sample\n\nYou can [`read`](crate::Reg::read) this register and get [`obsdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obsdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obsdata`]
module"]
#[doc(alias = "OBSDATA")]
pub type Obsdata = crate::Reg<obsdata::ObsdataSpec>;
#[doc = "GPIO Observation mode sample"]
pub mod obsdata;
#[doc = "IEOBS0 (rw) register accessor: Read only. Reflects the value of the input enable signals for pads 31-0 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieobs0`]
module"]
#[doc(alias = "IEOBS0")]
pub type Ieobs0 = crate::Reg<ieobs0::Ieobs0Spec>;
#[doc = "Read only. Reflects the value of the input enable signals for pads 31-0 sent to the pad."]
pub mod ieobs0;
#[doc = "IEOBS1 (rw) register accessor: Read only. Reflects the value of the input enable signals for pads 63-32 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieobs1`]
module"]
#[doc(alias = "IEOBS1")]
pub type Ieobs1 = crate::Reg<ieobs1::Ieobs1Spec>;
#[doc = "Read only. Reflects the value of the input enable signals for pads 63-32 sent to the pad."]
pub mod ieobs1;
#[doc = "IEOBS2 (rw) register accessor: Read only. Reflects the value of the input enable signals for pads 95-64 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieobs2`]
module"]
#[doc(alias = "IEOBS2")]
pub type Ieobs2 = crate::Reg<ieobs2::Ieobs2Spec>;
#[doc = "Read only. Reflects the value of the input enable signals for pads 95-64 sent to the pad."]
pub mod ieobs2;
#[doc = "IEOBS3 (rw) register accessor: Read only. Reflects the value of the input enable signals for pads 127-96 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`ieobs3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieobs3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieobs3`]
module"]
#[doc(alias = "IEOBS3")]
pub type Ieobs3 = crate::Reg<ieobs3::Ieobs3Spec>;
#[doc = "Read only. Reflects the value of the input enable signals for pads 127-96 sent to the pad."]
pub mod ieobs3;
#[doc = "OEOBS0 (rw) register accessor: Read only. Reflects the value of the output enable signals for pads 31-0 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oeobs0`]
module"]
#[doc(alias = "OEOBS0")]
pub type Oeobs0 = crate::Reg<oeobs0::Oeobs0Spec>;
#[doc = "Read only. Reflects the value of the output enable signals for pads 31-0 sent to the pad."]
pub mod oeobs0;
#[doc = "OEOBS1 (rw) register accessor: Read only. Reflects the value of the output enable signals for pads 63-32 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oeobs1`]
module"]
#[doc(alias = "OEOBS1")]
pub type Oeobs1 = crate::Reg<oeobs1::Oeobs1Spec>;
#[doc = "Read only. Reflects the value of the output enable signals for pads 63-32 sent to the pad."]
pub mod oeobs1;
#[doc = "OEOBS2 (rw) register accessor: Read only. Reflects the value of the output enable signals for pads 95-64 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oeobs2`]
module"]
#[doc(alias = "OEOBS2")]
pub type Oeobs2 = crate::Reg<oeobs2::Oeobs2Spec>;
#[doc = "Read only. Reflects the value of the output enable signals for pads 95-64 sent to the pad."]
pub mod oeobs2;
#[doc = "OEOBS3 (rw) register accessor: Read only. Reflects the value of the output enable signals for pads 127-96 sent to the pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`oeobs3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeobs3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oeobs3`]
module"]
#[doc(alias = "OEOBS3")]
pub type Oeobs3 = crate::Reg<oeobs3::Oeobs3Spec>;
#[doc = "Read only. Reflects the value of the output enable signals for pads 127-96 sent to the pad."]
pub mod oeobs3;
#[doc = "MCUN0INT0EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int0en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int0en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int0en`]
module"]
#[doc(alias = "MCUN0INT0EN")]
pub type Mcun0int0en = crate::Reg<mcun0int0en::Mcun0int0enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun0int0en;
#[doc = "MCUN0INT0STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int0stat`]
module"]
#[doc(alias = "MCUN0INT0STAT")]
pub type Mcun0int0stat = crate::Reg<mcun0int0stat::Mcun0int0statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun0int0stat;
#[doc = "MCUN0INT0CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int0clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int0clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int0clr`]
module"]
#[doc(alias = "MCUN0INT0CLR")]
pub type Mcun0int0clr = crate::Reg<mcun0int0clr::Mcun0int0clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun0int0clr;
#[doc = "MCUN0INT0SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int0set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int0set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int0set`]
module"]
#[doc(alias = "MCUN0INT0SET")]
pub type Mcun0int0set = crate::Reg<mcun0int0set::Mcun0int0setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun0int0set;
#[doc = "MCUN0INT1EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int1en`]
module"]
#[doc(alias = "MCUN0INT1EN")]
pub type Mcun0int1en = crate::Reg<mcun0int1en::Mcun0int1enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun0int1en;
#[doc = "MCUN0INT1STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int1stat`]
module"]
#[doc(alias = "MCUN0INT1STAT")]
pub type Mcun0int1stat = crate::Reg<mcun0int1stat::Mcun0int1statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun0int1stat;
#[doc = "MCUN0INT1CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int1clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int1clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int1clr`]
module"]
#[doc(alias = "MCUN0INT1CLR")]
pub type Mcun0int1clr = crate::Reg<mcun0int1clr::Mcun0int1clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun0int1clr;
#[doc = "MCUN0INT1SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int1set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int1set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int1set`]
module"]
#[doc(alias = "MCUN0INT1SET")]
pub type Mcun0int1set = crate::Reg<mcun0int1set::Mcun0int1setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun0int1set;
#[doc = "MCUN0INT2EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int2en`]
module"]
#[doc(alias = "MCUN0INT2EN")]
pub type Mcun0int2en = crate::Reg<mcun0int2en::Mcun0int2enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun0int2en;
#[doc = "MCUN0INT2STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int2stat`]
module"]
#[doc(alias = "MCUN0INT2STAT")]
pub type Mcun0int2stat = crate::Reg<mcun0int2stat::Mcun0int2statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun0int2stat;
#[doc = "MCUN0INT2CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int2clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int2clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int2clr`]
module"]
#[doc(alias = "MCUN0INT2CLR")]
pub type Mcun0int2clr = crate::Reg<mcun0int2clr::Mcun0int2clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun0int2clr;
#[doc = "MCUN0INT2SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int2set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int2set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int2set`]
module"]
#[doc(alias = "MCUN0INT2SET")]
pub type Mcun0int2set = crate::Reg<mcun0int2set::Mcun0int2setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun0int2set;
#[doc = "MCUN0INT3EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int3en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int3en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int3en`]
module"]
#[doc(alias = "MCUN0INT3EN")]
pub type Mcun0int3en = crate::Reg<mcun0int3en::Mcun0int3enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun0int3en;
#[doc = "MCUN0INT3STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int3stat`]
module"]
#[doc(alias = "MCUN0INT3STAT")]
pub type Mcun0int3stat = crate::Reg<mcun0int3stat::Mcun0int3statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun0int3stat;
#[doc = "MCUN0INT3CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int3clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int3clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int3clr`]
module"]
#[doc(alias = "MCUN0INT3CLR")]
pub type Mcun0int3clr = crate::Reg<mcun0int3clr::Mcun0int3clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun0int3clr;
#[doc = "MCUN0INT3SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun0int3set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun0int3set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun0int3set`]
module"]
#[doc(alias = "MCUN0INT3SET")]
pub type Mcun0int3set = crate::Reg<mcun0int3set::Mcun0int3setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun0int3set;
#[doc = "MCUN1INT0EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int0en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int0en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int0en`]
module"]
#[doc(alias = "MCUN1INT0EN")]
pub type Mcun1int0en = crate::Reg<mcun1int0en::Mcun1int0enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun1int0en;
#[doc = "MCUN1INT0STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int0stat`]
module"]
#[doc(alias = "MCUN1INT0STAT")]
pub type Mcun1int0stat = crate::Reg<mcun1int0stat::Mcun1int0statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun1int0stat;
#[doc = "MCUN1INT0CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int0clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int0clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int0clr`]
module"]
#[doc(alias = "MCUN1INT0CLR")]
pub type Mcun1int0clr = crate::Reg<mcun1int0clr::Mcun1int0clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun1int0clr;
#[doc = "MCUN1INT0SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int0set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int0set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int0set`]
module"]
#[doc(alias = "MCUN1INT0SET")]
pub type Mcun1int0set = crate::Reg<mcun1int0set::Mcun1int0setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun1int0set;
#[doc = "MCUN1INT1EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int1en`]
module"]
#[doc(alias = "MCUN1INT1EN")]
pub type Mcun1int1en = crate::Reg<mcun1int1en::Mcun1int1enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun1int1en;
#[doc = "MCUN1INT1STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int1stat`]
module"]
#[doc(alias = "MCUN1INT1STAT")]
pub type Mcun1int1stat = crate::Reg<mcun1int1stat::Mcun1int1statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun1int1stat;
#[doc = "MCUN1INT1CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int1clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int1clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int1clr`]
module"]
#[doc(alias = "MCUN1INT1CLR")]
pub type Mcun1int1clr = crate::Reg<mcun1int1clr::Mcun1int1clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun1int1clr;
#[doc = "MCUN1INT1SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int1set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int1set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int1set`]
module"]
#[doc(alias = "MCUN1INT1SET")]
pub type Mcun1int1set = crate::Reg<mcun1int1set::Mcun1int1setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun1int1set;
#[doc = "MCUN1INT2EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int2en`]
module"]
#[doc(alias = "MCUN1INT2EN")]
pub type Mcun1int2en = crate::Reg<mcun1int2en::Mcun1int2enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun1int2en;
#[doc = "MCUN1INT2STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int2stat`]
module"]
#[doc(alias = "MCUN1INT2STAT")]
pub type Mcun1int2stat = crate::Reg<mcun1int2stat::Mcun1int2statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun1int2stat;
#[doc = "MCUN1INT2CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int2clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int2clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int2clr`]
module"]
#[doc(alias = "MCUN1INT2CLR")]
pub type Mcun1int2clr = crate::Reg<mcun1int2clr::Mcun1int2clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun1int2clr;
#[doc = "MCUN1INT2SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int2set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int2set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int2set`]
module"]
#[doc(alias = "MCUN1INT2SET")]
pub type Mcun1int2set = crate::Reg<mcun1int2set::Mcun1int2setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun1int2set;
#[doc = "MCUN1INT3EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int3en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int3en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int3en`]
module"]
#[doc(alias = "MCUN1INT3EN")]
pub type Mcun1int3en = crate::Reg<mcun1int3en::Mcun1int3enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod mcun1int3en;
#[doc = "MCUN1INT3STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int3stat`]
module"]
#[doc(alias = "MCUN1INT3STAT")]
pub type Mcun1int3stat = crate::Reg<mcun1int3stat::Mcun1int3statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod mcun1int3stat;
#[doc = "MCUN1INT3CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int3clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int3clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int3clr`]
module"]
#[doc(alias = "MCUN1INT3CLR")]
pub type Mcun1int3clr = crate::Reg<mcun1int3clr::Mcun1int3clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod mcun1int3clr;
#[doc = "MCUN1INT3SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`mcun1int3set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcun1int3set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcun1int3set`]
module"]
#[doc(alias = "MCUN1INT3SET")]
pub type Mcun1int3set = crate::Reg<mcun1int3set::Mcun1int3setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod mcun1int3set;
#[doc = "DSP0N0INT0EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int0en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int0en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int0en`]
module"]
#[doc(alias = "DSP0N0INT0EN")]
pub type Dsp0n0int0en = crate::Reg<dsp0n0int0en::Dsp0n0int0enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n0int0en;
#[doc = "DSP0N0INT0STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int0stat`]
module"]
#[doc(alias = "DSP0N0INT0STAT")]
pub type Dsp0n0int0stat = crate::Reg<dsp0n0int0stat::Dsp0n0int0statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n0int0stat;
#[doc = "DSP0N0INT0CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int0clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int0clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int0clr`]
module"]
#[doc(alias = "DSP0N0INT0CLR")]
pub type Dsp0n0int0clr = crate::Reg<dsp0n0int0clr::Dsp0n0int0clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n0int0clr;
#[doc = "DSP0N0INT0SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int0set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int0set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int0set`]
module"]
#[doc(alias = "DSP0N0INT0SET")]
pub type Dsp0n0int0set = crate::Reg<dsp0n0int0set::Dsp0n0int0setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n0int0set;
#[doc = "DSP0N0INT1EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int1en`]
module"]
#[doc(alias = "DSP0N0INT1EN")]
pub type Dsp0n0int1en = crate::Reg<dsp0n0int1en::Dsp0n0int1enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n0int1en;
#[doc = "DSP0N0INT1STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int1stat`]
module"]
#[doc(alias = "DSP0N0INT1STAT")]
pub type Dsp0n0int1stat = crate::Reg<dsp0n0int1stat::Dsp0n0int1statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n0int1stat;
#[doc = "DSP0N0INT1CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int1clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int1clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int1clr`]
module"]
#[doc(alias = "DSP0N0INT1CLR")]
pub type Dsp0n0int1clr = crate::Reg<dsp0n0int1clr::Dsp0n0int1clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n0int1clr;
#[doc = "DSP0N0INT1SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int1set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int1set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int1set`]
module"]
#[doc(alias = "DSP0N0INT1SET")]
pub type Dsp0n0int1set = crate::Reg<dsp0n0int1set::Dsp0n0int1setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n0int1set;
#[doc = "DSP0N0INT2EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int2en`]
module"]
#[doc(alias = "DSP0N0INT2EN")]
pub type Dsp0n0int2en = crate::Reg<dsp0n0int2en::Dsp0n0int2enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n0int2en;
#[doc = "DSP0N0INT2STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int2stat`]
module"]
#[doc(alias = "DSP0N0INT2STAT")]
pub type Dsp0n0int2stat = crate::Reg<dsp0n0int2stat::Dsp0n0int2statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n0int2stat;
#[doc = "DSP0N0INT2CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int2clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int2clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int2clr`]
module"]
#[doc(alias = "DSP0N0INT2CLR")]
pub type Dsp0n0int2clr = crate::Reg<dsp0n0int2clr::Dsp0n0int2clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n0int2clr;
#[doc = "DSP0N0INT2SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int2set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int2set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int2set`]
module"]
#[doc(alias = "DSP0N0INT2SET")]
pub type Dsp0n0int2set = crate::Reg<dsp0n0int2set::Dsp0n0int2setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n0int2set;
#[doc = "DSP0N0INT3EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int3en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int3en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int3en`]
module"]
#[doc(alias = "DSP0N0INT3EN")]
pub type Dsp0n0int3en = crate::Reg<dsp0n0int3en::Dsp0n0int3enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n0int3en;
#[doc = "DSP0N0INT3STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int3stat`]
module"]
#[doc(alias = "DSP0N0INT3STAT")]
pub type Dsp0n0int3stat = crate::Reg<dsp0n0int3stat::Dsp0n0int3statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n0int3stat;
#[doc = "DSP0N0INT3CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int3clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int3clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int3clr`]
module"]
#[doc(alias = "DSP0N0INT3CLR")]
pub type Dsp0n0int3clr = crate::Reg<dsp0n0int3clr::Dsp0n0int3clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n0int3clr;
#[doc = "DSP0N0INT3SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n0int3set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n0int3set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n0int3set`]
module"]
#[doc(alias = "DSP0N0INT3SET")]
pub type Dsp0n0int3set = crate::Reg<dsp0n0int3set::Dsp0n0int3setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n0int3set;
#[doc = "DSP0N1INT0EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int0en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int0en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int0en`]
module"]
#[doc(alias = "DSP0N1INT0EN")]
pub type Dsp0n1int0en = crate::Reg<dsp0n1int0en::Dsp0n1int0enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n1int0en;
#[doc = "DSP0N1INT0STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int0stat`]
module"]
#[doc(alias = "DSP0N1INT0STAT")]
pub type Dsp0n1int0stat = crate::Reg<dsp0n1int0stat::Dsp0n1int0statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n1int0stat;
#[doc = "DSP0N1INT0CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int0clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int0clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int0clr`]
module"]
#[doc(alias = "DSP0N1INT0CLR")]
pub type Dsp0n1int0clr = crate::Reg<dsp0n1int0clr::Dsp0n1int0clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n1int0clr;
#[doc = "DSP0N1INT0SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int0set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int0set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int0set`]
module"]
#[doc(alias = "DSP0N1INT0SET")]
pub type Dsp0n1int0set = crate::Reg<dsp0n1int0set::Dsp0n1int0setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n1int0set;
#[doc = "DSP0N1INT1EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int1en`]
module"]
#[doc(alias = "DSP0N1INT1EN")]
pub type Dsp0n1int1en = crate::Reg<dsp0n1int1en::Dsp0n1int1enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n1int1en;
#[doc = "DSP0N1INT1STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int1stat`]
module"]
#[doc(alias = "DSP0N1INT1STAT")]
pub type Dsp0n1int1stat = crate::Reg<dsp0n1int1stat::Dsp0n1int1statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n1int1stat;
#[doc = "DSP0N1INT1CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int1clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int1clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int1clr`]
module"]
#[doc(alias = "DSP0N1INT1CLR")]
pub type Dsp0n1int1clr = crate::Reg<dsp0n1int1clr::Dsp0n1int1clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n1int1clr;
#[doc = "DSP0N1INT1SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int1set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int1set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int1set`]
module"]
#[doc(alias = "DSP0N1INT1SET")]
pub type Dsp0n1int1set = crate::Reg<dsp0n1int1set::Dsp0n1int1setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n1int1set;
#[doc = "DSP0N1INT2EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int2en`]
module"]
#[doc(alias = "DSP0N1INT2EN")]
pub type Dsp0n1int2en = crate::Reg<dsp0n1int2en::Dsp0n1int2enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n1int2en;
#[doc = "DSP0N1INT2STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int2stat`]
module"]
#[doc(alias = "DSP0N1INT2STAT")]
pub type Dsp0n1int2stat = crate::Reg<dsp0n1int2stat::Dsp0n1int2statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n1int2stat;
#[doc = "DSP0N1INT2CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int2clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int2clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int2clr`]
module"]
#[doc(alias = "DSP0N1INT2CLR")]
pub type Dsp0n1int2clr = crate::Reg<dsp0n1int2clr::Dsp0n1int2clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n1int2clr;
#[doc = "DSP0N1INT2SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int2set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int2set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int2set`]
module"]
#[doc(alias = "DSP0N1INT2SET")]
pub type Dsp0n1int2set = crate::Reg<dsp0n1int2set::Dsp0n1int2setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n1int2set;
#[doc = "DSP0N1INT3EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int3en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int3en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int3en`]
module"]
#[doc(alias = "DSP0N1INT3EN")]
pub type Dsp0n1int3en = crate::Reg<dsp0n1int3en::Dsp0n1int3enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp0n1int3en;
#[doc = "DSP0N1INT3STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int3stat`]
module"]
#[doc(alias = "DSP0N1INT3STAT")]
pub type Dsp0n1int3stat = crate::Reg<dsp0n1int3stat::Dsp0n1int3statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp0n1int3stat;
#[doc = "DSP0N1INT3CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int3clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int3clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int3clr`]
module"]
#[doc(alias = "DSP0N1INT3CLR")]
pub type Dsp0n1int3clr = crate::Reg<dsp0n1int3clr::Dsp0n1int3clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp0n1int3clr;
#[doc = "DSP0N1INT3SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0n1int3set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0n1int3set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0n1int3set`]
module"]
#[doc(alias = "DSP0N1INT3SET")]
pub type Dsp0n1int3set = crate::Reg<dsp0n1int3set::Dsp0n1int3setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp0n1int3set;
#[doc = "DSP1N0INT0EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int0en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int0en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int0en`]
module"]
#[doc(alias = "DSP1N0INT0EN")]
pub type Dsp1n0int0en = crate::Reg<dsp1n0int0en::Dsp1n0int0enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n0int0en;
#[doc = "DSP1N0INT0STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int0stat`]
module"]
#[doc(alias = "DSP1N0INT0STAT")]
pub type Dsp1n0int0stat = crate::Reg<dsp1n0int0stat::Dsp1n0int0statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n0int0stat;
#[doc = "DSP1N0INT0CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int0clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int0clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int0clr`]
module"]
#[doc(alias = "DSP1N0INT0CLR")]
pub type Dsp1n0int0clr = crate::Reg<dsp1n0int0clr::Dsp1n0int0clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n0int0clr;
#[doc = "DSP1N0INT0SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int0set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int0set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int0set`]
module"]
#[doc(alias = "DSP1N0INT0SET")]
pub type Dsp1n0int0set = crate::Reg<dsp1n0int0set::Dsp1n0int0setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n0int0set;
#[doc = "DSP1N0INT1EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int1en`]
module"]
#[doc(alias = "DSP1N0INT1EN")]
pub type Dsp1n0int1en = crate::Reg<dsp1n0int1en::Dsp1n0int1enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n0int1en;
#[doc = "DSP1N0INT1STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int1stat`]
module"]
#[doc(alias = "DSP1N0INT1STAT")]
pub type Dsp1n0int1stat = crate::Reg<dsp1n0int1stat::Dsp1n0int1statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n0int1stat;
#[doc = "DSP1N0INT1CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int1clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int1clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int1clr`]
module"]
#[doc(alias = "DSP1N0INT1CLR")]
pub type Dsp1n0int1clr = crate::Reg<dsp1n0int1clr::Dsp1n0int1clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n0int1clr;
#[doc = "DSP1N0INT1SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int1set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int1set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int1set`]
module"]
#[doc(alias = "DSP1N0INT1SET")]
pub type Dsp1n0int1set = crate::Reg<dsp1n0int1set::Dsp1n0int1setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n0int1set;
#[doc = "DSP1N0INT2EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int2en`]
module"]
#[doc(alias = "DSP1N0INT2EN")]
pub type Dsp1n0int2en = crate::Reg<dsp1n0int2en::Dsp1n0int2enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n0int2en;
#[doc = "DSP1N0INT2STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int2stat`]
module"]
#[doc(alias = "DSP1N0INT2STAT")]
pub type Dsp1n0int2stat = crate::Reg<dsp1n0int2stat::Dsp1n0int2statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n0int2stat;
#[doc = "DSP1N0INT2CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int2clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int2clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int2clr`]
module"]
#[doc(alias = "DSP1N0INT2CLR")]
pub type Dsp1n0int2clr = crate::Reg<dsp1n0int2clr::Dsp1n0int2clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n0int2clr;
#[doc = "DSP1N0INT2SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int2set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int2set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int2set`]
module"]
#[doc(alias = "DSP1N0INT2SET")]
pub type Dsp1n0int2set = crate::Reg<dsp1n0int2set::Dsp1n0int2setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n0int2set;
#[doc = "DSP1N0INT3EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int3en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int3en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int3en`]
module"]
#[doc(alias = "DSP1N0INT3EN")]
pub type Dsp1n0int3en = crate::Reg<dsp1n0int3en::Dsp1n0int3enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n0int3en;
#[doc = "DSP1N0INT3STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int3stat`]
module"]
#[doc(alias = "DSP1N0INT3STAT")]
pub type Dsp1n0int3stat = crate::Reg<dsp1n0int3stat::Dsp1n0int3statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n0int3stat;
#[doc = "DSP1N0INT3CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int3clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int3clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int3clr`]
module"]
#[doc(alias = "DSP1N0INT3CLR")]
pub type Dsp1n0int3clr = crate::Reg<dsp1n0int3clr::Dsp1n0int3clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n0int3clr;
#[doc = "DSP1N0INT3SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n0int3set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n0int3set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n0int3set`]
module"]
#[doc(alias = "DSP1N0INT3SET")]
pub type Dsp1n0int3set = crate::Reg<dsp1n0int3set::Dsp1n0int3setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n0int3set;
#[doc = "DSP1N1INT0EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int0en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int0en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int0en`]
module"]
#[doc(alias = "DSP1N1INT0EN")]
pub type Dsp1n1int0en = crate::Reg<dsp1n1int0en::Dsp1n1int0enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n1int0en;
#[doc = "DSP1N1INT0STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int0stat`]
module"]
#[doc(alias = "DSP1N1INT0STAT")]
pub type Dsp1n1int0stat = crate::Reg<dsp1n1int0stat::Dsp1n1int0statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n1int0stat;
#[doc = "DSP1N1INT0CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int0clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int0clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int0clr`]
module"]
#[doc(alias = "DSP1N1INT0CLR")]
pub type Dsp1n1int0clr = crate::Reg<dsp1n1int0clr::Dsp1n1int0clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n1int0clr;
#[doc = "DSP1N1INT0SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int0set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int0set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int0set`]
module"]
#[doc(alias = "DSP1N1INT0SET")]
pub type Dsp1n1int0set = crate::Reg<dsp1n1int0set::Dsp1n1int0setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n1int0set;
#[doc = "DSP1N1INT1EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int1en`]
module"]
#[doc(alias = "DSP1N1INT1EN")]
pub type Dsp1n1int1en = crate::Reg<dsp1n1int1en::Dsp1n1int1enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n1int1en;
#[doc = "DSP1N1INT1STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int1stat`]
module"]
#[doc(alias = "DSP1N1INT1STAT")]
pub type Dsp1n1int1stat = crate::Reg<dsp1n1int1stat::Dsp1n1int1statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n1int1stat;
#[doc = "DSP1N1INT1CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int1clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int1clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int1clr`]
module"]
#[doc(alias = "DSP1N1INT1CLR")]
pub type Dsp1n1int1clr = crate::Reg<dsp1n1int1clr::Dsp1n1int1clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n1int1clr;
#[doc = "DSP1N1INT1SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int1set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int1set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int1set`]
module"]
#[doc(alias = "DSP1N1INT1SET")]
pub type Dsp1n1int1set = crate::Reg<dsp1n1int1set::Dsp1n1int1setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n1int1set;
#[doc = "DSP1N1INT2EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int2en`]
module"]
#[doc(alias = "DSP1N1INT2EN")]
pub type Dsp1n1int2en = crate::Reg<dsp1n1int2en::Dsp1n1int2enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n1int2en;
#[doc = "DSP1N1INT2STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int2stat`]
module"]
#[doc(alias = "DSP1N1INT2STAT")]
pub type Dsp1n1int2stat = crate::Reg<dsp1n1int2stat::Dsp1n1int2statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n1int2stat;
#[doc = "DSP1N1INT2CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int2clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int2clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int2clr`]
module"]
#[doc(alias = "DSP1N1INT2CLR")]
pub type Dsp1n1int2clr = crate::Reg<dsp1n1int2clr::Dsp1n1int2clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n1int2clr;
#[doc = "DSP1N1INT2SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int2set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int2set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int2set`]
module"]
#[doc(alias = "DSP1N1INT2SET")]
pub type Dsp1n1int2set = crate::Reg<dsp1n1int2set::Dsp1n1int2setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n1int2set;
#[doc = "DSP1N1INT3EN (rw) register accessor: Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int3en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int3en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int3en`]
module"]
#[doc(alias = "DSP1N1INT3EN")]
pub type Dsp1n1int3en = crate::Reg<dsp1n1int3en::Dsp1n1int3enSpec>;
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt."]
pub mod dsp1n1int3en;
#[doc = "DSP1N1INT3STAT (rw) register accessor: Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int3stat`]
module"]
#[doc(alias = "DSP1N1INT3STAT")]
pub type Dsp1n1int3stat = crate::Reg<dsp1n1int3stat::Dsp1n1int3statSpec>;
#[doc = "Read bits from this register to discover the cause of a recent interrupt."]
pub mod dsp1n1int3stat;
#[doc = "DSP1N1INT3CLR (rw) register accessor: Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int3clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int3clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int3clr`]
module"]
#[doc(alias = "DSP1N1INT3CLR")]
pub type Dsp1n1int3clr = crate::Reg<dsp1n1int3clr::Dsp1n1int3clrSpec>;
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit."]
pub mod dsp1n1int3clr;
#[doc = "DSP1N1INT3SET (rw) register accessor: Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1n1int3set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1n1int3set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1n1int3set`]
module"]
#[doc(alias = "DSP1N1INT3SET")]
pub type Dsp1n1int3set = crate::Reg<dsp1n1int3set::Dsp1n1int3setSpec>;
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes)."]
pub mod dsp1n1int3set;
