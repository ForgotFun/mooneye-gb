// This file is part of Mooneye GB.
// Copyright (C) 2014-2018 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use config::HardwareConfig;
use cpu::{Cpu, Step};
use cpu::registers::Registers;
use emulation::{EmuTime, EmuEvents};
use ::GbKey;
use gameboy;
use hardware::Hardware;
pub use self::perf_counter::PerfCounter;

mod perf_counter;

pub struct Machine {
  cpu: Cpu,
  hardware: Hardware,
  step: Step,
}

impl Machine {
  pub fn new(config: HardwareConfig) -> Machine {
    Machine {
      cpu: Cpu::new(),
      hardware: Hardware::new(config),
      step: Step::Initial,
    }
  }
  pub fn emulate(&mut self, target_time: EmuTime) -> (EmuEvents, EmuTime) {
    let mut step = self.step;
    loop {
      step = self.cpu.execute_step(&mut self.hardware, step);
      if !self.hardware.emu_events().is_empty() || self.hardware.emu_time() >= target_time {
        break;
      }
    }
    self.step = step;
    (self.hardware.ack_emu_events(), self.hardware.emu_time())
  }
  pub fn key_down(&mut self, key: GbKey) {
    self.hardware.key_down(key);
  }
  pub fn key_up(&mut self, key: GbKey) {
    self.hardware.key_up(key);
  }
  pub fn regs(&self) -> Registers { self.cpu.regs }
  pub fn screen_buffer(&self) -> &gameboy::ScreenBuffer { self.hardware.screen_buffer() }
}
