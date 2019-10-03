(def for-each (fn for-each (array, func) 
  (for-each-recur 0, ((scope_get vec "len") array), array, func)
))

(def for-each-recur (fn for-each-recur (index, len, array, func)
  (if (= index len)
    array
    (do
      (func ((scope_get vec "nth") array, index), index)
      (for-each-recur (+ index 1), len, array, func)
    )
  )
))

(println default_gc_allocator)

(for-each ["Hello, world!", :keyword], println)