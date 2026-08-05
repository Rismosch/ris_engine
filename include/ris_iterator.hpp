#ifndef __RIS_ITERATOR_H__
#define __RIS_ITERATOR_H__

template <typename T> class RisIterator {
public:
  virtual ~RisIterator() = default;

  virtual bool move_next() = 0;
  virtual T *current() = 0;
};

#endif /* __RIS_ITERATOR_H__ */

/* END OF FILE */
