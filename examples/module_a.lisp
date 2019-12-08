(def hello_a (fn hello () 
  (println "Hello, from " __filename)
))

(export hello_a)

(import hello_b "./module_b")

(hello_b)