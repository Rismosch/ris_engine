#ifndef __RIS_ALLOC_H__
#define __RIS_ALLOC_H__

#include <stdlib.h>

#include "ris_primitives.hpp"

inline void *ris_raw_alloc(usize size) { return malloc(size); }

template <typename T> inline T *ris_alloc(usize count) {
  void *ptr = ris_raw_alloc(count * sizeof(T));
  return static_cast<T *>(ptr);
}

template <typename T> inline void ris_free(T *ptr) { free(ptr); }

#endif /* __RIS_ALLOC_H__ */

/* END OF FILE */
