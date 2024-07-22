#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    memorymap0: Memorymap0,
    memorymap1: Memorymap1,
    memorymap2: Memorymap2,
    memorymap3: Memorymap3,
    memorymap4: Memorymap4,
    memorymap5: Memorymap5,
    memorymap6: Memorymap6,
    memorymap7: Memorymap7,
    memorymap8: Memorymap8,
    memorymap9: Memorymap9,
    memorymap10: Memorymap10,
    memorymap11: Memorymap11,
    memorymap12: Memorymap12,
    memorymap13: Memorymap13,
    memorymap14: Memorymap14,
    memorymap15: Memorymap15,
    memorymap16: Memorymap16,
    memorymap17: Memorymap17,
    memorymap18: Memorymap18,
    memorymap19: Memorymap19,
    memorymap20: Memorymap20,
    memorymap21: Memorymap21,
    memorymap22: Memorymap22,
    memorymap23: Memorymap23,
    memorymap24: Memorymap24,
    memorymap25: Memorymap25,
    memorymap26: Memorymap26,
    memorymap27: Memorymap27,
    memorymap28: Memorymap28,
    memorymap29: Memorymap29,
    memorymap30: Memorymap30,
    memorymap31: Memorymap31,
    opcode: Opcode,
    nnpt0t1addr: Nnpt0t1addr,
    pkastatus: Pkastatus,
    pkaswreset: Pkaswreset,
    pkal0: Pkal0,
    pkal1: Pkal1,
    pkal2: Pkal2,
    pkal3: Pkal3,
    pkal4: Pkal4,
    pkal5: Pkal5,
    pkal6: Pkal6,
    pkal7: Pkal7,
    pkapiperdy: Pkapiperdy,
    pkadone: Pkadone,
    pkamonselect: Pkamonselect,
    _reserved47: [u8; 0x08],
    pkaversion: Pkaversion,
    _reserved48: [u8; 0x08],
    pkamonread: Pkamonread,
    pkasramaddr: Pkasramaddr,
    pkasramwdata: Pkasramwdata,
    pkasramrdata: Pkasramrdata,
    pkasramwrclr: Pkasramwrclr,
    pkasramraddr: Pkasramraddr,
    _reserved54: [u8; 0x08],
    pkawordaccess: Pkawordaccess,
    _reserved55: [u8; 0x04],
    pkabuffaddr: Pkabuffaddr,
    _reserved56: [u8; 0x04],
    rngimr: Rngimr,
    rngisr: Rngisr,
    rngicr: Rngicr,
    trngconfig: Trngconfig,
    trngvalid: Trngvalid,
    ehrdata0: Ehrdata0,
    ehrdata1: Ehrdata1,
    ehrdata2: Ehrdata2,
    ehrdata3: Ehrdata3,
    ehrdata4: Ehrdata4,
    ehrdata5: Ehrdata5,
    rndsourceenable: Rndsourceenable,
    samplecnt1: Samplecnt1,
    autocorrstatistic: Autocorrstatistic,
    trngdebugcontrol: Trngdebugcontrol,
    _reserved71: [u8; 0x04],
    rngswreset: Rngswreset,
    _reserved72: [u8; 0x70],
    rngdebugeninput: Rngdebugeninput,
    rngbusy: Rngbusy,
    rstbitscounter: Rstbitscounter,
    rngversion: Rngversion,
    rngclkenable: Rngclkenable,
    rngdmaenable: Rngdmaenable,
    rngdmasrcmask: Rngdmasrcmask,
    rngdmasramaddr: Rngdmasramaddr,
    _reserved80: [u8; 0x04],
    rngwatchdogval: Rngwatchdogval,
    rngdmastatus: Rngdmastatus,
    _reserved82: [u8; 0x01a0],
    chachacontrolreg: Chachacontrolreg,
    chachaversion: Chachaversion,
    chachakey0: Chachakey0,
    chachakey1: Chachakey1,
    chachakey2: Chachakey2,
    chachakey3: Chachakey3,
    chachakey4: Chachakey4,
    chachakey5: Chachakey5,
    chachakey6: Chachakey6,
    chachakey7: Chachakey7,
    chachaiv0: Chachaiv0,
    chachaiv1: Chachaiv1,
    chachabusy: Chachabusy,
    chachahwflags: Chachahwflags,
    chachablockcntlsb: Chachablockcntlsb,
    chachablockcntmsb: Chachablockcntmsb,
    chachaswreset: Chachaswreset,
    chachaforpolykey0: Chachaforpolykey0,
    chachaforpolykey1: Chachaforpolykey1,
    chachaforpolykey2: Chachaforpolykey2,
    chachaforpolykey3: Chachaforpolykey3,
    chachaforpolykey4: Chachaforpolykey4,
    chachaforpolykey5: Chachaforpolykey5,
    chachaforpolykey6: Chachaforpolykey6,
    chachaforpolykey7: Chachaforpolykey7,
    chachabytewordordercntlreg: Chachabytewordordercntlreg,
    chachadebugreg: Chachadebugreg,
    _reserved109: [u8; 0x14],
    aeskey00: Aeskey00,
    aeskey01: Aeskey01,
    aeskey02: Aeskey02,
    aeskey03: Aeskey03,
    aeskey04: Aeskey04,
    aeskey05: Aeskey05,
    aeskey06: Aeskey06,
    aeskey07: Aeskey07,
    aeskey10: Aeskey10,
    aeskey11: Aeskey11,
    aeskey12: Aeskey12,
    aeskey13: Aeskey13,
    aeskey14: Aeskey14,
    aeskey15: Aeskey15,
    aeskey16: Aeskey16,
    aeskey17: Aeskey17,
    aesiv00: Aesiv00,
    aesiv01: Aesiv01,
    aesiv02: Aesiv02,
    aesiv03: Aesiv03,
    aesiv10: Aesiv10,
    aesiv11: Aesiv11,
    aesiv12: Aesiv12,
    aesiv13: Aesiv13,
    aesctr00: Aesctr00,
    aesctr01: Aesctr01,
    aesctr02: Aesctr02,
    aesctr03: Aesctr03,
    aesbusy: Aesbusy,
    _reserved138: [u8; 0x04],
    aessk: Aessk,
    aescmacinit: Aescmacinit,
    _reserved140: [u8; 0x34],
    aessk1: Aessk1,
    _reserved141: [u8; 0x04],
    aesremainingbytes: Aesremainingbytes,
    aescontrol: Aescontrol,
    _reserved143: [u8; 0x04],
    aeshwflags: Aeshwflags,
    _reserved144: [u8; 0x0c],
    aesctrnoincrement: Aesctrnoincrement,
    _reserved145: [u8; 0x14],
    aesdfaison: Aesdfaison,
    _reserved146: [u8; 0x04],
    aesdfaerrstatus: Aesdfaerrstatus,
    _reserved147: [u8; 0x28],
    aescmacsize0kick: Aescmacsize0kick,
    _reserved148: [u8; 0x0118],
    hashh0: Hashh0,
    hashh1: Hashh1,
    hashh2: Hashh2,
    hashh3: Hashh3,
    hashh4: Hashh4,
    hashh5: Hashh5,
    hashh6: Hashh6,
    hashh7: Hashh7,
    hashh8: Hashh8,
    _reserved157: [u8; 0x20],
    autohwpadding: Autohwpadding,
    hashxordin: Hashxordin,
    _reserved159: [u8; 0x08],
    loadinitstate: Loadinitstate,
    _reserved160: [u8; 0x0c],
    hashselaesmac: Hashselaesmac,
    _reserved161: [u8; 0x0108],
    hashversion: Hashversion,
    _reserved162: [u8; 0x0c],
    hashcontrol: Hashcontrol,
    hashpaden: Hashpaden,
    hashpadcfg: Hashpadcfg,
    hashcurlen0: Hashcurlen0,
    hashcurlen1: Hashcurlen1,
    _reserved167: [u8; 0x08],
    hashparam: Hashparam,
    _reserved168: [u8; 0x04],
    hashaesswreset: Hashaesswreset,
    hashendianess: Hashendianess,
    _reserved170: [u8; 0x24],
    aesclkenable: Aesclkenable,
    _reserved171: [u8; 0x04],
    hashclkenable: Hashclkenable,
    pkaclkenable: Pkaclkenable,
    dmaclkenable: Dmaclkenable,
    clkstatus: Clkstatus,
    _reserved175: [u8; 0x30],
    chachaclkenable: Chachaclkenable,
    _reserved176: [u8; 0xa4],
    cryptoctl: Cryptoctl,
    _reserved177: [u8; 0x0c],
    cryptobusy: Cryptobusy,
    _reserved178: [u8; 0x08],
    hashbusy: Hashbusy,
    _reserved179: [u8; 0x10],
    contextid: Contextid,
    _reserved180: [u8; 0x2c],
    ghashsubkey00: Ghashsubkey00,
    ghashsubkey01: Ghashsubkey01,
    ghashsubkey02: Ghashsubkey02,
    ghashsubkey03: Ghashsubkey03,
    ghashiv00: Ghashiv00,
    ghashiv01: Ghashiv01,
    ghashiv02: Ghashiv02,
    ghashiv03: Ghashiv03,
    ghashbusy: Ghashbusy,
    ghashinit: Ghashinit,
    _reserved190: [u8; 0x78],
    hostrgfirr: Hostrgfirr,
    hostrgfimr: Hostrgfimr,
    hostrgficr: Hostrgficr,
    hostrgfendian: Hostrgfendian,
    _reserved194: [u8; 0x14],
    hostrgfsignature: Hostrgfsignature,
    hostboot: Hostboot,
    _reserved196: [u8; 0x0c],
    hostcryptokeysel: Hostcryptokeysel,
    _reserved197: [u8; 0x3c],
    hostcoreclkgatingenable: Hostcoreclkgatingenable,
    hostccisidle: Hostccisidle,
    hostpowerdown: Hostpowerdown,
    hostremoveghashengine: Hostremoveghashengine,
    hostremovechachaengine: Hostremovechachaengine,
    _reserved202: [u8; 0x74],
    ahbmsingles: Ahbmsingles,
    ahbmhprot: Ahbmhprot,
    ahbmhmastlock: Ahbmhmastlock,
    ahbmhnonsec: Ahbmhnonsec,
    _reserved206: [u8; 0xf0],
    dinbuffer: Dinbuffer,
    _reserved207: [u8; 0x1c],
    dinmemdmabusy: Dinmemdmabusy,
    _reserved208: [u8; 0x04],
    srclliword0: Srclliword0,
    srclliword1: Srclliword1,
    sramsrcaddr: Sramsrcaddr,
    dinsrambyteslen: Dinsrambyteslen,
    dinsramdmabusy: Dinsramdmabusy,
    dinsramendianness: Dinsramendianness,
    _reserved214: [u8; 0x08],
    dincpudatasize: Dincpudatasize,
    _reserved215: [u8; 0x04],
    fifoinempty: Fifoinempty,
    _reserved216: [u8; 0x04],
    dinfiforstpntr: Dinfiforstpntr,
    _reserved217: [u8; 0xa4],
    doutbuffer: Doutbuffer,
    _reserved218: [u8; 0x1c],
    doutmemdmabusy: Doutmemdmabusy,
    _reserved219: [u8; 0x04],
    dstlliword0: Dstlliword0,
    dstlliword1: Dstlliword1,
    sramdestaddr: Sramdestaddr,
    doutsrambyteslen: Doutsrambyteslen,
    doutsramdmabusy: Doutsramdmabusy,
    doutsramendianness: Doutsramendianness,
    _reserved225: [u8; 0x04],
    readalignlast: Readalignlast,
    _reserved226: [u8; 0x08],
    doutfifoempty: Doutfifoempty,
    _reserved227: [u8; 0x01ac],
    sramdata: Sramdata,
    sramaddr: Sramaddr,
    sramdataready: Sramdataready,
    _reserved230: [u8; 0xc4],
    peripheralid4: Peripheralid4,
    _reserved231: [u8; 0x0c],
    peripheralid0: Peripheralid0,
    peripheralid1: Peripheralid1,
    peripheralid2: Peripheralid2,
    peripheralid3: Peripheralid3,
    componentid0: Componentid0,
    componentid1: Componentid1,
    componentid2: Componentid2,
    componentid3: Componentid3,
    _reserved239: [u8; 0x0e00],
    hostdcuen0: Hostdcuen0,
    hostdcuen1: Hostdcuen1,
    hostdcuen2: Hostdcuen2,
    hostdcuen3: Hostdcuen3,
    hostdculock0: Hostdculock0,
    hostdculock1: Hostdculock1,
    hostdculock2: Hostdculock2,
    hostdculock3: Hostdculock3,
    aoicvdcurestrictionmask0: Aoicvdcurestrictionmask0,
    aoicvdcurestrictionmask1: Aoicvdcurestrictionmask1,
    aoicvdcurestrictionmask2: Aoicvdcurestrictionmask2,
    aoicvdcurestrictionmask3: Aoicvdcurestrictionmask3,
    aoccsecdebugreset: Aoccsecdebugreset,
    hostaolockbits: Hostaolockbits,
    aoapbfiltering: Aoapbfiltering,
    aoccgppc: Aoccgppc,
    hostrgfccswrst: Hostrgfccswrst,
    _reserved256: [u8; 0xc0],
    aibfuseprogcompleted: Aibfuseprogcompleted,
    nvmdebugstatus: Nvmdebugstatus,
    lcsisvalid: Lcsisvalid,
    nvmisidle: Nvmisidle,
    lcsreg: Lcsreg,
    hostshadowkdrreg: Hostshadowkdrreg,
    hostshadowkcpreg: Hostshadowkcpreg,
    hostshadowkcereg: Hostshadowkcereg,
    hostshadowkpicvreg: Hostshadowkpicvreg,
    hostshadowkceicvreg: Hostshadowkceicvreg,
    otpaddrwidthdef: Otpaddrwidthdef,
}
impl RegisterBlock {
    #[doc = "0x00 - This register maps the virtual register R0 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap0(&self) -> &Memorymap0 {
        &self.memorymap0
    }
    #[doc = "0x04 - This register maps the virtual register R1 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap1(&self) -> &Memorymap1 {
        &self.memorymap1
    }
    #[doc = "0x08 - This register maps the virtual register R2 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap2(&self) -> &Memorymap2 {
        &self.memorymap2
    }
    #[doc = "0x0c - This register maps the virtual register R3 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap3(&self) -> &Memorymap3 {
        &self.memorymap3
    }
    #[doc = "0x10 - This register maps the virtual register R4 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap4(&self) -> &Memorymap4 {
        &self.memorymap4
    }
    #[doc = "0x14 - This register maps the virtual register R5 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap5(&self) -> &Memorymap5 {
        &self.memorymap5
    }
    #[doc = "0x18 - This register maps the virtual register R6 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap6(&self) -> &Memorymap6 {
        &self.memorymap6
    }
    #[doc = "0x1c - This register maps the virtual register R7 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap7(&self) -> &Memorymap7 {
        &self.memorymap7
    }
    #[doc = "0x20 - This register maps the virtual register R8 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap8(&self) -> &Memorymap8 {
        &self.memorymap8
    }
    #[doc = "0x24 - This register maps the virtual register R9 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap9(&self) -> &Memorymap9 {
        &self.memorymap9
    }
    #[doc = "0x28 - This register maps the virtual register R10 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap10(&self) -> &Memorymap10 {
        &self.memorymap10
    }
    #[doc = "0x2c - This register maps the virtual register R11 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap11(&self) -> &Memorymap11 {
        &self.memorymap11
    }
    #[doc = "0x30 - This register maps the virtual register R12 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap12(&self) -> &Memorymap12 {
        &self.memorymap12
    }
    #[doc = "0x34 - This register maps the virtual register R13 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap13(&self) -> &Memorymap13 {
        &self.memorymap13
    }
    #[doc = "0x38 - This register maps the virtual register R14 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap14(&self) -> &Memorymap14 {
        &self.memorymap14
    }
    #[doc = "0x3c - This register maps the virtual register R15 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap15(&self) -> &Memorymap15 {
        &self.memorymap15
    }
    #[doc = "0x40 - This register maps the virtual register R16 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap16(&self) -> &Memorymap16 {
        &self.memorymap16
    }
    #[doc = "0x44 - This register maps the virtual register R17 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap17(&self) -> &Memorymap17 {
        &self.memorymap17
    }
    #[doc = "0x48 - This register maps the virtual register R18 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap18(&self) -> &Memorymap18 {
        &self.memorymap18
    }
    #[doc = "0x4c - This register maps the virtual register R19 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap19(&self) -> &Memorymap19 {
        &self.memorymap19
    }
    #[doc = "0x50 - This register maps the virtual register R20 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap20(&self) -> &Memorymap20 {
        &self.memorymap20
    }
    #[doc = "0x54 - This register maps the virtual register R21 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap21(&self) -> &Memorymap21 {
        &self.memorymap21
    }
    #[doc = "0x58 - This register maps the virtual register R22 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap22(&self) -> &Memorymap22 {
        &self.memorymap22
    }
    #[doc = "0x5c - This register maps the virtual register R23 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap23(&self) -> &Memorymap23 {
        &self.memorymap23
    }
    #[doc = "0x60 - This register maps the virtual register R24 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap24(&self) -> &Memorymap24 {
        &self.memorymap24
    }
    #[doc = "0x64 - This register maps the virtual register R25 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap25(&self) -> &Memorymap25 {
        &self.memorymap25
    }
    #[doc = "0x68 - This register maps the virtual register R26 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap26(&self) -> &Memorymap26 {
        &self.memorymap26
    }
    #[doc = "0x6c - This register maps the virtual register R27 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap27(&self) -> &Memorymap27 {
        &self.memorymap27
    }
    #[doc = "0x70 - This register maps the virtual register R28 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap28(&self) -> &Memorymap28 {
        &self.memorymap28
    }
    #[doc = "0x74 - This register maps the virtual register R29 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap29(&self) -> &Memorymap29 {
        &self.memorymap29
    }
    #[doc = "0x78 - This register maps the virtual register R30 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap30(&self) -> &Memorymap30 {
        &self.memorymap30
    }
    #[doc = "0x7c - This register maps the virtual register R31 to a physical address in memory."]
    #[inline(always)]
    pub const fn memorymap31(&self) -> &Memorymap31 {
        &self.memorymap31
    }
    #[doc = "0x80 - This register holds the PKAs OPCODE."]
    #[inline(always)]
    pub const fn opcode(&self) -> &Opcode {
        &self.opcode
    }
    #[doc = "0x84 - This register maps N_NP_T0_T1 to a virtual address."]
    #[inline(always)]
    pub const fn nnpt0t1addr(&self) -> &Nnpt0t1addr {
        &self.nnpt0t1addr
    }
    #[doc = "0x88 - This register holds the PKA pipe status."]
    #[inline(always)]
    pub const fn pkastatus(&self) -> &Pkastatus {
        &self.pkastatus
    }
    #[doc = "0x8c - Writing to this register triggers a software reset of the PKA."]
    #[inline(always)]
    pub const fn pkaswreset(&self) -> &Pkaswreset {
        &self.pkaswreset
    }
    #[doc = "0x90 - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal0(&self) -> &Pkal0 {
        &self.pkal0
    }
    #[doc = "0x94 - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal1(&self) -> &Pkal1 {
        &self.pkal1
    }
    #[doc = "0x98 - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal2(&self) -> &Pkal2 {
        &self.pkal2
    }
    #[doc = "0x9c - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal3(&self) -> &Pkal3 {
        &self.pkal3
    }
    #[doc = "0xa0 - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal4(&self) -> &Pkal4 {
        &self.pkal4
    }
    #[doc = "0xa4 - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal5(&self) -> &Pkal5 {
        &self.pkal5
    }
    #[doc = "0xa8 - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal6(&self) -> &Pkal6 {
        &self.pkal6
    }
    #[doc = "0xac - This register holds one of the optional size of the operation."]
    #[inline(always)]
    pub const fn pkal7(&self) -> &Pkal7 {
        &self.pkal7
    }
    #[doc = "0xb0 - This register indicates whether the PKA pipe is ready to receive a new OPCODE."]
    #[inline(always)]
    pub const fn pkapiperdy(&self) -> &Pkapiperdy {
        &self.pkapiperdy
    }
    #[doc = "0xb4 - This register indicates whether PKA operation is completed."]
    #[inline(always)]
    pub const fn pkadone(&self) -> &Pkadone {
        &self.pkadone
    }
    #[doc = "0xb8 - This register defines which PKA FSM monitor is being output."]
    #[inline(always)]
    pub const fn pkamonselect(&self) -> &Pkamonselect {
        &self.pkamonselect
    }
    #[doc = "0xc4 - This register holds the pka version"]
    #[inline(always)]
    pub const fn pkaversion(&self) -> &Pkaversion {
        &self.pkaversion
    }
    #[doc = "0xd0 - The PKA monitor bus register."]
    #[inline(always)]
    pub const fn pkamonread(&self) -> &Pkamonread {
        &self.pkamonread
    }
    #[doc = "0xd4 - first address given to PKA SRAM for write transactions."]
    #[inline(always)]
    pub const fn pkasramaddr(&self) -> &Pkasramaddr {
        &self.pkasramaddr
    }
    #[doc = "0xd8 - Write data to PKA SRAM."]
    #[inline(always)]
    pub const fn pkasramwdata(&self) -> &Pkasramwdata {
        &self.pkasramwdata
    }
    #[doc = "0xdc - Read data from PKA SRAM."]
    #[inline(always)]
    pub const fn pkasramrdata(&self) -> &Pkasramrdata {
        &self.pkasramrdata
    }
    #[doc = "0xe0 - Write buffer clean."]
    #[inline(always)]
    pub const fn pkasramwrclr(&self) -> &Pkasramwrclr {
        &self.pkasramwrclr
    }
    #[doc = "0xe4 - first address given to PKA SRAM for read transactions."]
    #[inline(always)]
    pub const fn pkasramraddr(&self) -> &Pkasramraddr {
        &self.pkasramraddr
    }
    #[doc = "0xf0 - This register holds the data written to PKA memory using the wop opcode."]
    #[inline(always)]
    pub const fn pkawordaccess(&self) -> &Pkawordaccess {
        &self.pkawordaccess
    }
    #[doc = "0xf8 - This register maps the virtual buffer registers to a physical address in memory."]
    #[inline(always)]
    pub const fn pkabuffaddr(&self) -> &Pkabuffaddr {
        &self.pkabuffaddr
    }
    #[doc = "0x100 - Interrupt masking register. Consists of {prng_imr trng_imr} bit\\[31-16\\]
- PRNG_IMR bit\\[15-0\\]
- TRNG_IMR(Ws - PRNG bit exists only if PRNG_EXISTS flag)"]
    #[inline(always)]
    pub const fn rngimr(&self) -> &Rngimr {
        &self.rngimr
    }
    #[doc = "0x104 - Status register. If corresponding RNG_IMR bit is unmasked, an interrupt is generated.Consists of trng_isr and prng_isr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG"]
    #[inline(always)]
    pub const fn rngisr(&self) -> &Rngisr {
        &self.rngisr
    }
    #[doc = "0x108 - Interrupt_status bit clear Register. Consists of trng_icr and prng_icr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG"]
    #[inline(always)]
    pub const fn rngicr(&self) -> &Rngicr {
        &self.rngicr
    }
    #[doc = "0x10c - This register handles TRNG configuration"]
    #[inline(always)]
    pub const fn trngconfig(&self) -> &Trngconfig {
        &self.trngconfig
    }
    #[doc = "0x110 - This register indicates that the TRNG data is valid."]
    #[inline(always)]
    pub const fn trngvalid(&self) -> &Trngvalid {
        &self.trngvalid
    }
    #[doc = "0x114 - This register contains the data collected in the TRNG\\[31_0\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub const fn ehrdata0(&self) -> &Ehrdata0 {
        &self.ehrdata0
    }
    #[doc = "0x118 - This register contains the data collected in the TRNG\\[63_32\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub const fn ehrdata1(&self) -> &Ehrdata1 {
        &self.ehrdata1
    }
    #[doc = "0x11c - This register contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub const fn ehrdata2(&self) -> &Ehrdata2 {
        &self.ehrdata2
    }
    #[doc = "0x120 - This register contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub const fn ehrdata3(&self) -> &Ehrdata3 {
        &self.ehrdata3
    }
    #[doc = "0x124 - This register contains the data collected in the TRNG\\[159_128\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub const fn ehrdata4(&self) -> &Ehrdata4 {
        &self.ehrdata4
    }
    #[doc = "0x128 - This register contains the data collected in the TRNG\\[191_160\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
    #[inline(always)]
    pub const fn ehrdata5(&self) -> &Ehrdata5 {
        &self.ehrdata5
    }
    #[doc = "0x12c - This register holds the enable signal for the random source."]
    #[inline(always)]
    pub const fn rndsourceenable(&self) -> &Rndsourceenable {
        &self.rndsourceenable
    }
    #[doc = "0x130 - Counts clocks between sampling of random bit."]
    #[inline(always)]
    pub const fn samplecnt1(&self) -> &Samplecnt1 {
        &self.samplecnt1
    }
    #[doc = "0x134 - Statistics about autocorrelation test activations."]
    #[inline(always)]
    pub const fn autocorrstatistic(&self) -> &Autocorrstatistic {
        &self.autocorrstatistic
    }
    #[doc = "0x138 - This register is used to debug the TRNG"]
    #[inline(always)]
    pub const fn trngdebugcontrol(&self) -> &Trngdebugcontrol {
        &self.trngdebugcontrol
    }
    #[doc = "0x140 - Generate SW reset solely to RNG block."]
    #[inline(always)]
    pub const fn rngswreset(&self) -> &Rngswreset {
        &self.rngswreset
    }
    #[doc = "0x1b4 - Defines the RNG in debug mode"]
    #[inline(always)]
    pub const fn rngdebugeninput(&self) -> &Rngdebugeninput {
        &self.rngdebugeninput
    }
    #[doc = "0x1b8 - RNG busy indication"]
    #[inline(always)]
    pub const fn rngbusy(&self) -> &Rngbusy {
        &self.rngbusy
    }
    #[doc = "0x1bc - Resets the counter of collected bits in the TRNG"]
    #[inline(always)]
    pub const fn rstbitscounter(&self) -> &Rstbitscounter {
        &self.rstbitscounter
    }
    #[doc = "0x1c0 - This register holds the RNG version"]
    #[inline(always)]
    pub const fn rngversion(&self) -> &Rngversion {
        &self.rngversion
    }
    #[doc = "0x1c4 - Writing to this register enables_disables the RNG clock."]
    #[inline(always)]
    pub const fn rngclkenable(&self) -> &Rngclkenable {
        &self.rngclkenable
    }
    #[doc = "0x1c8 - Writing to this register enables_disables the RNG DMA."]
    #[inline(always)]
    pub const fn rngdmaenable(&self) -> &Rngdmaenable {
        &self.rngdmaenable
    }
    #[doc = "0x1cc - This register defines which ring-oscillator length should be used when using the RNG DMA."]
    #[inline(always)]
    pub const fn rngdmasrcmask(&self) -> &Rngdmasrcmask {
        &self.rngdmasrcmask
    }
    #[doc = "0x1d0 - This register defines the start address of the DMA for the TRNG data."]
    #[inline(always)]
    pub const fn rngdmasramaddr(&self) -> &Rngdmasramaddr {
        &self.rngdmasramaddr
    }
    #[doc = "0x1d8 - This register defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 7:0 RNG_SAMPLES_NUM rw 0x0 Defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 31:8 RESERVED rw 0x0 ReservedThis register defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt."]
    #[inline(always)]
    pub const fn rngwatchdogval(&self) -> &Rngwatchdogval {
        &self.rngwatchdogval
    }
    #[doc = "0x1dc - This register holds the RNG DMA status."]
    #[inline(always)]
    pub const fn rngdmastatus(&self) -> &Rngdmastatus {
        &self.rngdmastatus
    }
    #[doc = "0x380 - CHACHA general configuration."]
    #[inline(always)]
    pub const fn chachacontrolreg(&self) -> &Chachacontrolreg {
        &self.chachacontrolreg
    }
    #[doc = "0x384 - CHACHA Version"]
    #[inline(always)]
    pub const fn chachaversion(&self) -> &Chachaversion {
        &self.chachaversion
    }
    #[doc = "0x388 - bits 255:224 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey0(&self) -> &Chachakey0 {
        &self.chachakey0
    }
    #[doc = "0x38c - bits 223:192 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey1(&self) -> &Chachakey1 {
        &self.chachakey1
    }
    #[doc = "0x390 - bits 191:160 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey2(&self) -> &Chachakey2 {
        &self.chachakey2
    }
    #[doc = "0x394 - bits159:128 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey3(&self) -> &Chachakey3 {
        &self.chachakey3
    }
    #[doc = "0x398 - bits 127:96 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey4(&self) -> &Chachakey4 {
        &self.chachakey4
    }
    #[doc = "0x39c - bits 95:64 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey5(&self) -> &Chachakey5 {
        &self.chachakey5
    }
    #[doc = "0x3a0 - bits 63:32 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey6(&self) -> &Chachakey6 {
        &self.chachakey6
    }
    #[doc = "0x3a4 - bits 31:0 of CHACHA Key"]
    #[inline(always)]
    pub const fn chachakey7(&self) -> &Chachakey7 {
        &self.chachakey7
    }
    #[doc = "0x3a8 - bits 31:0 of CHACHA_IV0 register"]
    #[inline(always)]
    pub const fn chachaiv0(&self) -> &Chachaiv0 {
        &self.chachaiv0
    }
    #[doc = "0x3ac - bits 31:0 of CHACHA_IV1 register"]
    #[inline(always)]
    pub const fn chachaiv1(&self) -> &Chachaiv1 {
        &self.chachaiv1
    }
    #[doc = "0x3b0 - This register is set when the CHACHA_SALSA core is active"]
    #[inline(always)]
    pub const fn chachabusy(&self) -> &Chachabusy {
        &self.chachabusy
    }
    #[doc = "0x3b4 - This register holds the pre-synthesis HW flag configuration of the CHACHA_SALSA engine"]
    #[inline(always)]
    pub const fn chachahwflags(&self) -> &Chachahwflags {
        &self.chachahwflags
    }
    #[doc = "0x3b8 - The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message."]
    #[inline(always)]
    pub const fn chachablockcntlsb(&self) -> &Chachablockcntlsb {
        &self.chachablockcntlsb
    }
    #[doc = "0x3bc - The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message."]
    #[inline(always)]
    pub const fn chachablockcntmsb(&self) -> &Chachablockcntmsb {
        &self.chachablockcntmsb
    }
    #[doc = "0x3c0 - Resets CHACHA_SALSA engine."]
    #[inline(always)]
    pub const fn chachaswreset(&self) -> &Chachaswreset {
        &self.chachaswreset
    }
    #[doc = "0x3c4 - bits 255:224 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey0(&self) -> &Chachaforpolykey0 {
        &self.chachaforpolykey0
    }
    #[doc = "0x3c8 - bits 223:192 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey1(&self) -> &Chachaforpolykey1 {
        &self.chachaforpolykey1
    }
    #[doc = "0x3cc - bits191:160 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey2(&self) -> &Chachaforpolykey2 {
        &self.chachaforpolykey2
    }
    #[doc = "0x3d0 - bits159:128 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey3(&self) -> &Chachaforpolykey3 {
        &self.chachaforpolykey3
    }
    #[doc = "0x3d4 - bits 127:96 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey4(&self) -> &Chachaforpolykey4 {
        &self.chachaforpolykey4
    }
    #[doc = "0x3d8 - bits 95:64 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey5(&self) -> &Chachaforpolykey5 {
        &self.chachaforpolykey5
    }
    #[doc = "0x3dc - bits 63:32 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey6(&self) -> &Chachaforpolykey6 {
        &self.chachaforpolykey6
    }
    #[doc = "0x3e0 - bits 31:0 of CHACHA_FOR_POLY_KEY"]
    #[inline(always)]
    pub const fn chachaforpolykey7(&self) -> &Chachaforpolykey7 {
        &self.chachaforpolykey7
    }
    #[doc = "0x3e4 - CHACHA_SALSA DATA ORDER configuration."]
    #[inline(always)]
    pub const fn chachabytewordordercntlreg(&self) -> &Chachabytewordordercntlreg {
        &self.chachabytewordordercntlreg
    }
    #[doc = "0x3e8 - This register is used to debug the CHACHA engine"]
    #[inline(always)]
    pub const fn chachadebugreg(&self) -> &Chachadebugreg {
        &self.chachadebugreg
    }
    #[doc = "0x400 - bits 31:0 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey00(&self) -> &Aeskey00 {
        &self.aeskey00
    }
    #[doc = "0x404 - bits 63:32 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey01(&self) -> &Aeskey01 {
        &self.aeskey01
    }
    #[doc = "0x408 - bits 95:64 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey02(&self) -> &Aeskey02 {
        &self.aeskey02
    }
    #[doc = "0x40c - bits 127:96 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey03(&self) -> &Aeskey03 {
        &self.aeskey03
    }
    #[doc = "0x410 - bits 159:128 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey04(&self) -> &Aeskey04 {
        &self.aeskey04
    }
    #[doc = "0x414 - bits 191:160 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey05(&self) -> &Aeskey05 {
        &self.aeskey05
    }
    #[doc = "0x418 - bits 223:192 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey06(&self) -> &Aeskey06 {
        &self.aeskey06
    }
    #[doc = "0x41c - bits 255:224 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey07(&self) -> &Aeskey07 {
        &self.aeskey07
    }
    #[doc = "0x420 - bits 31:0 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey10(&self) -> &Aeskey10 {
        &self.aeskey10
    }
    #[doc = "0x424 - bits 63:32 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey11(&self) -> &Aeskey11 {
        &self.aeskey11
    }
    #[doc = "0x428 - bits 95:64 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey12(&self) -> &Aeskey12 {
        &self.aeskey12
    }
    #[doc = "0x42c - bits 127:96 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey13(&self) -> &Aeskey13 {
        &self.aeskey13
    }
    #[doc = "0x430 - bits 159:128 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey14(&self) -> &Aeskey14 {
        &self.aeskey14
    }
    #[doc = "0x434 - bits 191:160 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey15(&self) -> &Aeskey15 {
        &self.aeskey15
    }
    #[doc = "0x438 - bits 223:192 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey16(&self) -> &Aeskey16 {
        &self.aeskey16
    }
    #[doc = "0x43c - bits 255:224 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
    #[inline(always)]
    pub const fn aeskey17(&self) -> &Aeskey17 {
        &self.aeskey17
    }
    #[doc = "0x440 - bits 31:0 of AES_IV0 register. AES IV0 is used as the AES IV (Initialization Value) register in non-tunneling operations,and as the first tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW)."]
    #[inline(always)]
    pub const fn aesiv00(&self) -> &Aesiv00 {
        &self.aesiv00
    }
    #[doc = "0x444 - bits 63:32 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description"]
    #[inline(always)]
    pub const fn aesiv01(&self) -> &Aesiv01 {
        &self.aesiv01
    }
    #[doc = "0x448 - bits 95:64 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description"]
    #[inline(always)]
    pub const fn aesiv02(&self) -> &Aesiv02 {
        &self.aesiv02
    }
    #[doc = "0x44c - bits 127:96 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description"]
    #[inline(always)]
    pub const fn aesiv03(&self) -> &Aesiv03 {
        &self.aesiv03
    }
    #[doc = "0x450 - bits 31:0 of AES_IV1 128b register.AES IV1 is used as the AES IV (Initialization Value) register as the second tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW."]
    #[inline(always)]
    pub const fn aesiv10(&self) -> &Aesiv10 {
        &self.aesiv10
    }
    #[doc = "0x454 - bits 63:32 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description"]
    #[inline(always)]
    pub const fn aesiv11(&self) -> &Aesiv11 {
        &self.aesiv11
    }
    #[doc = "0x458 - bits 95:64 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description"]
    #[inline(always)]
    pub const fn aesiv12(&self) -> &Aesiv12 {
        &self.aesiv12
    }
    #[doc = "0x45c - bits 127:96 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description"]
    #[inline(always)]
    pub const fn aesiv13(&self) -> &Aesiv13 {
        &self.aesiv13
    }
    #[doc = "0x460 - bits 31:0 of AES_CTR0 128b register.AES CTR0 is used as the AES CTR (counter) register in non-tunneling operations, and as the first tunnel stage CTR register in tunneling operations.The CTR register should be loaded according to the AES mode:in AES CTR_GCTR - the AES CTR register should be loaded with the counter value.in XTS-AES - the AES CTR register should be loaded with the i value (in order to calculate the T value from it, if HW T calculation is supported)."]
    #[inline(always)]
    pub const fn aesctr00(&self) -> &Aesctr00 {
        &self.aesctr00
    }
    #[doc = "0x464 - bits 63:32 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description."]
    #[inline(always)]
    pub const fn aesctr01(&self) -> &Aesctr01 {
        &self.aesctr01
    }
    #[doc = "0x468 - bits 95:64 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description."]
    #[inline(always)]
    pub const fn aesctr02(&self) -> &Aesctr02 {
        &self.aesctr02
    }
    #[doc = "0x46c - bits 127:96 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description."]
    #[inline(always)]
    pub const fn aesctr03(&self) -> &Aesctr03 {
        &self.aesctr03
    }
    #[doc = "0x470 - This register is set when the AES core is active"]
    #[inline(always)]
    pub const fn aesbusy(&self) -> &Aesbusy {
        &self.aesbusy
    }
    #[doc = "0x478 - writing to this address causes sampling of the HW key to the AES_KEY0 register"]
    #[inline(always)]
    pub const fn aessk(&self) -> &Aessk {
        &self.aessk
    }
    #[doc = "0x47c - Writing to this address triggers the AES engine generating of K1 and K2 for AES CMAC operations. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn aescmacinit(&self) -> &Aescmacinit {
        &self.aescmacinit
    }
    #[doc = "0x4b4 - writing to this address causes sampling of the HW key to the AES_KEY1 register"]
    #[inline(always)]
    pub const fn aessk1(&self) -> &Aessk1 {
        &self.aessk1
    }
    #[doc = "0x4bc - This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM."]
    #[inline(always)]
    pub const fn aesremainingbytes(&self) -> &Aesremainingbytes {
        &self.aesremainingbytes
    }
    #[doc = "0x4c0 - This register holds the configuration of the AES engine. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn aescontrol(&self) -> &Aescontrol {
        &self.aescontrol
    }
    #[doc = "0x4c8 - This register holds the pre-synthesis HW flag configuration of the AES engine"]
    #[inline(always)]
    pub const fn aeshwflags(&self) -> &Aeshwflags {
        &self.aeshwflags
    }
    #[doc = "0x4d8 - This register enables the AES CTR no increment mode (in which the counter mode is not incremented between 2 blocks)"]
    #[inline(always)]
    pub const fn aesctrnoincrement(&self) -> &Aesctrnoincrement {
        &self.aesctrnoincrement
    }
    #[doc = "0x4f0 - This register disable_enable the AES dfa. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn aesdfaison(&self) -> &Aesdfaison {
        &self.aesdfaison
    }
    #[doc = "0x4f8 - dfa error status register."]
    #[inline(always)]
    pub const fn aesdfaerrstatus(&self) -> &Aesdfaerrstatus {
        &self.aesdfaerrstatus
    }
    #[doc = "0x524 - writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register."]
    #[inline(always)]
    pub const fn aescmacsize0kick(&self) -> &Aescmacsize0kick {
        &self.aescmacsize0kick
    }
    #[doc = "0x640 - H0 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh0(&self) -> &Hashh0 {
        &self.hashh0
    }
    #[doc = "0x644 - H1 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh1(&self) -> &Hashh1 {
        &self.hashh1
    }
    #[doc = "0x648 - H2 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh2(&self) -> &Hashh2 {
        &self.hashh2
    }
    #[doc = "0x64c - H3 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh3(&self) -> &Hashh3 {
        &self.hashh3
    }
    #[doc = "0x650 - H4 data. can only be written in the following HASH_CONTROL modes: SHA1 SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh4(&self) -> &Hashh4 {
        &self.hashh4
    }
    #[doc = "0x654 - H5 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh5(&self) -> &Hashh5 {
        &self.hashh5
    }
    #[doc = "0x658 - H6 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh6(&self) -> &Hashh6 {
        &self.hashh6
    }
    #[doc = "0x65c - H7 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh7(&self) -> &Hashh7 {
        &self.hashh7
    }
    #[doc = "0x660 - H8 data. can only be written in the following HASH_CONTROL modes: SHA384 SHA512"]
    #[inline(always)]
    pub const fn hashh8(&self) -> &Hashh8 {
        &self.hashh8
    }
    #[doc = "0x684 - HW padding automatically activated by engine. For the special case of ZERO bytes data vector this register should not be used! instead use HASH_PAD_CFG"]
    #[inline(always)]
    pub const fn autohwpadding(&self) -> &Autohwpadding {
        &self.autohwpadding
    }
    #[doc = "0x688 - This register is always xored with the input to the hash engine,it should be 0 if xored is not reqiured ."]
    #[inline(always)]
    pub const fn hashxordin(&self) -> &Hashxordin {
        &self.hashxordin
    }
    #[doc = "0x694 - Indication to HASH that the following data is to be loaded into initial value registers in HASH(H0:H15) or IV to AES MAC"]
    #[inline(always)]
    pub const fn loadinitstate(&self) -> &Loadinitstate {
        &self.loadinitstate
    }
    #[doc = "0x6a4 - select the AES MAC module rather than the hash module"]
    #[inline(always)]
    pub const fn hashselaesmac(&self) -> &Hashselaesmac {
        &self.hashselaesmac
    }
    #[doc = "0x7b0 - HASH VERSION Register"]
    #[inline(always)]
    pub const fn hashversion(&self) -> &Hashversion {
        &self.hashversion
    }
    #[doc = "0x7c0 - Selects which HASH mode to run"]
    #[inline(always)]
    pub const fn hashcontrol(&self) -> &Hashcontrol {
        &self.hashcontrol
    }
    #[doc = "0x7c4 - Enables the hash hw padding."]
    #[inline(always)]
    pub const fn hashpaden(&self) -> &Hashpaden {
        &self.hashpaden
    }
    #[doc = "0x7c8 - This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hashpadcfg(&self) -> &Hashpadcfg {
        &self.hashpadcfg
    }
    #[doc = "0x7cc - This register holds the length of current hash operation bit 31:0."]
    #[inline(always)]
    pub const fn hashcurlen0(&self) -> &Hashcurlen0 {
        &self.hashcurlen0
    }
    #[doc = "0x7d0 - This register holds the length of current hash operation bit 63:32."]
    #[inline(always)]
    pub const fn hashcurlen1(&self) -> &Hashcurlen1 {
        &self.hashcurlen1
    }
    #[doc = "0x7dc - HASH_PARAM Register."]
    #[inline(always)]
    pub const fn hashparam(&self) -> &Hashparam {
        &self.hashparam
    }
    #[doc = "0x7e4 - Software reset of the AES."]
    #[inline(always)]
    pub const fn hashaesswreset(&self) -> &Hashaesswreset {
        &self.hashaesswreset
    }
    #[doc = "0x7e8 - This register holds the HASH_ENDIANESS configuration."]
    #[inline(always)]
    pub const fn hashendianess(&self) -> &Hashendianess {
        &self.hashendianess
    }
    #[doc = "0x810 - This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn aesclkenable(&self) -> &Aesclkenable {
        &self.aesclkenable
    }
    #[doc = "0x818 - The HASH clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hashclkenable(&self) -> &Hashclkenable {
        &self.hashclkenable
    }
    #[doc = "0x81c - The PKA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn pkaclkenable(&self) -> &Pkaclkenable {
        &self.pkaclkenable
    }
    #[doc = "0x820 - DMA_CLK enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn dmaclkenable(&self) -> &Dmaclkenable {
        &self.dmaclkenable
    }
    #[doc = "0x824 - The CryptoCell clocks status register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn clkstatus(&self) -> &Clkstatus {
        &self.clkstatus
    }
    #[doc = "0x858 - CHACHA _SALSA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn chachaclkenable(&self) -> &Chachaclkenable {
        &self.chachaclkenable
    }
    #[doc = "0x900 - Defines the cryptographic flow."]
    #[inline(always)]
    pub const fn cryptoctl(&self) -> &Cryptoctl {
        &self.cryptoctl
    }
    #[doc = "0x910 - This register is set when the cryptographic core is busy."]
    #[inline(always)]
    pub const fn cryptobusy(&self) -> &Cryptobusy {
        &self.cryptobusy
    }
    #[doc = "0x91c - This register is set when the Hash engine is busy."]
    #[inline(always)]
    pub const fn hashbusy(&self) -> &Hashbusy {
        &self.hashbusy
    }
    #[doc = "0x930 - A general RD_WR register. For Firmware use."]
    #[inline(always)]
    pub const fn contextid(&self) -> &Contextid {
        &self.contextid
    }
    #[doc = "0x960 - Bits 31:0 of GHASH Key0 (used as the GHASH module key)."]
    #[inline(always)]
    pub const fn ghashsubkey00(&self) -> &Ghashsubkey00 {
        &self.ghashsubkey00
    }
    #[doc = "0x964 - Bits 63:32 of GHASH Key0 (used as the GHASH module key)."]
    #[inline(always)]
    pub const fn ghashsubkey01(&self) -> &Ghashsubkey01 {
        &self.ghashsubkey01
    }
    #[doc = "0x968 - Bits 95:64 of GHASH Key0 (used as the GHASH module key)."]
    #[inline(always)]
    pub const fn ghashsubkey02(&self) -> &Ghashsubkey02 {
        &self.ghashsubkey02
    }
    #[doc = "0x96c - Bits 127:96 of GHASH Key0 (used as the GHASH module key)."]
    #[inline(always)]
    pub const fn ghashsubkey03(&self) -> &Ghashsubkey03 {
        &self.ghashsubkey03
    }
    #[doc = "0x970 - Bits 31:0 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
    #[inline(always)]
    pub const fn ghashiv00(&self) -> &Ghashiv00 {
        &self.ghashiv00
    }
    #[doc = "0x974 - Bits 63:32 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
    #[inline(always)]
    pub const fn ghashiv01(&self) -> &Ghashiv01 {
        &self.ghashiv01
    }
    #[doc = "0x978 - Bits 95:64 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
    #[inline(always)]
    pub const fn ghashiv02(&self) -> &Ghashiv02 {
        &self.ghashiv02
    }
    #[doc = "0x97c - Bits 127:96 of GHASH_IV0 register.GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
    #[inline(always)]
    pub const fn ghashiv03(&self) -> &Ghashiv03 {
        &self.ghashiv03
    }
    #[doc = "0x980 - The GHASH module GHASH_BUSY Register. This register is set when the GHASH core is active."]
    #[inline(always)]
    pub const fn ghashbusy(&self) -> &Ghashbusy {
        &self.ghashbusy
    }
    #[doc = "0x984 - Writing to this address sets the GHASH engine to be ready to a new GHASH operation."]
    #[inline(always)]
    pub const fn ghashinit(&self) -> &Ghashinit {
        &self.ghashinit
    }
    #[doc = "0xa00 - The Interrupt Request register. Each bit of this register holds the interrupt status of a single interrupt source."]
    #[inline(always)]
    pub const fn hostrgfirr(&self) -> &Hostrgfirr {
        &self.hostrgfirr
    }
    #[doc = "0xa04 - The Interrupt Mask register. Each bit of this register holds the mask of a single interrupt source."]
    #[inline(always)]
    pub const fn hostrgfimr(&self) -> &Hostrgfimr {
        &self.hostrgfimr
    }
    #[doc = "0xa08 - Interrupt Clear Register."]
    #[inline(always)]
    pub const fn hostrgficr(&self) -> &Hostrgficr {
        &self.hostrgficr
    }
    #[doc = "0xa0c - This register defines the endianness of the Host-accessible registers. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostrgfendian(&self) -> &Hostrgfendian {
        &self.hostrgfendian
    }
    #[doc = "0xa24 - This register holds the CryptoCell product signature."]
    #[inline(always)]
    pub const fn hostrgfsignature(&self) -> &Hostrgfsignature {
        &self.hostrgfsignature
    }
    #[doc = "0xa28 - This register holds the values of CryptoCells pre-synthesis flags"]
    #[inline(always)]
    pub const fn hostboot(&self) -> &Hostboot {
        &self.hostboot
    }
    #[doc = "0xa38 - AES hardware key select. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostcryptokeysel(&self) -> &Hostcryptokeysel {
        &self.hostcryptokeysel
    }
    #[doc = "0xa78 - This register enables the core clk gating by masking_enabling the cc_idle_state output signal."]
    #[inline(always)]
    pub const fn hostcoreclkgatingenable(&self) -> &Hostcoreclkgatingenable {
        &self.hostcoreclkgatingenable
    }
    #[doc = "0xa7c - This register holds the idle indication of CC . Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostccisidle(&self) -> &Hostccisidle {
        &self.hostccisidle
    }
    #[doc = "0xa80 - This register start the power-down sequence. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostpowerdown(&self) -> &Hostpowerdown {
        &self.hostpowerdown
    }
    #[doc = "0xa84 - These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis"]
    #[inline(always)]
    pub const fn hostremoveghashengine(&self) -> &Hostremoveghashengine {
        &self.hostremoveghashengine
    }
    #[doc = "0xa88 - These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis"]
    #[inline(always)]
    pub const fn hostremovechachaengine(&self) -> &Hostremovechachaengine {
        &self.hostremovechachaengine
    }
    #[doc = "0xb00 - This register forces the ahb transactions to be always singles."]
    #[inline(always)]
    pub const fn ahbmsingles(&self) -> &Ahbmsingles {
        &self.ahbmsingles
    }
    #[doc = "0xb04 - This register holds the ahb prot value"]
    #[inline(always)]
    pub const fn ahbmhprot(&self) -> &Ahbmhprot {
        &self.ahbmhprot
    }
    #[doc = "0xb08 - This register holds ahb hmastlock value"]
    #[inline(always)]
    pub const fn ahbmhmastlock(&self) -> &Ahbmhmastlock {
        &self.ahbmhmastlock
    }
    #[doc = "0xb0c - This register holds ahb hnonsec value"]
    #[inline(always)]
    pub const fn ahbmhnonsec(&self) -> &Ahbmhnonsec {
        &self.ahbmhnonsec
    }
    #[doc = "0xc00 - This address can be used by the CPU to write data directly to the DIN buffer to be sent to engines."]
    #[inline(always)]
    pub const fn dinbuffer(&self) -> &Dinbuffer {
        &self.dinbuffer
    }
    #[doc = "0xc20 - Indicates whether memory (AXI) source DMA (DIN) is busy."]
    #[inline(always)]
    pub const fn dinmemdmabusy(&self) -> &Dinmemdmabusy {
        &self.dinmemdmabusy
    }
    #[doc = "0xc28 - This register is used in direct LLI mode - holds the location of the data source in the memory (AXI)."]
    #[inline(always)]
    pub const fn srclliword0(&self) -> &Srclliword0 {
        &self.srclliword0
    }
    #[doc = "0xc2c - This register is used in direct LLI mode - holds the number of bytes to be read from the memory (AXI). Writing to this register triggers the DMA. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn srclliword1(&self) -> &Srclliword1 {
        &self.srclliword1
    }
    #[doc = "0xc30 - Location of data (start address) to be read from SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn sramsrcaddr(&self) -> &Sramsrcaddr {
        &self.sramsrcaddr
    }
    #[doc = "0xc34 - This register holds the size of the data (in bytes) to be read from the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn dinsrambyteslen(&self) -> &Dinsrambyteslen {
        &self.dinsrambyteslen
    }
    #[doc = "0xc38 - This register holds the status of the SRAM DMA DIN."]
    #[inline(always)]
    pub const fn dinsramdmabusy(&self) -> &Dinsramdmabusy {
        &self.dinsramdmabusy
    }
    #[doc = "0xc3c - This register defines the endianness of the DIN interface to SRAM."]
    #[inline(always)]
    pub const fn dinsramendianness(&self) -> &Dinsramendianness {
        &self.dinsramendianness
    }
    #[doc = "0xc48 - This register hold the number of bytes to be transmited using external DMA. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn dincpudatasize(&self) -> &Dincpudatasize {
        &self.dincpudatasize
    }
    #[doc = "0xc50 - DIN FIFO empty indication"]
    #[inline(always)]
    pub const fn fifoinempty(&self) -> &Fifoinempty {
        &self.fifoinempty
    }
    #[doc = "0xc58 - Writing to this register resets the DIN_FIFO pointers."]
    #[inline(always)]
    pub const fn dinfiforstpntr(&self) -> &Dinfiforstpntr {
        &self.dinfiforstpntr
    }
    #[doc = "0xd00 - Cryptographic result - CPU can directly access it. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn doutbuffer(&self) -> &Doutbuffer {
        &self.doutbuffer
    }
    #[doc = "0xd20 - DOUT memory DMA busy - Indicates that memory (AXI) destination DMA (DOUT) is busy,"]
    #[inline(always)]
    pub const fn doutmemdmabusy(&self) -> &Doutmemdmabusy {
        &self.doutmemdmabusy
    }
    #[doc = "0xd28 - This register is used in direct LLI mode - holds the location of the data destination in the memory (AXI)"]
    #[inline(always)]
    pub const fn dstlliword0(&self) -> &Dstlliword0 {
        &self.dstlliword0
    }
    #[doc = "0xd2c - This register is used in direct LLI mode - holds the number of bytes to be written to the memory (AXI). Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn dstlliword1(&self) -> &Dstlliword1 {
        &self.dstlliword1
    }
    #[doc = "0xd30 - Location of result to be sent to in SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn sramdestaddr(&self) -> &Sramdestaddr {
        &self.sramdestaddr
    }
    #[doc = "0xd34 - This register holds the size of the data (in bytes) to be written to the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn doutsrambyteslen(&self) -> &Doutsrambyteslen {
        &self.doutsrambyteslen
    }
    #[doc = "0xd38 - This register holds the status of the SRAM DMA DOUT."]
    #[inline(always)]
    pub const fn doutsramdmabusy(&self) -> &Doutsramdmabusy {
        &self.doutsramdmabusy
    }
    #[doc = "0xd3c - This register defines the endianness of the DOUT interface from SRAM."]
    #[inline(always)]
    pub const fn doutsramendianness(&self) -> &Doutsramendianness {
        &self.doutsramendianness
    }
    #[doc = "0xd44 - Indication that the next read from the CPU is the last one. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding)."]
    #[inline(always)]
    pub const fn readalignlast(&self) -> &Readalignlast {
        &self.readalignlast
    }
    #[doc = "0xd50 - DOUT_FIFO_EMPTY Register."]
    #[inline(always)]
    pub const fn doutfifoempty(&self) -> &Doutfifoempty {
        &self.doutfifoempty
    }
    #[doc = "0xf00 - READ WRITE DATA FROM SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn sramdata(&self) -> &Sramdata {
        &self.sramdata
    }
    #[doc = "0xf04 - first address given to SRAM DMA for read_write transactions from SRAM"]
    #[inline(always)]
    pub const fn sramaddr(&self) -> &Sramaddr {
        &self.sramaddr
    }
    #[doc = "0xf08 - The SRAM content is ready for read in SRAM_DATA."]
    #[inline(always)]
    pub const fn sramdataready(&self) -> &Sramdataready {
        &self.sramdataready
    }
    #[doc = "0xfd0 - Peripheral ID 4 (PID4)."]
    #[inline(always)]
    pub const fn peripheralid4(&self) -> &Peripheralid4 {
        &self.peripheralid4
    }
    #[doc = "0xfe0 - Peripheral ID 0 (PID0)."]
    #[inline(always)]
    pub const fn peripheralid0(&self) -> &Peripheralid0 {
        &self.peripheralid0
    }
    #[doc = "0xfe4 - Peripheral ID 1 (PID1)."]
    #[inline(always)]
    pub const fn peripheralid1(&self) -> &Peripheralid1 {
        &self.peripheralid1
    }
    #[doc = "0xfe8 - Peripheral ID 2 (PID2)."]
    #[inline(always)]
    pub const fn peripheralid2(&self) -> &Peripheralid2 {
        &self.peripheralid2
    }
    #[doc = "0xfec - Peripheral ID 3 (PID3)."]
    #[inline(always)]
    pub const fn peripheralid3(&self) -> &Peripheralid3 {
        &self.peripheralid3
    }
    #[doc = "0xff0 - Component ID0."]
    #[inline(always)]
    pub const fn componentid0(&self) -> &Componentid0 {
        &self.componentid0
    }
    #[doc = "0xff4 - Component ID1."]
    #[inline(always)]
    pub const fn componentid1(&self) -> &Componentid1 {
        &self.componentid1
    }
    #[doc = "0xff8 - Component ID2."]
    #[inline(always)]
    pub const fn componentid2(&self) -> &Componentid2 {
        &self.componentid2
    }
    #[doc = "0xffc - Component ID3."]
    #[inline(always)]
    pub const fn componentid3(&self) -> &Componentid3 {
        &self.componentid3
    }
    #[doc = "0x1e00 - The DCU \\[31:0\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdcuen0(&self) -> &Hostdcuen0 {
        &self.hostdcuen0
    }
    #[doc = "0x1e04 - The DCU \\[63:32\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdcuen1(&self) -> &Hostdcuen1 {
        &self.hostdcuen1
    }
    #[doc = "0x1e08 - The DCU \\[95:64\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdcuen2(&self) -> &Hostdcuen2 {
        &self.hostdcuen2
    }
    #[doc = "0x1e0c - The DCU \\[1271:96\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdcuen3(&self) -> &Hostdcuen3 {
        &self.hostdcuen3
    }
    #[doc = "0x1e10 - The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdculock0(&self) -> &Hostdculock0 {
        &self.hostdculock0
    }
    #[doc = "0x1e14 - The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdculock1(&self) -> &Hostdculock1 {
        &self.hostdculock1
    }
    #[doc = "0x1e18 - The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdculock2(&self) -> &Hostdculock2 {
        &self.hostdculock2
    }
    #[doc = "0x1e1c - The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostdculock3(&self) -> &Hostdculock3 {
        &self.hostdculock3
    }
    #[doc = "0x1e20 - The DCU lock register."]
    #[inline(always)]
    pub const fn aoicvdcurestrictionmask0(&self) -> &Aoicvdcurestrictionmask0 {
        &self.aoicvdcurestrictionmask0
    }
    #[doc = "0x1e24 - The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets"]
    #[inline(always)]
    pub const fn aoicvdcurestrictionmask1(&self) -> &Aoicvdcurestrictionmask1 {
        &self.aoicvdcurestrictionmask1
    }
    #[doc = "0x1e28 - The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets"]
    #[inline(always)]
    pub const fn aoicvdcurestrictionmask2(&self) -> &Aoicvdcurestrictionmask2 {
        &self.aoicvdcurestrictionmask2
    }
    #[doc = "0x1e2c - The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets"]
    #[inline(always)]
    pub const fn aoicvdcurestrictionmask3(&self) -> &Aoicvdcurestrictionmask3 {
        &self.aoicvdcurestrictionmask3
    }
    #[doc = "0x1e30 - The reset-upon-debug indication"]
    #[inline(always)]
    pub const fn aoccsecdebugreset(&self) -> &Aoccsecdebugreset {
        &self.aoccsecdebugreset
    }
    #[doc = "0x1e34 - These masks will define, per LCS, which DCU bits will be tied to zero, even if the Host tries to set them. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostaolockbits(&self) -> &Hostaolockbits {
        &self.hostaolockbits
    }
    #[doc = "0x1e38 - This register holds the AO_APB_FILTERING data. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn aoapbfiltering(&self) -> &Aoapbfiltering {
        &self.aoapbfiltering
    }
    #[doc = "0x1e3c - holds the AO_CC_GPPC value from AONote: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn aoccgppc(&self) -> &Aoccgppc {
        &self.aoccgppc
    }
    #[doc = "0x1e40 - Writing to this register generates a general reset to CryptoCell. This reset takes about 4 core clock cycles.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn hostrgfccswrst(&self) -> &Hostrgfccswrst {
        &self.hostrgfccswrst
    }
    #[doc = "0x1f04 - This register reflects the fuse_aib_prog_completed input, which indicates that the fuse programming was completed.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn aibfuseprogcompleted(&self) -> &Aibfuseprogcompleted {
        &self.aibfuseprogcompleted
    }
    #[doc = "0x1f08 - AIB debug status register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn nvmdebugstatus(&self) -> &Nvmdebugstatus {
        &self.nvmdebugstatus
    }
    #[doc = "0x1f0c - Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn lcsisvalid(&self) -> &Lcsisvalid {
        &self.lcsisvalid
    }
    #[doc = "0x1f10 - Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn nvmisidle(&self) -> &Nvmisidle {
        &self.nvmisidle
    }
    #[doc = "0x1f14 - The lifecycle state register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn lcsreg(&self) -> &Lcsreg {
        &self.lcsreg
    }
    #[doc = "0x1f18 - This register interface is used to update the RKEK(KDR) registers when the device is in CM or DM mode , it is Write-once (per warm boot) in RMA LCS, The RKEK is updated by shifting ."]
    #[inline(always)]
    pub const fn hostshadowkdrreg(&self) -> &Hostshadowkdrreg {
        &self.hostshadowkdrreg
    }
    #[doc = "0x1f1c - This register interface is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting"]
    #[inline(always)]
    pub const fn hostshadowkcpreg(&self) -> &Hostshadowkcpreg {
        &self.hostshadowkcpreg
    }
    #[doc = "0x1f20 - This register interface is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting"]
    #[inline(always)]
    pub const fn hostshadowkcereg(&self) -> &Hostshadowkcereg {
        &self.hostshadowkcereg
    }
    #[doc = "0x1f24 - This register interface is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting"]
    #[inline(always)]
    pub const fn hostshadowkpicvreg(&self) -> &Hostshadowkpicvreg {
        &self.hostshadowkpicvreg
    }
    #[doc = "0x1f28 - This register interface is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting"]
    #[inline(always)]
    pub const fn hostshadowkceicvreg(&self) -> &Hostshadowkceicvreg {
        &self.hostshadowkceicvreg
    }
    #[doc = "0x1f2c - OTP_ADDR_WIDTH parameter, that will define the integrated OTP address width (address in words). The supported sizes are 6 (for 2 Kbits),7,8,9,11 (for 64 Kbits). The default value in the provided RTL will be 6.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
    #[inline(always)]
    pub const fn otpaddrwidthdef(&self) -> &Otpaddrwidthdef {
        &self.otpaddrwidthdef
    }
}
#[doc = "MEMORYMAP0 (rw) register accessor: This register maps the virtual register R0 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap0`]
module"]
#[doc(alias = "MEMORYMAP0")]
pub type Memorymap0 = crate::Reg<memorymap0::Memorymap0Spec>;
#[doc = "This register maps the virtual register R0 to a physical address in memory."]
pub mod memorymap0;
#[doc = "MEMORYMAP1 (rw) register accessor: This register maps the virtual register R1 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap1`]
module"]
#[doc(alias = "MEMORYMAP1")]
pub type Memorymap1 = crate::Reg<memorymap1::Memorymap1Spec>;
#[doc = "This register maps the virtual register R1 to a physical address in memory."]
pub mod memorymap1;
#[doc = "MEMORYMAP2 (rw) register accessor: This register maps the virtual register R2 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap2`]
module"]
#[doc(alias = "MEMORYMAP2")]
pub type Memorymap2 = crate::Reg<memorymap2::Memorymap2Spec>;
#[doc = "This register maps the virtual register R2 to a physical address in memory."]
pub mod memorymap2;
#[doc = "MEMORYMAP3 (rw) register accessor: This register maps the virtual register R3 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap3`]
module"]
#[doc(alias = "MEMORYMAP3")]
pub type Memorymap3 = crate::Reg<memorymap3::Memorymap3Spec>;
#[doc = "This register maps the virtual register R3 to a physical address in memory."]
pub mod memorymap3;
#[doc = "MEMORYMAP4 (rw) register accessor: This register maps the virtual register R4 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap4`]
module"]
#[doc(alias = "MEMORYMAP4")]
pub type Memorymap4 = crate::Reg<memorymap4::Memorymap4Spec>;
#[doc = "This register maps the virtual register R4 to a physical address in memory."]
pub mod memorymap4;
#[doc = "MEMORYMAP5 (rw) register accessor: This register maps the virtual register R5 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap5`]
module"]
#[doc(alias = "MEMORYMAP5")]
pub type Memorymap5 = crate::Reg<memorymap5::Memorymap5Spec>;
#[doc = "This register maps the virtual register R5 to a physical address in memory."]
pub mod memorymap5;
#[doc = "MEMORYMAP6 (rw) register accessor: This register maps the virtual register R6 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap6`]
module"]
#[doc(alias = "MEMORYMAP6")]
pub type Memorymap6 = crate::Reg<memorymap6::Memorymap6Spec>;
#[doc = "This register maps the virtual register R6 to a physical address in memory."]
pub mod memorymap6;
#[doc = "MEMORYMAP7 (rw) register accessor: This register maps the virtual register R7 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap7`]
module"]
#[doc(alias = "MEMORYMAP7")]
pub type Memorymap7 = crate::Reg<memorymap7::Memorymap7Spec>;
#[doc = "This register maps the virtual register R7 to a physical address in memory."]
pub mod memorymap7;
#[doc = "MEMORYMAP8 (rw) register accessor: This register maps the virtual register R8 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap8`]
module"]
#[doc(alias = "MEMORYMAP8")]
pub type Memorymap8 = crate::Reg<memorymap8::Memorymap8Spec>;
#[doc = "This register maps the virtual register R8 to a physical address in memory."]
pub mod memorymap8;
#[doc = "MEMORYMAP9 (rw) register accessor: This register maps the virtual register R9 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap9`]
module"]
#[doc(alias = "MEMORYMAP9")]
pub type Memorymap9 = crate::Reg<memorymap9::Memorymap9Spec>;
#[doc = "This register maps the virtual register R9 to a physical address in memory."]
pub mod memorymap9;
#[doc = "MEMORYMAP10 (rw) register accessor: This register maps the virtual register R10 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap10`]
module"]
#[doc(alias = "MEMORYMAP10")]
pub type Memorymap10 = crate::Reg<memorymap10::Memorymap10Spec>;
#[doc = "This register maps the virtual register R10 to a physical address in memory."]
pub mod memorymap10;
#[doc = "MEMORYMAP11 (rw) register accessor: This register maps the virtual register R11 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap11`]
module"]
#[doc(alias = "MEMORYMAP11")]
pub type Memorymap11 = crate::Reg<memorymap11::Memorymap11Spec>;
#[doc = "This register maps the virtual register R11 to a physical address in memory."]
pub mod memorymap11;
#[doc = "MEMORYMAP12 (rw) register accessor: This register maps the virtual register R12 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap12`]
module"]
#[doc(alias = "MEMORYMAP12")]
pub type Memorymap12 = crate::Reg<memorymap12::Memorymap12Spec>;
#[doc = "This register maps the virtual register R12 to a physical address in memory."]
pub mod memorymap12;
#[doc = "MEMORYMAP13 (rw) register accessor: This register maps the virtual register R13 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap13`]
module"]
#[doc(alias = "MEMORYMAP13")]
pub type Memorymap13 = crate::Reg<memorymap13::Memorymap13Spec>;
#[doc = "This register maps the virtual register R13 to a physical address in memory."]
pub mod memorymap13;
#[doc = "MEMORYMAP14 (rw) register accessor: This register maps the virtual register R14 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap14`]
module"]
#[doc(alias = "MEMORYMAP14")]
pub type Memorymap14 = crate::Reg<memorymap14::Memorymap14Spec>;
#[doc = "This register maps the virtual register R14 to a physical address in memory."]
pub mod memorymap14;
#[doc = "MEMORYMAP15 (rw) register accessor: This register maps the virtual register R15 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap15`]
module"]
#[doc(alias = "MEMORYMAP15")]
pub type Memorymap15 = crate::Reg<memorymap15::Memorymap15Spec>;
#[doc = "This register maps the virtual register R15 to a physical address in memory."]
pub mod memorymap15;
#[doc = "MEMORYMAP16 (rw) register accessor: This register maps the virtual register R16 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap16`]
module"]
#[doc(alias = "MEMORYMAP16")]
pub type Memorymap16 = crate::Reg<memorymap16::Memorymap16Spec>;
#[doc = "This register maps the virtual register R16 to a physical address in memory."]
pub mod memorymap16;
#[doc = "MEMORYMAP17 (rw) register accessor: This register maps the virtual register R17 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap17`]
module"]
#[doc(alias = "MEMORYMAP17")]
pub type Memorymap17 = crate::Reg<memorymap17::Memorymap17Spec>;
#[doc = "This register maps the virtual register R17 to a physical address in memory."]
pub mod memorymap17;
#[doc = "MEMORYMAP18 (rw) register accessor: This register maps the virtual register R18 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap18`]
module"]
#[doc(alias = "MEMORYMAP18")]
pub type Memorymap18 = crate::Reg<memorymap18::Memorymap18Spec>;
#[doc = "This register maps the virtual register R18 to a physical address in memory."]
pub mod memorymap18;
#[doc = "MEMORYMAP19 (rw) register accessor: This register maps the virtual register R19 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap19`]
module"]
#[doc(alias = "MEMORYMAP19")]
pub type Memorymap19 = crate::Reg<memorymap19::Memorymap19Spec>;
#[doc = "This register maps the virtual register R19 to a physical address in memory."]
pub mod memorymap19;
#[doc = "MEMORYMAP20 (rw) register accessor: This register maps the virtual register R20 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap20`]
module"]
#[doc(alias = "MEMORYMAP20")]
pub type Memorymap20 = crate::Reg<memorymap20::Memorymap20Spec>;
#[doc = "This register maps the virtual register R20 to a physical address in memory."]
pub mod memorymap20;
#[doc = "MEMORYMAP21 (rw) register accessor: This register maps the virtual register R21 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap21`]
module"]
#[doc(alias = "MEMORYMAP21")]
pub type Memorymap21 = crate::Reg<memorymap21::Memorymap21Spec>;
#[doc = "This register maps the virtual register R21 to a physical address in memory."]
pub mod memorymap21;
#[doc = "MEMORYMAP22 (rw) register accessor: This register maps the virtual register R22 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap22`]
module"]
#[doc(alias = "MEMORYMAP22")]
pub type Memorymap22 = crate::Reg<memorymap22::Memorymap22Spec>;
#[doc = "This register maps the virtual register R22 to a physical address in memory."]
pub mod memorymap22;
#[doc = "MEMORYMAP23 (rw) register accessor: This register maps the virtual register R23 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap23`]
module"]
#[doc(alias = "MEMORYMAP23")]
pub type Memorymap23 = crate::Reg<memorymap23::Memorymap23Spec>;
#[doc = "This register maps the virtual register R23 to a physical address in memory."]
pub mod memorymap23;
#[doc = "MEMORYMAP24 (rw) register accessor: This register maps the virtual register R24 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap24`]
module"]
#[doc(alias = "MEMORYMAP24")]
pub type Memorymap24 = crate::Reg<memorymap24::Memorymap24Spec>;
#[doc = "This register maps the virtual register R24 to a physical address in memory."]
pub mod memorymap24;
#[doc = "MEMORYMAP25 (rw) register accessor: This register maps the virtual register R25 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap25`]
module"]
#[doc(alias = "MEMORYMAP25")]
pub type Memorymap25 = crate::Reg<memorymap25::Memorymap25Spec>;
#[doc = "This register maps the virtual register R25 to a physical address in memory."]
pub mod memorymap25;
#[doc = "MEMORYMAP26 (rw) register accessor: This register maps the virtual register R26 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap26`]
module"]
#[doc(alias = "MEMORYMAP26")]
pub type Memorymap26 = crate::Reg<memorymap26::Memorymap26Spec>;
#[doc = "This register maps the virtual register R26 to a physical address in memory."]
pub mod memorymap26;
#[doc = "MEMORYMAP27 (rw) register accessor: This register maps the virtual register R27 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap27`]
module"]
#[doc(alias = "MEMORYMAP27")]
pub type Memorymap27 = crate::Reg<memorymap27::Memorymap27Spec>;
#[doc = "This register maps the virtual register R27 to a physical address in memory."]
pub mod memorymap27;
#[doc = "MEMORYMAP28 (rw) register accessor: This register maps the virtual register R28 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap28`]
module"]
#[doc(alias = "MEMORYMAP28")]
pub type Memorymap28 = crate::Reg<memorymap28::Memorymap28Spec>;
#[doc = "This register maps the virtual register R28 to a physical address in memory."]
pub mod memorymap28;
#[doc = "MEMORYMAP29 (rw) register accessor: This register maps the virtual register R29 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap29`]
module"]
#[doc(alias = "MEMORYMAP29")]
pub type Memorymap29 = crate::Reg<memorymap29::Memorymap29Spec>;
#[doc = "This register maps the virtual register R29 to a physical address in memory."]
pub mod memorymap29;
#[doc = "MEMORYMAP30 (rw) register accessor: This register maps the virtual register R30 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap30`]
module"]
#[doc(alias = "MEMORYMAP30")]
pub type Memorymap30 = crate::Reg<memorymap30::Memorymap30Spec>;
#[doc = "This register maps the virtual register R30 to a physical address in memory."]
pub mod memorymap30;
#[doc = "MEMORYMAP31 (rw) register accessor: This register maps the virtual register R31 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memorymap31`]
module"]
#[doc(alias = "MEMORYMAP31")]
pub type Memorymap31 = crate::Reg<memorymap31::Memorymap31Spec>;
#[doc = "This register maps the virtual register R31 to a physical address in memory."]
pub mod memorymap31;
#[doc = "OPCODE (rw) register accessor: This register holds the PKAs OPCODE.\n\nYou can [`read`](crate::Reg::read) this register and get [`opcode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcode`]
module"]
#[doc(alias = "OPCODE")]
pub type Opcode = crate::Reg<opcode::OpcodeSpec>;
#[doc = "This register holds the PKAs OPCODE."]
pub mod opcode;
#[doc = "NNPT0T1ADDR (rw) register accessor: This register maps N_NP_T0_T1 to a virtual address.\n\nYou can [`read`](crate::Reg::read) this register and get [`nnpt0t1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nnpt0t1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nnpt0t1addr`]
module"]
#[doc(alias = "NNPT0T1ADDR")]
pub type Nnpt0t1addr = crate::Reg<nnpt0t1addr::Nnpt0t1addrSpec>;
#[doc = "This register maps N_NP_T0_T1 to a virtual address."]
pub mod nnpt0t1addr;
#[doc = "PKASTATUS (rw) register accessor: This register holds the PKA pipe status.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkastatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkastatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkastatus`]
module"]
#[doc(alias = "PKASTATUS")]
pub type Pkastatus = crate::Reg<pkastatus::PkastatusSpec>;
#[doc = "This register holds the PKA pipe status."]
pub mod pkastatus;
#[doc = "PKASWRESET (rw) register accessor: Writing to this register triggers a software reset of the PKA.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkaswreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkaswreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkaswreset`]
module"]
#[doc(alias = "PKASWRESET")]
pub type Pkaswreset = crate::Reg<pkaswreset::PkaswresetSpec>;
#[doc = "Writing to this register triggers a software reset of the PKA."]
pub mod pkaswreset;
#[doc = "PKAL0 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal0`]
module"]
#[doc(alias = "PKAL0")]
pub type Pkal0 = crate::Reg<pkal0::Pkal0Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal0;
#[doc = "PKAL1 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal1`]
module"]
#[doc(alias = "PKAL1")]
pub type Pkal1 = crate::Reg<pkal1::Pkal1Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal1;
#[doc = "PKAL2 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal2`]
module"]
#[doc(alias = "PKAL2")]
pub type Pkal2 = crate::Reg<pkal2::Pkal2Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal2;
#[doc = "PKAL3 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal3`]
module"]
#[doc(alias = "PKAL3")]
pub type Pkal3 = crate::Reg<pkal3::Pkal3Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal3;
#[doc = "PKAL4 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal4`]
module"]
#[doc(alias = "PKAL4")]
pub type Pkal4 = crate::Reg<pkal4::Pkal4Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal4;
#[doc = "PKAL5 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal5`]
module"]
#[doc(alias = "PKAL5")]
pub type Pkal5 = crate::Reg<pkal5::Pkal5Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal5;
#[doc = "PKAL6 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal6`]
module"]
#[doc(alias = "PKAL6")]
pub type Pkal6 = crate::Reg<pkal6::Pkal6Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal6;
#[doc = "PKAL7 (rw) register accessor: This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkal7`]
module"]
#[doc(alias = "PKAL7")]
pub type Pkal7 = crate::Reg<pkal7::Pkal7Spec>;
#[doc = "This register holds one of the optional size of the operation."]
pub mod pkal7;
#[doc = "PKAPIPERDY (rw) register accessor: This register indicates whether the PKA pipe is ready to receive a new OPCODE.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkapiperdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkapiperdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkapiperdy`]
module"]
#[doc(alias = "PKAPIPERDY")]
pub type Pkapiperdy = crate::Reg<pkapiperdy::PkapiperdySpec>;
#[doc = "This register indicates whether the PKA pipe is ready to receive a new OPCODE."]
pub mod pkapiperdy;
#[doc = "PKADONE (rw) register accessor: This register indicates whether PKA operation is completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkadone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkadone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkadone`]
module"]
#[doc(alias = "PKADONE")]
pub type Pkadone = crate::Reg<pkadone::PkadoneSpec>;
#[doc = "This register indicates whether PKA operation is completed."]
pub mod pkadone;
#[doc = "PKAMONSELECT (rw) register accessor: This register defines which PKA FSM monitor is being output.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkamonselect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkamonselect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkamonselect`]
module"]
#[doc(alias = "PKAMONSELECT")]
pub type Pkamonselect = crate::Reg<pkamonselect::PkamonselectSpec>;
#[doc = "This register defines which PKA FSM monitor is being output."]
pub mod pkamonselect;
#[doc = "PKAVERSION (rw) register accessor: This register holds the pka version\n\nYou can [`read`](crate::Reg::read) this register and get [`pkaversion::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkaversion::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkaversion`]
module"]
#[doc(alias = "PKAVERSION")]
pub type Pkaversion = crate::Reg<pkaversion::PkaversionSpec>;
#[doc = "This register holds the pka version"]
pub mod pkaversion;
#[doc = "PKAMONREAD (rw) register accessor: The PKA monitor bus register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkamonread::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkamonread::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkamonread`]
module"]
#[doc(alias = "PKAMONREAD")]
pub type Pkamonread = crate::Reg<pkamonread::PkamonreadSpec>;
#[doc = "The PKA monitor bus register."]
pub mod pkamonread;
#[doc = "PKASRAMADDR (rw) register accessor: first address given to PKA SRAM for write transactions.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkasramaddr`]
module"]
#[doc(alias = "PKASRAMADDR")]
pub type Pkasramaddr = crate::Reg<pkasramaddr::PkasramaddrSpec>;
#[doc = "first address given to PKA SRAM for write transactions."]
pub mod pkasramaddr;
#[doc = "PKASRAMWDATA (rw) register accessor: Write data to PKA SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramwdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramwdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkasramwdata`]
module"]
#[doc(alias = "PKASRAMWDATA")]
pub type Pkasramwdata = crate::Reg<pkasramwdata::PkasramwdataSpec>;
#[doc = "Write data to PKA SRAM."]
pub mod pkasramwdata;
#[doc = "PKASRAMRDATA (rw) register accessor: Read data from PKA SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramrdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramrdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkasramrdata`]
module"]
#[doc(alias = "PKASRAMRDATA")]
pub type Pkasramrdata = crate::Reg<pkasramrdata::PkasramrdataSpec>;
#[doc = "Read data from PKA SRAM."]
pub mod pkasramrdata;
#[doc = "PKASRAMWRCLR (rw) register accessor: Write buffer clean.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramwrclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramwrclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkasramwrclr`]
module"]
#[doc(alias = "PKASRAMWRCLR")]
pub type Pkasramwrclr = crate::Reg<pkasramwrclr::PkasramwrclrSpec>;
#[doc = "Write buffer clean."]
pub mod pkasramwrclr;
#[doc = "PKASRAMRADDR (rw) register accessor: first address given to PKA SRAM for read transactions.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramraddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramraddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkasramraddr`]
module"]
#[doc(alias = "PKASRAMRADDR")]
pub type Pkasramraddr = crate::Reg<pkasramraddr::PkasramraddrSpec>;
#[doc = "first address given to PKA SRAM for read transactions."]
pub mod pkasramraddr;
#[doc = "PKAWORDACCESS (rw) register accessor: This register holds the data written to PKA memory using the wop opcode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkawordaccess::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkawordaccess::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkawordaccess`]
module"]
#[doc(alias = "PKAWORDACCESS")]
pub type Pkawordaccess = crate::Reg<pkawordaccess::PkawordaccessSpec>;
#[doc = "This register holds the data written to PKA memory using the wop opcode."]
pub mod pkawordaccess;
#[doc = "PKABUFFADDR (rw) register accessor: This register maps the virtual buffer registers to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkabuffaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkabuffaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkabuffaddr`]
module"]
#[doc(alias = "PKABUFFADDR")]
pub type Pkabuffaddr = crate::Reg<pkabuffaddr::PkabuffaddrSpec>;
#[doc = "This register maps the virtual buffer registers to a physical address in memory."]
pub mod pkabuffaddr;
#[doc = "RNGIMR (rw) register accessor: Interrupt masking register. Consists of {prng_imr trng_imr} bit\\[31-16\\]
- PRNG_IMR bit\\[15-0\\]
- TRNG_IMR(Ws - PRNG bit exists only if PRNG_EXISTS flag)\n\nYou can [`read`](crate::Reg::read) this register and get [`rngimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngimr`]
module"]
#[doc(alias = "RNGIMR")]
pub type Rngimr = crate::Reg<rngimr::RngimrSpec>;
#[doc = "Interrupt masking register. Consists of {prng_imr trng_imr} bit\\[31-16\\]
- PRNG_IMR bit\\[15-0\\]
- TRNG_IMR(Ws - PRNG bit exists only if PRNG_EXISTS flag)"]
pub mod rngimr;
#[doc = "RNGISR (rw) register accessor: Status register. If corresponding RNG_IMR bit is unmasked, an interrupt is generated.Consists of trng_isr and prng_isr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`rngisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngisr`]
module"]
#[doc(alias = "RNGISR")]
pub type Rngisr = crate::Reg<rngisr::RngisrSpec>;
#[doc = "Status register. If corresponding RNG_IMR bit is unmasked, an interrupt is generated.Consists of trng_isr and prng_isr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG"]
pub mod rngisr;
#[doc = "RNGICR (rw) register accessor: Interrupt_status bit clear Register. Consists of trng_icr and prng_icr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`rngicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngicr`]
module"]
#[doc(alias = "RNGICR")]
pub type Rngicr = crate::Reg<rngicr::RngicrSpec>;
#[doc = "Interrupt_status bit clear Register. Consists of trng_icr and prng_icr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG"]
pub mod rngicr;
#[doc = "TRNGCONFIG (rw) register accessor: This register handles TRNG configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`trngconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trngconfig`]
module"]
#[doc(alias = "TRNGCONFIG")]
pub type Trngconfig = crate::Reg<trngconfig::TrngconfigSpec>;
#[doc = "This register handles TRNG configuration"]
pub mod trngconfig;
#[doc = "TRNGVALID (rw) register accessor: This register indicates that the TRNG data is valid.\n\nYou can [`read`](crate::Reg::read) this register and get [`trngvalid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngvalid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trngvalid`]
module"]
#[doc(alias = "TRNGVALID")]
pub type Trngvalid = crate::Reg<trngvalid::TrngvalidSpec>;
#[doc = "This register indicates that the TRNG data is valid."]
pub mod trngvalid;
#[doc = "EHRDATA0 (rw) register accessor: This register contains the data collected in the TRNG\\[31_0\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehrdata0`]
module"]
#[doc(alias = "EHRDATA0")]
pub type Ehrdata0 = crate::Reg<ehrdata0::Ehrdata0Spec>;
#[doc = "This register contains the data collected in the TRNG\\[31_0\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub mod ehrdata0;
#[doc = "EHRDATA1 (rw) register accessor: This register contains the data collected in the TRNG\\[63_32\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehrdata1`]
module"]
#[doc(alias = "EHRDATA1")]
pub type Ehrdata1 = crate::Reg<ehrdata1::Ehrdata1Spec>;
#[doc = "This register contains the data collected in the TRNG\\[63_32\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub mod ehrdata1;
#[doc = "EHRDATA2 (rw) register accessor: This register contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehrdata2`]
module"]
#[doc(alias = "EHRDATA2")]
pub type Ehrdata2 = crate::Reg<ehrdata2::Ehrdata2Spec>;
#[doc = "This register contains the data collected in the TRNG\\[95_64\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub mod ehrdata2;
#[doc = "EHRDATA3 (rw) register accessor: This register contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehrdata3`]
module"]
#[doc(alias = "EHRDATA3")]
pub type Ehrdata3 = crate::Reg<ehrdata3::Ehrdata3Spec>;
#[doc = "This register contains the data collected in the TRNG\\[127_96\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub mod ehrdata3;
#[doc = "EHRDATA4 (rw) register accessor: This register contains the data collected in the TRNG\\[159_128\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehrdata4`]
module"]
#[doc(alias = "EHRDATA4")]
pub type Ehrdata4 = crate::Reg<ehrdata4::Ehrdata4Spec>;
#[doc = "This register contains the data collected in the TRNG\\[159_128\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub mod ehrdata4;
#[doc = "EHRDATA5 (rw) register accessor: This register contains the data collected in the TRNG\\[191_160\\]. Note: can only be set while in debug mode (rng_debug_enable input is set).\n\nYou can [`read`](crate::Reg::read) this register and get [`ehrdata5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehrdata5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehrdata5`]
module"]
#[doc(alias = "EHRDATA5")]
pub type Ehrdata5 = crate::Reg<ehrdata5::Ehrdata5Spec>;
#[doc = "This register contains the data collected in the TRNG\\[191_160\\]. Note: can only be set while in debug mode (rng_debug_enable input is set)."]
pub mod ehrdata5;
#[doc = "RNDSOURCEENABLE (rw) register accessor: This register holds the enable signal for the random source.\n\nYou can [`read`](crate::Reg::read) this register and get [`rndsourceenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rndsourceenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rndsourceenable`]
module"]
#[doc(alias = "RNDSOURCEENABLE")]
pub type Rndsourceenable = crate::Reg<rndsourceenable::RndsourceenableSpec>;
#[doc = "This register holds the enable signal for the random source."]
pub mod rndsourceenable;
#[doc = "SAMPLECNT1 (rw) register accessor: Counts clocks between sampling of random bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`samplecnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samplecnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samplecnt1`]
module"]
#[doc(alias = "SAMPLECNT1")]
pub type Samplecnt1 = crate::Reg<samplecnt1::Samplecnt1Spec>;
#[doc = "Counts clocks between sampling of random bit."]
pub mod samplecnt1;
#[doc = "AUTOCORRSTATISTIC (rw) register accessor: Statistics about autocorrelation test activations.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorrstatistic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorrstatistic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocorrstatistic`]
module"]
#[doc(alias = "AUTOCORRSTATISTIC")]
pub type Autocorrstatistic = crate::Reg<autocorrstatistic::AutocorrstatisticSpec>;
#[doc = "Statistics about autocorrelation test activations."]
pub mod autocorrstatistic;
#[doc = "TRNGDEBUGCONTROL (rw) register accessor: This register is used to debug the TRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`trngdebugcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngdebugcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trngdebugcontrol`]
module"]
#[doc(alias = "TRNGDEBUGCONTROL")]
pub type Trngdebugcontrol = crate::Reg<trngdebugcontrol::TrngdebugcontrolSpec>;
#[doc = "This register is used to debug the TRNG"]
pub mod trngdebugcontrol;
#[doc = "RNGSWRESET (rw) register accessor: Generate SW reset solely to RNG block.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngswreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngswreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngswreset`]
module"]
#[doc(alias = "RNGSWRESET")]
pub type Rngswreset = crate::Reg<rngswreset::RngswresetSpec>;
#[doc = "Generate SW reset solely to RNG block."]
pub mod rngswreset;
#[doc = "RNGDEBUGENINPUT (rw) register accessor: Defines the RNG in debug mode\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdebugeninput::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdebugeninput::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngdebugeninput`]
module"]
#[doc(alias = "RNGDEBUGENINPUT")]
pub type Rngdebugeninput = crate::Reg<rngdebugeninput::RngdebugeninputSpec>;
#[doc = "Defines the RNG in debug mode"]
pub mod rngdebugeninput;
#[doc = "RNGBUSY (rw) register accessor: RNG busy indication\n\nYou can [`read`](crate::Reg::read) this register and get [`rngbusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngbusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngbusy`]
module"]
#[doc(alias = "RNGBUSY")]
pub type Rngbusy = crate::Reg<rngbusy::RngbusySpec>;
#[doc = "RNG busy indication"]
pub mod rngbusy;
#[doc = "RSTBITSCOUNTER (rw) register accessor: Resets the counter of collected bits in the TRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`rstbitscounter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstbitscounter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstbitscounter`]
module"]
#[doc(alias = "RSTBITSCOUNTER")]
pub type Rstbitscounter = crate::Reg<rstbitscounter::RstbitscounterSpec>;
#[doc = "Resets the counter of collected bits in the TRNG"]
pub mod rstbitscounter;
#[doc = "RNGVERSION (rw) register accessor: This register holds the RNG version\n\nYou can [`read`](crate::Reg::read) this register and get [`rngversion::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngversion::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngversion`]
module"]
#[doc(alias = "RNGVERSION")]
pub type Rngversion = crate::Reg<rngversion::RngversionSpec>;
#[doc = "This register holds the RNG version"]
pub mod rngversion;
#[doc = "RNGCLKENABLE (rw) register accessor: Writing to this register enables_disables the RNG clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngclkenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngclkenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngclkenable`]
module"]
#[doc(alias = "RNGCLKENABLE")]
pub type Rngclkenable = crate::Reg<rngclkenable::RngclkenableSpec>;
#[doc = "Writing to this register enables_disables the RNG clock."]
pub mod rngclkenable;
#[doc = "RNGDMAENABLE (rw) register accessor: Writing to this register enables_disables the RNG DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmaenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmaenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngdmaenable`]
module"]
#[doc(alias = "RNGDMAENABLE")]
pub type Rngdmaenable = crate::Reg<rngdmaenable::RngdmaenableSpec>;
#[doc = "Writing to this register enables_disables the RNG DMA."]
pub mod rngdmaenable;
#[doc = "RNGDMASRCMASK (rw) register accessor: This register defines which ring-oscillator length should be used when using the RNG DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmasrcmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmasrcmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngdmasrcmask`]
module"]
#[doc(alias = "RNGDMASRCMASK")]
pub type Rngdmasrcmask = crate::Reg<rngdmasrcmask::RngdmasrcmaskSpec>;
#[doc = "This register defines which ring-oscillator length should be used when using the RNG DMA."]
pub mod rngdmasrcmask;
#[doc = "RNGDMASRAMADDR (rw) register accessor: This register defines the start address of the DMA for the TRNG data.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmasramaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmasramaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngdmasramaddr`]
module"]
#[doc(alias = "RNGDMASRAMADDR")]
pub type Rngdmasramaddr = crate::Reg<rngdmasramaddr::RngdmasramaddrSpec>;
#[doc = "This register defines the start address of the DMA for the TRNG data."]
pub mod rngdmasramaddr;
#[doc = "RNGWATCHDOGVAL (rw) register accessor: This register defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 7:0 RNG_SAMPLES_NUM rw 0x0 Defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 31:8 RESERVED rw 0x0 ReservedThis register defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngwatchdogval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngwatchdogval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngwatchdogval`]
module"]
#[doc(alias = "RNGWATCHDOGVAL")]
pub type Rngwatchdogval = crate::Reg<rngwatchdogval::RngwatchdogvalSpec>;
#[doc = "This register defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 7:0 RNG_SAMPLES_NUM rw 0x0 Defines the number of 192-bits samples that the DMA collects per RNG configuration.bitfield 31:8 RESERVED rw 0x0 ReservedThis register defines the maximum number of clock cycles per TRNG collection of 192 samples. If the number of cycles for a collection exceeds this threshold, TRNG signals an interrupt."]
pub mod rngwatchdogval;
#[doc = "RNGDMASTATUS (rw) register accessor: This register holds the RNG DMA status.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmastatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmastatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngdmastatus`]
module"]
#[doc(alias = "RNGDMASTATUS")]
pub type Rngdmastatus = crate::Reg<rngdmastatus::RngdmastatusSpec>;
#[doc = "This register holds the RNG DMA status."]
pub mod rngdmastatus;
#[doc = "CHACHACONTROLREG (rw) register accessor: CHACHA general configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachacontrolreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachacontrolreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachacontrolreg`]
module"]
#[doc(alias = "CHACHACONTROLREG")]
pub type Chachacontrolreg = crate::Reg<chachacontrolreg::ChachacontrolregSpec>;
#[doc = "CHACHA general configuration."]
pub mod chachacontrolreg;
#[doc = "CHACHAVERSION (rw) register accessor: CHACHA Version\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaversion::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaversion::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaversion`]
module"]
#[doc(alias = "CHACHAVERSION")]
pub type Chachaversion = crate::Reg<chachaversion::ChachaversionSpec>;
#[doc = "CHACHA Version"]
pub mod chachaversion;
#[doc = "CHACHAKEY0 (rw) register accessor: bits 255:224 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey0`]
module"]
#[doc(alias = "CHACHAKEY0")]
pub type Chachakey0 = crate::Reg<chachakey0::Chachakey0Spec>;
#[doc = "bits 255:224 of CHACHA Key"]
pub mod chachakey0;
#[doc = "CHACHAKEY1 (rw) register accessor: bits 223:192 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey1`]
module"]
#[doc(alias = "CHACHAKEY1")]
pub type Chachakey1 = crate::Reg<chachakey1::Chachakey1Spec>;
#[doc = "bits 223:192 of CHACHA Key"]
pub mod chachakey1;
#[doc = "CHACHAKEY2 (rw) register accessor: bits 191:160 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey2`]
module"]
#[doc(alias = "CHACHAKEY2")]
pub type Chachakey2 = crate::Reg<chachakey2::Chachakey2Spec>;
#[doc = "bits 191:160 of CHACHA Key"]
pub mod chachakey2;
#[doc = "CHACHAKEY3 (rw) register accessor: bits159:128 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey3`]
module"]
#[doc(alias = "CHACHAKEY3")]
pub type Chachakey3 = crate::Reg<chachakey3::Chachakey3Spec>;
#[doc = "bits159:128 of CHACHA Key"]
pub mod chachakey3;
#[doc = "CHACHAKEY4 (rw) register accessor: bits 127:96 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey4`]
module"]
#[doc(alias = "CHACHAKEY4")]
pub type Chachakey4 = crate::Reg<chachakey4::Chachakey4Spec>;
#[doc = "bits 127:96 of CHACHA Key"]
pub mod chachakey4;
#[doc = "CHACHAKEY5 (rw) register accessor: bits 95:64 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey5`]
module"]
#[doc(alias = "CHACHAKEY5")]
pub type Chachakey5 = crate::Reg<chachakey5::Chachakey5Spec>;
#[doc = "bits 95:64 of CHACHA Key"]
pub mod chachakey5;
#[doc = "CHACHAKEY6 (rw) register accessor: bits 63:32 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey6`]
module"]
#[doc(alias = "CHACHAKEY6")]
pub type Chachakey6 = crate::Reg<chachakey6::Chachakey6Spec>;
#[doc = "bits 63:32 of CHACHA Key"]
pub mod chachakey6;
#[doc = "CHACHAKEY7 (rw) register accessor: bits 31:0 of CHACHA Key\n\nYou can [`read`](crate::Reg::read) this register and get [`chachakey7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachakey7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachakey7`]
module"]
#[doc(alias = "CHACHAKEY7")]
pub type Chachakey7 = crate::Reg<chachakey7::Chachakey7Spec>;
#[doc = "bits 31:0 of CHACHA Key"]
pub mod chachakey7;
#[doc = "CHACHAIV0 (rw) register accessor: bits 31:0 of CHACHA_IV0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaiv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaiv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaiv0`]
module"]
#[doc(alias = "CHACHAIV0")]
pub type Chachaiv0 = crate::Reg<chachaiv0::Chachaiv0Spec>;
#[doc = "bits 31:0 of CHACHA_IV0 register"]
pub mod chachaiv0;
#[doc = "CHACHAIV1 (rw) register accessor: bits 31:0 of CHACHA_IV1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaiv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaiv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaiv1`]
module"]
#[doc(alias = "CHACHAIV1")]
pub type Chachaiv1 = crate::Reg<chachaiv1::Chachaiv1Spec>;
#[doc = "bits 31:0 of CHACHA_IV1 register"]
pub mod chachaiv1;
#[doc = "CHACHABUSY (rw) register accessor: This register is set when the CHACHA_SALSA core is active\n\nYou can [`read`](crate::Reg::read) this register and get [`chachabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachabusy`]
module"]
#[doc(alias = "CHACHABUSY")]
pub type Chachabusy = crate::Reg<chachabusy::ChachabusySpec>;
#[doc = "This register is set when the CHACHA_SALSA core is active"]
pub mod chachabusy;
#[doc = "CHACHAHWFLAGS (rw) register accessor: This register holds the pre-synthesis HW flag configuration of the CHACHA_SALSA engine\n\nYou can [`read`](crate::Reg::read) this register and get [`chachahwflags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachahwflags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachahwflags`]
module"]
#[doc(alias = "CHACHAHWFLAGS")]
pub type Chachahwflags = crate::Reg<chachahwflags::ChachahwflagsSpec>;
#[doc = "This register holds the pre-synthesis HW flag configuration of the CHACHA_SALSA engine"]
pub mod chachahwflags;
#[doc = "CHACHABLOCKCNTLSB (rw) register accessor: The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachablockcntlsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachablockcntlsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachablockcntlsb`]
module"]
#[doc(alias = "CHACHABLOCKCNTLSB")]
pub type Chachablockcntlsb = crate::Reg<chachablockcntlsb::ChachablockcntlsbSpec>;
#[doc = "The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message."]
pub mod chachablockcntlsb;
#[doc = "CHACHABLOCKCNTMSB (rw) register accessor: The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachablockcntmsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachablockcntmsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachablockcntmsb`]
module"]
#[doc(alias = "CHACHABLOCKCNTMSB")]
pub type Chachablockcntmsb = crate::Reg<chachablockcntmsb::ChachablockcntmsbSpec>;
#[doc = "The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message."]
pub mod chachablockcntmsb;
#[doc = "CHACHASWRESET (rw) register accessor: Resets CHACHA_SALSA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaswreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaswreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaswreset`]
module"]
#[doc(alias = "CHACHASWRESET")]
pub type Chachaswreset = crate::Reg<chachaswreset::ChachaswresetSpec>;
#[doc = "Resets CHACHA_SALSA engine."]
pub mod chachaswreset;
#[doc = "CHACHAFORPOLYKEY0 (rw) register accessor: bits 255:224 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey0`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY0")]
pub type Chachaforpolykey0 = crate::Reg<chachaforpolykey0::Chachaforpolykey0Spec>;
#[doc = "bits 255:224 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey0;
#[doc = "CHACHAFORPOLYKEY1 (rw) register accessor: bits 223:192 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey1`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY1")]
pub type Chachaforpolykey1 = crate::Reg<chachaforpolykey1::Chachaforpolykey1Spec>;
#[doc = "bits 223:192 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey1;
#[doc = "CHACHAFORPOLYKEY2 (rw) register accessor: bits191:160 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey2`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY2")]
pub type Chachaforpolykey2 = crate::Reg<chachaforpolykey2::Chachaforpolykey2Spec>;
#[doc = "bits191:160 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey2;
#[doc = "CHACHAFORPOLYKEY3 (rw) register accessor: bits159:128 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey3`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY3")]
pub type Chachaforpolykey3 = crate::Reg<chachaforpolykey3::Chachaforpolykey3Spec>;
#[doc = "bits159:128 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey3;
#[doc = "CHACHAFORPOLYKEY4 (rw) register accessor: bits 127:96 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey4`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY4")]
pub type Chachaforpolykey4 = crate::Reg<chachaforpolykey4::Chachaforpolykey4Spec>;
#[doc = "bits 127:96 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey4;
#[doc = "CHACHAFORPOLYKEY5 (rw) register accessor: bits 95:64 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey5`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY5")]
pub type Chachaforpolykey5 = crate::Reg<chachaforpolykey5::Chachaforpolykey5Spec>;
#[doc = "bits 95:64 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey5;
#[doc = "CHACHAFORPOLYKEY6 (rw) register accessor: bits 63:32 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey6`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY6")]
pub type Chachaforpolykey6 = crate::Reg<chachaforpolykey6::Chachaforpolykey6Spec>;
#[doc = "bits 63:32 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey6;
#[doc = "CHACHAFORPOLYKEY7 (rw) register accessor: bits 31:0 of CHACHA_FOR_POLY_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaforpolykey7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaforpolykey7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaforpolykey7`]
module"]
#[doc(alias = "CHACHAFORPOLYKEY7")]
pub type Chachaforpolykey7 = crate::Reg<chachaforpolykey7::Chachaforpolykey7Spec>;
#[doc = "bits 31:0 of CHACHA_FOR_POLY_KEY"]
pub mod chachaforpolykey7;
#[doc = "CHACHABYTEWORDORDERCNTLREG (rw) register accessor: CHACHA_SALSA DATA ORDER configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachabytewordordercntlreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachabytewordordercntlreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachabytewordordercntlreg`]
module"]
#[doc(alias = "CHACHABYTEWORDORDERCNTLREG")]
pub type Chachabytewordordercntlreg =
    crate::Reg<chachabytewordordercntlreg::ChachabytewordordercntlregSpec>;
#[doc = "CHACHA_SALSA DATA ORDER configuration."]
pub mod chachabytewordordercntlreg;
#[doc = "CHACHADEBUGREG (rw) register accessor: This register is used to debug the CHACHA engine\n\nYou can [`read`](crate::Reg::read) this register and get [`chachadebugreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachadebugreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachadebugreg`]
module"]
#[doc(alias = "CHACHADEBUGREG")]
pub type Chachadebugreg = crate::Reg<chachadebugreg::ChachadebugregSpec>;
#[doc = "This register is used to debug the CHACHA engine"]
pub mod chachadebugreg;
#[doc = "AESKEY00 (rw) register accessor: bits 31:0 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey00`]
module"]
#[doc(alias = "AESKEY00")]
pub type Aeskey00 = crate::Reg<aeskey00::Aeskey00Spec>;
#[doc = "bits 31:0 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey00;
#[doc = "AESKEY01 (rw) register accessor: bits 63:32 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey01`]
module"]
#[doc(alias = "AESKEY01")]
pub type Aeskey01 = crate::Reg<aeskey01::Aeskey01Spec>;
#[doc = "bits 63:32 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey01;
#[doc = "AESKEY02 (rw) register accessor: bits 95:64 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey02`]
module"]
#[doc(alias = "AESKEY02")]
pub type Aeskey02 = crate::Reg<aeskey02::Aeskey02Spec>;
#[doc = "bits 95:64 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey02;
#[doc = "AESKEY03 (rw) register accessor: bits 127:96 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey03`]
module"]
#[doc(alias = "AESKEY03")]
pub type Aeskey03 = crate::Reg<aeskey03::Aeskey03Spec>;
#[doc = "bits 127:96 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey03;
#[doc = "AESKEY04 (rw) register accessor: bits 159:128 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey04`]
module"]
#[doc(alias = "AESKEY04")]
pub type Aeskey04 = crate::Reg<aeskey04::Aeskey04Spec>;
#[doc = "bits 159:128 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey04;
#[doc = "AESKEY05 (rw) register accessor: bits 191:160 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey05`]
module"]
#[doc(alias = "AESKEY05")]
pub type Aeskey05 = crate::Reg<aeskey05::Aeskey05Spec>;
#[doc = "bits 191:160 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey05;
#[doc = "AESKEY06 (rw) register accessor: bits 223:192 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey06`]
module"]
#[doc(alias = "AESKEY06")]
pub type Aeskey06 = crate::Reg<aeskey06::Aeskey06Spec>;
#[doc = "bits 223:192 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey06;
#[doc = "AESKEY07 (rw) register accessor: bits 255:224 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey07`]
module"]
#[doc(alias = "AESKEY07")]
pub type Aeskey07 = crate::Reg<aeskey07::Aeskey07Spec>;
#[doc = "bits 255:224 of AES Key0 (used as the AES key in non-tunneling operations, and as the first tunnel stage key in tunneling operations)."]
pub mod aeskey07;
#[doc = "AESKEY10 (rw) register accessor: bits 31:0 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey10`]
module"]
#[doc(alias = "AESKEY10")]
pub type Aeskey10 = crate::Reg<aeskey10::Aeskey10Spec>;
#[doc = "bits 31:0 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey10;
#[doc = "AESKEY11 (rw) register accessor: bits 63:32 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey11`]
module"]
#[doc(alias = "AESKEY11")]
pub type Aeskey11 = crate::Reg<aeskey11::Aeskey11Spec>;
#[doc = "bits 63:32 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey11;
#[doc = "AESKEY12 (rw) register accessor: bits 95:64 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey12`]
module"]
#[doc(alias = "AESKEY12")]
pub type Aeskey12 = crate::Reg<aeskey12::Aeskey12Spec>;
#[doc = "bits 95:64 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey12;
#[doc = "AESKEY13 (rw) register accessor: bits 127:96 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey13`]
module"]
#[doc(alias = "AESKEY13")]
pub type Aeskey13 = crate::Reg<aeskey13::Aeskey13Spec>;
#[doc = "bits 127:96 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey13;
#[doc = "AESKEY14 (rw) register accessor: bits 159:128 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey14`]
module"]
#[doc(alias = "AESKEY14")]
pub type Aeskey14 = crate::Reg<aeskey14::Aeskey14Spec>;
#[doc = "bits 159:128 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey14;
#[doc = "AESKEY15 (rw) register accessor: bits 191:160 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey15`]
module"]
#[doc(alias = "AESKEY15")]
pub type Aeskey15 = crate::Reg<aeskey15::Aeskey15Spec>;
#[doc = "bits 191:160 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey15;
#[doc = "AESKEY16 (rw) register accessor: bits 223:192 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey16`]
module"]
#[doc(alias = "AESKEY16")]
pub type Aeskey16 = crate::Reg<aeskey16::Aeskey16Spec>;
#[doc = "bits 223:192 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey16;
#[doc = "AESKEY17 (rw) register accessor: bits 255:224 of AES Key1 (used as the second AES tunnel stage key in tunneling operations).\n\nYou can [`read`](crate::Reg::read) this register and get [`aeskey17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeskey17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey17`]
module"]
#[doc(alias = "AESKEY17")]
pub type Aeskey17 = crate::Reg<aeskey17::Aeskey17Spec>;
#[doc = "bits 255:224 of AES Key1 (used as the second AES tunnel stage key in tunneling operations)."]
pub mod aeskey17;
#[doc = "AESIV00 (rw) register accessor: bits 31:0 of AES_IV0 register. AES IV0 is used as the AES IV (Initialization Value) register in non-tunneling operations,and as the first tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW).\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv00`]
module"]
#[doc(alias = "AESIV00")]
pub type Aesiv00 = crate::Reg<aesiv00::Aesiv00Spec>;
#[doc = "bits 31:0 of AES_IV0 register. AES IV0 is used as the AES IV (Initialization Value) register in non-tunneling operations,and as the first tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW)."]
pub mod aesiv00;
#[doc = "AESIV01 (rw) register accessor: bits 63:32 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv01`]
module"]
#[doc(alias = "AESIV01")]
pub type Aesiv01 = crate::Reg<aesiv01::Aesiv01Spec>;
#[doc = "bits 63:32 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description"]
pub mod aesiv01;
#[doc = "AESIV02 (rw) register accessor: bits 95:64 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv02`]
module"]
#[doc(alias = "AESIV02")]
pub type Aesiv02 = crate::Reg<aesiv02::Aesiv02Spec>;
#[doc = "bits 95:64 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description"]
pub mod aesiv02;
#[doc = "AESIV03 (rw) register accessor: bits 127:96 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv03`]
module"]
#[doc(alias = "AESIV03")]
pub type Aesiv03 = crate::Reg<aesiv03::Aesiv03Spec>;
#[doc = "bits 127:96 of AES_IV0 128b register.For the description of AES_IV0, see the AES_IV_0_0 register description"]
pub mod aesiv03;
#[doc = "AESIV10 (rw) register accessor: bits 31:0 of AES_IV1 128b register.AES IV1 is used as the AES IV (Initialization Value) register as the second tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv10`]
module"]
#[doc(alias = "AESIV10")]
pub type Aesiv10 = crate::Reg<aesiv10::Aesiv10Spec>;
#[doc = "bits 31:0 of AES_IV1 128b register.AES IV1 is used as the AES IV (Initialization Value) register as the second tunnel stage IV register in tunneling operations.The IV register should be loaded according to the AES mode:in AES CBC_CBC-MAC - the AES IV register should be loaded with the IV (initialization vector).in XTS-AES - the AES IV register should be loaded with the T value (unless the HW T calculation mode is active, in which the T value is calculated by the HW."]
pub mod aesiv10;
#[doc = "AESIV11 (rw) register accessor: bits 63:32 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv11`]
module"]
#[doc(alias = "AESIV11")]
pub type Aesiv11 = crate::Reg<aesiv11::Aesiv11Spec>;
#[doc = "bits 63:32 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description"]
pub mod aesiv11;
#[doc = "AESIV12 (rw) register accessor: bits 95:64 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv12`]
module"]
#[doc(alias = "AESIV12")]
pub type Aesiv12 = crate::Reg<aesiv12::Aesiv12Spec>;
#[doc = "bits 95:64 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description"]
pub mod aesiv12;
#[doc = "AESIV13 (rw) register accessor: bits 127:96 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description\n\nYou can [`read`](crate::Reg::read) this register and get [`aesiv13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesiv13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv13`]
module"]
#[doc(alias = "AESIV13")]
pub type Aesiv13 = crate::Reg<aesiv13::Aesiv13Spec>;
#[doc = "bits 127:96 of AES_IV1 128b register.For the description of AES_IV1, see the AES_IV_1_0 register description"]
pub mod aesiv13;
#[doc = "AESCTR00 (rw) register accessor: bits 31:0 of AES_CTR0 128b register.AES CTR0 is used as the AES CTR (counter) register in non-tunneling operations, and as the first tunnel stage CTR register in tunneling operations.The CTR register should be loaded according to the AES mode:in AES CTR_GCTR - the AES CTR register should be loaded with the counter value.in XTS-AES - the AES CTR register should be loaded with the i value (in order to calculate the T value from it, if HW T calculation is supported).\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesctr00`]
module"]
#[doc(alias = "AESCTR00")]
pub type Aesctr00 = crate::Reg<aesctr00::Aesctr00Spec>;
#[doc = "bits 31:0 of AES_CTR0 128b register.AES CTR0 is used as the AES CTR (counter) register in non-tunneling operations, and as the first tunnel stage CTR register in tunneling operations.The CTR register should be loaded according to the AES mode:in AES CTR_GCTR - the AES CTR register should be loaded with the counter value.in XTS-AES - the AES CTR register should be loaded with the i value (in order to calculate the T value from it, if HW T calculation is supported)."]
pub mod aesctr00;
#[doc = "AESCTR01 (rw) register accessor: bits 63:32 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesctr01`]
module"]
#[doc(alias = "AESCTR01")]
pub type Aesctr01 = crate::Reg<aesctr01::Aesctr01Spec>;
#[doc = "bits 63:32 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description."]
pub mod aesctr01;
#[doc = "AESCTR02 (rw) register accessor: bits 95:64 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesctr02`]
module"]
#[doc(alias = "AESCTR02")]
pub type Aesctr02 = crate::Reg<aesctr02::Aesctr02Spec>;
#[doc = "bits 95:64 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description."]
pub mod aesctr02;
#[doc = "AESCTR03 (rw) register accessor: bits 127:96 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctr03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctr03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesctr03`]
module"]
#[doc(alias = "AESCTR03")]
pub type Aesctr03 = crate::Reg<aesctr03::Aesctr03Spec>;
#[doc = "bits 127:96 of AES_CTR0 128b register.For the description of AES_CTR0, see the AES_CTR_0_0 register description."]
pub mod aesctr03;
#[doc = "AESBUSY (rw) register accessor: This register is set when the AES core is active\n\nYou can [`read`](crate::Reg::read) this register and get [`aesbusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesbusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesbusy`]
module"]
#[doc(alias = "AESBUSY")]
pub type Aesbusy = crate::Reg<aesbusy::AesbusySpec>;
#[doc = "This register is set when the AES core is active"]
pub mod aesbusy;
#[doc = "AESSK (rw) register accessor: writing to this address causes sampling of the HW key to the AES_KEY0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`aessk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aessk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aessk`]
module"]
#[doc(alias = "AESSK")]
pub type Aessk = crate::Reg<aessk::AesskSpec>;
#[doc = "writing to this address causes sampling of the HW key to the AES_KEY0 register"]
pub mod aessk;
#[doc = "AESCMACINIT (rw) register accessor: Writing to this address triggers the AES engine generating of K1 and K2 for AES CMAC operations. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aescmacinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aescmacinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aescmacinit`]
module"]
#[doc(alias = "AESCMACINIT")]
pub type Aescmacinit = crate::Reg<aescmacinit::AescmacinitSpec>;
#[doc = "Writing to this address triggers the AES engine generating of K1 and K2 for AES CMAC operations. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod aescmacinit;
#[doc = "AESSK1 (rw) register accessor: writing to this address causes sampling of the HW key to the AES_KEY1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`aessk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aessk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aessk1`]
module"]
#[doc(alias = "AESSK1")]
pub type Aessk1 = crate::Reg<aessk1::Aessk1Spec>;
#[doc = "writing to this address causes sampling of the HW key to the AES_KEY1 register"]
pub mod aessk1;
#[doc = "AESREMAININGBYTES (rw) register accessor: This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesremainingbytes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesremainingbytes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesremainingbytes`]
module"]
#[doc(alias = "AESREMAININGBYTES")]
pub type Aesremainingbytes = crate::Reg<aesremainingbytes::AesremainingbytesSpec>;
#[doc = "This register should be set with the amount of remaining bytes until the end of the current AES operation. The AES engine counts down from this value to determine the last _ one before last blocks in AES CMAC, XTS AES and AES CCM."]
pub mod aesremainingbytes;
#[doc = "AESCONTROL (rw) register accessor: This register holds the configuration of the AES engine. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aescontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aescontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aescontrol`]
module"]
#[doc(alias = "AESCONTROL")]
pub type Aescontrol = crate::Reg<aescontrol::AescontrolSpec>;
#[doc = "This register holds the configuration of the AES engine. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod aescontrol;
#[doc = "AESHWFLAGS (rw) register accessor: This register holds the pre-synthesis HW flag configuration of the AES engine\n\nYou can [`read`](crate::Reg::read) this register and get [`aeshwflags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aeshwflags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeshwflags`]
module"]
#[doc(alias = "AESHWFLAGS")]
pub type Aeshwflags = crate::Reg<aeshwflags::AeshwflagsSpec>;
#[doc = "This register holds the pre-synthesis HW flag configuration of the AES engine"]
pub mod aeshwflags;
#[doc = "AESCTRNOINCREMENT (rw) register accessor: This register enables the AES CTR no increment mode (in which the counter mode is not incremented between 2 blocks)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctrnoincrement::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctrnoincrement::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesctrnoincrement`]
module"]
#[doc(alias = "AESCTRNOINCREMENT")]
pub type Aesctrnoincrement = crate::Reg<aesctrnoincrement::AesctrnoincrementSpec>;
#[doc = "This register enables the AES CTR no increment mode (in which the counter mode is not incremented between 2 blocks)"]
pub mod aesctrnoincrement;
#[doc = "AESDFAISON (rw) register accessor: This register disable_enable the AES dfa. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesdfaison::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesdfaison::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdfaison`]
module"]
#[doc(alias = "AESDFAISON")]
pub type Aesdfaison = crate::Reg<aesdfaison::AesdfaisonSpec>;
#[doc = "This register disable_enable the AES dfa. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod aesdfaison;
#[doc = "AESDFAERRSTATUS (rw) register accessor: dfa error status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesdfaerrstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesdfaerrstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdfaerrstatus`]
module"]
#[doc(alias = "AESDFAERRSTATUS")]
pub type Aesdfaerrstatus = crate::Reg<aesdfaerrstatus::AesdfaerrstatusSpec>;
#[doc = "dfa error status register."]
pub mod aesdfaerrstatus;
#[doc = "AESCMACSIZE0KICK (rw) register accessor: writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`aescmacsize0kick::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aescmacsize0kick::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aescmacsize0kick`]
module"]
#[doc(alias = "AESCMACSIZE0KICK")]
pub type Aescmacsize0kick = crate::Reg<aescmacsize0kick::Aescmacsize0kickSpec>;
#[doc = "writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV0 register."]
pub mod aescmacsize0kick;
#[doc = "HASHH0 (rw) register accessor: H0 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh0`]
module"]
#[doc(alias = "HASHH0")]
pub type Hashh0 = crate::Reg<hashh0::Hashh0Spec>;
#[doc = "H0 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
pub mod hashh0;
#[doc = "HASHH1 (rw) register accessor: H1 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh1`]
module"]
#[doc(alias = "HASHH1")]
pub type Hashh1 = crate::Reg<hashh1::Hashh1Spec>;
#[doc = "H1 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
pub mod hashh1;
#[doc = "HASHH2 (rw) register accessor: H2 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh2`]
module"]
#[doc(alias = "HASHH2")]
pub type Hashh2 = crate::Reg<hashh2::Hashh2Spec>;
#[doc = "H2 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
pub mod hashh2;
#[doc = "HASHH3 (rw) register accessor: H3 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh3`]
module"]
#[doc(alias = "HASHH3")]
pub type Hashh3 = crate::Reg<hashh3::Hashh3Spec>;
#[doc = "H3 data. can only be written in the following HASH_CONTROL modes: MD5 SHA1 SHA224 SHA256 SHA384 SHA512"]
pub mod hashh3;
#[doc = "HASHH4 (rw) register accessor: H4 data. can only be written in the following HASH_CONTROL modes: SHA1 SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh4`]
module"]
#[doc(alias = "HASHH4")]
pub type Hashh4 = crate::Reg<hashh4::Hashh4Spec>;
#[doc = "H4 data. can only be written in the following HASH_CONTROL modes: SHA1 SHA224 SHA256 SHA384 SHA512"]
pub mod hashh4;
#[doc = "HASHH5 (rw) register accessor: H5 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh5`]
module"]
#[doc(alias = "HASHH5")]
pub type Hashh5 = crate::Reg<hashh5::Hashh5Spec>;
#[doc = "H5 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512"]
pub mod hashh5;
#[doc = "HASHH6 (rw) register accessor: H6 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh6`]
module"]
#[doc(alias = "HASHH6")]
pub type Hashh6 = crate::Reg<hashh6::Hashh6Spec>;
#[doc = "H6 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512"]
pub mod hashh6;
#[doc = "HASHH7 (rw) register accessor: H7 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh7`]
module"]
#[doc(alias = "HASHH7")]
pub type Hashh7 = crate::Reg<hashh7::Hashh7Spec>;
#[doc = "H7 data. can only be written in the following HASH_CONTROL modes: SHA224 SHA256 SHA384 SHA512"]
pub mod hashh7;
#[doc = "HASHH8 (rw) register accessor: H8 data. can only be written in the following HASH_CONTROL modes: SHA384 SHA512\n\nYou can [`read`](crate::Reg::read) this register and get [`hashh8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashh8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashh8`]
module"]
#[doc(alias = "HASHH8")]
pub type Hashh8 = crate::Reg<hashh8::Hashh8Spec>;
#[doc = "H8 data. can only be written in the following HASH_CONTROL modes: SHA384 SHA512"]
pub mod hashh8;
#[doc = "AUTOHWPADDING (rw) register accessor: HW padding automatically activated by engine. For the special case of ZERO bytes data vector this register should not be used! instead use HASH_PAD_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`autohwpadding::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autohwpadding::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autohwpadding`]
module"]
#[doc(alias = "AUTOHWPADDING")]
pub type Autohwpadding = crate::Reg<autohwpadding::AutohwpaddingSpec>;
#[doc = "HW padding automatically activated by engine. For the special case of ZERO bytes data vector this register should not be used! instead use HASH_PAD_CFG"]
pub mod autohwpadding;
#[doc = "HASHXORDIN (rw) register accessor: This register is always xored with the input to the hash engine,it should be 0 if xored is not reqiured .\n\nYou can [`read`](crate::Reg::read) this register and get [`hashxordin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashxordin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashxordin`]
module"]
#[doc(alias = "HASHXORDIN")]
pub type Hashxordin = crate::Reg<hashxordin::HashxordinSpec>;
#[doc = "This register is always xored with the input to the hash engine,it should be 0 if xored is not reqiured ."]
pub mod hashxordin;
#[doc = "LOADINITSTATE (rw) register accessor: Indication to HASH that the following data is to be loaded into initial value registers in HASH(H0:H15) or IV to AES MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`loadinitstate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadinitstate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loadinitstate`]
module"]
#[doc(alias = "LOADINITSTATE")]
pub type Loadinitstate = crate::Reg<loadinitstate::LoadinitstateSpec>;
#[doc = "Indication to HASH that the following data is to be loaded into initial value registers in HASH(H0:H15) or IV to AES MAC"]
pub mod loadinitstate;
#[doc = "HASHSELAESMAC (rw) register accessor: select the AES MAC module rather than the hash module\n\nYou can [`read`](crate::Reg::read) this register and get [`hashselaesmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashselaesmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashselaesmac`]
module"]
#[doc(alias = "HASHSELAESMAC")]
pub type Hashselaesmac = crate::Reg<hashselaesmac::HashselaesmacSpec>;
#[doc = "select the AES MAC module rather than the hash module"]
pub mod hashselaesmac;
#[doc = "HASHVERSION (rw) register accessor: HASH VERSION Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hashversion::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashversion::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashversion`]
module"]
#[doc(alias = "HASHVERSION")]
pub type Hashversion = crate::Reg<hashversion::HashversionSpec>;
#[doc = "HASH VERSION Register"]
pub mod hashversion;
#[doc = "HASHCONTROL (rw) register accessor: Selects which HASH mode to run\n\nYou can [`read`](crate::Reg::read) this register and get [`hashcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashcontrol`]
module"]
#[doc(alias = "HASHCONTROL")]
pub type Hashcontrol = crate::Reg<hashcontrol::HashcontrolSpec>;
#[doc = "Selects which HASH mode to run"]
pub mod hashcontrol;
#[doc = "HASHPADEN (rw) register accessor: Enables the hash hw padding.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashpaden::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashpaden::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashpaden`]
module"]
#[doc(alias = "HASHPADEN")]
pub type Hashpaden = crate::Reg<hashpaden::HashpadenSpec>;
#[doc = "Enables the hash hw padding."]
pub mod hashpaden;
#[doc = "HASHPADCFG (rw) register accessor: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashpadcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashpadcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashpadcfg`]
module"]
#[doc(alias = "HASHPADCFG")]
pub type Hashpadcfg = crate::Reg<hashpadcfg::HashpadcfgSpec>;
#[doc = "This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hashpadcfg;
#[doc = "HASHCURLEN0 (rw) register accessor: This register holds the length of current hash operation bit 31:0.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashcurlen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashcurlen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashcurlen0`]
module"]
#[doc(alias = "HASHCURLEN0")]
pub type Hashcurlen0 = crate::Reg<hashcurlen0::Hashcurlen0Spec>;
#[doc = "This register holds the length of current hash operation bit 31:0."]
pub mod hashcurlen0;
#[doc = "HASHCURLEN1 (rw) register accessor: This register holds the length of current hash operation bit 63:32.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashcurlen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashcurlen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashcurlen1`]
module"]
#[doc(alias = "HASHCURLEN1")]
pub type Hashcurlen1 = crate::Reg<hashcurlen1::Hashcurlen1Spec>;
#[doc = "This register holds the length of current hash operation bit 63:32."]
pub mod hashcurlen1;
#[doc = "HASHPARAM (rw) register accessor: HASH_PARAM Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashparam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashparam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashparam`]
module"]
#[doc(alias = "HASHPARAM")]
pub type Hashparam = crate::Reg<hashparam::HashparamSpec>;
#[doc = "HASH_PARAM Register."]
pub mod hashparam;
#[doc = "HASHAESSWRESET (rw) register accessor: Software reset of the AES.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashaesswreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashaesswreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashaesswreset`]
module"]
#[doc(alias = "HASHAESSWRESET")]
pub type Hashaesswreset = crate::Reg<hashaesswreset::HashaesswresetSpec>;
#[doc = "Software reset of the AES."]
pub mod hashaesswreset;
#[doc = "HASHENDIANESS (rw) register accessor: This register holds the HASH_ENDIANESS configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashendianess::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashendianess::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashendianess`]
module"]
#[doc(alias = "HASHENDIANESS")]
pub type Hashendianess = crate::Reg<hashendianess::HashendianessSpec>;
#[doc = "This register holds the HASH_ENDIANESS configuration."]
pub mod hashendianess;
#[doc = "AESCLKENABLE (rw) register accessor: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesclkenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesclkenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesclkenable`]
module"]
#[doc(alias = "AESCLKENABLE")]
pub type Aesclkenable = crate::Reg<aesclkenable::AesclkenableSpec>;
#[doc = "This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod aesclkenable;
#[doc = "HASHCLKENABLE (rw) register accessor: The HASH clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashclkenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashclkenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashclkenable`]
module"]
#[doc(alias = "HASHCLKENABLE")]
pub type Hashclkenable = crate::Reg<hashclkenable::HashclkenableSpec>;
#[doc = "The HASH clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hashclkenable;
#[doc = "PKACLKENABLE (rw) register accessor: The PKA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkaclkenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkaclkenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkaclkenable`]
module"]
#[doc(alias = "PKACLKENABLE")]
pub type Pkaclkenable = crate::Reg<pkaclkenable::PkaclkenableSpec>;
#[doc = "The PKA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod pkaclkenable;
#[doc = "DMACLKENABLE (rw) register accessor: DMA_CLK enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaclkenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaclkenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaclkenable`]
module"]
#[doc(alias = "DMACLKENABLE")]
pub type Dmaclkenable = crate::Reg<dmaclkenable::DmaclkenableSpec>;
#[doc = "DMA_CLK enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod dmaclkenable;
#[doc = "CLKSTATUS (rw) register accessor: The CryptoCell clocks status register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkstatus`]
module"]
#[doc(alias = "CLKSTATUS")]
pub type Clkstatus = crate::Reg<clkstatus::ClkstatusSpec>;
#[doc = "The CryptoCell clocks status register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod clkstatus;
#[doc = "CHACHACLKENABLE (rw) register accessor: CHACHA _SALSA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaclkenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaclkenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chachaclkenable`]
module"]
#[doc(alias = "CHACHACLKENABLE")]
pub type Chachaclkenable = crate::Reg<chachaclkenable::ChachaclkenableSpec>;
#[doc = "CHACHA _SALSA clock enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod chachaclkenable;
#[doc = "CRYPTOCTL (rw) register accessor: Defines the cryptographic flow.\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryptoctl`]
module"]
#[doc(alias = "CRYPTOCTL")]
pub type Cryptoctl = crate::Reg<cryptoctl::CryptoctlSpec>;
#[doc = "Defines the cryptographic flow."]
pub mod cryptoctl;
#[doc = "CRYPTOBUSY (rw) register accessor: This register is set when the cryptographic core is busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptobusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptobusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryptobusy`]
module"]
#[doc(alias = "CRYPTOBUSY")]
pub type Cryptobusy = crate::Reg<cryptobusy::CryptobusySpec>;
#[doc = "This register is set when the cryptographic core is busy."]
pub mod cryptobusy;
#[doc = "HASHBUSY (rw) register accessor: This register is set when the Hash engine is busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashbusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashbusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashbusy`]
module"]
#[doc(alias = "HASHBUSY")]
pub type Hashbusy = crate::Reg<hashbusy::HashbusySpec>;
#[doc = "This register is set when the Hash engine is busy."]
pub mod hashbusy;
#[doc = "CONTEXTID (rw) register accessor: A general RD_WR register. For Firmware use.\n\nYou can [`read`](crate::Reg::read) this register and get [`contextid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`contextid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@contextid`]
module"]
#[doc(alias = "CONTEXTID")]
pub type Contextid = crate::Reg<contextid::ContextidSpec>;
#[doc = "A general RD_WR register. For Firmware use."]
pub mod contextid;
#[doc = "GHASHSUBKEY00 (rw) register accessor: Bits 31:0 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashsubkey00`]
module"]
#[doc(alias = "GHASHSUBKEY00")]
pub type Ghashsubkey00 = crate::Reg<ghashsubkey00::Ghashsubkey00Spec>;
#[doc = "Bits 31:0 of GHASH Key0 (used as the GHASH module key)."]
pub mod ghashsubkey00;
#[doc = "GHASHSUBKEY01 (rw) register accessor: Bits 63:32 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashsubkey01`]
module"]
#[doc(alias = "GHASHSUBKEY01")]
pub type Ghashsubkey01 = crate::Reg<ghashsubkey01::Ghashsubkey01Spec>;
#[doc = "Bits 63:32 of GHASH Key0 (used as the GHASH module key)."]
pub mod ghashsubkey01;
#[doc = "GHASHSUBKEY02 (rw) register accessor: Bits 95:64 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashsubkey02`]
module"]
#[doc(alias = "GHASHSUBKEY02")]
pub type Ghashsubkey02 = crate::Reg<ghashsubkey02::Ghashsubkey02Spec>;
#[doc = "Bits 95:64 of GHASH Key0 (used as the GHASH module key)."]
pub mod ghashsubkey02;
#[doc = "GHASHSUBKEY03 (rw) register accessor: Bits 127:96 of GHASH Key0 (used as the GHASH module key).\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashsubkey03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashsubkey03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashsubkey03`]
module"]
#[doc(alias = "GHASHSUBKEY03")]
pub type Ghashsubkey03 = crate::Reg<ghashsubkey03::Ghashsubkey03Spec>;
#[doc = "Bits 127:96 of GHASH Key0 (used as the GHASH module key)."]
pub mod ghashsubkey03;
#[doc = "GHASHIV00 (rw) register accessor: Bits 31:0 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashiv00`]
module"]
#[doc(alias = "GHASHIV00")]
pub type Ghashiv00 = crate::Reg<ghashiv00::Ghashiv00Spec>;
#[doc = "Bits 31:0 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
pub mod ghashiv00;
#[doc = "GHASHIV01 (rw) register accessor: Bits 63:32 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashiv01`]
module"]
#[doc(alias = "GHASHIV01")]
pub type Ghashiv01 = crate::Reg<ghashiv01::Ghashiv01Spec>;
#[doc = "Bits 63:32 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
pub mod ghashiv01;
#[doc = "GHASHIV02 (rw) register accessor: Bits 95:64 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashiv02`]
module"]
#[doc(alias = "GHASHIV02")]
pub type Ghashiv02 = crate::Reg<ghashiv02::Ghashiv02Spec>;
#[doc = "Bits 95:64 of GHASH_IV0 register. GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
pub mod ghashiv02;
#[doc = "GHASHIV03 (rw) register accessor: Bits 127:96 of GHASH_IV0 register.GHASH IV0 is used as the GHASH IV (Initialization Value) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashiv03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashiv03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashiv03`]
module"]
#[doc(alias = "GHASHIV03")]
pub type Ghashiv03 = crate::Reg<ghashiv03::Ghashiv03Spec>;
#[doc = "Bits 127:96 of GHASH_IV0 register.GHASH IV0 is used as the GHASH IV (Initialization Value) register."]
pub mod ghashiv03;
#[doc = "GHASHBUSY (rw) register accessor: The GHASH module GHASH_BUSY Register. This register is set when the GHASH core is active.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashbusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashbusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashbusy`]
module"]
#[doc(alias = "GHASHBUSY")]
pub type Ghashbusy = crate::Reg<ghashbusy::GhashbusySpec>;
#[doc = "The GHASH module GHASH_BUSY Register. This register is set when the GHASH core is active."]
pub mod ghashbusy;
#[doc = "GHASHINIT (rw) register accessor: Writing to this address sets the GHASH engine to be ready to a new GHASH operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghashinit`]
module"]
#[doc(alias = "GHASHINIT")]
pub type Ghashinit = crate::Reg<ghashinit::GhashinitSpec>;
#[doc = "Writing to this address sets the GHASH engine to be ready to a new GHASH operation."]
pub mod ghashinit;
#[doc = "HOSTRGFIRR (rw) register accessor: The Interrupt Request register. Each bit of this register holds the interrupt status of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfirr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfirr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostrgfirr`]
module"]
#[doc(alias = "HOSTRGFIRR")]
pub type Hostrgfirr = crate::Reg<hostrgfirr::HostrgfirrSpec>;
#[doc = "The Interrupt Request register. Each bit of this register holds the interrupt status of a single interrupt source."]
pub mod hostrgfirr;
#[doc = "HOSTRGFIMR (rw) register accessor: The Interrupt Mask register. Each bit of this register holds the mask of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostrgfimr`]
module"]
#[doc(alias = "HOSTRGFIMR")]
pub type Hostrgfimr = crate::Reg<hostrgfimr::HostrgfimrSpec>;
#[doc = "The Interrupt Mask register. Each bit of this register holds the mask of a single interrupt source."]
pub mod hostrgfimr;
#[doc = "HOSTRGFICR (rw) register accessor: Interrupt Clear Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgficr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgficr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostrgficr`]
module"]
#[doc(alias = "HOSTRGFICR")]
pub type Hostrgficr = crate::Reg<hostrgficr::HostrgficrSpec>;
#[doc = "Interrupt Clear Register."]
pub mod hostrgficr;
#[doc = "HOSTRGFENDIAN (rw) register accessor: This register defines the endianness of the Host-accessible registers. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfendian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfendian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostrgfendian`]
module"]
#[doc(alias = "HOSTRGFENDIAN")]
pub type Hostrgfendian = crate::Reg<hostrgfendian::HostrgfendianSpec>;
#[doc = "This register defines the endianness of the Host-accessible registers. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostrgfendian;
#[doc = "HOSTRGFSIGNATURE (rw) register accessor: This register holds the CryptoCell product signature.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfsignature::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfsignature::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostrgfsignature`]
module"]
#[doc(alias = "HOSTRGFSIGNATURE")]
pub type Hostrgfsignature = crate::Reg<hostrgfsignature::HostrgfsignatureSpec>;
#[doc = "This register holds the CryptoCell product signature."]
pub mod hostrgfsignature;
#[doc = "HOSTBOOT (rw) register accessor: This register holds the values of CryptoCells pre-synthesis flags\n\nYou can [`read`](crate::Reg::read) this register and get [`hostboot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostboot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostboot`]
module"]
#[doc(alias = "HOSTBOOT")]
pub type Hostboot = crate::Reg<hostboot::HostbootSpec>;
#[doc = "This register holds the values of CryptoCells pre-synthesis flags"]
pub mod hostboot;
#[doc = "HOSTCRYPTOKEYSEL (rw) register accessor: AES hardware key select. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostcryptokeysel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostcryptokeysel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostcryptokeysel`]
module"]
#[doc(alias = "HOSTCRYPTOKEYSEL")]
pub type Hostcryptokeysel = crate::Reg<hostcryptokeysel::HostcryptokeyselSpec>;
#[doc = "AES hardware key select. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostcryptokeysel;
#[doc = "HOSTCORECLKGATINGENABLE (rw) register accessor: This register enables the core clk gating by masking_enabling the cc_idle_state output signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostcoreclkgatingenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostcoreclkgatingenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostcoreclkgatingenable`]
module"]
#[doc(alias = "HOSTCORECLKGATINGENABLE")]
pub type Hostcoreclkgatingenable = crate::Reg<hostcoreclkgatingenable::HostcoreclkgatingenableSpec>;
#[doc = "This register enables the core clk gating by masking_enabling the cc_idle_state output signal."]
pub mod hostcoreclkgatingenable;
#[doc = "HOSTCCISIDLE (rw) register accessor: This register holds the idle indication of CC . Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostccisidle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostccisidle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostccisidle`]
module"]
#[doc(alias = "HOSTCCISIDLE")]
pub type Hostccisidle = crate::Reg<hostccisidle::HostccisidleSpec>;
#[doc = "This register holds the idle indication of CC . Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostccisidle;
#[doc = "HOSTPOWERDOWN (rw) register accessor: This register start the power-down sequence. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostpowerdown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostpowerdown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostpowerdown`]
module"]
#[doc(alias = "HOSTPOWERDOWN")]
pub type Hostpowerdown = crate::Reg<hostpowerdown::HostpowerdownSpec>;
#[doc = "This register start the power-down sequence. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostpowerdown;
#[doc = "HOSTREMOVEGHASHENGINE (rw) register accessor: These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis\n\nYou can [`read`](crate::Reg::read) this register and get [`hostremoveghashengine::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostremoveghashengine::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostremoveghashengine`]
module"]
#[doc(alias = "HOSTREMOVEGHASHENGINE")]
pub type Hostremoveghashengine = crate::Reg<hostremoveghashengine::HostremoveghashengineSpec>;
#[doc = "These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis"]
pub mod hostremoveghashengine;
#[doc = "HOSTREMOVECHACHAENGINE (rw) register accessor: These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis\n\nYou can [`read`](crate::Reg::read) this register and get [`hostremovechachaengine::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostremovechachaengine::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostremovechachaengine`]
module"]
#[doc(alias = "HOSTREMOVECHACHAENGINE")]
pub type Hostremovechachaengine = crate::Reg<hostremovechachaengine::HostremovechachaengineSpec>;
#[doc = "These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis"]
pub mod hostremovechachaengine;
#[doc = "AHBMSINGLES (rw) register accessor: This register forces the ahb transactions to be always singles.\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmsingles::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmsingles::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmsingles`]
module"]
#[doc(alias = "AHBMSINGLES")]
pub type Ahbmsingles = crate::Reg<ahbmsingles::AhbmsinglesSpec>;
#[doc = "This register forces the ahb transactions to be always singles."]
pub mod ahbmsingles;
#[doc = "AHBMHPROT (rw) register accessor: This register holds the ahb prot value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmhprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmhprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmhprot`]
module"]
#[doc(alias = "AHBMHPROT")]
pub type Ahbmhprot = crate::Reg<ahbmhprot::AhbmhprotSpec>;
#[doc = "This register holds the ahb prot value"]
pub mod ahbmhprot;
#[doc = "AHBMHMASTLOCK (rw) register accessor: This register holds ahb hmastlock value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmhmastlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmhmastlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmhmastlock`]
module"]
#[doc(alias = "AHBMHMASTLOCK")]
pub type Ahbmhmastlock = crate::Reg<ahbmhmastlock::AhbmhmastlockSpec>;
#[doc = "This register holds ahb hmastlock value"]
pub mod ahbmhmastlock;
#[doc = "AHBMHNONSEC (rw) register accessor: This register holds ahb hnonsec value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmhnonsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmhnonsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmhnonsec`]
module"]
#[doc(alias = "AHBMHNONSEC")]
pub type Ahbmhnonsec = crate::Reg<ahbmhnonsec::AhbmhnonsecSpec>;
#[doc = "This register holds ahb hnonsec value"]
pub mod ahbmhnonsec;
#[doc = "DINBUFFER (rw) register accessor: This address can be used by the CPU to write data directly to the DIN buffer to be sent to engines.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinbuffer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinbuffer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinbuffer`]
module"]
#[doc(alias = "DINBUFFER")]
pub type Dinbuffer = crate::Reg<dinbuffer::DinbufferSpec>;
#[doc = "This address can be used by the CPU to write data directly to the DIN buffer to be sent to engines."]
pub mod dinbuffer;
#[doc = "DINMEMDMABUSY (rw) register accessor: Indicates whether memory (AXI) source DMA (DIN) is busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinmemdmabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinmemdmabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinmemdmabusy`]
module"]
#[doc(alias = "DINMEMDMABUSY")]
pub type Dinmemdmabusy = crate::Reg<dinmemdmabusy::DinmemdmabusySpec>;
#[doc = "Indicates whether memory (AXI) source DMA (DIN) is busy."]
pub mod dinmemdmabusy;
#[doc = "SRCLLIWORD0 (rw) register accessor: This register is used in direct LLI mode - holds the location of the data source in the memory (AXI).\n\nYou can [`read`](crate::Reg::read) this register and get [`srclliword0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srclliword0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srclliword0`]
module"]
#[doc(alias = "SRCLLIWORD0")]
pub type Srclliword0 = crate::Reg<srclliword0::Srclliword0Spec>;
#[doc = "This register is used in direct LLI mode - holds the location of the data source in the memory (AXI)."]
pub mod srclliword0;
#[doc = "SRCLLIWORD1 (rw) register accessor: This register is used in direct LLI mode - holds the number of bytes to be read from the memory (AXI). Writing to this register triggers the DMA. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`srclliword1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srclliword1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srclliword1`]
module"]
#[doc(alias = "SRCLLIWORD1")]
pub type Srclliword1 = crate::Reg<srclliword1::Srclliword1Spec>;
#[doc = "This register is used in direct LLI mode - holds the number of bytes to be read from the memory (AXI). Writing to this register triggers the DMA. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod srclliword1;
#[doc = "SRAMSRCADDR (rw) register accessor: Location of data (start address) to be read from SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramsrcaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramsrcaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramsrcaddr`]
module"]
#[doc(alias = "SRAMSRCADDR")]
pub type Sramsrcaddr = crate::Reg<sramsrcaddr::SramsrcaddrSpec>;
#[doc = "Location of data (start address) to be read from SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod sramsrcaddr;
#[doc = "DINSRAMBYTESLEN (rw) register accessor: This register holds the size of the data (in bytes) to be read from the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinsrambyteslen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinsrambyteslen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinsrambyteslen`]
module"]
#[doc(alias = "DINSRAMBYTESLEN")]
pub type Dinsrambyteslen = crate::Reg<dinsrambyteslen::DinsrambyteslenSpec>;
#[doc = "This register holds the size of the data (in bytes) to be read from the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod dinsrambyteslen;
#[doc = "DINSRAMDMABUSY (rw) register accessor: This register holds the status of the SRAM DMA DIN.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinsramdmabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinsramdmabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinsramdmabusy`]
module"]
#[doc(alias = "DINSRAMDMABUSY")]
pub type Dinsramdmabusy = crate::Reg<dinsramdmabusy::DinsramdmabusySpec>;
#[doc = "This register holds the status of the SRAM DMA DIN."]
pub mod dinsramdmabusy;
#[doc = "DINSRAMENDIANNESS (rw) register accessor: This register defines the endianness of the DIN interface to SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinsramendianness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinsramendianness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinsramendianness`]
module"]
#[doc(alias = "DINSRAMENDIANNESS")]
pub type Dinsramendianness = crate::Reg<dinsramendianness::DinsramendiannessSpec>;
#[doc = "This register defines the endianness of the DIN interface to SRAM."]
pub mod dinsramendianness;
#[doc = "DINCPUDATASIZE (rw) register accessor: This register hold the number of bytes to be transmited using external DMA. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dincpudatasize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dincpudatasize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dincpudatasize`]
module"]
#[doc(alias = "DINCPUDATASIZE")]
pub type Dincpudatasize = crate::Reg<dincpudatasize::DincpudatasizeSpec>;
#[doc = "This register hold the number of bytes to be transmited using external DMA. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod dincpudatasize;
#[doc = "FIFOINEMPTY (rw) register accessor: DIN FIFO empty indication\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoinempty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoinempty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoinempty`]
module"]
#[doc(alias = "FIFOINEMPTY")]
pub type Fifoinempty = crate::Reg<fifoinempty::FifoinemptySpec>;
#[doc = "DIN FIFO empty indication"]
pub mod fifoinempty;
#[doc = "DINFIFORSTPNTR (rw) register accessor: Writing to this register resets the DIN_FIFO pointers.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinfiforstpntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinfiforstpntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinfiforstpntr`]
module"]
#[doc(alias = "DINFIFORSTPNTR")]
pub type Dinfiforstpntr = crate::Reg<dinfiforstpntr::DinfiforstpntrSpec>;
#[doc = "Writing to this register resets the DIN_FIFO pointers."]
pub mod dinfiforstpntr;
#[doc = "DOUTBUFFER (rw) register accessor: Cryptographic result - CPU can directly access it. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutbuffer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutbuffer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutbuffer`]
module"]
#[doc(alias = "DOUTBUFFER")]
pub type Doutbuffer = crate::Reg<doutbuffer::DoutbufferSpec>;
#[doc = "Cryptographic result - CPU can directly access it. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod doutbuffer;
#[doc = "DOUTMEMDMABUSY (rw) register accessor: DOUT memory DMA busy - Indicates that memory (AXI) destination DMA (DOUT) is busy,\n\nYou can [`read`](crate::Reg::read) this register and get [`doutmemdmabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutmemdmabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutmemdmabusy`]
module"]
#[doc(alias = "DOUTMEMDMABUSY")]
pub type Doutmemdmabusy = crate::Reg<doutmemdmabusy::DoutmemdmabusySpec>;
#[doc = "DOUT memory DMA busy - Indicates that memory (AXI) destination DMA (DOUT) is busy,"]
pub mod doutmemdmabusy;
#[doc = "DSTLLIWORD0 (rw) register accessor: This register is used in direct LLI mode - holds the location of the data destination in the memory (AXI)\n\nYou can [`read`](crate::Reg::read) this register and get [`dstlliword0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstlliword0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstlliword0`]
module"]
#[doc(alias = "DSTLLIWORD0")]
pub type Dstlliword0 = crate::Reg<dstlliword0::Dstlliword0Spec>;
#[doc = "This register is used in direct LLI mode - holds the location of the data destination in the memory (AXI)"]
pub mod dstlliword0;
#[doc = "DSTLLIWORD1 (rw) register accessor: This register is used in direct LLI mode - holds the number of bytes to be written to the memory (AXI). Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dstlliword1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstlliword1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstlliword1`]
module"]
#[doc(alias = "DSTLLIWORD1")]
pub type Dstlliword1 = crate::Reg<dstlliword1::Dstlliword1Spec>;
#[doc = "This register is used in direct LLI mode - holds the number of bytes to be written to the memory (AXI). Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod dstlliword1;
#[doc = "SRAMDESTADDR (rw) register accessor: Location of result to be sent to in SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramdestaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramdestaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramdestaddr`]
module"]
#[doc(alias = "SRAMDESTADDR")]
pub type Sramdestaddr = crate::Reg<sramdestaddr::SramdestaddrSpec>;
#[doc = "Location of result to be sent to in SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod sramdestaddr;
#[doc = "DOUTSRAMBYTESLEN (rw) register accessor: This register holds the size of the data (in bytes) to be written to the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutsrambyteslen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutsrambyteslen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutsrambyteslen`]
module"]
#[doc(alias = "DOUTSRAMBYTESLEN")]
pub type Doutsrambyteslen = crate::Reg<doutsrambyteslen::DoutsrambyteslenSpec>;
#[doc = "This register holds the size of the data (in bytes) to be written to the SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod doutsrambyteslen;
#[doc = "DOUTSRAMDMABUSY (rw) register accessor: This register holds the status of the SRAM DMA DOUT.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutsramdmabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutsramdmabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutsramdmabusy`]
module"]
#[doc(alias = "DOUTSRAMDMABUSY")]
pub type Doutsramdmabusy = crate::Reg<doutsramdmabusy::DoutsramdmabusySpec>;
#[doc = "This register holds the status of the SRAM DMA DOUT."]
pub mod doutsramdmabusy;
#[doc = "DOUTSRAMENDIANNESS (rw) register accessor: This register defines the endianness of the DOUT interface from SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutsramendianness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutsramendianness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutsramendianness`]
module"]
#[doc(alias = "DOUTSRAMENDIANNESS")]
pub type Doutsramendianness = crate::Reg<doutsramendianness::DoutsramendiannessSpec>;
#[doc = "This register defines the endianness of the DOUT interface from SRAM."]
pub mod doutsramendianness;
#[doc = "READALIGNLAST (rw) register accessor: Indication that the next read from the CPU is the last one. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding).\n\nYou can [`read`](crate::Reg::read) this register and get [`readalignlast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readalignlast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readalignlast`]
module"]
#[doc(alias = "READALIGNLAST")]
pub type Readalignlast = crate::Reg<readalignlast::ReadalignlastSpec>;
#[doc = "Indication that the next read from the CPU is the last one. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding)."]
pub mod readalignlast;
#[doc = "DOUTFIFOEMPTY (rw) register accessor: DOUT_FIFO_EMPTY Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutfifoempty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutfifoempty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutfifoempty`]
module"]
#[doc(alias = "DOUTFIFOEMPTY")]
pub type Doutfifoempty = crate::Reg<doutfifoempty::DoutfifoemptySpec>;
#[doc = "DOUT_FIFO_EMPTY Register."]
pub mod doutfifoempty;
#[doc = "SRAMDATA (rw) register accessor: READ WRITE DATA FROM SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramdata`]
module"]
#[doc(alias = "SRAMDATA")]
pub type Sramdata = crate::Reg<sramdata::SramdataSpec>;
#[doc = "READ WRITE DATA FROM SRAM. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod sramdata;
#[doc = "SRAMADDR (rw) register accessor: first address given to SRAM DMA for read_write transactions from SRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`sramaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramaddr`]
module"]
#[doc(alias = "SRAMADDR")]
pub type Sramaddr = crate::Reg<sramaddr::SramaddrSpec>;
#[doc = "first address given to SRAM DMA for read_write transactions from SRAM"]
pub mod sramaddr;
#[doc = "SRAMDATAREADY (rw) register accessor: The SRAM content is ready for read in SRAM_DATA.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramdataready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramdataready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramdataready`]
module"]
#[doc(alias = "SRAMDATAREADY")]
pub type Sramdataready = crate::Reg<sramdataready::SramdatareadySpec>;
#[doc = "The SRAM content is ready for read in SRAM_DATA."]
pub mod sramdataready;
#[doc = "PERIPHERALID4 (rw) register accessor: Peripheral ID 4 (PID4).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peripheralid4`]
module"]
#[doc(alias = "PERIPHERALID4")]
pub type Peripheralid4 = crate::Reg<peripheralid4::Peripheralid4Spec>;
#[doc = "Peripheral ID 4 (PID4)."]
pub mod peripheralid4;
#[doc = "PERIPHERALID0 (rw) register accessor: Peripheral ID 0 (PID0).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peripheralid0`]
module"]
#[doc(alias = "PERIPHERALID0")]
pub type Peripheralid0 = crate::Reg<peripheralid0::Peripheralid0Spec>;
#[doc = "Peripheral ID 0 (PID0)."]
pub mod peripheralid0;
#[doc = "PERIPHERALID1 (rw) register accessor: Peripheral ID 1 (PID1).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peripheralid1`]
module"]
#[doc(alias = "PERIPHERALID1")]
pub type Peripheralid1 = crate::Reg<peripheralid1::Peripheralid1Spec>;
#[doc = "Peripheral ID 1 (PID1)."]
pub mod peripheralid1;
#[doc = "PERIPHERALID2 (rw) register accessor: Peripheral ID 2 (PID2).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peripheralid2`]
module"]
#[doc(alias = "PERIPHERALID2")]
pub type Peripheralid2 = crate::Reg<peripheralid2::Peripheralid2Spec>;
#[doc = "Peripheral ID 2 (PID2)."]
pub mod peripheralid2;
#[doc = "PERIPHERALID3 (rw) register accessor: Peripheral ID 3 (PID3).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peripheralid3`]
module"]
#[doc(alias = "PERIPHERALID3")]
pub type Peripheralid3 = crate::Reg<peripheralid3::Peripheralid3Spec>;
#[doc = "Peripheral ID 3 (PID3)."]
pub mod peripheralid3;
#[doc = "COMPONENTID0 (rw) register accessor: Component ID0.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@componentid0`]
module"]
#[doc(alias = "COMPONENTID0")]
pub type Componentid0 = crate::Reg<componentid0::Componentid0Spec>;
#[doc = "Component ID0."]
pub mod componentid0;
#[doc = "COMPONENTID1 (rw) register accessor: Component ID1.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@componentid1`]
module"]
#[doc(alias = "COMPONENTID1")]
pub type Componentid1 = crate::Reg<componentid1::Componentid1Spec>;
#[doc = "Component ID1."]
pub mod componentid1;
#[doc = "COMPONENTID2 (rw) register accessor: Component ID2.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@componentid2`]
module"]
#[doc(alias = "COMPONENTID2")]
pub type Componentid2 = crate::Reg<componentid2::Componentid2Spec>;
#[doc = "Component ID2."]
pub mod componentid2;
#[doc = "COMPONENTID3 (rw) register accessor: Component ID3.\n\nYou can [`read`](crate::Reg::read) this register and get [`componentid3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`componentid3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@componentid3`]
module"]
#[doc(alias = "COMPONENTID3")]
pub type Componentid3 = crate::Reg<componentid3::Componentid3Spec>;
#[doc = "Component ID3."]
pub mod componentid3;
#[doc = "HOSTDCUEN0 (rw) register accessor: The DCU \\[31:0\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdcuen0`]
module"]
#[doc(alias = "HOSTDCUEN0")]
pub type Hostdcuen0 = crate::Reg<hostdcuen0::Hostdcuen0Spec>;
#[doc = "The DCU \\[31:0\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdcuen0;
#[doc = "HOSTDCUEN1 (rw) register accessor: The DCU \\[63:32\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdcuen1`]
module"]
#[doc(alias = "HOSTDCUEN1")]
pub type Hostdcuen1 = crate::Reg<hostdcuen1::Hostdcuen1Spec>;
#[doc = "The DCU \\[63:32\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdcuen1;
#[doc = "HOSTDCUEN2 (rw) register accessor: The DCU \\[95:64\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdcuen2`]
module"]
#[doc(alias = "HOSTDCUEN2")]
pub type Hostdcuen2 = crate::Reg<hostdcuen2::Hostdcuen2Spec>;
#[doc = "The DCU \\[95:64\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdcuen2;
#[doc = "HOSTDCUEN3 (rw) register accessor: The DCU \\[1271:96\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdcuen3`]
module"]
#[doc(alias = "HOSTDCUEN3")]
pub type Hostdcuen3 = crate::Reg<hostdcuen3::Hostdcuen3Spec>;
#[doc = "The DCU \\[1271:96\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdcuen3;
#[doc = "HOSTDCULOCK0 (rw) register accessor: The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdculock0`]
module"]
#[doc(alias = "HOSTDCULOCK0")]
pub type Hostdculock0 = crate::Reg<hostdculock0::Hostdculock0Spec>;
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdculock0;
#[doc = "HOSTDCULOCK1 (rw) register accessor: The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdculock1`]
module"]
#[doc(alias = "HOSTDCULOCK1")]
pub type Hostdculock1 = crate::Reg<hostdculock1::Hostdculock1Spec>;
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdculock1;
#[doc = "HOSTDCULOCK2 (rw) register accessor: The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdculock2`]
module"]
#[doc(alias = "HOSTDCULOCK2")]
pub type Hostdculock2 = crate::Reg<hostdculock2::Hostdculock2Spec>;
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdculock2;
#[doc = "HOSTDCULOCK3 (rw) register accessor: The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostdculock3`]
module"]
#[doc(alias = "HOSTDCULOCK3")]
pub type Hostdculock3 = crate::Reg<hostdculock3::Hostdculock3Spec>;
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostdculock3;
#[doc = "AOICVDCURESTRICTIONMASK0 (rw) register accessor: The DCU lock register.\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoicvdcurestrictionmask0`]
module"]
#[doc(alias = "AOICVDCURESTRICTIONMASK0")]
pub type Aoicvdcurestrictionmask0 =
    crate::Reg<aoicvdcurestrictionmask0::Aoicvdcurestrictionmask0Spec>;
#[doc = "The DCU lock register."]
pub mod aoicvdcurestrictionmask0;
#[doc = "AOICVDCURESTRICTIONMASK1 (rw) register accessor: The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoicvdcurestrictionmask1`]
module"]
#[doc(alias = "AOICVDCURESTRICTIONMASK1")]
pub type Aoicvdcurestrictionmask1 =
    crate::Reg<aoicvdcurestrictionmask1::Aoicvdcurestrictionmask1Spec>;
#[doc = "The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets"]
pub mod aoicvdcurestrictionmask1;
#[doc = "AOICVDCURESTRICTIONMASK2 (rw) register accessor: The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoicvdcurestrictionmask2`]
module"]
#[doc(alias = "AOICVDCURESTRICTIONMASK2")]
pub type Aoicvdcurestrictionmask2 =
    crate::Reg<aoicvdcurestrictionmask2::Aoicvdcurestrictionmask2Spec>;
#[doc = "The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets"]
pub mod aoicvdcurestrictionmask2;
#[doc = "AOICVDCURESTRICTIONMASK3 (rw) register accessor: The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoicvdcurestrictionmask3`]
module"]
#[doc(alias = "AOICVDCURESTRICTIONMASK3")]
pub type Aoicvdcurestrictionmask3 =
    crate::Reg<aoicvdcurestrictionmask3::Aoicvdcurestrictionmask3Spec>;
#[doc = "The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets"]
pub mod aoicvdcurestrictionmask3;
#[doc = "AOCCSECDEBUGRESET (rw) register accessor: The reset-upon-debug indication\n\nYou can [`read`](crate::Reg::read) this register and get [`aoccsecdebugreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoccsecdebugreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoccsecdebugreset`]
module"]
#[doc(alias = "AOCCSECDEBUGRESET")]
pub type Aoccsecdebugreset = crate::Reg<aoccsecdebugreset::AoccsecdebugresetSpec>;
#[doc = "The reset-upon-debug indication"]
pub mod aoccsecdebugreset;
#[doc = "HOSTAOLOCKBITS (rw) register accessor: These masks will define, per LCS, which DCU bits will be tied to zero, even if the Host tries to set them. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostaolockbits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostaolockbits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostaolockbits`]
module"]
#[doc(alias = "HOSTAOLOCKBITS")]
pub type Hostaolockbits = crate::Reg<hostaolockbits::HostaolockbitsSpec>;
#[doc = "These masks will define, per LCS, which DCU bits will be tied to zero, even if the Host tries to set them. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostaolockbits;
#[doc = "AOAPBFILTERING (rw) register accessor: This register holds the AO_APB_FILTERING data. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aoapbfiltering::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoapbfiltering::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoapbfiltering`]
module"]
#[doc(alias = "AOAPBFILTERING")]
pub type Aoapbfiltering = crate::Reg<aoapbfiltering::AoapbfilteringSpec>;
#[doc = "This register holds the AO_APB_FILTERING data. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod aoapbfiltering;
#[doc = "AOCCGPPC (rw) register accessor: holds the AO_CC_GPPC value from AONote: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aoccgppc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoccgppc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoccgppc`]
module"]
#[doc(alias = "AOCCGPPC")]
pub type Aoccgppc = crate::Reg<aoccgppc::AoccgppcSpec>;
#[doc = "holds the AO_CC_GPPC value from AONote: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod aoccgppc;
#[doc = "HOSTRGFCCSWRST (rw) register accessor: Writing to this register generates a general reset to CryptoCell. This reset takes about 4 core clock cycles.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfccswrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfccswrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostrgfccswrst`]
module"]
#[doc(alias = "HOSTRGFCCSWRST")]
pub type Hostrgfccswrst = crate::Reg<hostrgfccswrst::HostrgfccswrstSpec>;
#[doc = "Writing to this register generates a general reset to CryptoCell. This reset takes about 4 core clock cycles.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod hostrgfccswrst;
#[doc = "AIBFUSEPROGCOMPLETED (rw) register accessor: This register reflects the fuse_aib_prog_completed input, which indicates that the fuse programming was completed.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aibfuseprogcompleted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aibfuseprogcompleted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aibfuseprogcompleted`]
module"]
#[doc(alias = "AIBFUSEPROGCOMPLETED")]
pub type Aibfuseprogcompleted = crate::Reg<aibfuseprogcompleted::AibfuseprogcompletedSpec>;
#[doc = "This register reflects the fuse_aib_prog_completed input, which indicates that the fuse programming was completed.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod aibfuseprogcompleted;
#[doc = "NVMDEBUGSTATUS (rw) register accessor: AIB debug status register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`nvmdebugstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvmdebugstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvmdebugstatus`]
module"]
#[doc(alias = "NVMDEBUGSTATUS")]
pub type Nvmdebugstatus = crate::Reg<nvmdebugstatus::NvmdebugstatusSpec>;
#[doc = "AIB debug status register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod nvmdebugstatus;
#[doc = "LCSISVALID (rw) register accessor: Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsisvalid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsisvalid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsisvalid`]
module"]
#[doc(alias = "LCSISVALID")]
pub type Lcsisvalid = crate::Reg<lcsisvalid::LcsisvalidSpec>;
#[doc = "Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod lcsisvalid;
#[doc = "NVMISIDLE (rw) register accessor: Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`nvmisidle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvmisidle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvmisidle`]
module"]
#[doc(alias = "NVMISIDLE")]
pub type Nvmisidle = crate::Reg<nvmisidle::NvmisidleSpec>;
#[doc = "Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod nvmisidle;
#[doc = "LCSREG (rw) register accessor: The lifecycle state register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcsreg`]
module"]
#[doc(alias = "LCSREG")]
pub type Lcsreg = crate::Reg<lcsreg::LcsregSpec>;
#[doc = "The lifecycle state register. Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod lcsreg;
#[doc = "HOSTSHADOWKDRREG (rw) register accessor: This register interface is used to update the RKEK(KDR) registers when the device is in CM or DM mode , it is Write-once (per warm boot) in RMA LCS, The RKEK is updated by shifting .\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkdrreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkdrreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostshadowkdrreg`]
module"]
#[doc(alias = "HOSTSHADOWKDRREG")]
pub type Hostshadowkdrreg = crate::Reg<hostshadowkdrreg::HostshadowkdrregSpec>;
#[doc = "This register interface is used to update the RKEK(KDR) registers when the device is in CM or DM mode , it is Write-once (per warm boot) in RMA LCS, The RKEK is updated by shifting ."]
pub mod hostshadowkdrreg;
#[doc = "HOSTSHADOWKCPREG (rw) register accessor: This register interface is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkcpreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkcpreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostshadowkcpreg`]
module"]
#[doc(alias = "HOSTSHADOWKCPREG")]
pub type Hostshadowkcpreg = crate::Reg<hostshadowkcpreg::HostshadowkcpregSpec>;
#[doc = "This register interface is used to update the KCP registers when the device is in CM or DM mode, The KCP is updated by shifting"]
pub mod hostshadowkcpreg;
#[doc = "HOSTSHADOWKCEREG (rw) register accessor: This register interface is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkcereg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkcereg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostshadowkcereg`]
module"]
#[doc(alias = "HOSTSHADOWKCEREG")]
pub type Hostshadowkcereg = crate::Reg<hostshadowkcereg::HostshadowkceregSpec>;
#[doc = "This register interface is used to update the KCE registers when the device is in CM or DM mode, The KCE is updated by shifting"]
pub mod hostshadowkcereg;
#[doc = "HOSTSHADOWKPICVREG (rw) register accessor: This register interface is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkpicvreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkpicvreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostshadowkpicvreg`]
module"]
#[doc(alias = "HOSTSHADOWKPICVREG")]
pub type Hostshadowkpicvreg = crate::Reg<hostshadowkpicvreg::HostshadowkpicvregSpec>;
#[doc = "This register interface is used to update the KPICV registers when the device is in CM or DM mode, The KPICV is updated by shifting"]
pub mod hostshadowkpicvreg;
#[doc = "HOSTSHADOWKCEICVREG (rw) register accessor: This register interface is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting\n\nYou can [`read`](crate::Reg::read) this register and get [`hostshadowkceicvreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostshadowkceicvreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hostshadowkceicvreg`]
module"]
#[doc(alias = "HOSTSHADOWKCEICVREG")]
pub type Hostshadowkceicvreg = crate::Reg<hostshadowkceicvreg::HostshadowkceicvregSpec>;
#[doc = "This register interface is used to update the KCEICV registers when the device is in CM or DM mode, The KCEICV is updated by shifting"]
pub mod hostshadowkceicvreg;
#[doc = "OTPADDRWIDTHDEF (rw) register accessor: OTP_ADDR_WIDTH parameter, that will define the integrated OTP address width (address in words). The supported sizes are 6 (for 2 Kbits),7,8,9,11 (for 64 Kbits). The default value in the provided RTL will be 6.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`otpaddrwidthdef::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpaddrwidthdef::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpaddrwidthdef`]
module"]
#[doc(alias = "OTPADDRWIDTHDEF")]
pub type Otpaddrwidthdef = crate::Reg<otpaddrwidthdef::OtpaddrwidthdefSpec>;
#[doc = "OTP_ADDR_WIDTH parameter, that will define the integrated OTP address width (address in words). The supported sizes are 6 (for 2 Kbits),7,8,9,11 (for 64 Kbits). The default value in the provided RTL will be 6.Note: This is a special register, affected by internal logic. Test result of this register is NA."]
pub mod otpaddrwidthdef;
