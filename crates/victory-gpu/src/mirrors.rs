//! GPU views of the title's CPU memory, and the CPU view behind each.
//!
//! A retail game reserves its 64 KiB-page memory twice in a row: once
//! readable by the CPU (`PAGE_READWRITE`), once with `PAGE_NOACCESS |
//! PAGE_GRAPHICS_NOACCESS`. It maps physical pages only into the first,
//! yet places GPU resources at the same offset in the second (its Bink
//! video planes: the decoder writes the first view, the video shader
//! samples textures in the second). On the console the second reservation
//! is a GPU-only view of the first; here it is separate, empty memory, so
//! the video read as its colour of zero luma and chroma: a black screen.
//!
//! `source` names the CPU-view address behind an address of such a GPU
//! view, which the texture staging reads and watches instead (the same
//! pairing as xcl's `kernelx::mirrors`).

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

/// The console's 64 KiB pages: `MEM_LARGE_PAGES | MEM_PHYSICAL`.
const PAGES_64K: u32 = 0x2040_0000;
/// `PAGE_GRAPHICS_NOACCESS`: the GPU-only access bit.
const GRAPHICS_NOACCESS: u32 = 0x800;
/// The host's `PAGE_NOACCESS`.
const PAGE_NOACCESS: u32 = 0x01;

struct State {
    /// CPU-readable reservations of 64 KiB pages, oldest first: (base, size).
    cpu: Vec<(usize, usize)>,
    /// GPU view base → (size, CPU view base).
    gpu: BTreeMap<usize, (usize, usize)>,
}

static STATE: Mutex<State> = Mutex::new(State {
    cpu: Vec::new(),
    gpu: BTreeMap::new(),
});

/// Bumped whenever a mapping changes inside a GPU view, so texture staging
/// can re-ask where the bytes are ([`source`]).
static GENERATION: AtomicU64 = AtomicU64::new(0);

/// A reservation of `size` bytes at `base` with allocation type `t` and
/// protection `p` (both raw, before the host mask).
pub fn reserved(base: usize, size: usize, t: u32, p: u32) {
    if t & PAGES_64K != PAGES_64K || t & 0x1000 != 0 {
        return;
    }
    let mut s = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if p & GRAPHICS_NOACCESS != 0 && p & 0xFF == PAGE_NOACCESS {
        // The most recent CPU view of the same size without a GPU view yet.
        let taken: Vec<usize> = s.gpu.values().map(|&(_, c)| c).collect();
        if let Some(&(cpu, _)) = s
            .cpu
            .iter()
            .rev()
            .find(|&&(b, n)| n == size && !taken.contains(&b))
        {
            s.gpu.insert(base, (size, cpu));
            tracing::info!(
                gpu_view = format_args!("{base:#x}"),
                cpu_view = format_args!("{cpu:#x}"),
                size,
                "a no-CPU-access reservation is the GPU view of a CPU view"
            );
        }
    } else if p & 0xFF != PAGE_NOACCESS {
        s.cpu.push((base, size));
    }
}

/// The reservation at `base` was released.
pub fn released(base: usize) {
    let mut s = STATE.lock().unwrap_or_else(|e| e.into_inner());
    s.cpu.retain(|&(b, _)| b != base);
    s.gpu.remove(&base);
    s.gpu.retain(|_, &mut (_, c)| c != base);
}

/// The CPU-view address behind `va` when `va` lies in a GPU view.
pub fn source(va: usize) -> Option<usize> {
    let s = STATE.lock().unwrap_or_else(|e| e.into_inner());
    let (&gpu, &(size, cpu)) = s.gpu.range(..=va).next_back()?;
    (va < gpu + size).then(|| cpu + (va - gpu))
}

/// The runtime mapped or unmapped `[at, at + len)` of title memory: any
/// GPU view that range touches may read from another address now.
pub fn remapped(at: usize, len: usize) {
    let end = at.saturating_add(len);
    let s = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if s.gpu
        .range(..end)
        .next_back()
        .is_some_and(|(&base, &(size, _))| base < end && at < base + size)
    {
        GENERATION.fetch_add(1, Ordering::AcqRel);
    }
}

/// Bumped by [`remapped`]: texture staging compares it to learn whether the
/// source of a texture's bytes may have changed.
pub fn generation() -> u64 {
    GENERATION.load(Ordering::Acquire)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// As the game does: a CPU view of 64 KiB pages, then a GPU view of the
    /// same size with `PAGE_NOACCESS | PAGE_GRAPHICS_NOACCESS`. Sizes differ
    /// between tests: the state is shared, and pairing matches by size.
    #[test]
    fn pairs_a_gpu_view_with_the_latest_same_size_cpu_view() {
        const SIZE: usize = 1 << 20;
        const CPU: usize = 0x1_0000_0000;
        const GPU: usize = 0x8_0000_0000;
        reserved(CPU, SIZE, PAGES_64K, 0x04); // PAGE_READWRITE
        reserved(GPU, SIZE, PAGES_64K, PAGE_NOACCESS | GRAPHICS_NOACCESS);
        assert_eq!(source(GPU), Some(CPU));
        assert_eq!(source(GPU + SIZE / 2), Some(CPU + SIZE / 2));
        assert_eq!(source(GPU + SIZE), None);
        assert_eq!(source(CPU), None);
    }

    /// A CPU view already paired with another GPU view is not taken again.
    #[test]
    fn a_cpu_view_pairs_with_one_gpu_view_only() {
        const SIZE: usize = 2 << 20;
        const CPU: usize = 0x2_0000_0000;
        const A: usize = 0x9_0000_0000;
        const B: usize = 0xa_0000_0000;
        reserved(CPU, SIZE, PAGES_64K, 0x04);
        reserved(A, SIZE, PAGES_64K, PAGE_NOACCESS | GRAPHICS_NOACCESS);
        reserved(B, SIZE, PAGES_64K, PAGE_NOACCESS | GRAPHICS_NOACCESS);
        assert_eq!(source(A), Some(CPU));
        assert_eq!(source(B), None);
    }

    /// A remap inside a GPU view bumps the generation; one outside does not.
    #[test]
    fn a_remap_inside_a_gpu_view_bumps_the_generation() {
        const SIZE: usize = 3 << 20;
        const CPU: usize = 0x3_0000_0000;
        const GPU: usize = 0xb_0000_0000;
        reserved(CPU, SIZE, PAGES_64K, 0x04);
        reserved(GPU, SIZE, PAGES_64K, PAGE_NOACCESS | GRAPHICS_NOACCESS);
        let before = generation();
        remapped(0x1000, 0x1000);
        assert_eq!(generation(), before);
        remapped(GPU, 0x1000);
        assert!(generation() > before);
    }
}
