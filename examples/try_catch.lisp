(def-fn throw_error []
  (throw "Throw me!")
)

(def-fn call-me [] (throw_error))

(try 
  (call-me)
  (fn [error]
    (println error)
  )
)