#ifndef __RIS_ALLOC_H__
#define __RIS_ALLOC_H__

#include <stdlib.h>

struct RisAllocationArgs {
  size_t size;
  size_t align;
  const char *file;
  uint32_t line;
};

struct RisAllocation {
  void *ptr;
  RisAllocationArgs args;
};

#define RIS_ALLOC(__size, __align)                                             \
  ris_alloc(RisAllocationArgs{                                                 \
      .size = __size,                                                          \
      .align = __align,                                                        \
      .file = __FILE__,                                                        \
      .line = __LINE__,                                                        \
  });

#define RIS_FREE(__ptr) ris_free(__ptr);

void *ris_untracked_alloc(size_t size, size_t align);
void ris_untracked_free(void *ptr);

void *ris_alloc(RisAllocationArgs args);
void ris_free(void *ptr);

#endif /* __RIS_ALLOC_H__ */

/* END OF FILE */
