(import def-fn "./lib")

(def NONE_VALUE {})

(def-fn option.some (value) 
  { :value value })

(def-fn option.none () 
  { :value NONE_VALUE })

(def-fn option.from (value) 
  (if (= value nil)
    (option.some value)
    (option.none)))

(def-fn option.is_some (option)
  (!= (map.get option, :value) NONE_VALUE))

(def-fn option.is_none (option)
  (! (option.is_some option))

(def-fn option.expect (option, error)
  (if (is_some option)
    (map.get option, :value)
    (panic error)))

(def-fn option.unwrap (option)
  (option.expect option, "tried to unwrap none option"))

(export option.some, option.none, option.from, option.is_some, option.is_none, option.unwrap)