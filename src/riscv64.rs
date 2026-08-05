include!(concat!(env!("OUT_DIR"), "/riscv64_consts.rs"));

pub const fn floating_point_registers() -> usize {
    if FLOAT_ABI_DOUBLE {
        12
    } else if FLOAT_ABI_SOFT {
        0
    } else {
        panic!("unsupported RISC-V float ABI");
    }
}
