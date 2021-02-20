; def-macro defines a macro and puts it in the current scope
(def def-macro (macro def-macro [name params body]
  (expand def `name (macro `name `params `body)))) 

; defines def-fn which defines a function and puts it in the current scope
(def-macro def-fn [name, params, body] 
  (expand def `name (fn `name `params `body)))

(def-fn for-each-recur [index, len, array, func]
  (if (= index, len)
    array
    (do
      (func (vector.get array, index), index)
      (for-each-recur (usize.add index 1_usize), len, array, func)
    )
  )
)

(def-fn for-each [array, func] 
  (for-each-recur 0_usize, (vector.len array), array, func)
)