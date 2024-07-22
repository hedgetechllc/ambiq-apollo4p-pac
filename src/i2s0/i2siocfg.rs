#[doc = "Register `I2SIOCFG` reader"]
pub type R = crate::R<I2siocfgSpec>;
#[doc = "Register `I2SIOCFG` writer"]
pub type W = crate::W<I2siocfgSpec>;
#[doc = "Field `OEN` reader - Output enable for SDATA output"]
pub type OenR = crate::BitReader;
#[doc = "Field `OEN` writer - Output enable for SDATA output"]
pub type OenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPER` reader - Frame period in units of sclk. Period is FPER + 1 sclks in length. 0: 1 sclk, 0x3F: 64 sclks"]
pub type FperR = crate::FieldReader<u16>;
#[doc = "Field `FPER` writer - Frame period in units of sclk. Period is FPER + 1 sclks in length. 0: 1 sclk, 0x3F: 64 sclks"]
pub type FperW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FSP` reader - Polarity of fsync/lr_clk signal. 0: Active high. 1: Active low"]
pub type FspR = crate::BitReader;
#[doc = "Field `FSP` writer - Polarity of fsync/lr_clk signal. 0: Active high. 1: Active low"]
pub type FspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTX` reader - Transmit clock edge polarity bit. 0: sdata is transmitted starting from the falling edge of sclk. 1: sdata is transmitted starting from the rising edge of sclk."]
pub type PrtxR = crate::BitReader;
#[doc = "Field `PRTX` writer - Transmit clock edge polarity bit. 0: sdata is transmitted starting from the falling edge of sclk. 1: sdata is transmitted starting from the rising edge of sclk."]
pub type PrtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSL` reader - Master/Slave clock configuration. 0: External clock(sclk and lr_clk provided externally). 1: Internal clock (sclk and lr_clk sourced internally)."]
pub type MslR = crate::BitReader;
#[doc = "Field `MSL` writer - Master/Slave clock configuration. 0: External clock(sclk and lr_clk provided externally). 1: Internal clock (sclk and lr_clk sourced internally)."]
pub type MslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRx` reader - Receive clock edge polarity bit. 0: sdata is sampled on the rising edge of sclk. 1: sdata is sampled on the falling edge of sclk."]
pub type PrxR = crate::BitReader;
#[doc = "Field `PRx` writer - Receive clock edge polarity bit. 0: sdata is sampled on the rising edge of sclk. 1: sdata is sampled on the falling edge of sclk."]
pub type PrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWID` reader - period of fsync/lr_clk in units of sclks"]
pub type FwidR = crate::FieldReader;
#[doc = "Field `FWID` writer - period of fsync/lr_clk in units of sclks"]
pub type FwidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Output enable for SDATA output"]
    #[inline(always)]
    pub fn oen(&self) -> OenR {
        OenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Frame period in units of sclk. Period is FPER + 1 sclks in length. 0: 1 sclk, 0x3F: 64 sclks"]
    #[inline(always)]
    pub fn fper(&self) -> FperR {
        FperR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Polarity of fsync/lr_clk signal. 0: Active high. 1: Active low"]
    #[inline(always)]
    pub fn fsp(&self) -> FspR {
        FspR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit clock edge polarity bit. 0: sdata is transmitted starting from the falling edge of sclk. 1: sdata is transmitted starting from the rising edge of sclk."]
    #[inline(always)]
    pub fn prtx(&self) -> PrtxR {
        PrtxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Master/Slave clock configuration. 0: External clock(sclk and lr_clk provided externally). 1: Internal clock (sclk and lr_clk sourced internally)."]
    #[inline(always)]
    pub fn msl(&self) -> MslR {
        MslR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive clock edge polarity bit. 0: sdata is sampled on the rising edge of sclk. 1: sdata is sampled on the falling edge of sclk."]
    #[inline(always)]
    pub fn prx(&self) -> PrxR {
        PrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - period of fsync/lr_clk in units of sclks"]
    #[inline(always)]
    pub fn fwid(&self) -> FwidR {
        FwidR::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Output enable for SDATA output"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OenW<I2siocfgSpec> {
        OenW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Frame period in units of sclk. Period is FPER + 1 sclks in length. 0: 1 sclk, 0x3F: 64 sclks"]
    #[inline(always)]
    #[must_use]
    pub fn fper(&mut self) -> FperW<I2siocfgSpec> {
        FperW::new(self, 4)
    }
    #[doc = "Bit 16 - Polarity of fsync/lr_clk signal. 0: Active high. 1: Active low"]
    #[inline(always)]
    #[must_use]
    pub fn fsp(&mut self) -> FspW<I2siocfgSpec> {
        FspW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit clock edge polarity bit. 0: sdata is transmitted starting from the falling edge of sclk. 1: sdata is transmitted starting from the rising edge of sclk."]
    #[inline(always)]
    #[must_use]
    pub fn prtx(&mut self) -> PrtxW<I2siocfgSpec> {
        PrtxW::new(self, 17)
    }
    #[doc = "Bit 18 - Master/Slave clock configuration. 0: External clock(sclk and lr_clk provided externally). 1: Internal clock (sclk and lr_clk sourced internally)."]
    #[inline(always)]
    #[must_use]
    pub fn msl(&mut self) -> MslW<I2siocfgSpec> {
        MslW::new(self, 18)
    }
    #[doc = "Bit 19 - Receive clock edge polarity bit. 0: sdata is sampled on the rising edge of sclk. 1: sdata is sampled on the falling edge of sclk."]
    #[inline(always)]
    #[must_use]
    pub fn prx(&mut self) -> PrxW<I2siocfgSpec> {
        PrxW::new(self, 19)
    }
    #[doc = "Bits 20:27 - period of fsync/lr_clk in units of sclks"]
    #[inline(always)]
    #[must_use]
    pub fn fwid(&mut self) -> FwidW<I2siocfgSpec> {
        FwidW::new(self, 20)
    }
}
#[doc = "Specified polarity and clock configuration of the I2S IPB clocks and IO signals\n\nYou can [`read`](crate::Reg::read) this register and get [`i2siocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2siocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2siocfgSpec;
impl crate::RegisterSpec for I2siocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2siocfg::R`](R) reader structure"]
impl crate::Readable for I2siocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2siocfg::W`](W) writer structure"]
impl crate::Writable for I2siocfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SIOCFG to value 0x01f1_03f0"]
impl crate::Resettable for I2siocfgSpec {
    const RESET_VALUE: u32 = 0x01f1_03f0;
}
