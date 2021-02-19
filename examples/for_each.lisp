(def for-each-recur (fn for-each-recur [index, len, array, func]
  (if (= index, len)
    array
    (do
      (func (vector.get array, index), index)
      (for-each-recur (usize.add index 1_usize), len, array, func)
    )
  )
))

(def for-each (fn for-each [array, func] 
  (for-each-recur 0_usize, (vector.len array), array, func)
))

(println (for-each ["Hello, world!", :keyword], println))