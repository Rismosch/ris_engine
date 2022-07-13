use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CpuInfo {
    cpu_cache_line_size: i32,
    cpu_count: i32,
    has_3d_now: bool,
    has_alti_vec: bool,
    has_avx: bool,
    has_avx2: bool,
    has_avx512f: bool,
    has_mmx: bool,
    has_rdtsc: bool,
    has_sse: bool,
    has_sse2: bool,
    has_sse3: bool,
    has_sse41: bool,
    has_sse42: bool,
    system_ram: i32,
}

pub fn cpu_info() -> CpuInfo {
    CpuInfo {
        cpu_cache_line_size: sdl2::cpuinfo::cpu_cache_line_size(),
        cpu_count: sdl2::cpuinfo::cpu_count(),
        has_3d_now: sdl2::cpuinfo::has_3d_now(),
        has_alti_vec: sdl2::cpuinfo::has_alti_vec(),
        has_avx: sdl2::cpuinfo::has_avx(),
        has_avx2: sdl2::cpuinfo::has_avx2(),
        has_avx512f: sdl2::cpuinfo::has_avx512f(),
        has_mmx: sdl2::cpuinfo::has_mmx(),
        has_rdtsc: sdl2::cpuinfo::has_rdtsc(),
        has_sse: sdl2::cpuinfo::has_sse(),
        has_sse2: sdl2::cpuinfo::has_sse2(),
        has_sse3: sdl2::cpuinfo::has_sse3(),
        has_sse41: sdl2::cpuinfo::has_sse41(),
        has_sse42: sdl2::cpuinfo::has_sse42(),
        system_ram: sdl2::cpuinfo::system_ram(),
    }
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "CPU")?;
        writeln!(f, "Cache Line Size: {}", self.cpu_cache_line_size)?;
        writeln!(f, "Count:        {}", self.cpu_count)?;
        writeln!(f, "System RAM:   {}", self.system_ram)?;
        writeln!(f, "has 3d now:   {}", self.has_3d_now)?;
        writeln!(f, "has alti vec: {}", self.has_alti_vec)?;
        writeln!(f, "has avx:      {}", self.has_avx)?;
        writeln!(f, "has avx2:     {}", self.has_avx2)?;
        writeln!(f, "has avx512f:  {}", self.has_avx512f)?;
        writeln!(f, "has mmx:      {}", self.has_mmx)?;
        writeln!(f, "has rdtsc:    {}", self.has_rdtsc)?;
        writeln!(f, "has sse:      {}", self.has_sse)?;
        writeln!(f, "has sse2:     {}", self.has_sse2)?;
        writeln!(f, "has sse3:     {}", self.has_sse3)?;
        writeln!(f, "has sse41:    {}", self.has_sse41)?;
        writeln!(f, "has sse42:    {}", self.has_sse42)?;

        Ok(())
    }
}
