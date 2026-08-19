#ifndef __RIS_ASSERT_H__
#define __RIS_ASSERT_H__

#include <stdio.h>
#include <stdlib.h>

#define RIS_ABORT_MESSAGE_SIZE 2048

struct RisException {
  char message[RIS_ABORT_MESSAGE_SIZE];
};

inline void __ris_abort(RisException exception) {
#ifdef RIS_ENABLE_UNIT_TESTS
  throw exception;
#else
  printf("%s\n", exception.message);
  // abort();
#endif
}

#define RIS_PANIC(__message) __ris_panic(__FILE__, __LINE__, __message)
inline void __ris_panic(const char *file, int line, const char *message) {
  RisException exception;
  sprintf(exception.message, "panic at %s:%i \"%s\"\n", file, line, message);
  __ris_abort(exception);
}

#ifdef NDEBUG
#define RIS_ASSERT(__expression) ((void)0)
#else
#define RIS_ASSERT(__expression)                                               \
  ((__expression) ? (void)0                                                    \
                  : __ris_failed_assert(__FILE__, __LINE__, #__expression))
inline void __ris_failed_assert(const char *file, int line,
                                const char *expression) {
  RisException exception;
  sprintf(exception.message, "assert `%s` failed at %s:%i\n", expression, file,
          line);
  __ris_abort(exception);
}
#endif /* NDEBUG */

#endif /* __RIS_ASSERT_H__ */

/* END OF FILE */
