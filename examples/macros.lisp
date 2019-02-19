(def def-macro (macro def-macro (name params body)
  (expand def `name (macro `name `params `body)))) 

(def-macro def-fn (name, params, body) 
  (expand def `name (fn `name `params `body)))

(def-fn add (a, b) 
  (+ a, b))
(def-fn add-one (a) 
  (add a, 1))

(def x (add 2, 2))
(add-one x)