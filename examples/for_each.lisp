(def for-each (fn (vec, func) 
  (for-each-recur 0, ((kind-get :len Vec) vec), vec, func)
))

(def for-each-recur (fn (index, len, vec, func)
  (if (= index len)
    vec
    (do
      (func ((kind-get :nth Vec) vec, index), index)
      (for-each-recur (+ index 1), len, vec, func)
    )
  )
))

(for-each ["Hello, world!", :keyword], println)