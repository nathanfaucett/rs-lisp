(def-fn hello_b []
  (println "Hello, from " __filename)
)

(export hello_b)

(import hello_a "./module_a.lisp")

(hello_a)