extern crate core_affinity;

#[cfg(target_arch = "aarch64")]
pub fn current_cycle() -> usize {
    let r;
    unsafe {
        core::arch::asm!("mrs {}, pmccntr_el0", out(reg) r);
    }
    r
}

#[cfg(target_arch = "x86_64")]
pub fn current_cycle() -> usize {
    unsafe {
        core::arch::x86_64::_mm_lfence();
        let value = core::arch::x86_64::_rdtsc();
        core::arch::x86_64::_mm_lfence();
        value as usize
    }
}

pub fn pinned_on_core(idx: usize) {
    let core_ids = core_affinity::get_core_ids().unwrap();
    // println!("get core ids {:?}", core_ids);

    let idx = if idx >= core_ids.len() {
        println!(
            "[warn] pinned core {idx} exceeds the range of PC core number, set to 0 by default"
        );
        0
    } else {
        idx
    };

    let pin_core = core_ids[idx];
    println!("Thread Pinned on core {:?}", pin_core);
    let res = core_affinity::set_for_current(pin_core);
    if !res {
        println!("failed to set core_affinity on {:?}", pin_core);
        return;
    }
}
