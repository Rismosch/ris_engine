#ifndef __RIS_COLLECTIONS_H__
#define __RIS_COLLECTIONS_H__

#include "ris_alloc.hpp"
#include "ris_primitives.hpp"

template <typename T> class RisArray {
public:
  T *data();
  usize len();
  usize capacity();

  T *get(usize index);
  void push(T value);
};

template <typename T> class StackArray {};

template <typename T> class StaticArray : public RisArray<T> {
private:
  T *_data;
  usize _len;
  usize _capacity;

public:
  StaticArray(usize capacity) : _len(0), _capacity(capacity) {
    _data = ris_alloc<T>(capacity);
  }

  T *data() { return _data; }
  usize len() { return _len; }
  usize capacity() { return _capacity; }

  T *get(usize index) {
#ifndef NDEBUG
    if (index >= _len) {
      // TODO: panic
    }
#endif

    return &_data[index];
  }

  void push(T value) {
#ifndef NDEBUG
    if (_len == _capacity) {
      // TODO: panic
    }
#endif

    _data[_len] = value;
    _len += 1;
  }
};

template <typename T> class DynArray {};

#endif /* __RIS_COLLECTIONS_H__ */

/* END OF FILE */
