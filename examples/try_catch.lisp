(def throw_error (fn throw_error []
  (throw "Throw me!")
))

(try 
  (throw_error)
  (fn [error]
    (println error)
  )
)