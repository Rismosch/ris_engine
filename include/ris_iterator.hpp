#ifndef __RIS_ITERATOR_H__
#define __RIS_ITERATOR_H__

#include "ris_macro.hpp"

template <typename T> class RisIterator {
public:
  virtual ~RisIterator() = default;

  virtual bool move_next() = 0;
  virtual T *current() = 0;

  bool try_get_next(T **next) {
    if (move_next()) {
      *next = current();
      return true;
    }

    return false;
  }
};

#define RIS_FOREACH(__type, __variable, __collection)                          \
  __RIS_FOREACH_IMPL(__type, __variable, __collection, RIS_UNIQUE())
#define __RIS_FOREACH_IMPL(__type, __variable, __collection, __iterator)       \
  __type *__variable;                                                          \
  for (auto __iterator = __collection.iter();                                  \
       __iterator.try_get_next(&__variable);)

#endif /* __RIS_ITERATOR_H__ */

/* END OF FILE */
