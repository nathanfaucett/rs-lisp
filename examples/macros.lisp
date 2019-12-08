; def-macro defines a macro and puts it in the current scope
(def def-macro (macro def-macro (name params body)
  (expand def `name (macro `name `params `body)))) 

; defines def-fn which defines a function and puts it in the current scope
(def-macro def-fn (name, params, body) 
  (expand def `name (fn `name `params `body)))

(def-fn equals (a, b) 
  (= a, b))
(def-fn equals-one (a) 
  (equals a, 1))

(println "2 == 1", (equals-one 2))
(println "1 == 1", (equals-one 1))