(def-fn equals [a, b]
  (= a, b))
(def-fn equals-one [a]
  (equals a, 1))

(println "2 == 1", (equals-one 2))
(println "1 == 1", (equals-one 1))