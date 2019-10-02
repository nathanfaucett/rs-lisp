; def-macro defines a macro and puts it in the current scope
(def def-macro (macro def-macro (name params body)
  (expand def `name (macro `name `params `body)))) 

; defines def-fn which defines a function and puts it in the current scope
(def-macro def-fn (name, params, body) 
  (expand def `name (fn `name `params `body)))

(def-fn add (a, b) 
  (+ a, b))
(def-fn add-one (a) 
  (add a, 1))

(println default_gc_allocator)

(def x (add 2, 2))
(add-one x)