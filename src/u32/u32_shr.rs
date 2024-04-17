#![allow(dead_code)]

use bitcoin::opcodes::all::{OP_2DROP, OP_DROP, OP_DUP, OP_ELSE, OP_FROMALTSTACK, OP_TOALTSTACK};

use crate::treepp::{pushable, script, Script};
use core::panic;
use crate::u32::{
    u32_and::*,
    u32_rrot::*,
    u32_std::*,
};

/// Right shift of an u32 element by 16 bits
pub fn u32_shr16() -> Script {
    script! {
      OP_2SWAP
      OP_2DROP
      0
      0
    }
}


/// Right shift of an u32 element by 8 bits
pub fn u32_shr8() -> Script {
    script! {
      3 OP_ROLL
      OP_DROP
      0
    }
}

/// Right shift of an u32 element by 24 bits
pub fn u32_shr24() -> Script {
    script! {
      OP_TOALTSTACK
      OP_2DROP
      OP_DROP
      OP_FROMALTSTACK
    }
}


/// Expects the u8_xor_table to be on the stack
pub fn u32_shr(shr_num :usize, ap: u32) -> Script{
    assert!(shr_num < 32);
    match sepcial_case(shr_num) {
        Some(res) => return res,
        None => {}
    }
    let remainder: usize = shr_num % 8;
    let offset: usize = (shr_num - remainder) / 8;
    let mut b: Vec<u8> = vec![0_u8; (offset) as usize];
    if b.len() < 4 {
        b.append(&mut [(0xff >> remainder) as u8].to_vec());
    }
    for i in 0..(4-b.len())
    {
        b.append(&mut [0xff].to_vec())
    }
    //b: b0 b1 b2 b3
    let script = script!(
        {u32_rrot(shr_num)}
        // stack: a0 a1 a2 a3
        if b[3] == 0 {
            OP_DROP
        } else {
            {b[3]}
            // stack: a0 a1 a2 a3 b3
            {u8_and(5 + (ap - 2) * 4)}
            OP_TOALTSTACK
        }
        if b[2] == 0 {
            OP_DROP
        } else {
            {b[2]}
            // stack: a0 a1 a2 b2
            // alt: r3
            {u8_and(4 + (ap - 2) * 4)}
            OP_TOALTSTACK
        }
        if b[1] == 0 {
            OP_DROP
        } else {
            {b[1]}
            // stack: a0 a1 b1
            // alt: r3 r2
            {u8_and(3 + (ap - 2) * 4)}
            OP_TOALTSTACK
        }
        if b[0] == 0 {
            OP_DROP
            0
        } else {
            {b[0]}
            // stack: a0 b0
            // alt: r3 r2 r1
            {u8_and(2 + (ap - 2) * 4)}
        }
        // stack: r0
        if b[1] == 0 {
            0
        } else {
            OP_FROMALTSTACK
        }
        // stack: r0 r1
        if b[2] == 0 {
            0
        } else {
            OP_FROMALTSTACK
        }
        // stack: r0 r1 r2
        if b[3] == 0 {
            0
        } else {
            OP_FROMALTSTACK
        }
    );
    script
}

pub fn sepcial_case(rot_num: usize) -> Option<Script> {
    let res: Option<Script> = match rot_num {
        0 => script! {}.into(),           
        8 => script! {u32_shr8}.into(),   
        16 => script! {u32_shr16}.into(), 
        24 => script! {u32_shr24}.into(),  
        _ => None,
    };
    res
}
/// Expects the u8_xor_table to be on the stack
pub fn u32_shr_debug(shr_num :usize, ap: u32) -> Script{
    assert!(shr_num < 32);
    match sepcial_case(shr_num) {
        Some(res) => return res,
        None => {}
    }
    let remainder: usize = shr_num % 8;
    let offset: usize = (shr_num - remainder) / 8;
    let mut b: Vec<u8> = vec![0_u8; (offset) as usize];
    if b.len() < 4 {
        b.append(&mut [(0xff >> remainder) as u8].to_vec());
    }
    for i in 0..(4-b.len())
    {
        b.append(&mut [0xff].to_vec())
    }
    //b: b0 b1 b2 b3
    let script = script!(
        {u32_rrot(shr_num)}
        // stack: a0 a1 a2 a3
        if b[3] == 0 {
            OP_DROP
        } else {
            {b[3]}
            // stack: a0 a1 a2 a3 b3
            {u8_and(5 + (ap - 2) * 4)}
            OP_TOALTSTACK
        }
        if b[2] == 0 {
            OP_DROP
        } else {
            {b[2]}
            // stack: a0 a1 a2 b2
            // alt: r3
            {u8_and(4 + (ap - 2) * 4)}
            OP_TOALTSTACK
        }
        if b[1] == 0 {
            OP_DROP
        } else {
            {b[1]}
            // stack: a0 a1 b1
            // alt: r3 r2
            {u8_and(3 + (ap - 2) * 4)}
            OP_TOALTSTACK
        }
        if b[0] == 0 {
            OP_DROP
            0
        } else {
            {b[0]}
            // stack: a0 b0
            // alt: r3 r2 r1
            {u8_and(2 + (ap - 2) * 4)}
        }
        // stack: r0
        if b[1] == 0 {
            0
        } else {
            OP_FROMALTSTACK
        }
        // stack: r0 r1
        if b[2] == 0 {
            0
        } else {
            OP_FROMALTSTACK
        }
        // stack: r0 r1 r2
        if b[3] == 0 {
            0
        } else {
            OP_FROMALTSTACK
        }
    );
    script
}