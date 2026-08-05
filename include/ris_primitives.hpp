#ifndef __RIS_PRIMITIVES_H__
#define __RIS_PRIMITIVES_H__

#include <gctypes.h>

// Definitions ======================================================
using u8 = u8;
using u16 = u16;
using u32 = u32;
using u64 = u64;
using i8 = s8;
using i16 = s16;
using i32 = s32;
using i64 = s64;

using usize = size_t;
using isize = ssize_t;

using f32 = f32;
using f64 = f64;

// Constants ========================================================
#define USIZE_MIN 0
#define USIZE_MAX ~static_cast<usize>(0)

#endif /* __RIS_PRIMITIVES_H__ */

/* END OF FILE */
