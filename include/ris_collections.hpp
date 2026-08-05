#ifndef __RIS_COLLECTIONS_H__
#define __RIS_COLLECTIONS_H__

#include "ris_alloc.hpp"
#include "ris_iterator.hpp"
#include "ris_primitives.hpp"

template <typename T> class RisArrayIterator;

template <typename T> class RisArray {
public:
  virtual ~RisArray() = default;

  virtual T *data() = 0;
  virtual usize len() const = 0;
  virtual usize capacity() const = 0;

  virtual RisArrayIterator<T> iter() = 0;

  virtual T *get(usize index) = 0;
  virtual void push(T value) = 0;
};

template <typename T> class RisArrayIterator : public RisIterator<T> {
private:
  usize _current_index = USIZE_MAX;
  RisArray<T> *_array;

public:
  RisArrayIterator(RisArray<T> *array) : _array(array) {}

  bool move_next() override {
    if (_current_index == USIZE_MAX) {
      _current_index = 0;
      return true;
    }

    if (_current_index < _array->len() - 1) {
      _current_index += 1;
      return true;
    }

    return false;
  }

  T *current() override {
#ifndef NDEBUG
    if (_current_index < _array->len()) {
      // TODO: panic
    }
#endif

    return _array->get(_current_index);
  }
};

// template <typename T> class StackArray {};

template <typename T> class StaticArray : public RisArray<T> {
private:
  T *_data;
  usize _len;
  usize _capacity;

public:
  StaticArray(usize capacity) : _len(0), _capacity(capacity) {
    _data = ris_alloc<T>(capacity);
  }

  T *data() override { return _data; }
  usize len() const override { return _len; }
  usize capacity() const override { return _capacity; }

  RisArrayIterator<T> iter() override { return RisArrayIterator<T>(this); }

  T *get(usize index) override {
#ifndef NDEBUG
    if (index >= _len) {
      // TODO: panic
    }
#endif

    return &_data[index];
  }

  void push(T value) override {
#ifndef NDEBUG
    if (_len == _capacity) {
      // TODO: panic
    }
#endif

    _data[_len] = value;
    _len += 1;
  }
};

// template <typename T> class DynArray {};

#endif /* __RIS_COLLECTIONS_H__ */

/* END OF FILE */
