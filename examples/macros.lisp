(def def-macro (macro def-macro (name params body)
  (expand def `name (macro `name `params `body))
)) 

(def-macro def-fn (name, params, body) 
  (expand def `name (fn `name `params `body))
)

(def-fn test (a, b) [a, b])
(def-fn ident (x) x)

(test 1, 2)
(ident 1)