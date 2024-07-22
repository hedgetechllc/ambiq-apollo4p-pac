#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcuperfreq: Mcuperfreq,
    devpwren: Devpwren,
    devpwrstatus: Devpwrstatus,
    audsspwren: Audsspwren,
    audsspwrstatus: Audsspwrstatus,
    mempwren: Mempwren,
    mempwrstatus: Mempwrstatus,
    memretcfg: Memretcfg,
    syspwrstatus: Syspwrstatus,
    ssrampwren: Ssrampwren,
    ssrampwrst: Ssrampwrst,
    ssramretcfg: Ssramretcfg,
    devpwreventen: Devpwreventen,
    mempwreventen: Mempwreventen,
    _reserved14: [u8; 0x08],
    mmsoverride: Mmsoverride,
    _reserved15: [u8; 0x0c],
    dsp0pwrctrl: Dsp0pwrctrl,
    dsp0perfreq: Dsp0perfreq,
    dsp0mempwren: Dsp0mempwren,
    dsp0mempwrst: Dsp0mempwrst,
    dsp0memretcfg: Dsp0memretcfg,
    _reserved20: [u8; 0x0c],
    dsp1pwrctrl: Dsp1pwrctrl,
    dsp1perfreq: Dsp1perfreq,
    dsp1mempwren: Dsp1mempwren,
    dsp1mempwrst: Dsp1mempwrst,
    dsp1memretcfg: Dsp1memretcfg,
    pwrackovr: Pwrackovr,
    pwrcntdefval: Pwrcntdefval,
    _reserved27: [u8; 0x74],
    vrctrl: Vrctrl,
    legacyvrlpovr: Legacyvrlpovr,
    vrstatus: Vrstatus,
    _reserved30: [u8; 0x34],
    pwrweightulp0: Pwrweightulp0,
    pwrweightulp1: Pwrweightulp1,
    pwrweightulp2: Pwrweightulp2,
    pwrweightulp3: Pwrweightulp3,
    pwrweightulp4: Pwrweightulp4,
    pwrweightulp5: Pwrweightulp5,
    pwrweightlp0: Pwrweightlp0,
    pwrweightlp1: Pwrweightlp1,
    pwrweightlp2: Pwrweightlp2,
    pwrweightlp3: Pwrweightlp3,
    pwrweightlp4: Pwrweightlp4,
    pwrweightlp5: Pwrweightlp5,
    pwrweighthp0: Pwrweighthp0,
    pwrweighthp1: Pwrweighthp1,
    pwrweighthp2: Pwrweighthp2,
    pwrweighthp3: Pwrweighthp3,
    pwrweighthp4: Pwrweighthp4,
    pwrweighthp5: Pwrweighthp5,
    pwrweightslp: Pwrweightslp,
    vrdemotionthr: Vrdemotionthr,
    sramctrl: Sramctrl,
    adcstatus: Adcstatus,
    audadcstatus: Audadcstatus,
    _reserved53: [u8; 0x64],
    emonctrl: Emonctrl,
    emoncfg0: Emoncfg0,
    emoncfg1: Emoncfg1,
    emoncfg2: Emoncfg2,
    emoncfg3: Emoncfg3,
    emoncfg4: Emoncfg4,
    emoncfg5: Emoncfg5,
    emoncfg6: Emoncfg6,
    emoncfg7: Emoncfg7,
    _reserved62: [u8; 0x04],
    emoncount0: Emoncount0,
    emoncount1: Emoncount1,
    emoncount2: Emoncount2,
    emoncount3: Emoncount3,
    emoncount4: Emoncount4,
    emoncount5: Emoncount5,
    emoncount6: Emoncount6,
    emoncount7: Emoncount7,
    _reserved70: [u8; 0x04],
    emonstatus: Emonstatus,
}
impl RegisterBlock {
    #[doc = "0x00 - This register provides the performance mode knobs for MCU. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change."]
    #[inline(always)]
    pub const fn mcuperfreq(&self) -> &Mcuperfreq {
        &self.mcuperfreq
    }
    #[doc = "0x04 - This enables various peripherals power domains."]
    #[inline(always)]
    pub const fn devpwren(&self) -> &Devpwren {
        &self.devpwren
    }
    #[doc = "0x08 - This provides the power status for the peripheral device domains controlled through DEVPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up."]
    #[inline(always)]
    pub const fn devpwrstatus(&self) -> &Devpwrstatus {
        &self.devpwrstatus
    }
    #[doc = "0x0c - This enables various power domains in audio subsystem."]
    #[inline(always)]
    pub const fn audsspwren(&self) -> &Audsspwren {
        &self.audsspwren
    }
    #[doc = "0x10 - This provides the power status for the peripheral domains controlled through AUDSSPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up."]
    #[inline(always)]
    pub const fn audsspwrstatus(&self) -> &Audsspwrstatus {
        &self.audsspwrstatus
    }
    #[doc = "0x14 - This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the MEMRETCFG register. If this register is not set, then power will always be disabled to the memory bank."]
    #[inline(always)]
    pub const fn mempwren(&self) -> &Mempwren {
        &self.mempwren
    }
    #[doc = "0x18 - It provides the power status for all the memory banks including- caches, nvm (0 and 1) and all the SRAM groups. The status here should reflect the enable provided by the MEMPWREN register. There may be a lag time between setting the bits in MEMPWREN register and MEMPWRSTATUS register, due to the need to cycle the power gate and isolation seqeunces to the memory banks."]
    #[inline(always)]
    pub const fn mempwrstatus(&self) -> &Mempwrstatus {
        &self.mempwrstatus
    }
    #[doc = "0x1c - This controls the power down of the SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when the core goes into deep sleep. Upon wake, the data within the SRAMs are retained. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep."]
    #[inline(always)]
    pub const fn memretcfg(&self) -> &Memretcfg {
        &self.memretcfg
    }
    #[doc = "0x20 - Power ON Status for domains that are not part of devpwrstatus or mempwrstatus"]
    #[inline(always)]
    pub const fn syspwrstatus(&self) -> &Syspwrstatus {
        &self.syspwrstatus
    }
    #[doc = "0x24 - This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the SSRAMRETCFG register. If this register is not set, then power will always be disabled to the memory bank."]
    #[inline(always)]
    pub const fn ssrampwren(&self) -> &Ssrampwren {
        &self.ssrampwren
    }
    #[doc = "0x28 - It provides the power status for shared sram banks. The status here should reflect the enable provided by the SSRAMPWREN register."]
    #[inline(always)]
    pub const fn ssrampwrst(&self) -> &Ssrampwrst {
        &self.ssrampwrst
    }
    #[doc = "0x2c - This controls the power down of the Shared SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when none of the CPU agents are in powered up and active mode. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep."]
    #[inline(always)]
    pub const fn ssramretcfg(&self) -> &Ssramretcfg {
        &self.ssramretcfg
    }
    #[doc = "0x30 - This register controls which feature trigger will result in an event to the CPU. It includes all the power on status for the core domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core."]
    #[inline(always)]
    pub const fn devpwreventen(&self) -> &Devpwreventen {
        &self.devpwreventen
    }
    #[doc = "0x34 - This register controls which power enable for the memories will result in an event to the CPU. It includes all the power on status for the memory domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core."]
    #[inline(always)]
    pub const fn mempwreventen(&self) -> &Mempwreventen {
        &self.mempwreventen
    }
    #[doc = "0x40 - Power domain behavior overrides related to MMS ( Multimedia System )."]
    #[inline(always)]
    pub const fn mmsoverride(&self) -> &Mmsoverride {
        &self.mmsoverride
    }
    #[doc = "0x50 - Power and RST controls for DSP0"]
    #[inline(always)]
    pub const fn dsp0pwrctrl(&self) -> &Dsp0pwrctrl {
        &self.dsp0pwrctrl
    }
    #[doc = "0x54 - This register provides the performance mode knobs for DSP0. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change."]
    #[inline(always)]
    pub const fn dsp0perfreq(&self) -> &Dsp0perfreq {
        &self.dsp0perfreq
    }
    #[doc = "0x58 - This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP0MEMRETCFG register when DSP0 is OFF."]
    #[inline(always)]
    pub const fn dsp0mempwren(&self) -> &Dsp0mempwren {
        &self.dsp0mempwren
    }
    #[doc = "0x5c - It provides the power status for all the memories of DSP0 subsystem"]
    #[inline(always)]
    pub const fn dsp0mempwrst(&self) -> &Dsp0mempwrst {
        &self.dsp0mempwrst
    }
    #[doc = "0x60 - This controls the power down of the DRAM/IRAM/CACHE banks when DSP0 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP0 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP0 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP0 is powered off."]
    #[inline(always)]
    pub const fn dsp0memretcfg(&self) -> &Dsp0memretcfg {
        &self.dsp0memretcfg
    }
    #[doc = "0x70 - Power and RST controls for DSP1"]
    #[inline(always)]
    pub const fn dsp1pwrctrl(&self) -> &Dsp1pwrctrl {
        &self.dsp1pwrctrl
    }
    #[doc = "0x74 - This register provides the performance mode knobs for DSP1. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change."]
    #[inline(always)]
    pub const fn dsp1perfreq(&self) -> &Dsp1perfreq {
        &self.dsp1perfreq
    }
    #[doc = "0x78 - This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP1MEMRETCFG register when DSP1 is OFF."]
    #[inline(always)]
    pub const fn dsp1mempwren(&self) -> &Dsp1mempwren {
        &self.dsp1mempwren
    }
    #[doc = "0x7c - It provides the power status for all the memories of DSP1 subsystem"]
    #[inline(always)]
    pub const fn dsp1mempwrst(&self) -> &Dsp1mempwrst {
        &self.dsp1mempwrst
    }
    #[doc = "0x80 - This controls the power down of the DRAM/IRAM/CACHE banks when DSP1 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP1 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP1 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP1 is powered off."]
    #[inline(always)]
    pub const fn dsp1memretcfg(&self) -> &Dsp1memretcfg {
        &self.dsp1memretcfg
    }
    #[doc = "0x84 - This register contains override bit fields for various power domain power switch acknowledge notification. As a part of power up sequnce, Power controller will look for power switch ack to advance power up sequence. It is possible for power controller to be a forever wait state in case of a unforeseen HW bug. This register defines a bit fileds to work around if needed in such a situation. The default behavior is to use HW power switch ack. Software can set it to override this feature for each power switch."]
    #[inline(always)]
    pub const fn pwrackovr(&self) -> &Pwrackovr {
        &self.pwrackovr
    }
    #[doc = "0x88 - This register contains programmble dealy values for various state michines. Fields contain dev st machine default value, SIMOBUCK state machine wait delay counter etc."]
    #[inline(always)]
    pub const fn pwrcntdefval(&self) -> &Pwrcntdefval {
        &self.pwrcntdefval
    }
    #[doc = "0x100 - This register includes additional debug control bits. This is an internal Ambiq-only register. Customers should not attempt to change this or else functionality cannot be guaranteed."]
    #[inline(always)]
    pub const fn vrctrl(&self) -> &Vrctrl {
        &self.vrctrl
    }
    #[doc = "0x104 - When an override is set for a power domain, VR logic will ignore that power domain state in making a decision to go into lp state."]
    #[inline(always)]
    pub const fn legacyvrlpovr(&self) -> &Legacyvrlpovr {
        &self.legacyvrlpovr
    }
    #[doc = "0x108 - Provides BUCK and LDOs status."]
    #[inline(always)]
    pub const fn vrstatus(&self) -> &Vrstatus {
        &self.vrstatus
    }
    #[doc = "0x140 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightulp0(&self) -> &Pwrweightulp0 {
        &self.pwrweightulp0
    }
    #[doc = "0x144 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightulp1(&self) -> &Pwrweightulp1 {
        &self.pwrweightulp1
    }
    #[doc = "0x148 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightulp2(&self) -> &Pwrweightulp2 {
        &self.pwrweightulp2
    }
    #[doc = "0x14c - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightulp3(&self) -> &Pwrweightulp3 {
        &self.pwrweightulp3
    }
    #[doc = "0x150 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightulp4(&self) -> &Pwrweightulp4 {
        &self.pwrweightulp4
    }
    #[doc = "0x154 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightulp5(&self) -> &Pwrweightulp5 {
        &self.pwrweightulp5
    }
    #[doc = "0x158 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightlp0(&self) -> &Pwrweightlp0 {
        &self.pwrweightlp0
    }
    #[doc = "0x15c - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightlp1(&self) -> &Pwrweightlp1 {
        &self.pwrweightlp1
    }
    #[doc = "0x160 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightlp2(&self) -> &Pwrweightlp2 {
        &self.pwrweightlp2
    }
    #[doc = "0x164 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightlp3(&self) -> &Pwrweightlp3 {
        &self.pwrweightlp3
    }
    #[doc = "0x168 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightlp4(&self) -> &Pwrweightlp4 {
        &self.pwrweightlp4
    }
    #[doc = "0x16c - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightlp5(&self) -> &Pwrweightlp5 {
        &self.pwrweightlp5
    }
    #[doc = "0x170 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweighthp0(&self) -> &Pwrweighthp0 {
        &self.pwrweighthp0
    }
    #[doc = "0x174 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweighthp1(&self) -> &Pwrweighthp1 {
        &self.pwrweighthp1
    }
    #[doc = "0x178 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweighthp2(&self) -> &Pwrweighthp2 {
        &self.pwrweighthp2
    }
    #[doc = "0x17c - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweighthp3(&self) -> &Pwrweighthp3 {
        &self.pwrweighthp3
    }
    #[doc = "0x180 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweighthp4(&self) -> &Pwrweighthp4 {
        &self.pwrweighthp4
    }
    #[doc = "0x184 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweighthp5(&self) -> &Pwrweighthp5 {
        &self.pwrweighthp5
    }
    #[doc = "0x188 - Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn pwrweightslp(&self) -> &Pwrweightslp {
        &self.pwrweightslp
    }
    #[doc = "0x18c - Weights specified in PWRWEIGHT* registers are applied to each of the masters active requests. The aggregate of all the masters is compared against the this threshold value to change the buck from active to inactive mode."]
    #[inline(always)]
    pub const fn vrdemotionthr(&self) -> &Vrdemotionthr {
        &self.vrdemotionthr
    }
    #[doc = "0x190 - This register provides additional fine-tune power management controls for the SRAMs and the SRAM controller. This includes enabling light sleep for the SRAM and TCM banks, and clock gating for reduced dynamic power."]
    #[inline(always)]
    pub const fn sramctrl(&self) -> &Sramctrl {
        &self.sramctrl
    }
    #[doc = "0x194 - This provides the power status for various blocks within the ADC. These status comes directly from the ADC module and is captured through this interface."]
    #[inline(always)]
    pub const fn adcstatus(&self) -> &Adcstatus {
        &self.adcstatus
    }
    #[doc = "0x198 - This provides the power status for various blocks within the audio ADC. These status comes directly from the audio ADC module and is captured through this interface."]
    #[inline(always)]
    pub const fn audadcstatus(&self) -> &Audadcstatus {
        &self.audadcstatus
    }
    #[doc = "0x200 - Controls each of the energy monitor conuters"]
    #[inline(always)]
    pub const fn emonctrl(&self) -> &Emonctrl {
        &self.emonctrl
    }
    #[doc = "0x204 - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg0(&self) -> &Emoncfg0 {
        &self.emoncfg0
    }
    #[doc = "0x208 - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg1(&self) -> &Emoncfg1 {
        &self.emoncfg1
    }
    #[doc = "0x20c - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg2(&self) -> &Emoncfg2 {
        &self.emoncfg2
    }
    #[doc = "0x210 - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg3(&self) -> &Emoncfg3 {
        &self.emoncfg3
    }
    #[doc = "0x214 - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg4(&self) -> &Emoncfg4 {
        &self.emoncfg4
    }
    #[doc = "0x218 - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg5(&self) -> &Emoncfg5 {
        &self.emoncfg5
    }
    #[doc = "0x21c - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg6(&self) -> &Emoncfg6 {
        &self.emoncfg6
    }
    #[doc = "0x220 - The counter increments when the counter is enabled and the mode selected here matches the power mode."]
    #[inline(always)]
    pub const fn emoncfg7(&self) -> &Emoncfg7 {
        &self.emoncfg7
    }
    #[doc = "0x228 - Energy Monitor count value for counter 0"]
    #[inline(always)]
    pub const fn emoncount0(&self) -> &Emoncount0 {
        &self.emoncount0
    }
    #[doc = "0x22c - Energy Monitor count value for counter 1"]
    #[inline(always)]
    pub const fn emoncount1(&self) -> &Emoncount1 {
        &self.emoncount1
    }
    #[doc = "0x230 - Energy Monitor count value for counter 2"]
    #[inline(always)]
    pub const fn emoncount2(&self) -> &Emoncount2 {
        &self.emoncount2
    }
    #[doc = "0x234 - Energy Monitor count value for counter 3"]
    #[inline(always)]
    pub const fn emoncount3(&self) -> &Emoncount3 {
        &self.emoncount3
    }
    #[doc = "0x238 - Energy Monitor count value for counter 4"]
    #[inline(always)]
    pub const fn emoncount4(&self) -> &Emoncount4 {
        &self.emoncount4
    }
    #[doc = "0x23c - Energy Monitor count value for counter 5"]
    #[inline(always)]
    pub const fn emoncount5(&self) -> &Emoncount5 {
        &self.emoncount5
    }
    #[doc = "0x240 - Energy Monitor count value for counter 6"]
    #[inline(always)]
    pub const fn emoncount6(&self) -> &Emoncount6 {
        &self.emoncount6
    }
    #[doc = "0x244 - Energy Monitor count value for counter 7"]
    #[inline(always)]
    pub const fn emoncount7(&self) -> &Emoncount7 {
        &self.emoncount7
    }
    #[doc = "0x24c - Energy Monitor status"]
    #[inline(always)]
    pub const fn emonstatus(&self) -> &Emonstatus {
        &self.emonstatus
    }
}
#[doc = "MCUPERFREQ (rw) register accessor: This register provides the performance mode knobs for MCU. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcuperfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcuperfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuperfreq`]
module"]
#[doc(alias = "MCUPERFREQ")]
pub type Mcuperfreq = crate::Reg<mcuperfreq::McuperfreqSpec>;
#[doc = "This register provides the performance mode knobs for MCU. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change."]
pub mod mcuperfreq;
#[doc = "DEVPWREN (rw) register accessor: This enables various peripherals power domains.\n\nYou can [`read`](crate::Reg::read) this register and get [`devpwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devpwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devpwren`]
module"]
#[doc(alias = "DEVPWREN")]
pub type Devpwren = crate::Reg<devpwren::DevpwrenSpec>;
#[doc = "This enables various peripherals power domains."]
pub mod devpwren;
#[doc = "DEVPWRSTATUS (rw) register accessor: This provides the power status for the peripheral device domains controlled through DEVPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up.\n\nYou can [`read`](crate::Reg::read) this register and get [`devpwrstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devpwrstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devpwrstatus`]
module"]
#[doc(alias = "DEVPWRSTATUS")]
pub type Devpwrstatus = crate::Reg<devpwrstatus::DevpwrstatusSpec>;
#[doc = "This provides the power status for the peripheral device domains controlled through DEVPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up."]
pub mod devpwrstatus;
#[doc = "AUDSSPWREN (rw) register accessor: This enables various power domains in audio subsystem.\n\nYou can [`read`](crate::Reg::read) this register and get [`audsspwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audsspwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audsspwren`]
module"]
#[doc(alias = "AUDSSPWREN")]
pub type Audsspwren = crate::Reg<audsspwren::AudsspwrenSpec>;
#[doc = "This enables various power domains in audio subsystem."]
pub mod audsspwren;
#[doc = "AUDSSPWRSTATUS (rw) register accessor: This provides the power status for the peripheral domains controlled through AUDSSPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up.\n\nYou can [`read`](crate::Reg::read) this register and get [`audsspwrstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audsspwrstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audsspwrstatus`]
module"]
#[doc(alias = "AUDSSPWRSTATUS")]
pub type Audsspwrstatus = crate::Reg<audsspwrstatus::AudsspwrstatusSpec>;
#[doc = "This provides the power status for the peripheral domains controlled through AUDSSPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up."]
pub mod audsspwrstatus;
#[doc = "MEMPWREN (rw) register accessor: This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the MEMRETCFG register. If this register is not set, then power will always be disabled to the memory bank.\n\nYou can [`read`](crate::Reg::read) this register and get [`mempwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mempwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mempwren`]
module"]
#[doc(alias = "MEMPWREN")]
pub type Mempwren = crate::Reg<mempwren::MempwrenSpec>;
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the MEMRETCFG register. If this register is not set, then power will always be disabled to the memory bank."]
pub mod mempwren;
#[doc = "MEMPWRSTATUS (rw) register accessor: It provides the power status for all the memory banks including- caches, nvm (0 and 1) and all the SRAM groups. The status here should reflect the enable provided by the MEMPWREN register. There may be a lag time between setting the bits in MEMPWREN register and MEMPWRSTATUS register, due to the need to cycle the power gate and isolation seqeunces to the memory banks.\n\nYou can [`read`](crate::Reg::read) this register and get [`mempwrstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mempwrstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mempwrstatus`]
module"]
#[doc(alias = "MEMPWRSTATUS")]
pub type Mempwrstatus = crate::Reg<mempwrstatus::MempwrstatusSpec>;
#[doc = "It provides the power status for all the memory banks including- caches, nvm (0 and 1) and all the SRAM groups. The status here should reflect the enable provided by the MEMPWREN register. There may be a lag time between setting the bits in MEMPWREN register and MEMPWRSTATUS register, due to the need to cycle the power gate and isolation seqeunces to the memory banks."]
pub mod mempwrstatus;
#[doc = "MEMRETCFG (rw) register accessor: This controls the power down of the SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when the core goes into deep sleep. Upon wake, the data within the SRAMs are retained. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep.\n\nYou can [`read`](crate::Reg::read) this register and get [`memretcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memretcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memretcfg`]
module"]
#[doc(alias = "MEMRETCFG")]
pub type Memretcfg = crate::Reg<memretcfg::MemretcfgSpec>;
#[doc = "This controls the power down of the SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when the core goes into deep sleep. Upon wake, the data within the SRAMs are retained. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep."]
pub mod memretcfg;
#[doc = "SYSPWRSTATUS (rw) register accessor: Power ON Status for domains that are not part of devpwrstatus or mempwrstatus\n\nYou can [`read`](crate::Reg::read) this register and get [`syspwrstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspwrstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspwrstatus`]
module"]
#[doc(alias = "SYSPWRSTATUS")]
pub type Syspwrstatus = crate::Reg<syspwrstatus::SyspwrstatusSpec>;
#[doc = "Power ON Status for domains that are not part of devpwrstatus or mempwrstatus"]
pub mod syspwrstatus;
#[doc = "SSRAMPWREN (rw) register accessor: This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the SSRAMRETCFG register. If this register is not set, then power will always be disabled to the memory bank.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrampwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrampwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssrampwren`]
module"]
#[doc(alias = "SSRAMPWREN")]
pub type Ssrampwren = crate::Reg<ssrampwren::SsrampwrenSpec>;
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the SSRAMRETCFG register. If this register is not set, then power will always be disabled to the memory bank."]
pub mod ssrampwren;
#[doc = "SSRAMPWRST (rw) register accessor: It provides the power status for shared sram banks. The status here should reflect the enable provided by the SSRAMPWREN register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrampwrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrampwrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssrampwrst`]
module"]
#[doc(alias = "SSRAMPWRST")]
pub type Ssrampwrst = crate::Reg<ssrampwrst::SsrampwrstSpec>;
#[doc = "It provides the power status for shared sram banks. The status here should reflect the enable provided by the SSRAMPWREN register."]
pub mod ssrampwrst;
#[doc = "SSRAMRETCFG (rw) register accessor: This controls the power down of the Shared SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when none of the CPU agents are in powered up and active mode. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssramretcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssramretcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssramretcfg`]
module"]
#[doc(alias = "SSRAMRETCFG")]
pub type Ssramretcfg = crate::Reg<ssramretcfg::SsramretcfgSpec>;
#[doc = "This controls the power down of the Shared SRAM banks in deep sleep mode. If this is set, then the power for that SRAM bank will be gated when the core goes into deep sleep. Upon wake, the data within the SRAMs will be erased. If this is not set, retention voltage will be applied to the SRAM bank when none of the CPU agents are in powered up and active mode. Do not set this if the SRAM bank is used as the target for DMA transfer while CPU in deepsleep."]
pub mod ssramretcfg;
#[doc = "DEVPWREVENTEN (rw) register accessor: This register controls which feature trigger will result in an event to the CPU. It includes all the power on status for the core domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core.\n\nYou can [`read`](crate::Reg::read) this register and get [`devpwreventen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devpwreventen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devpwreventen`]
module"]
#[doc(alias = "DEVPWREVENTEN")]
pub type Devpwreventen = crate::Reg<devpwreventen::DevpwreventenSpec>;
#[doc = "This register controls which feature trigger will result in an event to the CPU. It includes all the power on status for the core domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core."]
pub mod devpwreventen;
#[doc = "MEMPWREVENTEN (rw) register accessor: This register controls which power enable for the memories will result in an event to the CPU. It includes all the power on status for the memory domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core.\n\nYou can [`read`](crate::Reg::read) this register and get [`mempwreventen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mempwreventen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mempwreventen`]
module"]
#[doc(alias = "MEMPWREVENTEN")]
pub type Mempwreventen = crate::Reg<mempwreventen::MempwreventenSpec>;
#[doc = "This register controls which power enable for the memories will result in an event to the CPU. It includes all the power on status for the memory domains. If any bits are set, then if the domain is turned on, it will result in an event to the ARM core."]
pub mod mempwreventen;
#[doc = "MMSOVERRIDE (rw) register accessor: Power domain behavior overrides related to MMS ( Multimedia System ).\n\nYou can [`read`](crate::Reg::read) this register and get [`mmsoverride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsoverride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmsoverride`]
module"]
#[doc(alias = "MMSOVERRIDE")]
pub type Mmsoverride = crate::Reg<mmsoverride::MmsoverrideSpec>;
#[doc = "Power domain behavior overrides related to MMS ( Multimedia System )."]
pub mod mmsoverride;
#[doc = "DSP0PWRCTRL (rw) register accessor: Power and RST controls for DSP0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0pwrctrl`]
module"]
#[doc(alias = "DSP0PWRCTRL")]
pub type Dsp0pwrctrl = crate::Reg<dsp0pwrctrl::Dsp0pwrctrlSpec>;
#[doc = "Power and RST controls for DSP0"]
pub mod dsp0pwrctrl;
#[doc = "DSP0PERFREQ (rw) register accessor: This register provides the performance mode knobs for DSP0. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0perfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0perfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0perfreq`]
module"]
#[doc(alias = "DSP0PERFREQ")]
pub type Dsp0perfreq = crate::Reg<dsp0perfreq::Dsp0perfreqSpec>;
#[doc = "This register provides the performance mode knobs for DSP0. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change."]
pub mod dsp0perfreq;
#[doc = "DSP0MEMPWREN (rw) register accessor: This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP0MEMRETCFG register when DSP0 is OFF.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mempwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mempwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0mempwren`]
module"]
#[doc(alias = "DSP0MEMPWREN")]
pub type Dsp0mempwren = crate::Reg<dsp0mempwren::Dsp0mempwrenSpec>;
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP0MEMRETCFG register when DSP0 is OFF."]
pub mod dsp0mempwren;
#[doc = "DSP0MEMPWRST (rw) register accessor: It provides the power status for all the memories of DSP0 subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0mempwrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0mempwrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0mempwrst`]
module"]
#[doc(alias = "DSP0MEMPWRST")]
pub type Dsp0mempwrst = crate::Reg<dsp0mempwrst::Dsp0mempwrstSpec>;
#[doc = "It provides the power status for all the memories of DSP0 subsystem"]
pub mod dsp0mempwrst;
#[doc = "DSP0MEMRETCFG (rw) register accessor: This controls the power down of the DRAM/IRAM/CACHE banks when DSP0 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP0 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP0 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP0 is powered off.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0memretcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0memretcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp0memretcfg`]
module"]
#[doc(alias = "DSP0MEMRETCFG")]
pub type Dsp0memretcfg = crate::Reg<dsp0memretcfg::Dsp0memretcfgSpec>;
#[doc = "This controls the power down of the DRAM/IRAM/CACHE banks when DSP0 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP0 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP0 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP0 is powered off."]
pub mod dsp0memretcfg;
#[doc = "DSP1PWRCTRL (rw) register accessor: Power and RST controls for DSP1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1pwrctrl`]
module"]
#[doc(alias = "DSP1PWRCTRL")]
pub type Dsp1pwrctrl = crate::Reg<dsp1pwrctrl::Dsp1pwrctrlSpec>;
#[doc = "Power and RST controls for DSP1"]
pub mod dsp1pwrctrl;
#[doc = "DSP1PERFREQ (rw) register accessor: This register provides the performance mode knobs for DSP1. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1perfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1perfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1perfreq`]
module"]
#[doc(alias = "DSP1PERFREQ")]
pub type Dsp1perfreq = crate::Reg<dsp1perfreq::Dsp1perfreqSpec>;
#[doc = "This register provides the performance mode knobs for DSP1. S/w should write the *PERFREQ field to desired mode and wait for the *PERFACK and check for the *PERFSTATUS. Some times system may not allow certain modes but *PERFACK should always follow *PERFREQ change."]
pub mod dsp1perfreq;
#[doc = "DSP1MEMPWREN (rw) register accessor: This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP1MEMRETCFG register when DSP1 is OFF.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mempwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mempwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1mempwren`]
module"]
#[doc(alias = "DSP1MEMPWREN")]
pub type Dsp1mempwren = crate::Reg<dsp1mempwren::Dsp1mempwrenSpec>;
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the DSP1MEMRETCFG register when DSP1 is OFF."]
pub mod dsp1mempwren;
#[doc = "DSP1MEMPWRST (rw) register accessor: It provides the power status for all the memories of DSP1 subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1mempwrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1mempwrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1mempwrst`]
module"]
#[doc(alias = "DSP1MEMPWRST")]
pub type Dsp1mempwrst = crate::Reg<dsp1mempwrst::Dsp1mempwrstSpec>;
#[doc = "It provides the power status for all the memories of DSP1 subsystem"]
pub mod dsp1mempwrst;
#[doc = "DSP1MEMRETCFG (rw) register accessor: This controls the power down of the DRAM/IRAM/CACHE banks when DSP1 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP1 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP1 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP1 is powered off.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp1memretcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp1memretcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsp1memretcfg`]
module"]
#[doc(alias = "DSP1MEMRETCFG")]
pub type Dsp1memretcfg = crate::Reg<dsp1memretcfg::Dsp1memretcfgSpec>;
#[doc = "This controls the power down of the DRAM/IRAM/CACHE banks when DSP1 is powered off. If this is set, then the power for that corresponding SRAM bank will be gated when the DSP1 is powered off and data is erased. If this is not set, retention voltage will be applied when DSP1 is powered off. Do not set this if the SRAM bank is used as the target for DMA transfer while DSP1 is powered off."]
pub mod dsp1memretcfg;
#[doc = "PWRACKOVR (rw) register accessor: This register contains override bit fields for various power domain power switch acknowledge notification. As a part of power up sequnce, Power controller will look for power switch ack to advance power up sequence. It is possible for power controller to be a forever wait state in case of a unforeseen HW bug. This register defines a bit fileds to work around if needed in such a situation. The default behavior is to use HW power switch ack. Software can set it to override this feature for each power switch.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrackovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrackovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrackovr`]
module"]
#[doc(alias = "PWRACKOVR")]
pub type Pwrackovr = crate::Reg<pwrackovr::PwrackovrSpec>;
#[doc = "This register contains override bit fields for various power domain power switch acknowledge notification. As a part of power up sequnce, Power controller will look for power switch ack to advance power up sequence. It is possible for power controller to be a forever wait state in case of a unforeseen HW bug. This register defines a bit fileds to work around if needed in such a situation. The default behavior is to use HW power switch ack. Software can set it to override this feature for each power switch."]
pub mod pwrackovr;
#[doc = "PWRCNTDEFVAL (rw) register accessor: This register contains programmble dealy values for various state michines. Fields contain dev st machine default value, SIMOBUCK state machine wait delay counter etc.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcntdefval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcntdefval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcntdefval`]
module"]
#[doc(alias = "PWRCNTDEFVAL")]
pub type Pwrcntdefval = crate::Reg<pwrcntdefval::PwrcntdefvalSpec>;
#[doc = "This register contains programmble dealy values for various state michines. Fields contain dev st machine default value, SIMOBUCK state machine wait delay counter etc."]
pub mod pwrcntdefval;
#[doc = "VRCTRL (rw) register accessor: This register includes additional debug control bits. This is an internal Ambiq-only register. Customers should not attempt to change this or else functionality cannot be guaranteed.\n\nYou can [`read`](crate::Reg::read) this register and get [`vrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrctrl`]
module"]
#[doc(alias = "VRCTRL")]
pub type Vrctrl = crate::Reg<vrctrl::VrctrlSpec>;
#[doc = "This register includes additional debug control bits. This is an internal Ambiq-only register. Customers should not attempt to change this or else functionality cannot be guaranteed."]
pub mod vrctrl;
#[doc = "LEGACYVRLPOVR (rw) register accessor: When an override is set for a power domain, VR logic will ignore that power domain state in making a decision to go into lp state.\n\nYou can [`read`](crate::Reg::read) this register and get [`legacyvrlpovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`legacyvrlpovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@legacyvrlpovr`]
module"]
#[doc(alias = "LEGACYVRLPOVR")]
pub type Legacyvrlpovr = crate::Reg<legacyvrlpovr::LegacyvrlpovrSpec>;
#[doc = "When an override is set for a power domain, VR logic will ignore that power domain state in making a decision to go into lp state."]
pub mod legacyvrlpovr;
#[doc = "VRSTATUS (rw) register accessor: Provides BUCK and LDOs status.\n\nYou can [`read`](crate::Reg::read) this register and get [`vrstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrstatus`]
module"]
#[doc(alias = "VRSTATUS")]
pub type Vrstatus = crate::Reg<vrstatus::VrstatusSpec>;
#[doc = "Provides BUCK and LDOs status."]
pub mod vrstatus;
#[doc = "PWRWEIGHTULP0 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightulp0`]
module"]
#[doc(alias = "PWRWEIGHTULP0")]
pub type Pwrweightulp0 = crate::Reg<pwrweightulp0::Pwrweightulp0Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightulp0;
#[doc = "PWRWEIGHTULP1 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightulp1`]
module"]
#[doc(alias = "PWRWEIGHTULP1")]
pub type Pwrweightulp1 = crate::Reg<pwrweightulp1::Pwrweightulp1Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightulp1;
#[doc = "PWRWEIGHTULP2 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightulp2`]
module"]
#[doc(alias = "PWRWEIGHTULP2")]
pub type Pwrweightulp2 = crate::Reg<pwrweightulp2::Pwrweightulp2Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightulp2;
#[doc = "PWRWEIGHTULP3 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightulp3`]
module"]
#[doc(alias = "PWRWEIGHTULP3")]
pub type Pwrweightulp3 = crate::Reg<pwrweightulp3::Pwrweightulp3Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightulp3;
#[doc = "PWRWEIGHTULP4 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightulp4`]
module"]
#[doc(alias = "PWRWEIGHTULP4")]
pub type Pwrweightulp4 = crate::Reg<pwrweightulp4::Pwrweightulp4Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightulp4;
#[doc = "PWRWEIGHTULP5 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightulp5`]
module"]
#[doc(alias = "PWRWEIGHTULP5")]
pub type Pwrweightulp5 = crate::Reg<pwrweightulp5::Pwrweightulp5Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightulp5;
#[doc = "PWRWEIGHTLP0 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightlp0`]
module"]
#[doc(alias = "PWRWEIGHTLP0")]
pub type Pwrweightlp0 = crate::Reg<pwrweightlp0::Pwrweightlp0Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightlp0;
#[doc = "PWRWEIGHTLP1 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightlp1`]
module"]
#[doc(alias = "PWRWEIGHTLP1")]
pub type Pwrweightlp1 = crate::Reg<pwrweightlp1::Pwrweightlp1Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightlp1;
#[doc = "PWRWEIGHTLP2 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightlp2`]
module"]
#[doc(alias = "PWRWEIGHTLP2")]
pub type Pwrweightlp2 = crate::Reg<pwrweightlp2::Pwrweightlp2Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightlp2;
#[doc = "PWRWEIGHTLP3 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightlp3`]
module"]
#[doc(alias = "PWRWEIGHTLP3")]
pub type Pwrweightlp3 = crate::Reg<pwrweightlp3::Pwrweightlp3Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightlp3;
#[doc = "PWRWEIGHTLP4 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightlp4`]
module"]
#[doc(alias = "PWRWEIGHTLP4")]
pub type Pwrweightlp4 = crate::Reg<pwrweightlp4::Pwrweightlp4Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightlp4;
#[doc = "PWRWEIGHTLP5 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightlp5`]
module"]
#[doc(alias = "PWRWEIGHTLP5")]
pub type Pwrweightlp5 = crate::Reg<pwrweightlp5::Pwrweightlp5Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightlp5;
#[doc = "PWRWEIGHTHP0 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweighthp0`]
module"]
#[doc(alias = "PWRWEIGHTHP0")]
pub type Pwrweighthp0 = crate::Reg<pwrweighthp0::Pwrweighthp0Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweighthp0;
#[doc = "PWRWEIGHTHP1 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweighthp1`]
module"]
#[doc(alias = "PWRWEIGHTHP1")]
pub type Pwrweighthp1 = crate::Reg<pwrweighthp1::Pwrweighthp1Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweighthp1;
#[doc = "PWRWEIGHTHP2 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweighthp2`]
module"]
#[doc(alias = "PWRWEIGHTHP2")]
pub type Pwrweighthp2 = crate::Reg<pwrweighthp2::Pwrweighthp2Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweighthp2;
#[doc = "PWRWEIGHTHP3 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweighthp3`]
module"]
#[doc(alias = "PWRWEIGHTHP3")]
pub type Pwrweighthp3 = crate::Reg<pwrweighthp3::Pwrweighthp3Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweighthp3;
#[doc = "PWRWEIGHTHP4 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweighthp4`]
module"]
#[doc(alias = "PWRWEIGHTHP4")]
pub type Pwrweighthp4 = crate::Reg<pwrweighthp4::Pwrweighthp4Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweighthp4;
#[doc = "PWRWEIGHTHP5 (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweighthp5`]
module"]
#[doc(alias = "PWRWEIGHTHP5")]
pub type Pwrweighthp5 = crate::Reg<pwrweighthp5::Pwrweighthp5Spec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweighthp5;
#[doc = "PWRWEIGHTSLP (rw) register accessor: Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightslp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightslp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrweightslp`]
module"]
#[doc(alias = "PWRWEIGHTSLP")]
pub type Pwrweightslp = crate::Reg<pwrweightslp::PwrweightslpSpec>;
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode."]
pub mod pwrweightslp;
#[doc = "VRDEMOTIONTHR (rw) register accessor: Weights specified in PWRWEIGHT* registers are applied to each of the masters active requests. The aggregate of all the masters is compared against the this threshold value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`vrdemotionthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrdemotionthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrdemotionthr`]
module"]
#[doc(alias = "VRDEMOTIONTHR")]
pub type Vrdemotionthr = crate::Reg<vrdemotionthr::VrdemotionthrSpec>;
#[doc = "Weights specified in PWRWEIGHT* registers are applied to each of the masters active requests. The aggregate of all the masters is compared against the this threshold value to change the buck from active to inactive mode."]
pub mod vrdemotionthr;
#[doc = "SRAMCTRL (rw) register accessor: This register provides additional fine-tune power management controls for the SRAMs and the SRAM controller. This includes enabling light sleep for the SRAM and TCM banks, and clock gating for reduced dynamic power.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramctrl`]
module"]
#[doc(alias = "SRAMCTRL")]
pub type Sramctrl = crate::Reg<sramctrl::SramctrlSpec>;
#[doc = "This register provides additional fine-tune power management controls for the SRAMs and the SRAM controller. This includes enabling light sleep for the SRAM and TCM banks, and clock gating for reduced dynamic power."]
pub mod sramctrl;
#[doc = "ADCSTATUS (rw) register accessor: This provides the power status for various blocks within the ADC. These status comes directly from the ADC module and is captured through this interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`adcstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcstatus`]
module"]
#[doc(alias = "ADCSTATUS")]
pub type Adcstatus = crate::Reg<adcstatus::AdcstatusSpec>;
#[doc = "This provides the power status for various blocks within the ADC. These status comes directly from the ADC module and is captured through this interface."]
pub mod adcstatus;
#[doc = "AUDADCSTATUS (rw) register accessor: This provides the power status for various blocks within the audio ADC. These status comes directly from the audio ADC module and is captured through this interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`audadcstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audadcstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audadcstatus`]
module"]
#[doc(alias = "AUDADCSTATUS")]
pub type Audadcstatus = crate::Reg<audadcstatus::AudadcstatusSpec>;
#[doc = "This provides the power status for various blocks within the audio ADC. These status comes directly from the audio ADC module and is captured through this interface."]
pub mod audadcstatus;
#[doc = "EMONCTRL (rw) register accessor: Controls each of the energy monitor conuters\n\nYou can [`read`](crate::Reg::read) this register and get [`emonctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emonctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emonctrl`]
module"]
#[doc(alias = "EMONCTRL")]
pub type Emonctrl = crate::Reg<emonctrl::EmonctrlSpec>;
#[doc = "Controls each of the energy monitor conuters"]
pub mod emonctrl;
#[doc = "EMONCFG0 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg0`]
module"]
#[doc(alias = "EMONCFG0")]
pub type Emoncfg0 = crate::Reg<emoncfg0::Emoncfg0Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg0;
#[doc = "EMONCFG1 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg1`]
module"]
#[doc(alias = "EMONCFG1")]
pub type Emoncfg1 = crate::Reg<emoncfg1::Emoncfg1Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg1;
#[doc = "EMONCFG2 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg2`]
module"]
#[doc(alias = "EMONCFG2")]
pub type Emoncfg2 = crate::Reg<emoncfg2::Emoncfg2Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg2;
#[doc = "EMONCFG3 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg3`]
module"]
#[doc(alias = "EMONCFG3")]
pub type Emoncfg3 = crate::Reg<emoncfg3::Emoncfg3Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg3;
#[doc = "EMONCFG4 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg4`]
module"]
#[doc(alias = "EMONCFG4")]
pub type Emoncfg4 = crate::Reg<emoncfg4::Emoncfg4Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg4;
#[doc = "EMONCFG5 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg5`]
module"]
#[doc(alias = "EMONCFG5")]
pub type Emoncfg5 = crate::Reg<emoncfg5::Emoncfg5Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg5;
#[doc = "EMONCFG6 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg6`]
module"]
#[doc(alias = "EMONCFG6")]
pub type Emoncfg6 = crate::Reg<emoncfg6::Emoncfg6Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg6;
#[doc = "EMONCFG7 (rw) register accessor: The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncfg7`]
module"]
#[doc(alias = "EMONCFG7")]
pub type Emoncfg7 = crate::Reg<emoncfg7::Emoncfg7Spec>;
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode."]
pub mod emoncfg7;
#[doc = "EMONCOUNT0 (rw) register accessor: Energy Monitor count value for counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount0`]
module"]
#[doc(alias = "EMONCOUNT0")]
pub type Emoncount0 = crate::Reg<emoncount0::Emoncount0Spec>;
#[doc = "Energy Monitor count value for counter 0"]
pub mod emoncount0;
#[doc = "EMONCOUNT1 (rw) register accessor: Energy Monitor count value for counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount1`]
module"]
#[doc(alias = "EMONCOUNT1")]
pub type Emoncount1 = crate::Reg<emoncount1::Emoncount1Spec>;
#[doc = "Energy Monitor count value for counter 1"]
pub mod emoncount1;
#[doc = "EMONCOUNT2 (rw) register accessor: Energy Monitor count value for counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount2`]
module"]
#[doc(alias = "EMONCOUNT2")]
pub type Emoncount2 = crate::Reg<emoncount2::Emoncount2Spec>;
#[doc = "Energy Monitor count value for counter 2"]
pub mod emoncount2;
#[doc = "EMONCOUNT3 (rw) register accessor: Energy Monitor count value for counter 3\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount3`]
module"]
#[doc(alias = "EMONCOUNT3")]
pub type Emoncount3 = crate::Reg<emoncount3::Emoncount3Spec>;
#[doc = "Energy Monitor count value for counter 3"]
pub mod emoncount3;
#[doc = "EMONCOUNT4 (rw) register accessor: Energy Monitor count value for counter 4\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount4`]
module"]
#[doc(alias = "EMONCOUNT4")]
pub type Emoncount4 = crate::Reg<emoncount4::Emoncount4Spec>;
#[doc = "Energy Monitor count value for counter 4"]
pub mod emoncount4;
#[doc = "EMONCOUNT5 (rw) register accessor: Energy Monitor count value for counter 5\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount5`]
module"]
#[doc(alias = "EMONCOUNT5")]
pub type Emoncount5 = crate::Reg<emoncount5::Emoncount5Spec>;
#[doc = "Energy Monitor count value for counter 5"]
pub mod emoncount5;
#[doc = "EMONCOUNT6 (rw) register accessor: Energy Monitor count value for counter 6\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount6`]
module"]
#[doc(alias = "EMONCOUNT6")]
pub type Emoncount6 = crate::Reg<emoncount6::Emoncount6Spec>;
#[doc = "Energy Monitor count value for counter 6"]
pub mod emoncount6;
#[doc = "EMONCOUNT7 (rw) register accessor: Energy Monitor count value for counter 7\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncount7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncount7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emoncount7`]
module"]
#[doc(alias = "EMONCOUNT7")]
pub type Emoncount7 = crate::Reg<emoncount7::Emoncount7Spec>;
#[doc = "Energy Monitor count value for counter 7"]
pub mod emoncount7;
#[doc = "EMONSTATUS (rw) register accessor: Energy Monitor status\n\nYou can [`read`](crate::Reg::read) this register and get [`emonstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emonstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emonstatus`]
module"]
#[doc(alias = "EMONSTATUS")]
pub type Emonstatus = crate::Reg<emonstatus::EmonstatusSpec>;
#[doc = "Energy Monitor status"]
pub mod emonstatus;
