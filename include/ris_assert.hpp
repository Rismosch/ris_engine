#ifndef __RIS_ASSERT_H__
#define __RIS_ASSERT_H__

#include <stdio.h>
#include <stdlib.h>

char __ris_last_exception_message[1024];

inline void __ris_abort() {
#ifdef RIS_ENABLE_UNIT_TESTS
  throw __ris_last_exception_message;
#else
  printf("%s\n", __ris_last_exception_message);
  // abort();
#endif
}

#define RIS_PANIC(__message) __ris_panic(__FILE__, __LINE__, __message)
inline void __ris_panic(const char *file, int line, const char *message) {
  sprintf(__ris_last_exception_message, "panic at %s:%i \"%s\"\n", file, line,
          message);
  __ris_abort();
}

#ifdef NDEBUG
#define RIS_ASSERT(__expression) ((void)0)
#else
#define RIS_ASSERT(__expression)                                               \
  ((__expression) ? (void)0                                                    \
                  : __ris_failed_assert(__FILE__, __LINE__, #__expression))
inline void __ris_failed_assert(const char *file, int line,
                                const char *expression) {
  sprintf(__ris_last_exception_message, "assert \"%s\" failed at %s:%i\n",
          expression, file, line);
  __ris_abort();
}
#endif /* NDEBUG */

#endif /* __RIS_ASSERT_H__ */

/* END OF FILE */
