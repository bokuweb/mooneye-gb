// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use cpu::registers::{
  Flags,
  ZERO, ADD_SUBTRACT, HALF_CARRY
};
use cpu::test::run_test;

#[test]
fn test_20() {
  let machine = run_test(
    &[0x20, 0x01, 0xed, 0xed], // JR NZ, e
    |_| {}
  );
  assert_eq!(machine.hardware.clock_cycles(), 12);
  assert_eq!(machine.cpu.regs.pc, 0x03);
}

#[test]
fn test_20_negative() {
  let machine = run_test(
    &[0x00, 0xed, 0x20, -3i8 as u8], // JR NZ, e
    |machine| {
      machine.cpu.regs.pc = 0x02;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 12);
  assert_eq!(machine.cpu.regs.pc, 0x01);
}

#[test]
fn test_20_nojump() {
  let machine = run_test(
    &[0x20, 0x01, 0xed, 0x00], // JR NZ, e
    |machine| {
      machine.cpu.regs.f = ZERO;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 8);
  assert_eq!(machine.cpu.regs.pc, 0x02);
}

#[test]
fn test_22() {
  let machine = run_test(
    &[0x22, 0xed, 0x00], // LDI (HL), A
    |machine| {
      machine.cpu.regs.a = 0x42;
      machine.cpu.regs.h = 0x00;
      machine.cpu.regs.l = 0x02;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 8);
  assert_eq!(machine.cpu.regs.h, 0x00);
  assert_eq!(machine.cpu.regs.l, 0x03);
  assert_eq!(machine.hardware.memory[0x02], 0x42);
}

#[test]
fn test_24() {
  let machine = run_test(
    &[0x24], // INC H
    |machine| {
      machine.cpu.regs.h = 0x42;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.h, 0x43);
  assert_eq!(machine.cpu.regs.f, Flags::empty());
}

#[test]
fn test_24_zero() {
  let machine = run_test(
    &[0x24], // INC H
    |machine| {
      machine.cpu.regs.h = 0xff;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.h, 0x00);
  assert_eq!(machine.cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_24_half_carry() {
  let machine = run_test(
    &[0x24], // INC H
    |machine| {
      machine.cpu.regs.h = 0x0f;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.h, 0x10);
  assert_eq!(machine.cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_25() {
  let machine = run_test(
    &[0x25], // DEC H
    |machine| {
      machine.cpu.regs.h = 0x42;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.h, 0x41);
  assert_eq!(machine.cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_25_zero() {
  let machine = run_test(
    &[0x25], // DEC H
    |machine| {
      machine.cpu.regs.h = 0x01;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.h, 0x00);
  assert_eq!(machine.cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_25_half_carry() {
  let machine = run_test(
    &[0x25], // DEC H
    |machine| {
      machine.cpu.regs.h = 0x00;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.h, 0xff);
  assert_eq!(machine.cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_26() {
  let machine = run_test(
    &[0x26, 0x42], // LD H, n
    |_| {}
  );
  assert_eq!(machine.hardware.clock_cycles(), 8);
  assert_eq!(machine.cpu.regs.h, 0x42);
}

#[test]
fn test_28() {
  let machine = run_test(
    &[0x28, 0x01, 0xed, 0xed], // JR Z, e
    |machine| {
      machine.cpu.regs.f = ZERO;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 12);
  assert_eq!(machine.cpu.regs.pc, 0x03);
}

#[test]
fn test_28_negative() {
  let machine = run_test(
    &[0x00, 0xed, 0x28, -3i8 as u8], // JR Z, e
    |machine| {
      machine.cpu.regs.f = ZERO;
      machine.cpu.regs.pc = 0x02;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 12);
  assert_eq!(machine.cpu.regs.pc, 0x01);
}

#[test]
fn test_28_nojump() {
  let machine = run_test(
    &[0x28, 0x01, 0xed, 0x00], // JR Z, e
    |_| {}
  );
  assert_eq!(machine.hardware.clock_cycles(), 8);
  assert_eq!(machine.cpu.regs.pc, 0x02);
}

#[test]
fn test_2a() {
  let machine = run_test(
    &[0x2a, 0xed, 0x42], // LD A, (HL+)
    |machine| {
      machine.cpu.regs.h = 0x00;
      machine.cpu.regs.l = 0x02;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 8);
  assert_eq!(machine.cpu.regs.a, 0x42);
  assert_eq!(machine.cpu.regs.h, 0x00);
  assert_eq!(machine.cpu.regs.l, 0x03);
}

#[test]
fn test_2c() {
  let machine = run_test(
    &[0x2c], // INC L
    |machine| {
      machine.cpu.regs.l = 0x42;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.l, 0x43);
  assert_eq!(machine.cpu.regs.f, Flags::empty());
}

#[test]
fn test_2c_zero() {
  let machine = run_test(
    &[0x2c], // INC L
    |machine| {
      machine.cpu.regs.l = 0xff;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.l, 0x00);
  assert_eq!(machine.cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_2c_half_carry() {
  let machine = run_test(
    &[0x2c], // INC L
    |machine| {
      machine.cpu.regs.l = 0x0f;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.l, 0x10);
  assert_eq!(machine.cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_2d() {
  let machine = run_test(
    &[0x2d], // DEC L
    |machine| {
      machine.cpu.regs.l = 0x42;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.l, 0x41);
  assert_eq!(machine.cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_2d_zero() {
  let machine = run_test(
    &[0x2d], // DEC L
    |machine| {
      machine.cpu.regs.l = 0x01;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.l, 0x00);
  assert_eq!(machine.cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_2d_half_carry() {
  let machine = run_test(
    &[0x2d], // DEC L
    |machine| {
      machine.cpu.regs.l = 0x00;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.l, 0xff);
  assert_eq!(machine.cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_2e() {
  let machine = run_test(
    &[0x2e, 0x42], // LD L, n
    |_| {}
  );
  assert_eq!(machine.hardware.clock_cycles(), 8);
  assert_eq!(machine.cpu.regs.l, 0x42);
}

#[test]
fn test_2f() {
  let machine = run_test(
    &[0x2f], // CPL
    |machine| {
      machine.cpu.regs.a = 0xaa;
    }
  );
  assert_eq!(machine.hardware.clock_cycles(), 4);
  assert_eq!(machine.cpu.regs.a, 0x55);
  assert_eq!(machine.cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}
