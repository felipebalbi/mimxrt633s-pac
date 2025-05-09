#[doc = "Register `PSCCTL1_CLR` writer"]
pub type W = crate::W<Pscctl1ClrSpec>;
#[doc = "SDIO0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Sdio0Clk> for bool {
    #[inline(always)]
    fn from(variant: Sdio0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO0_CLK` writer - SDIO0 clock clear"]
pub type Sdio0ClkW<'a, REG> = crate::BitWriter<'a, REG, Sdio0Clk>;
impl<'a, REG> Sdio0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Clk::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio0Clk::ClrClock)
    }
}
#[doc = "SDIO1 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio1Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Sdio1Clk> for bool {
    #[inline(always)]
    fn from(variant: Sdio1Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO1_CLK` writer - SDIO1 clock clear"]
pub type Sdio1ClkW<'a, REG> = crate::BitWriter<'a, REG, Sdio1Clk>;
impl<'a, REG> Sdio1ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Clk::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio1Clk::ClrClock)
    }
}
#[doc = "Analog comparator clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmp0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Acmp0Clk> for bool {
    #[inline(always)]
    fn from(variant: Acmp0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0_CLK` writer - Analog comparator clock clear"]
pub type Acmp0ClkW<'a, REG> = crate::BitWriter<'a, REG, Acmp0Clk>;
impl<'a, REG> Acmp0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0Clk::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0Clk::ClrClock)
    }
}
#[doc = "ADC clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Adc0Clk> for bool {
    #[inline(always)]
    fn from(variant: Adc0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0_CLK` writer - ADC clock clear"]
pub type Adc0ClkW<'a, REG> = crate::BitWriter<'a, REG, Adc0Clk>;
impl<'a, REG> Adc0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Clk::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0Clk::ClrClock)
    }
}
#[doc = "SHSGPIO0 clock clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shsgpio0Clk {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the PSCCTL1 Bit"]
    ClrClock = 1,
}
impl From<Shsgpio0Clk> for bool {
    #[inline(always)]
    fn from(variant: Shsgpio0Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHSGPIO0_CLK` writer - SHSGPIO0 clock clear"]
pub type Shsgpio0ClkW<'a, REG> = crate::BitWriter<'a, REG, Shsgpio0Clk>;
impl<'a, REG> Shsgpio0ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0Clk::NoEffect)
    }
    #[doc = "Clears the PSCCTL1 Bit"]
    #[inline(always)]
    pub fn clr_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Shsgpio0Clk::ClrClock)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Pscctl1ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - SDIO0 clock clear"]
    #[inline(always)]
    pub fn sdio0_clk(&mut self) -> Sdio0ClkW<Pscctl1ClrSpec> {
        Sdio0ClkW::new(self, 2)
    }
    #[doc = "Bit 3 - SDIO1 clock clear"]
    #[inline(always)]
    pub fn sdio1_clk(&mut self) -> Sdio1ClkW<Pscctl1ClrSpec> {
        Sdio1ClkW::new(self, 3)
    }
    #[doc = "Bit 15 - Analog comparator clock clear"]
    #[inline(always)]
    pub fn acmp0_clk(&mut self) -> Acmp0ClkW<Pscctl1ClrSpec> {
        Acmp0ClkW::new(self, 15)
    }
    #[doc = "Bit 16 - ADC clock clear"]
    #[inline(always)]
    pub fn adc0_clk(&mut self) -> Adc0ClkW<Pscctl1ClrSpec> {
        Adc0ClkW::new(self, 16)
    }
    #[doc = "Bit 24 - SHSGPIO0 clock clear"]
    #[inline(always)]
    pub fn shsgpio0_clk(&mut self) -> Shsgpio0ClkW<Pscctl1ClrSpec> {
        Shsgpio0ClkW::new(self, 24)
    }
}
#[doc = "clock clear register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscctl1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pscctl1ClrSpec;
impl crate::RegisterSpec for Pscctl1ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscctl1_clr::W`](W) writer structure"]
impl crate::Writable for Pscctl1ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCCTL1_CLR to value 0"]
impl crate::Resettable for Pscctl1ClrSpec {
    const RESET_VALUE: u32 = 0;
}
