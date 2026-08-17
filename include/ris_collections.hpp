#ifndef __RIS_COLLECTIONS_H__
#define __RIS_COLLECTIONS_H__

#include "ris_assert.hpp"
#include "ris_iterator.hpp"
#include "ris_memory.hpp"
#include "ris_primitives.hpp"

#define RIS_DEFAULT_DYN_ARRAY_CAPACITY 4

// Interface ===================================================================
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

// Iterator ====================================================================
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

    if (_current_index < _array->len()) {
      _current_index += 1;
      return _current_index < _array->len();
    }

    return false;
  }

  T *current() override {
    RIS_ASSERT(_current_index < _array->len());
    return _array->get(_current_index);
  }
};

// StackArray ==================================================================
template <typename T, usize N> class StackArray : public RisArray<T> {
private:
  T _data[N];
  usize _len;

public:
  explicit StackArray() : _len(0) {}

  ~StackArray() {
    for (auto it = iter(); it.move_next();) {
      auto x = it.current();
      x->~T();
    }
  }

  T *data() override { return _data; }
  usize len() const override { return _len; }
  usize capacity() const override { return N; }

  RisArrayIterator<T> iter() override { return RisArrayIterator<T>(this); }

  T *get(usize index) override {
    RIS_ASSERT(index < _len);
    return &_data[index];
  }

  void push(T value) override {
    RIS_ASSERT(_len < N);
    _data[_len] = value;
    _len += 1;
  }
};

// HeapArray ===================================================================
template <typename T> class HeapArray : public RisArray<T> {
protected:
  T *_data;
  usize _len;
  usize _capacity;

public:
  explicit HeapArray(usize capacity) : _len(0), _capacity(capacity) {
    RIS_ASSERT(capacity != 0);
    _data = ris_alloc<T>(capacity);
  }

  HeapArray(const HeapArray &) = delete;
  HeapArray &operator=(const HeapArray &) = delete;
  HeapArray(HeapArray &&) noexcept = default;
  HeapArray &operator=(HeapArray &&) noexcept = default;

  ~HeapArray() {
    for (auto it = iter(); it.move_next();) {
      auto x = it.current();
      x->~T();
    }
    ris_free(_data);
  }

  T *data() override { return _data; }
  usize len() const override { return _len; }
  usize capacity() const override { return _capacity; }

  RisArrayIterator<T> iter() override { return RisArrayIterator<T>(this); }

  T *get(usize index) override {
    RIS_ASSERT(index < _len);
    return &_data[index];
  }

  void push(T value) override {
    RIS_ASSERT(_len < _capacity);
    _push(value);
  }

protected:
  void _push(T value) {
    _data[_len] = value;
    _len += 1;
  }
};

// DynArray ====================================================================
template <typename T> class DynArray : public HeapArray<T> {
public:
  explicit DynArray(usize capacity) : HeapArray<T>(capacity) {}
  explicit DynArray() : HeapArray<T>(RIS_DEFAULT_DYN_ARRAY_CAPACITY) {}

  void push(T value) override {
    if (HeapArray<T>::_len == HeapArray<T>::_capacity) {
      usize new_capacity = HeapArray<T>::_capacity * 2;
      T *new_data = ris_alloc<T>(new_capacity);
      ris_memcpy(new_data, HeapArray<T>::_data, HeapArray<T>::_len);
      ris_free(HeapArray<T>::_data);
      HeapArray<T>::_data = new_data;
      HeapArray<T>::_capacity = new_capacity;
    }

    HeapArray<T>::_push(value);
  }
};

#endif /* __RIS_COLLECTIONS_H__ */

/* END OF FILE */
