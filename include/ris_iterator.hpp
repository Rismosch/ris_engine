#ifndef __RIS_ITERATOR_H__
#define __RIS_ITERATOR_H__

#include "ris_macro.hpp"

template <typename T> class RisIterator {
public:
  virtual ~RisIterator() = default;

  virtual bool move_next() = 0;
  virtual T *current() = 0;

  // utility for RIS_FOREACH
  T *null() { return nullptr; }

  bool try_get_next(T **next) {
    if (move_next()) {
      *next = current();
      return true;
    }

    return false;
  }
};

#define RIS_FOREACH(__variable, __iter)                                        \
  __RIS_FOREACH_IMPL(__variable, __iter, RIS_UNIQUE_NAME())

#define __RIS_FOREACH_IMPL(__variable, __iter, __iter_name)                    \
  auto __iter_name = __iter;                                                   \
  auto __variable = __iter_name.null();                                        \
  while (__iter_name.try_get_next(&__variable))

#endif /* __RIS_ITERATOR_H__ */

/* END OF FILE */
