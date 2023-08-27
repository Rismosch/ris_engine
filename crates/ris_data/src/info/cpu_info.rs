use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CpuInfo {
    pub cpu_cache_line_size: i32,
    pub cpu_count: i32,
    pub has_3d_now: bool,
    pub has_alti_vec: bool,
    pub has_avx: bool,
    pub has_avx2: bool,
    pub has_avx512f: bool,
    pub has_mmx: bool,
    pub has_rdtsc: bool,
    pub has_sse: bool,
    pub has_sse2: bool,
    pub has_sse3: bool,
    pub has_sse41: bool,
    pub has_sse42: bool,
    pub system_ram: i32,
}

impl CpuInfo {
    pub fn new() -> CpuInfo {
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
}

impl Default for CpuInfo {
    fn default() -> Self {
        CpuInfo::new()
    }
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "CPU")?;
        writeln!(f, "cpu_cache_line_size: {}", self.cpu_cache_line_size)?;
        writeln!(f, "cpu_count:           {}", self.cpu_count)?;
        writeln!(f, "system_ram:          {}", self.system_ram)?;
        writeln!(f, "has_3d_now:          {}", self.has_3d_now)?;
        writeln!(f, "has_alti_vec:        {}", self.has_alti_vec)?;
        writeln!(f, "has_avx:             {}", self.has_avx)?;
        writeln!(f, "has_avx2:            {}", self.has_avx2)?;
        writeln!(f, "has_avx512f:         {}", self.has_avx512f)?;
        writeln!(f, "has_mmx:             {}", self.has_mmx)?;
        writeln!(f, "has_rdtsc:           {}", self.has_rdtsc)?;
        writeln!(f, "has_sse:             {}", self.has_sse)?;
        writeln!(f, "has_sse2:            {}", self.has_sse2)?;
        writeln!(f, "has_sse3:            {}", self.has_sse3)?;
        writeln!(f, "has_sse41:           {}", self.has_sse41)?;
        writeln!(f, "has_sse42:           {}", self.has_sse42)?;

        Ok(())
    }
}
