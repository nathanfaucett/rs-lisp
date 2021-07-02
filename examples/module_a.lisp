(def-fn hello_a []
  (println "Hello, from " __filename)
)

(export hello_a)

(import hello_b "./module_b")

(hello_b)