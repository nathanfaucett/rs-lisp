(def for-each (fn (vec, func) 
  (for-each-recur 0, ((get-kind-data Vec :len) vec), vec, func)
))

(def for-each-recur (fn (index, len, vec, func)
  (if (= index len)
    vec
    (do
      (func ((get-kind-data Vec :nth) vec, index), index)
      (for-each-recur (+ index 1), len, vec, func)
    )
  )
))

(println gc_allocator)

(for-each ["Hello, world!", :keyword], println)